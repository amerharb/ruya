# The vegetable stand, Ghibli: the prompt for ChatGPT

Written 2026-11-09, to repaint `art/vegetable-stand/realistic.png` the way
the Ghibli fruit stand repainted the realistic one. Best asked in the chat
that made the realistic vegetable stand, so it has the picture; in a new
one, attach `art/vegetable-stand/realistic.png`, and
`art/fruit-stand/ghibli.png` for the look. Paste the first part, then the
second in the same chat once the picture is right. Put the picture and the
answer to the second part in `-AmerLocal/`; they become
`art/vegetable-stand/ghibli.png` and `art/vegetable-stand/ghibli.boxes.json`.

## 1. The picture

> Paint this vegetable stand again in a Studio Ghibli–inspired style:
> hand-painted, warm sunlight, soft colours and gentle detail, the same
> look as the Ghibli-style fruit stand. Keep the stand exactly as it is —
> the same view, landscape, 1536 × 1024, and every crate and every
> vegetable in the same place and at the same size, so the two pictures
> line up. No people, no text and no logos.

## 2. The boxes

> Now list each crate with its bounding box in the picture, as JSON:
> `scene_id` "vegetable-stand", `style` "ghibli", the image's size, and
> `items`, each with `id`, `name`, `bbox` (x, y, width, height) and
> `bbox_xyxy`, in pixels from the top-left, a crate's box taking in its
> vegetables and the crate together. Use these ids, the top tier first,
> each tier from left to right: pumpkin, cabbage, eggplant, cauliflower,
> broccoli, corn, artichoke, lettuce, tomato, cucumber, carrot, potato,
> bell-pepper, zucchini, onion, sweet-potato, radish, pea, mushroom,
> chili-pepper, asparagus, garlic, okra, beet.
