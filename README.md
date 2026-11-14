# Ruya

Ruya — رؤية, "sight" — is a picture dictionary. A big picture of a place, a
kitchen, a living room, a street, a fruit stand, a vegetable stand or a
farmyard, fills the screen; point at a thing in it and its name appears,
in the language you picked. Change the language and every name changes
with it.

It is written in Rust with [Leptos](https://leptos.dev), compiled to
WebAssembly and built with [Trunk](https://trunk-rs.github.io/trunk). Nothing
is written in JavaScript; Trunk adds the small loader browsers still need to
start WebAssembly.

## Using it

- **Mouse:** hover a thing to see its name. Click it to keep the name up.
- **Touch:** tap a thing. Tap anywhere else to hide the name.
- **Keyboard:** Tab moves from thing to thing; each one is announced by its
  name. Escape hides the name.
- The thing shown is outlined and the rest of the picture dims.
- On a phone held upright the picture fills the height and scrolls sideways,
  so a spoon is still big enough to tap. It opens at the middle, or,
  reached by an edge or a swipe, at the end it was entered from.
- **Place:** the scene's name in the bar is a list of the scenes. The
  picture's edges go through them too, round in a circle: a strip along
  the right edge to the next scene, along the left to the one before, in
  the list's order. Pointed at, a strip shades and shows an arrow; on a
  touch screen both stay faintly in view.
- **Swipe:** on a touch screen, a swipe across the picture does what the
  edges do: to the left for the next scene, to the right for the one
  before. Only a flick counts — at least 50 pixels sideways, twice as far
  as up or down, within half a second — so a tap still names a thing and
  a pinch still zooms. On a phone held upright a swipe first scrolls the
  picture to its end, and the next one that way turns the scene. A scene
  reached by a swipe or an edge slides in from that side, unless the
  device asks for reduced motion.
- **Style:** the little pictures beside the scene's name switch between
  the styles the scene comes in (below).
- **All names:** the button with two tags shows every thing's name at once,
  as a small tag on the thing — one per thing, on the biggest of its
  regions, so two chairs get one. Tags that would overlap or run off the
  picture are moved apart. Pointing still works: the thing pointed at gets
  the big label, and the other tags fade back. Pressed again, the names go.
- **Singular and plural:** the dot buttons beside it show a name's other
  forms, a line each under it — one dot the singular, three the plural, and
  in Arabic two dots between them, the dual (قطة, قطتان, قطط). Any of them
  can be pressed at once, but not none: the last one pressed stays. The
  big label and the tags both show the forms chosen. A dual chosen in
  Arabic waits through a language that has none and shows again on coming
  back. Japanese has no plural, so it shows no dot buttons at all.
- **Furigana:** in Japanese, the button showing 字 with じ over it puts each
  kanji's reading over it in small hiragana — ねこ over 猫 — in the label
  and the tags, so a name can be read aloud by someone who does not know
  its kanji. Katakana words already spell their sound and get none. The
  button is there only in Japanese.
- **Voice:** the speaker button, left of the language list, has the names
  said aloud. Pressed, a thing tapped or clicked says its name in each
  form shown, singular then plural; with the keyboard, Enter or the space
  bar on a thing says it. Ruya opens with it off, and tapping without it
  stays silent. Every language has a voice.
- On a phone held upright the bar takes two rows: the scene's name, the
  all-names button and the forms or furigana, then the style, the speaker
  and the language.

## Styles

A scene can be shown in more than one style, each its own picture of the
same place:

- **Basic** — a flat drawing in a few colours, with less to look at.
- **Realistic** — a painted picture, made in ChatGPT. The default.
- **Ghibli** — the realistic picture painted again in the manner of Studio
  Ghibli's films, warm and hand-drawn, made in ChatGPT.

The bar shows a thumbnail of each, the one shown ringed; a scene in one
style only shows none. Each style has its own regions, since the fridge
stands somewhere else in each, but the names belong to the scene's things,
so they are written once and the same in every style. More styles can come
later — a line drawing, a photo. A scene opens in the style picked if it
has it, else in its first.

## Languages

English, العربية (Arabic), Deutsch (German), Svenska (Swedish), Español
(Spanish), Українська (Ukrainian), עברית (Hebrew) and 日本語 (Japanese). The
picker lists each in its own script. Arabic and Hebrew names read right to
left; the layout itself stays put.

Ruya opens in the language of the address (below), else the first of the
browser's languages it has, else English.

### How the names are written

- **Gender shows**: German with its article (der Kühlschrank, die Pfanne,
  das Glas), Spanish with its article (la nevera, el plato), Swedish with
  en or ett (en stol, ett bord). English, Arabic, Hebrew, Ukrainian and
  Japanese have the bare noun.
- **Lowercase**, except German nouns.
- **Arabic** is Modern Standard Arabic as printed, without vowel marks.
- **Japanese** is written as it usually is: kanji where the word has them
  in everyday use (窓, 冷蔵庫), hiragana where its kanji are not (りんご, as
  林檎 is rare), katakana for words from abroad (テレビ, フォーク). The
  readings, for furigana, go over a whole run of kanji at a time (れいぞうこ
  over 冷蔵庫), as compounds are usually given.
- **Spanish** is Spain's (nevera, grifo). **English** is American (faucet,
  stove, cabinet).
- **Plurals** keep the same rules: German and Spanish with their article
  (die Kühlschränke, las neveras), the others bare — Swedish too, as en and
  ett have no plural (stolar, bord). A word with no singular, or the same
  word for one and many, repeats it (uk двері, he אופניים, sv fönster).
- **Arabic's dual**, for two, ends in ان (ثلاجتان), losing its ن before the
  noun that completes it (عمودا إنارة). Hebrew counts two with its plural,
  its dual kept for a few words, a pair of eyes or of days.
- **Fruit and vegetables in Arabic** have a word for one, which ends in ة
  (تفاحة, بصلة), and a collective for the produce itself (تفاح, بصل): the
  one is the name, the collective its many, and two are the one's dual
  (تفاحتان). One with no word for one apart — mostly words from abroad,
  مانجو, بطاطس, بروكلي — has the same word for one and many, and its two are
  counted with the word that fits it: a fruit by ثمرة (ثمرتا مانجو), a
  vegetable by حبة (حبتا بطاطس), a head by رأس (رأسا بروكلي). The farm's
  hens, ducks and geese are named the same way: دجاجة, بطة, إوزة for one,
  دجاج, بط, إوز for many.
- **Japanese has no plural**: 猫 is one cat or many, so a Japanese name has
  its one form only.
- Where a word needs explaining, the thing's `note` in the scene file says why
  — the pot has two handles, so it is a gryta and an olla, not a kastrull or
  a cazo.

## URL parameters

- `l` — the language, e.g. `?l=sv`. Picking a language writes it, so a link
  opens on what its sender saw. An unknown value is ignored. A language
  guessed from the browser is not written.
- `p` — the place, the scene's id, e.g. `?p=living-room`. Written when one
  is picked; absent, or unknown, is the kitchen. (`p`, as `s` was taken.)
- `s` — the style, `basic`, `realistic` or `ghibli`, e.g. `?s=basic`.
  Written when one is picked; absent, or unknown, is realistic. A scene without the
  style asked for opens in its first.
- `n` — `?n=all` opens with every name showing. Written when the all-names
  button is pressed, and taken out again when it is released.
- `f` — the forms each name shows, of `one`, `two` and `many`, joined by
  `-`: `?f=one-many` is singular and plural. Written when a form button is
  pressed; absent — or back at the singular alone — the singular alone.
- `r` — `?r=on` opens with furigana over Japanese kanji. Written when the
  furigana button is pressed, and taken out again when it is released.
- `v` — `?v=on` opens with the speaker on, so a thing tapped says its name.
  Written when the speaker button is pressed, and taken out again when it
  is released.

## Voice

The names are said by recordings, one per name and form, made ahead of
time rather than by the browser's own speech: the same voice on every
device, and no voice missing on a phone that has none for the language.
The voices are Microsoft Edge's online text-to-speech, each picked by ear
from the two to four Edge has for the language; where the sawt apps speak
it, theirs. Each says a name as the label writes it, its article and all:

- **English:** Ava, `en-US-AvaNeural`.
- **Arabic:** Hamed, `ar-SA-HamedNeural`, saying the dual too (ثلاجة,
  ثلاجتان, ثلاجات). The names are given him as the labels write them,
  without vowel marks, so he supplies the vowels; a word he gets wrong
  can be given its marks in `SAY`, for his voice alone.
- **German:** Katja, `de-DE-KatjaNeural` (der Kühlschrank, die
  Kühlschränke).
- **Swedish:** Sofie, `sv-SE-SofieNeural` (en stol, stolar).
- **Spanish:** Elvira, `es-ES-ElviraNeural`, a voice from Spain, as the
  names are Spain's (la nevera, las neveras).
- **Ukrainian:** Polina, `uk-UA-PolinaNeural`.
- **Hebrew:** Hila, `he-IL-HilaNeural`, the names without vowel marks,
  as Arabic's are.
- **Japanese:** Keita, `ja-JP-KeitaNeural`, the name in its one form, as
  written in kanji and kana; sawt has no Japanese voice to share.

A recording is `public/scenes/<scene>/voice/<lang>/<thing>.<number>.mp3`
— `fridge.one.mp3`, `fridge.two.mp3` in Arabic, `fridge.many.mp3` —
fetched the first time it is said. Each is the name with a full stop after
it, the silence Edge puts around a word trimmed to a breath, so a tap is
answered at once and the plural follows close behind: about 5 KB, mono
MP3. One `<audio>` plays
them all (`src/voice.rs`): a thing tapped while another is speaking cuts
it short.

`python3 tools/voice.py <lang>` records what is missing: a new thing's names,
or one whose name changed since; scene ids after the language keep it to
those scenes. What each recording says is logged in
`art/<scene>/voice.<lang>.json`, and `cargo test` fails while a name and
its recording differ. `--all` records everything again; a new voice in
the script does that too. Edge gives a different recording each time, so
listen to a new one rather than compare files. A word Edge says wrong
gets what to say instead in the script's `SAY`.

## Counting opens

Opened on ruya.info, Ruya tells sada, the collector the sawt apps and
amerharb.com report to (sada.sawt.info), that it was opened, once per load
(`src/sada.rs`). `{ "site": "ruya.info", "kind": "open" }` is all it sends:
no cookies, no identifiers, nothing of what is pointed at or chosen. sada
adds only the country the request came from.

A copy served anywhere else, as by `trunk serve`, sends nothing, unless a
`sada-url` entry in localStorage points it at a collector — the switch for
trying it out, as on amerharb.com:
`localStorage.setItem('sada-url', 'http://localhost:8080')`, and
`localStorage.removeItem('sada-url')` to stop. A send that fails is
swallowed: counting never breaks the page.

## Scenes

A scene is a file in `scenes/` naming the things in a place, with a picture
per style and the regions over it:

```json
{
	"id": "kitchen",
	"name": { "en": "kitchen", "ar": "مطبخ", "de": "die Küche", … },
	"things": {
		"fridge": {
			"name": { "en": "fridge", "ar": "ثلاجة", … },
			"plural": { "en": "fridges", "ar": "ثلاجات", … },
			"dual": { "ar": "ثلاجتان" },
			"reading": { "ja": "冷蔵庫[れいぞうこ]" },
			"note": "es: nevera, the everyday word in Spain"
		}
	},
	"styles": {
		"realistic": {
			"picture": {
				"src": "scenes/kitchen/realistic.webp",
				"thumb": "scenes/kitchen/realistic.thumb.webp",
				"width": 1536,
				"height": 1024
			},
			"regions": [
				{ "thing": "fridge", "d": "M1300 92L1520 66L1520 846L1300 740Z" }
			]
		},
		"basic": { … }
	}
}
```

- **`things`** are named once, by an id of `a-z`, `0-9` and `-`. Names are
  a fixed set of fields, one per language; a file missing one does not
  parse. **`name`** is for one of the thing, **`plural`** for many, a field
  per language that has a plural — all but Japanese — and **`dual`** for
  two, a field per language that has one: Arabic. **`reading`** is the
  name with its furigana, for a Japanese name with kanji: each run of
  kanji followed by its reading in hiragana, in brackets — 冷蔵庫[れいぞうこ],
  肘[ひじ]掛[か]け椅子[いす]. `note`, optional, says why a word was chosen.
- **`styles`** holds any of `basic`, `realistic` and `ghibli`. A style the
  scene lacks is left out.
- The **picture** is in the scene's own folder, `public/scenes/<scene>/`,
  named by its style, and shown as an image: a WebP for a painted scene, an
  SVG for a drawn one. `width` and `height` are the
  coordinate space the regions are written in and the picture's shape; the
  file may have more pixels, in the same proportion. **`thumb`** is the
  style's button in the bar: the picture at 96 × 64, twice the size it is
  shown, as WebP.
- Each region's **`thing`** says what it is, and **`d`** is its outline over
  the picture: an SVG path made of polygons only (M, L, H, V, Z). A ring
  inside another is a hole. Two regions may be the same thing, as the
  realistic kitchen's two chairs are.
- **Order matters**: where two regions overlap, the later one is the one
  tapped. A thing lying on another (a plate on the table) comes after it.
- The name shows above the region. **`at`**, optional, is a point on the
  region whose x the label centres on; it is needed where the region's
  middle is not on it, as for a chair seen between its legs.
- **`signs`**, optional, are words painted in the pictures, which follow
  the language: the street's bus reads "12 Central Station". The scene's
  `signs` give each sign's words in every language, as names are given
  (the bus's destination only; its 12 stays as painted). A style's
  `signs` say where it is, `"rect": [x0, y0, x1, y1]` in picture units,
  and which language the picture shows it in, `"lang": "en"`. In every
  other language a patch is laid over that rect: a small WebP of the sign
  with that language's words, `public/scenes/<scene>/<style>.<sign>.<lang>.webp`,
  made by `tools/signs.py` (below). The picture itself stays as it came;
  the patches lie under the regions, so a tap on the sign is the bus's.

`cargo test` checks every scene: every name present and in its language's
script, a thing's for one, two or many wherever its language has that
form and nowhere else; a reading for every Japanese name with kanji, in
hiragana, over every kanji, and the name again without its brackets;
every region naming a thing, and every thing in some style; every
region inside the picture and at least 48 units across, so it can be tapped
on a phone; every label point on its region; a thing lying on another after
it; each picture and thumbnail there, an SVG with the scene's `viewBox`
or a bitmap of the scene's shape — the picture at least its size; and
every sign worded in every language and in some picture, inside it, with a
patch for every language but the picture's own, of its rect's shape; and,
in a language with a voice, an MP3 of every name in each of its forms,
logged as saying the name as it now is, and none left over.

### Adding a thing, a scene or a language

- **A thing:** add it to `things` with its names, and a region for it to
  each style it is in; record its names with `python3 tools/voice.py
  <lang>` for each language; run `cargo test`.
- **A scene:** a folder for it, `public/scenes/<id>/`, holding a picture
  and a thumbnail per style (`basic.svg`, `realistic.webp`,
  `realistic.thumb.webp`, …); its file in `scenes/`, named by its id; and a
  line in `SOURCES` in `src/scene.rs`, which is the order of the list in the
  bar. Its originals go in `art/<id>/` the same way.
- **A style:** a variant in `Style` with its code and its name in each
  language (`src/style.rs`), a field in `Styles` (`src/scene.rs`), then the
  picture and thumbnail, named by the style's code, in each scene's folder
  that has it, and its regions in that scene's file.
- **A language:** a variant in `Lang` and a field in `Names`, and in
  `Plurals` and `Duals` if it has a plural or a dual (`src/lang.rs`), with
  `Lang::numbers` saying which; in `Readings` too if its names need
  readings over them, as Japanese does, with `Lang::readings_name` naming
  them. The compiler then points at every match to extend, and
  `cargo test` at every scene still missing a name. Then each sign's words
  in the language, and its patches, with `python3 tools/signs.py
  scenes/<scene>.json` (adding the font for a new script to its `FONTS`).
- **A voice for a language:** pick one of Edge's voices by ear; put it in
  `VOICES` in `tools/voice.py`, with the language's forms in `NUMBERS`,
  and run `python3 tools/voice.py <lang>`; then give the speaker button its
  name in the language, in `Lang::voice_name` (`src/lang.rs`), which is
  what makes the button show.

## Pictures and licences

- **Realistic** and **Ghibli** pictures are made in ChatGPT. Each one's
  original is kept as it came in `art/<scene>/`, named by its style, and
  the WebP the app shows is made from it with
  `cwebp -q 80 -m 6 -metadata none art/<scene>/<style>.png -o public/scenes/<scene>/<style>.webp`:
  a sixteenth to a thirtieth of the bytes, and the same picture to the eye.
- **Basic** pictures are SVG, drawn by hand in a 1536 × 1024 `viewBox`:
  flat shapes, no text. The fruit and the vegetable stand's, too many heaps
  to place by hand, and the farmyard's are drawn by scripts,
  `python3 art/<scene>/basic.py`, which write the SVG and the regions in
  the scene file: a stand's says what is in its crates and how to draw it,
  the stand around them being `tools/stand.py`'s, and the farmyard's draws
  each thing with the stand's shapes. Change the script and run it again,
  rather than the SVG.
- **Thumbnails** are made from the picture with
  `cwebp -q 75 -m 6 -resize 96 64 <picture>.png -o public/scenes/<scene>/<style>.thumb.webp`;
  an SVG is first made a PNG, with
  `rsvg-convert -w 192 -h 128 <picture>.svg -o <picture>.png`
  (`brew install librsvg`).
- **Sign patches** are made by `python3 tools/signs.py scenes/<scene>.json`,
  from the original in `art/<scene>/`: for each sign and each language but
  the one painted, it cuts the sign's rect out, fills the painted letters
  from the panel around them, and sets the language's words in their
  colour, on their baseline, as tall as the painted capitals and squeezed
  sideways rather than stretched if they are long. Pango shapes the
  Arabic, Hebrew and Japanese, which Pillow here cannot. The glow and
  softness per style are in `art/<scene>/signs.json`, a glow like LED
  letters for realistic and softer, painted ones for Ghibli. Run it again
  when a sign's words change, a language is added, or a picture is
  redrawn. It needs Pillow, `rsvg-convert` and `cwebp`.

Record each ChatGPT picture's date and prompt here, so it can be remade
or extended in the same style; a prompt written before the picture is
kept beside it in `art/<scene>/`.

| Scene | Style | Original | Made | Prompt |
| --- | --- | --- | --- | --- |
| kitchen | realistic | `art/kitchen/realistic.png`, 1536 × 1024 | ChatGPT, supplied 2026-10-25 | not recorded |
| kitchen | basic | `public/scenes/kitchen/basic.svg` | drawn by hand, 2026-10-25 | — |
| kitchen | ghibli | `art/kitchen/ghibli.png`, 1536 × 1024 | ChatGPT, supplied 2026-10-25 | not recorded |
| living room | realistic | `art/living-room/realistic.png`, 1536 × 1024 | ChatGPT, supplied 2026-10-25 | `art/living-room/realistic.prompt.md` |
| living room | basic | `public/scenes/living-room/basic.svg` | drawn by hand, 2026-10-25 | — |
| living room | ghibli | `art/living-room/ghibli.png`, 1536 × 1024 | ChatGPT, supplied 2026-10-26 | `art/living-room/ghibli.prompt.md` |
| street | realistic | `art/street/realistic.png`, 1536 × 1024 | ChatGPT, supplied 2026-11-08 | not recorded; `art/street/realistic.prompt.md` made the first version |
| street | basic | `public/scenes/street/basic.svg` | drawn by hand, 2026-11-02 | — |
| street | ghibli | `art/street/ghibli.png`, 1536 × 1024 | ChatGPT, supplied 2026-11-08 | not recorded |
| fruit stand | realistic | `art/fruit-stand/realistic.png`, 1536 × 1024 | ChatGPT, supplied 2026-11-09 | `art/fruit-stand/realistic.prompt.md` |
| fruit stand | basic | `public/scenes/fruit-stand/basic.svg` | drawn by `art/fruit-stand/basic.py`, 2026-11-09 | — |
| fruit stand | ghibli | `art/fruit-stand/ghibli.png`, 1536 × 1024 | ChatGPT, supplied 2026-11-09 | not recorded |
| vegetable stand | realistic | `art/vegetable-stand/realistic.png`, 1536 × 1024 | ChatGPT, supplied 2026-11-10 | `art/vegetable-stand/realistic.prompt.md` |
| vegetable stand | basic | `public/scenes/vegetable-stand/basic.svg` | drawn by `art/vegetable-stand/basic.py`, 2026-11-09 | — |
| vegetable stand | ghibli | `art/vegetable-stand/ghibli.png`, 1536 × 1024 | ChatGPT, supplied 2026-11-10 | `art/vegetable-stand/ghibli.prompt.md` |
| farmyard | realistic | `art/farmyard/realistic.png`, 1536 × 1024 | ChatGPT, supplied 2026-11-14 | `art/farmyard/realistic.prompt.md` |
| farmyard | basic | `public/scenes/farmyard/basic.svg` | drawn by `art/farmyard/basic.py`, 2026-11-14 | — |
| farmyard | ghibli | `art/farmyard/ghibli.png`, 1536 × 1024 | ChatGPT, supplied 2026-11-14 | not recorded |

ChatGPT gave the realistic kitchen's things as boxes too, kept in
`art/kitchen/realistic.boxes.json`. They were approximate — the fork and
spoon about 50 px low, the window short of its sill — so each region was
redrawn by eye over the picture: rectangles where they fit, polygons for
the fork, spoon, chairs and table. The second chair, nearest the viewer, is
not in ChatGPT's list and has a second `chair` region.

The Ghibli kitchen's boxes, in `art/kitchen/ghibli.boxes.json`, were
closer, but a box cannot follow the table or the chair behind its corner,
and the sink's was 30 px tall, under the 48 a region needs; its regions
were redrawn the same way. The far chair's region follows the table's edge
rather than overlapping it, so a tap on the table's corner is the table's.

The realistic living room's boxes, in
`art/living-room/realistic.boxes.json`, put both curtains in one box and
all the pillows in another, though the prompt asked for one each. The
picture has four pillows, three on the couch and one in the armchair,
each its own `pillow` region; the armchair's comes after the armchair, as
it lies on it. The book on the coffee table is a stack of two, its spine
the one piece of text in the picture.

The Ghibli living room is the realistic one repainted, every thing a few
pixels off where it was, so its regions were redrawn over it the same way.
Its boxes, in `art/living-room/ghibli.boxes.json`, again merged the
curtains and the pillows.

The drawn street's road has a solid line along each edge. The near lane,
between its edge line and the dashed line, is the one `lane` region; the
far lane and the strip between the curb and the far edge line are the
road's, so a tap there says road. The curb and the dashed
line are thinner than the 48 units a region needs, so their regions are 48
tall, centred on them; the dashed line's comes after the lanes, the curb's
after the road. The motorcycle has the near lane to itself, left of the bus.

The realistic street did not keep to its prompt's layout: it is a street
corner seen at an angle, not straight across the road. The first
picture, supplied 2026-11-07, had no dog, and its boxes were far off —
the sun's by 150 px, the crosswalk's by more. The one kept, supplied
2026-11-08, is the same corner with a dog and a motorcycle on the
sidewalk, the crosswalk moved to the corner and the car side-on, and has
all 26 things; it came with no boxes, so every region was drawn by eye.
The prompt asked for no text, but the bus's sign reads "12 Central
Station" — now a sign that follows the language, its destination patched
in each — the number plates are blurred letters, and a driver can be made
out behind the bus's windscreen. The picture has four birds, each its own
`bird` region and drawn bigger than the bird, so it can be tapped; two
clouds, three trees and the house's two windows not behind leaves have a
region each. The road is all the asphalt, the side street the car is on
included. As in the drawing, one lane is the `lane`: the one right of the
dashed line, up to the stop line; the rest is the road's. The curb and
the dashed line are bands about 30 wide along them. The car comes after
the bus and the traffic light after the car, each standing in front of
the other's edge; the dog comes after the bicycle and the streetlight.
As WebP it is 299 KB, twice the kitchen: the leaves are detail that does
not compress.

The Ghibli street is the realistic one repainted, every thing within a
few pixels of where it was, so its regions are the realistic ones moved
where it differs: the streetlight 12 px left, the mailbox wider, the sun
and one bird lower. Its sky has more clouds, and two more of them have a
region. It came with no boxes. As WebP it is 403 KB, the biggest picture:
its painted leaves and clouds are fine detail all over.

The drawn fruit stand is three tiers of eight crates under a striped
awning, one fruit to a crate: the big fruit on top, the tree fruit in the
middle, the berries and small fruit at the bottom. Where a fruit's inside
is what tells it apart, one of it is cut — the watermelon, the melon, the
coconut, the papaya, the pomegranate, the kiwi and the fig. Each fruit's
region is its heap and its crate together, the convex hull of both, so a
tap on the crate names what is in it; the crates themselves have no name.
The SVG is 98 KB, 16 KB compressed: every apple and blueberry is its own
shape.

The realistic fruit stand kept to its prompt: three tiers of eight
crates, every fruit in the drawing's order, the cut ones cut as asked, and
no text anywhere. It is seen from the left rather than straight on, the
stand reaching back to the right, on a cobbled street under a tree. It
came with no boxes, so each region was drawn by eye: the crate and its
heap, as in the drawing, the crate's front up its two sides to the top of
the heap. The top tier comes first, the bottom last, so where a heap
stands in front of the crate behind it — the grapes' stalks and the
cherries' before the middle tier — a tap there is the nearer crate's. The
Ghibli fruit stand is the realistic one repainted, every crate where it
was, so its regions are the realistic ones as they are. As WebP they are
360 KB and 466 KB, the Ghibli now the biggest picture: heaps of small
fruit are detail all over.

The drawn vegetable stand is the fruit stand's, its awning green: the big
vegetables on top, those sold by the piece in the middle, the small ones
at the bottom. They are whole, but for one pea pod lying open on the
others, its peas showing. Its SVG is 134 KB, 29 KB compressed, with every
kernel on the corn.

The realistic vegetable stand kept to its prompt as the fruit stand did:
the same street and view, the awning green and white, every vegetable in
the drawing's order. It came with no boxes, so each region was drawn by
eye, the crate and its heap as for the fruit stand, but meeting its
neighbours edge to edge, with no overlap: where leaves reach over the
crate behind — the carrots' tops, the radishes' and the beets' leaves, the
asparagus tips — the crate behind gives way, so a tap on them names the
vegetable they belong to. The Ghibli vegetable stand is the realistic one
repainted, every crate where it was, so its regions are the realistic ones
as they are. As WebP they are 276 KB and 386 KB.

The drawn farmyard has no stand to share, so its script draws each thing
in a function of its own: the farmhouse with the cat on its step, a
beehive, the well and its bucket, the barn with a pitchfork against it and
the chicken coop along the back; the pond on the left, a duck and three
ducklings on it; the goose, the hen and three chicks, the rooster and the
turkey in the middle; the dog, the wheelbarrow, a rabbit and a nest of eggs
in front. Each thing's region is the convex hull of its shapes, the
pitchfork's widened so a finger finds it, and every chick and duckling has
one of its own. The things are drawn back to front and their regions are
in that order, so where a nearer thing stands over one behind it, a tap
there is the nearer one's. Its SVG is 22 KB, 5 KB compressed.

The realistic farmyard's prompt, `art/farmyard/realistic.prompt.md`,
written 2026-11-14, is the prompt alone, to paste whole into ChatGPT in the
browser, which sees none of the repo's files: it names every thing in
words, but leaves the yard's layout to the picture, which need not match
the drawing, and asks for no boxes, since each region is drawn by eye
along its thing's outline. The picture goes in `-AmerLocal/` and becomes
`art/farmyard/realistic.png`.

The realistic farmyard, supplied 2026-11-14, has all 20 things, whole and
apart, in a layout of its own that happens to be close to the drawing's:
the farmhouse, the beehive, the well and its bucket, the barn with the
pitchfork against it and the coop at the back; the pond on the left with
the duck and the ducklings, the goose, the hen and her chicks, the rooster
and the turkey across the middle; the dog, the wheelbarrow, the rabbit and
the nest in front. Each region was drawn by eye along its thing's outline
over a grid. A chick or a duckling is smaller than a fingertip, so each has
an octagon 48 units across, centred on it, the three of a kind overlapping
a little. The Ghibli farmyard, supplied the same day, is the realistic one
repainted, with its prompt not recorded. Nearly every thing is where it
was, so its regions are the realistic ones but for five: the dog stands a
little further left with a fuller tail, and the hen, the chicks, the
turkey's fan and the rabbit's tail moved a few units. As WebP they are
475 KB and 510 KB, the biggest pictures in Ruya.

For later scenes, the plan is to paint a flat colour per thing on a mask
layer, as a PNG — never a lossy format, which smears the colours — and
trace it into polygons with a script.

## Development

- Rust stable, managed by rustup; `rust-toolchain.toml` adds the
  `wasm32-unknown-unknown` target on first use.
- Trunk 0.22: `brew install trunk`, or `cargo install trunk --locked`.
- `trunk serve` — http://127.0.0.1:8090, rebuilding on every change.
- `trunk build --release` — the site in `dist/`: `index.html`, the
  WebAssembly, its loader, the stylesheet and `scenes/`. The release
  WebAssembly is about 468 KB, 183 KB compressed, the scene files
  included.
- `cargo test`, `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`
  — run on the host, no browser needed. CI runs them and a release build.

## Deploying

Ruya is published by GitHub Pages at https://ruya.info, from the repository
amerharb/ruya. Every push to `main` runs CI; when every check passes, the
release build it made is deployed. A pull request is checked and built but
not deployed. The site is `dist/` as Trunk builds it: plain static files,
built for the domain's root.

- **Pages settings:** source "GitHub Actions", custom domain `ruya.info`,
  HTTPS enforced.
- **DNS** at Squarespace Domains, where ruya.info is registered (2026-11-01):
  - `A` records for `@`: 185.199.108.153, 185.199.109.153,
    185.199.110.153 and 185.199.111.153.
  - `AAAA` records for `@`: 2606:50c0:8000::153, 2606:50c0:8001::153,
    2606:50c0:8002::153 and 2606:50c0:8003::153.
  - `CNAME` for `www`: `amerharb.github.io`.
- Pages caches every file for ten minutes and cannot be told otherwise, so
  a redeploy reaches everyone within ten minutes; the hashed WebAssembly,
  loader and stylesheet could be kept forever on a host that allowed it.
