# The living room, Ghibli: the prompt for ChatGPT

Written 2026-10-25, to repaint `art/living-room-realistic.png` the way the
Ghibli kitchen repainted the realistic one. Best asked in the chat that made
the realistic living room, so it has the picture; in a new one, attach
`art/living-room-realistic.png`, and `art/kitchen-ghibli.png` for the look.
Paste the first part, then the second in the same chat once the picture is
right. Put the picture and the answer to the second part in `-AmerLocal/`;
they become `art/living-room-ghibli.png` and
`art/living-room-ghibli.boxes.json`.

## 1. The picture

> Paint this living room again in a Studio Ghibli–inspired style:
> hand-painted, warm late-afternoon sunlight, soft colours and gentle
> detail, the same look as the Ghibli-style kitchen. Keep the room exactly
> as it is — the same view, landscape, 1536 × 1024, and every piece of
> furniture and every object in the same place and at the same size, so
> the two pictures line up. No people, no text — leave the book's spine
> plain — and no logos.
>
> Keep each of these clearly visible and easy to tell apart: the window,
> the two curtains, the framed picture, the door, the bookshelf, the floor
> lamp, the couch and its three pillows, the TV, the plant, the rug, the
> armchair and its pillow, the coffee table, the book on it, the vase, the
> flowers and the sleeping cat.

## 2. The boxes

> Now list each of those things with its bounding box in the picture, as
> JSON: `scene_id` "living-room", `style` "ghibli", the image's size, and
> `items`, each with `id`, `name`, `bbox` (x, y, width, height) and
> `bbox_xyxy`, in pixels from the top-left. Use these ids: window,
> curtain, picture, door, bookshelf, lamp, couch, pillow, tv, plant, rug,
> armchair, coffee-table, book, vase, flower, cat. Give each curtain its
> own item, and each of the four pillows — three on the couch, one in the
> armchair — its own item too.
