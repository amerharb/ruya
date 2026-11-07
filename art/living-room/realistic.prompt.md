# The living room, realistic: the prompt for ChatGPT

Written 2026-10-25, to match the realistic kitchen and the things in
`scenes/living-room.json`. Best asked in the chat that made the kitchen, so
the look carries over; in a new one, attach `art/kitchen/realistic.png`
with it. Paste the first part, then the second in the same chat once the
picture is right. Save the picture as
`art/living-room/realistic.png` and the answer to the second part as
`art/living-room/realistic.boxes.json`.

## 1. The picture

> A realistic, photo-like picture of a cozy, tidy living room, seen straight
> on at eye level, landscape, 1536 × 1024. Warm natural daylight and soft
> shadows, like a modern furniture catalogue, in the same style as the
> kitchen. No people, no text, no logos, no clutter.
>
> From left to right along the back wall:
>
> - a tall wooden bookshelf full of books;
> - a fabric armchair in front of it, a little to the right, and a floor
>   lamp with a fabric shade beside the armchair;
> - in the middle, a three-seat couch with two throw pillows, and above it
>   a window with a curtain on each side;
> - right of the couch, a potted plant standing on the floor;
> - a TV on a low cabinet, with one framed picture on the wall above it;
> - at the far right, a closed white door.
>
> In front of the couch, a low wooden coffee table on a rug. On the table,
> one closed book lying flat and one vase with a few flowers. A cat sleeps
> curled up on the rug.
>
> Keep every one of these things clearly visible and easy to tell apart,
> each big enough to point at with a finger, and add nothing else on the
> floor, the table or the walls.

## 2. The boxes

> Now list each of those things with its bounding box in the picture, as
> JSON: `scene_id` "living-room", the image's size, and `items`, each
> with `id`, `name`, `bbox` (x, y, width, height) and `bbox_xyxy`, in
> pixels from the top-left. Use these ids: window,
> curtain, picture, door, bookshelf, lamp, couch, pillow, tv, plant, rug,
> armchair, coffee-table, book, vase, flower, cat. Give each curtain and
> each pillow its own item.
