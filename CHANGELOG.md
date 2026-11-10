# Ruya Changelog

<!-- https://keepachangelog.com/en/1.0.0/ -->

## [0.3.0] (unreleased)

### Added
- **Swiping between scenes** — on a touch screen, a swipe across the
  picture turns the scene as the edges do: to the left for the next, to
  the right for the one before, round in a circle. Only a flick counts —
  at least 50 pixels sideways, twice as far as up or down, within half a
  second — so a tap still names a thing, and a slow drag, a pinch or a
  page zoomed in is left alone. On a phone held upright, where the picture
  scrolls sideways, a swipe first takes it to its end and the next one
  turns the scene; the scene turned to opens at the end it was entered
  from, so swiping on the same way carries straight across it.
- **Scenes slide in** — a scene reached by a swipe or an edge slides in
  from that side, in a fifth of a second, once its picture is in; with
  reduced motion asked for, it is simply there. Once a scene's picture is
  in, the pictures of the scenes either side are fetched in the same
  style, so the next one is there to slide.
- **The fruit stand** — the fourth scene, a market stand with 24 fruits, a
  crate of each under a striped awning: pineapple, watermelon, banana,
  coconut, melon, mango, papaya and pomegranate on top; apricot, apple,
  lemon, plum, orange, pear, tangerine and peach in the middle; strawberry,
  blueberry, grape, cherry, kiwi, raspberry, fig and date at the bottom.
  A realistic picture and a Ghibli one, made in ChatGPT (360 KB and 466 KB
  as WebP), the stand seen from the left on a cobbled street, and a basic
  one drawn in SVG by a script, `art/fruit-stand/basic.py`, which writes
  the drawing's regions too. In every style a fruit's region is its heap
  and its crate, so a tap on the crate names what is in it. Named in all
  eight languages, Arabic's by the word for one, its dual, and the
  collective for many (تفاحة, تفاحتان, تفاح).
- **The vegetable stand** — the fifth scene, the fruit stand's twin under
  a green-and-white awning, with 24 vegetables: pumpkin, cabbage,
  eggplant, cauliflower, broccoli, corn, artichoke and lettuce on top;
  tomato, cucumber, carrot, potato, bell pepper, zucchini, onion and sweet
  potato in the middle; radish, pea, mushroom, chili pepper, asparagus,
  garlic, okra and beet at the bottom. A realistic picture and a Ghibli
  one, made in ChatGPT (276 KB and 386 KB as WebP), on the fruit stand's
  street and seen the same way, and a basic one drawn in SVG by
  `art/vegetable-stand/basic.py`, on the same stand as the fruit, which is
  now `tools/stand.py`, shared by the two. A vegetable's region is its
  heap and its crate, and in the painted pictures its leaves too, where
  they reach over the crate behind, so a tap on the carrots' tops names
  the carrot. Named in all eight languages; in Arabic a vegetable with no
  word for one apart counts its two by حبة, or by رأس for a head
  (حبتا بطاطس, رأسا بروكلي).

### Fixed
- **A picture a few pixels off** — where the whole picture shows, a name
  tag reaching past its edge, shown or not, let it scroll a few pixels,
  and it opened scrolled by half of them: the street 3 px to the left at
  1024 pixels wide. How far a picture can scroll is now measured on the
  picture itself.

## [0.2.0] 2026-11-09

### Added
- **All names** — a button in the bar, two little tags, that shows every
  thing's name at once: a small tag on each thing, one per thing on the
  biggest of its regions. Tags that would overlap, or run off the picture,
  are moved apart the least they need. Pointing at a thing still gives it
  the big label, the other tags fading back. The choice is kept in the
  address as `?n=all`. On a phone held upright the button ends the bar's
  first row, beside the scene's name.
- **The edges go round the scenes** — a strip along the picture's right
  edge goes to the next scene, along its left to the one before, the last
  leading back to the first. Pointed at, a strip shades and shows an
  arrow; on a touch screen both stay faintly in view. Each is a button
  named for the scene it goes to.
- **The street** — the third scene, and the first outdoors, with 26
  things — sun, cloud, bird, house, roof, chimney, window, door, tree,
  streetlight, store, traffic light, sidewalk, curb, road, lane, lane line,
  bicycle, bench, trash can, dog, mailbox, crosswalk, car, bus and
  motorcycle — drawn by hand in SVG, and a street corner in the realistic
  and Ghibli styles, made in ChatGPT (299 KB and 403 KB as WebP), with all
  26 in every style.
- **Japanese** — the eighth language, 日本語 in the picker: every scene and
  thing named as Japanese is usually written, kanji where the word has
  them and katakana for words from abroad (窓, テレビ). Japanese has no
  plural, so its names have the one form, and the bar shows no dot
  buttons while it is chosen.
- **Furigana** — in Japanese, a button showing 字 with じ over it puts each
  kanji's reading over it in small hiragana, in the label and the tags:
  ねこ over 猫, ひじ, か and いす over 肘掛け椅子. All 37 things written with
  kanji have their readings in the scene files; katakana words need none.
  The choice is kept in the address as `?r=on`.
