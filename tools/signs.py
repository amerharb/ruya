#!/usr/bin/env python3
"""Make the sign patches: a sign painted in a scene's picture, in every
language but the one it is painted in, as a small WebP the app lays over
the picture at the sign's place.

    python3 tools/signs.py scenes/street.json

For each style's sign in the scene file (its `rect`, in picture units) and
each language but the one the picture shows:

1. the rect is cut from the original, `art/<scene>/<style>.png`;
2. the painted letters (amber pixels) are taken out, filled from the panel
   around them;
3. the language's words are set where the painted ones stood, in their
   amber, as tall as they were and no wider than the rect — squeezed a
   little if need be, never stretched — with a glow and a softness that
   match the picture, from `art/<scene>/signs.json`;
4. rsvg-convert renders it, Pango and HarfBuzz shaping Arabic, Hebrew and
   Japanese, and cwebp writes `public/scenes/<scene>/<style>.<sign>.<lang>.webp`
   at the picture's own pixel size, no sharper than the paint around it.

Options: --out DIR writes the patches there instead; --look FILE takes the
look from another file. Needs Pillow, rsvg-convert (brew install librsvg)
and cwebp (brew install webp).
"""
import argparse
import base64
import io
import json
import subprocess
import sys
import tempfile
from pathlib import Path
from xml.sax.saxutils import escape

from PIL import Image, ImageFilter

ROOT = Path(__file__).resolve().parent.parent

# a family per script, bold, as fontconfig knows them on macOS; Pango falls
# back from there if a glyph is missing
FONTS = {
    "ar": ("Geeza Pro", "bold", "normal"),
    "he": ("Arial Hebrew", "bold", "normal"),
    "ja": ("Hiragino Sans", "bold", "normal"),
}
LATIN = ("Helvetica Neue", "bold", "condensed")

# the letter whose height is made the painted capitals' height, and by how
# much: H for Latin and Cyrillic, alef for Arabic, he for Hebrew, a kanji for
# Japanese — so a descender or an accent does not shrink the rest
REFERENCE = {"ar": ("ا", 1.0), "he": ("ה", 0.95), "ja": ("中", 1.0)}
LATIN_REFERENCE = ("H", 1.0)

# how far a language's words may be squeezed sideways before they shrink
SQUEEZE = 0.65


def amber(img):
    """The painted letters: warm, saturated and bright pixels, as a mask."""
    hsv = img.convert("HSV")
    h, s, v = (hsv.getchannel(c).load() for c in "HSV")
    mask = Image.new("L", img.size)
    m = mask.load()
    for y in range(img.height):
        for x in range(img.width):
            # Pillow's hue runs 0-255: amber to yellow is about 10 to 45
            if 8 <= h[x, y] <= 48 and s[x, y] >= 90 and v[x, y] >= 80:
                m[x, y] = 255
    return mask


def erase(crop, letters, rounds=80):
    """Take the letters out, filling them from the panel around them: blur,
    keep the panel's own pixels, again, until the hole is smooth."""
    hole = letters.filter(ImageFilter.MaxFilter(5))
    filled = crop.copy()
    for _ in range(rounds):
        blurred = filled.filter(ImageFilter.BoxBlur(2))
        filled = Image.composite(blurred, crop, hole)
    return filled


