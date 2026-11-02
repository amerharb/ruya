# The street, realistic: the prompt for ChatGPT

Written 2026-11-02, to match the realistic kitchen and living room and the
things in `scenes/street.json`; the hand-drawn `public/scenes/street-basic.svg`
has the same street in the same order. Best asked in the chat that made the
living room, so the look carries over; in a new one, attach
`art/living-room-realistic.png` with it. Paste the first part, then the
second in the same chat once the picture is right. Put the picture and the
answer to the second part in `-AmerLocal/`; they become
`art/street-realistic.png` and `art/street-realistic.boxes.json`.

## 1. The picture

> A realistic, photo-like picture of a quiet, tidy town street on a sunny
> day, seen straight on from across the road at eye level, landscape,
> 1536 × 1024. Warm natural daylight and soft shadows, in the same style as
> the kitchen and the living room. No people, no text, no writing on any
> sign or shop, no logos and no number plates.
>
> From left to right along the far sidewalk:
>
> - a small house with a pitched tiled roof and a chimney, and a bicycle
>   parked in front of it;
> - a tree, with a park bench in front of it;
> - a streetlight, and beside it a small trash can;
> - a small store with a striped awning over its shop window, and a dog
>   standing on the sidewalk in front of it;
> - a blue mailbox for posting letters;
> - at the far right, a traffic light at the corner.
>
> In the road, nearer the viewer: a car in the far lane, a bus stopped in
> the near lane, and a crosswalk across the road at the right, in front of
> the traffic light. In the sky: the sun at the top right, one white cloud
> and one bird flying.
>
> Keep every one of these things clearly visible and easy to tell apart,
> each big enough to point at with a finger, and add nothing else on the
> sidewalk, the road or in the sky.

## 2. The boxes

> Now list each of those things with its bounding box in the picture, as
> JSON: `scene_id` "street", the image's size, and `items`, each with `id`,
> `name`, `bbox` (x, y, width, height) and `bbox_xyxy`, in pixels from the
> top-left. Use these ids: sun, cloud, bird, house, roof, tree,
> streetlight, store, traffic-light, sidewalk, bicycle, bench, trash-can,
> dog, mailbox, crosswalk, car, bus. Give the roof its own item as well as
> the house.
