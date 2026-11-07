# Ruya Changelog

<!-- https://keepachangelog.com/en/1.0.0/ -->

## [0.2.0] (unreleased)

### Added
- **All names** — a button in the bar, two little tags, that shows every
  thing's name at once: a small tag on each thing, one per thing on the
  biggest of its regions. Tags that would overlap, or run off the picture,
  are moved apart the least they need. Pointing at a thing still gives it
  the big label, the other tags fading back. The choice is kept in the
  address as `?n=all`. On a phone held upright the button ends the bar's
  first row, beside the scene's name.
- **The street** — the third scene, and the first outdoors, with eighteen
  things — sun, cloud, bird, house, roof, tree, streetlight, store, traffic
  light, sidewalk, bicycle, bench, trash can, dog, mailbox, crosswalk, car
  and bus — drawn by hand in SVG. Its realistic picture's prompt for
  ChatGPT is kept in `art/`.
- **The TV stand** — the living room's eighteenth thing, the low cabinet
  the TV stands on, in all three styles. It was in every picture from the
  start, but with no region it had no name.

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
