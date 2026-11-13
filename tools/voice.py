#!/usr/bin/env python3
"""Record the names: each thing's name in a language Ruya has a voice for,
in each form the language has, said by one of Edge's neural voices, as the
MP3 the app plays when the thing is tapped with the speaker on.

    python3 tools/voice.py en          # what is missing or out of date
    python3 tools/voice.py en --all    # every recording again
    python3 tools/voice.py en --dry    # say what would be recorded
    python3 tools/voice.py en kitchen  # one scene's, or a few

For every scene file and each thing in it, its name for one and for many
(and for two, in a language with a dual) is said with a full stop after it:
a statement's falling tone, and some voices give no sound for a bare word.
Edge's silence is trimmed from both ends, so a tap is answered at once and
the plural follows its singular closely, and ffmpeg writes
`public/scenes/<scene>/voice/<lang>/<thing>.<number>.mp3`, mono, 24 kHz,
48 kbps. A word said the same in two scenes is recorded once and copied.

What each recording says is logged in `art/<scene>/voice.<lang>.json`, by
the name it was made for, so `cargo test` catches a name changed since its
recording; running this again redoes just that one. A new voice redoes
them all.

Edge gives a different recording of the same text every time, so a
recording is checked by listening, not by its bytes or its length. Needs
edge-tts (pip install edge-tts) and ffmpeg (brew install ffmpeg).
"""
import argparse
import json
import shutil
import subprocess
import sys
import tempfile
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

# the voice per language, picked by ear
VOICES = {
    "en": "en-US-AvaNeural",
    "de": "de-DE-KatjaNeural",
    "ar": "ar-SA-HamedNeural",
}

# what to say instead, where a voice reads the written name wrong
SAY = {
    "en": {},
    "de": {},
    "ar": {},
}

# the numbers a language has, as `Lang::numbers` in src/lang.rs, by the code
# that names its recording and the scene file's field for it
NUMBERS = {
    "en": ["one", "many"],
    "de": ["one", "many"],
    "ar": ["one", "two", "many"],
}
FIELDS = {"one": "name", "two": "dual", "many": "plural"}

# Edge's silence off both ends, a breath of it kept: 40 ms before the word,
# 120 ms after it
TRIM = (
    "silenceremove=start_periods=1:start_threshold=-50dB:start_silence=0.04,"
    "areverse,"
    "silenceremove=start_periods=1:start_threshold=-50dB:start_silence=0.12,"
    "areverse"
)


def record(text, voice, out):
    """`text` said by `voice`, trimmed, as an MP3 at `out`."""
    with tempfile.TemporaryDirectory() as tmp:
        raw = Path(tmp) / "raw.mp3"
        # Edge now and then sends no sound at all; it comes on a later try
        for attempt in range(5):
            said = subprocess.run(
                [sys.executable, "-m", "edge_tts", "-v", voice, "-t", f"{text}.",
                 "--write-media", str(raw)],
                capture_output=True,
            )
            if said.returncode == 0 and raw.exists() and raw.stat().st_size > 2000:
                break
            time.sleep(1 + attempt)
        else:
            sys.exit(f"{voice} gave no sound for '{text}'")
        subprocess.run(
            ["ffmpeg", "-y", "-loglevel", "error", "-i", str(raw), "-af", TRIM,
             "-ac", "1", "-ar", "24000", "-c:a", "libmp3lame", "-b:a", "48k",
             "-map_metadata", "-1", "-id3v2_version", "0", str(out)],
            check=True,
        )


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("lang", choices=sorted(VOICES))
    parser.add_argument("scenes", nargs="*", help="scene ids; every scene if none")
    parser.add_argument("--all", action="store_true", help="record every name again")
    parser.add_argument("--dry", action="store_true", help="say what would be recorded")
    args = parser.parse_args()
    lang, voice = args.lang, VOICES[args.lang]

    made = {}  # what was said this run, by its recording
    count = 0
    paths = sorted((ROOT / "scenes").glob("*.json"))
    if args.scenes:
        unknown = set(args.scenes) - {p.stem for p in paths}
        if unknown:
            sys.exit(f"no scene {', '.join(sorted(unknown))}")
        paths = [p for p in paths if p.stem in args.scenes]
    for path in paths:
        scene = json.loads(path.read_text())
        sid = scene["id"]
        out = ROOT / "public" / "scenes" / sid / "voice" / lang
        log_path = ROOT / "art" / sid / f"voice.{lang}.json"
        log = json.loads(log_path.read_text()) if log_path.exists() else {"voice": voice, "said": {}}
        again = args.all or log["voice"] != voice

        said = {}
        for tid, thing in sorted(scene["things"].items()):
            for number in NUMBERS[lang]:
                name = thing[FIELDS[number]][lang]
                key = f"{tid}.{number}"
                said[key] = name
                clip = out / f"{key}.mp3"
                if not again and clip.exists() and log["said"].get(key) == name:
                    continue
                text = SAY[lang].get(name, name)
                print(f"  {sid}/{key}: {text}")
                count += 1
                if args.dry:
                    continue
                out.mkdir(parents=True, exist_ok=True)
                if text in made:
                    shutil.copyfile(made[text], clip)
                else:
                    record(text, voice, clip)
                    made[text] = clip

        # a thing taken out, or renamed, leaves no recording behind
        for stale in sorted(out.glob("*.mp3")) if out.exists() else []:
            if stale.stem not in said:
                print(f"  {sid}/{stale.name}: no name's, removed")
                if not args.dry:
                    stale.unlink()
        if not args.dry:
            log_path.parent.mkdir(parents=True, exist_ok=True)
            log = {"voice": voice, "said": said}
            log_path.write_text(json.dumps(log, ensure_ascii=False, indent="\t") + "\n")

    print(f"{lang}, {voice}: {count} recording{'s' * (count != 1)} "
          f"{'to make' if args.dry else 'made'}")


if __name__ == "__main__":
    main()
