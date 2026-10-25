# Ruya Changelog

<!-- https://keepachangelog.com/en/1.0.0/ -->

## [0.1.0] (unreleased)

### Added
- **The kitchen** — the first scene, a picture made in ChatGPT and shown as
  WebP (127 KB, from a 3.6 MB PNG kept in `art/`), with nineteen things to
  point at — window, cabinet, frying pan, clock, fridge, stove, sink, faucet,
  pot, kettle, table, two chairs, plate, fork, spoon, cup, glass and bowl.
  ChatGPT's boxes for them were approximate; each region was redrawn by eye
  over the picture. A flat SVG kitchen drawn by hand stood in until it came.
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
  fingertip wide.
- **Checks** run by `cargo test` on every scene file, and CI running format,
  lint, tests and a release build.
