# Ruya

Ruya — رؤية, "sight" — is a picture dictionary. A big picture of a place, a
kitchen, a living room or a street, fills the screen; point at a thing in it
and its name appears, in the language you picked. Change the language and
every name changes with it.

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
  so a spoon is still big enough to tap. It opens at the middle.
- **Place:** the scene's name in the bar is a list of the scenes. The
  picture's edges go through them too, round in a circle: a strip along
  the right edge to the next scene, along the left to the one before, in
  the list's order. Pointed at, a strip shades and shows an arrow; on a
  touch screen both stay faintly in view.
- **Style:** the little pictures in the bar switch between the styles the
  scene comes in (below).
- **All names:** the button with two tags shows every thing's name at once,
  as a small tag on the thing — one per thing, on the biggest of its
  regions, so two chairs get one. Tags that would overlap or run off the
  picture are moved apart. Pointing still works: the thing pointed at gets
  the big label, and the other tags fade back. Pressed again, the names go.
- On a phone held upright the bar takes two rows: the scene's name and the
  all-names button, then the style and the language.

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
(Spanish), Українська (Ukrainian) and עברית (Hebrew). The picker lists each in
its own script. Arabic and Hebrew names read right to left; the layout itself
stays put.

Ruya opens in the language of the address (below), else the first of the
browser's languages it has, else English.

### How the names are written

- **Gender shows**: German with its article (der Kühlschrank, die Pfanne,
  das Glas), Spanish with its article (la nevera, el plato), Swedish with
  en or ett (en stol, ett bord). English, Arabic, Hebrew and Ukrainian have
  the bare noun.
- **Lowercase**, except German nouns.
- **Arabic** is Modern Standard Arabic as printed, without vowel marks.
- **Spanish** is Spain's (nevera, grifo). **English** is American (faucet,
  stove, cabinet).
- **Plurals** keep the same rules: German and Spanish with their article
  (die Kühlschränke, las neveras), the others bare — Swedish too, as en and
  ett have no plural (stolar, bord). A word with no singular, or the same
  word for one and many, repeats it (uk двері, he אופניים, sv fönster).
- **Arabic's dual**, for two, ends in ان (ثلاجتان), losing its ن before the
  noun that completes it (عمودا إنارة). Hebrew counts two with its plural,
  its dual kept for a few words, a pair of eyes or of days.
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
  per language that has a plural — all seven — and **`dual`** for two, a
  field per language that has one: Arabic. A language without a plural,
  as Japanese has none, would have no field in `plural`. `note`, optional,
  says why a word was chosen. Only `name` is shown so far.
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

`cargo test` checks every scene: every name present and in its language's
script, a thing's for one, two or many wherever its language has that
form and nowhere else; every region naming a thing, and every thing in some style; every
region inside the picture and at least 48 units across, so it can be tapped
on a phone; every label point on its region; a thing lying on another after
it; and each picture and thumbnail there, an SVG with the scene's `viewBox`
or a bitmap of the scene's shape — the picture at least its size.

### Adding a thing, a scene or a language

- **A thing:** add it to `things` with its names, and a region for it to
  each style it is in; run `cargo test`.
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
  `Lang::numbers` saying which. The compiler then points at every match to
  extend, and `cargo test` at every scene still missing a name.

## Pictures and licences

- **Realistic** and **Ghibli** pictures are made in ChatGPT. Each one's
  original is kept as it came in `art/<scene>/`, named by its style, and
  the WebP the app shows is made from it with
  `cwebp -q 80 -m 6 -metadata none art/<scene>/<style>.png -o public/scenes/<scene>/<style>.webp`:
  a sixteenth to a thirtieth of the bytes, and the same picture to the eye.
- **Basic** pictures are SVG, drawn by hand in a 1536 × 1024 `viewBox`:
  flat shapes, no text.
- **Thumbnails** are made from the picture with
  `cwebp -q 75 -m 6 -resize 96 64 <picture>.png -o public/scenes/<scene>/<style>.thumb.webp`;
  an SVG is first made a PNG, with
  `rsvg-convert -w 192 -h 128 <picture>.svg -o <picture>.png`
  (`brew install librsvg`).

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
Station", the number plates are blurred letters, and a driver can be made
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
  WebAssembly is about 290 KB, 128 KB compressed, the scene files
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
