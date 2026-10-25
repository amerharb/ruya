# Ruya

Ruya — رؤية, "sight" — is a picture dictionary. A big picture of a place, a
kitchen to begin with, fills the screen; point at a thing in it and its name
appears, in the language you picked. Change the language and every name
changes with it.

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
- **Style:** the little pictures in the bar switch between the styles the
  scene comes in (below).

## Styles

A scene can be shown in more than one style, each its own picture of the
same place:

- **Realistic** — a painted picture, made in ChatGPT. The default.
- **Basic** — a flat drawing in a few colours, with less to look at.

The bar shows a thumbnail of each, the one shown ringed; a scene in one
style only shows none. Each style has its own regions, since the fridge
stands somewhere else in each, but the names belong to the scene's things,
so they are written once and the same in every style. More styles can come
later — a line drawing, a photo.

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
- Where a word needs explaining, the thing's `note` in the scene file says why
  — the pot has two handles, so it is a gryta and an olla, not a kastrull or
  a cazo.

## URL parameters

- `l` — the language, e.g. `?l=sv`. Picking a language writes it, so a link
  opens on what its sender saw. An unknown value is ignored. A language
  guessed from the browser is not written.
- `s` — the style, `basic` or `realistic`, e.g. `?s=basic`. Written when
  one is picked; absent, or unknown, is realistic. A scene without the
  style asked for opens in its first.

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
			"note": "es: nevera, the everyday word in Spain"
		}
	},
	"styles": {
		"realistic": {
			"picture": {
				"src": "scenes/kitchen-realistic.webp",
				"thumb": "scenes/kitchen-realistic.thumb.webp",
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
  parse. `note`, optional, says why a word was chosen.
- **`styles`** holds `basic`, `realistic` or both. A style the scene lacks
  is left out.
- The **picture** is in `public/scenes/`, shown as an image: a WebP for a
  painted scene, an SVG for a drawn one. `width` and `height` are the
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
script; every region naming a thing, and every thing in some style; every
region inside the picture and at least 48 units across, so it can be tapped
on a phone; every label point on its region; a thing lying on another after
it; and each picture and thumbnail there, an SVG with the scene's `viewBox`
or a bitmap of the scene's shape — the picture at least its size.

### Adding a thing, a scene or a language

- **A thing:** add it to `things` with its names, and a region for it to
  each style it is in; run `cargo test`.
- **A scene:** its pictures and thumbnails in `public/scenes/`, its file in
  `scenes/`, and a line in `SOURCES` in `src/scene.rs`. (For now the app
  shows the kitchen only.)
- **A style:** a variant in `Style` with its code and its name in each
  language (`src/style.rs`), a field in `Styles` (`src/scene.rs`), then the
  picture, thumbnail and regions in each scene that has it.
- **A language:** a variant in `Lang` and a field in `Names`
  (`src/lang.rs`). The compiler then points at every match to extend, and
  `cargo test` at every scene still missing the name.

## Pictures and licences

- **Realistic** pictures are made in ChatGPT. Each one's original is kept
  in `art/`, as it came, and the WebP the app shows is made from it with
  `cwebp -q 80 -m 6 -metadata none art/<scene>-realistic.png -o public/scenes/<scene>-realistic.webp`:
  about a thirtieth of the bytes, and the same picture to the eye.
- **Basic** pictures are SVG, drawn by hand in a 1536 × 1024 `viewBox`:
  flat shapes, no text.
- **Thumbnails** are made from the picture with
  `cwebp -q 75 -m 6 -resize 96 64 <picture>.png -o public/scenes/<scene>-<style>.thumb.webp`;
  an SVG is first saved as a PNG, in a browser or any viewer.

Record each realistic picture's date and prompt here, so it can be remade
or extended in the same style.

| Scene | Style | Original | Made | Prompt |
| --- | --- | --- | --- | --- |
| kitchen | realistic | `art/kitchen-realistic.png`, 1536 × 1024 | ChatGPT, supplied 2026-10-25 | not recorded |
| kitchen | basic | `public/scenes/kitchen-basic.svg` | drawn by hand, 2026-10-25 | — |

ChatGPT gave the realistic kitchen's things as boxes too, kept in
`art/kitchen-realistic.boxes.json`. They were approximate — the fork and
spoon about 50 px low, the window short of its sill — so each region was
redrawn by eye over the picture: rectangles where they fit, polygons for
the fork, spoon, chairs and table. The second chair, nearest the viewer, is
not in ChatGPT's list and has a second `chair` region.

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
  WebAssembly is about 240 KB, 110 KB compressed.
- `cargo test`, `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`
  — run on the host, no browser needed. CI runs them and a release build.

## Deploying

Not yet: ruya.info is to be registered, and the host chosen then. `dist/` is
plain static files — any host that serves `.wasm` as `application/wasm`
will do. Vercel cannot build Rust itself, so the plan is to build in GitHub
Actions and upload `dist/`. Hashed files can be cached forever;
`index.html` and `scenes/` should not be.