def ink(crop, letters):
    """The letters' colour: the brightest half of their pixels, averaged."""
    px = [p for p, on in zip(crop.getdata(), letters.getdata()) if on]
    px.sort(key=sum, reverse=True)
    top = px[: max(1, len(px) // 2)]
    r, g, b = (sum(c[i] for c in top) // len(top) for i in range(3))
    return f"#{r:02x}{g:02x}{b:02x}"


def text_element(words, lang, size, x, y, fill):
    family, weight, stretch = FONTS.get(lang, LATIN)
    return (
        f'<text x="{x}" y="{y}" font-family="{family}" font-weight="{weight}" '
        f'font-stretch="{stretch}" font-size="{size}" fill="{fill}" '
        f'xml:lang="{lang}">{escape(words)}</text>'
    )


def render(svg, width, height):
    with tempfile.TemporaryDirectory() as tmp:
        src, out = Path(tmp) / "in.svg", Path(tmp) / "out.png"
        src.write_text(svg)
        subprocess.run(
            ["rsvg-convert", "-w", str(width), "-h", str(height), str(src), "-o", str(out)],
            check=True,
        )
        return Image.open(out).convert("RGBA")


def measure(words, lang):
    """The words' ink box at size 100, drawn from (0, 0): left, top, right,
    bottom, in those units, a pixel each."""
    w, h = 4000, 400
    svg = (
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" '
        f'viewBox="-1000 -250 {w} {h}">{text_element(words, lang, 100, 0, 0, "#000")}</svg>'
    )
    box = render(svg, w, h).getchannel("A").getbbox()
    if box is None:
        sys.exit(f"{lang}: {words!r} renders as nothing")
    left, top, right, bottom = box
    return left - 1000, top - 250, right - 1000, bottom - 250


def patch(picture, rect, words, lang, look):
    """The sign at `rect` with `words` in place of what is painted there."""
    x0, y0, x1, y1 = rect
    k = picture.width / 1536  # picture units to the original's pixels
    crop = picture.crop((round(x0 * k), round(y0 * k), round(x1 * k), round(y1 * k)))
    letters = amber(crop)
    stood = letters.getbbox()
    if stood is None:
        sys.exit(f"no painted letters inside {rect}")
    clean = erase(crop, letters)
    colour = ink(crop, letters)

    # where the painted words stood, in picture units: their capitals' top
    # and their baseline, and the rect's width less a margin; the new words
    # stand on the same baseline, centred where the old ones were
    top, bottom = y0 + stood[1] / k, y0 + stood[3] / k
    room = (x1 - x0) - 2.0
    band_top, band_bottom = y0 + 0.5, y1 - 0.5
    mid_x = x0 + (x1 - x0) / 2

    glyph, factor = REFERENCE.get(lang, LATIN_REFERENCE)
    _, ref_top, _, ref_bottom = measure(glyph, lang)
    left, t, right, b = measure(words, lang)
    sy = (bottom - top) * factor / (ref_bottom - ref_top)
    sx = sy
    if (right - left) * sx > room:
        sx = room / (right - left)
        sy = min(sy, sx / SQUEEZE)
    # all of the words, accents and descenders too, inside the panel: smaller
    # only as far as that needs
    tall = sy * (b - t)
    if tall > band_bottom - band_top:
        fit = (band_bottom - band_top) / tall
        sx, sy = sx * fit, sy * fit
    # on the painted baseline, moved up or down only to stay inside
    ty = bottom - sy * ref_bottom
    ty -= max(0.0, ty + sy * b - band_bottom)
    ty += max(0.0, band_top - (ty + sy * t))
    tx = mid_x - sx * (left + right) / 2

    buf = io.BytesIO()
    clean.save(buf, "PNG")
    data = base64.b64encode(buf.getvalue()).decode()
    w, h = x1 - x0, y1 - y0
    svg = f"""<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"
     width="{w}" height="{h}" viewBox="{x0} {y0} {w} {h}">
  <defs>
    <filter id="lit" x="-10%" y="-60%" width="120%" height="220%">
      <feGaussianBlur in="SourceGraphic" stdDeviation="{look["glow"]}" result="halo"/>
      <feComponentTransfer in="halo" result="faint">
        <feFuncA type="linear" slope="{look["halo"]}"/>
      </feComponentTransfer>
      <feMerge result="both"><feMergeNode in="faint"/><feMergeNode in="SourceGraphic"/></feMerge>
      <feGaussianBlur in="both" stdDeviation="{look["blur"]}"/>
    </filter>
  </defs>
  <image x="{x0}" y="{y0}" width="{w}" height="{h}" preserveAspectRatio="none"
         xlink:href="data:image/png;base64,{data}"/>
  <g filter="url(#lit)">
    <g transform="matrix({sx:.5f} 0 0 {sy:.5f} {tx:.3f} {ty:.3f})">
      {text_element(words, lang, 100, 0, 0, colour)}
    </g>
  </g>
</svg>"""
    return render(svg, crop.width, crop.height)


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("scene", type=Path)
    parser.add_argument("--out", type=Path)
    parser.add_argument("--look", type=Path)
    args = parser.parse_args()

    scene = json.loads(args.scene.read_text())
    sid = scene["id"]
    out = args.out or ROOT / "public" / "scenes" / sid
    looks = json.loads((args.look or ROOT / "art" / sid / "signs.json").read_text())
    out.mkdir(parents=True, exist_ok=True)

    for style, rendering in scene["styles"].items():
        spots = rendering.get("signs", [])
        if not spots:
            continue
        picture = Image.open(ROOT / "art" / sid / f"{style}.png").convert("RGB")
        for spot in spots:
            text = scene["signs"][spot["sign"]]["text"]
            for lang, words in text.items():
                if lang == spot["lang"]:
                    continue
                img = patch(picture, spot["rect"], words, lang, looks[style])
                png = out / f"{style}.{spot['sign']}.{lang}.png"
                webp = png.with_suffix(".webp")
                img.convert("RGB").save(png)
                subprocess.run(
                    ["cwebp", "-quiet", "-q", "85", "-m", "6", "-metadata", "none",
                     str(png), "-o", str(webp)],
                    check=True,
                )
                png.unlink()
                print(webp.relative_to(ROOT) if webp.is_relative_to(ROOT) else webp)


if __name__ == "__main__":
    main()