- **Plurals and duals** — every thing's name for many, in each language
  that has a plural, and Arabic's for two (قطة, قطتان, قطط), beside its
  name for one, in the scene files; `cargo test` checks each is there
  wherever its language has the form. Dot buttons beside the all-names
  button show them: one dot the singular, three the plural, and in Arabic
  two, the dual. Any of them at once, never none; the forms chosen stack a
  line each, singular on top, in the label and the tags, and a screen
  reader says them in turn. The choice is kept in the address as `?f=`.
- **The bus's sign in every language** — the street's bus reads "12
  Central Station" in English, and its destination follows the language:
  المحطة المركزية, Hauptbahnhof, Centralstationen, Estación Central,
  Центральний вокзал, התחנה המרכזית, 中央駅, the 12 as painted. Each is a
  patch about a kilobyte in size, laid over the realistic and Ghibli
  pictures at the sign's place, made from the sign itself by
  `tools/signs.py`: the painted letters filled from the panel, the new
  words set in their amber, with an LED glow or a painted softness. The
  pictures are unchanged.
- **Opens counted** — on ruya.info, Ruya tells sada, the collector the
  sawt apps report to, that it was opened: `{ site: "ruya.info", kind:
  "open" }` once per load, and nothing more — no cookies, no identifiers.
  Anywhere else it stays silent, unless a `sada-url` entry in localStorage
  points it at a collector. sada takes it once it lists ruya.info.
- **The TV stand** — the living room's eighteenth thing, the low cabinet
  the TV stands on, in all three styles. It was in every picture from the
  start, but with no region it had no name.

### Changed
- **A folder per scene** — each scene's pictures and thumbnails are in
  `public/scenes/<scene>/`, named by style (`kitchen/realistic.webp`), and
  its originals, boxes and prompts in `art/<scene>/` the same way.

### Fixed
- **The scene's list in Arabic and Hebrew** stays at the bar's left, beside
  the brand, as in every other language; it had moved right, next to the
  style switch. Its options still read right to left.

## [0.1.0] 2026-11-01

The first version of ruya.info, a picture dictionary: two scenes, the
kitchen and the living room, each in three styles, where pointing at a thing
shows its name in one of seven languages. Published by GitHub Pages from
amerharb/ruya, over HTTPS; nothing needs a server.

### Added
- **The kitchen** — the first scene, a picture made in ChatGPT and shown as
  WebP (127 KB, from a 3.6 MB PNG kept in `art/`), with nineteen things to
  point at — window, cabinet, frying pan, clock, fridge, stove, sink, faucet,
  pot, kettle, table, two chairs, plate, fork, spoon, cup, glass and bowl.
  ChatGPT's boxes for them were approximate; each region was redrawn by eye
  over the picture.
- **Two styles** — the kitchen as a realistic picture, the default, and as a
  basic drawing: flat shapes in a few colours, drawn by hand in SVG. A
  thumbnail of each in the bar switches between them, the choice kept in the
  address as `?s=`. The names belong to the scene's things, written once for
  every style; each style has its own regions over its own picture.
- **A Ghibli style** — a third style: each scene's realistic picture painted
  again in the manner of Studio Ghibli's films, made in ChatGPT (133 KB and
  177 KB as WebP, from PNGs kept in `art/`), with its regions redrawn by
  eye.
- **The living room** — the second scene, with seventeen things — window,
  curtains, picture, door, bookshelf, lamp, couch, pillows, TV, plant, rug,
  armchair, coffee table, book, vase, flower and a sleeping cat. Realistic
  and Ghibli, made in ChatGPT from the prompts kept in `art/` (191 KB and
  177 KB as WebP), and basic, drawn by hand in SVG.
- **A choice of scene** — the scene's name in the bar is a list of the
  scenes, each named in the chosen language, the choice kept in the address
  as `?p=`.
- **Names in seven languages** — English, Arabic, German, Swedish, Spanish,
  Ukrainian and Hebrew. German and Spanish carry their article and Swedish en
  or ett; Arabic is Modern Standard without vowel marks; Spanish is Spain's
  and English American.
- **Point, tap or Tab** — hover with a mouse, tap on a phone, or Tab with the
  keyboard to see a thing's name above it, the thing outlined and the rest
  dimmed. A click or tap keeps it up; tapping elsewhere or Escape hides it.
  Each thing is a button named in the chosen language for screen readers.
- **A language picker** listing each language in its own script, the choice
  kept in the address as `?l=`, and `<html lang>` following it. Arabic and
  Hebrew names read right to left.
- **Phones held upright** see the picture at the screen's full height,
  scrolling sideways and opening at the middle, so the smallest thing is a
  fingertip wide, and the bar in two rows.
- **Checks** run by `cargo test` on every scene file, and CI running format,
  lint, tests and a release build; on `main`, once they pass, that build is
  deployed to ruya.info.
