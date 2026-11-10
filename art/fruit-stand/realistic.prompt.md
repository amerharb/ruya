# The fruit stand, realistic: the prompt for ChatGPT

Written 2026-11-09, to match the realistic kitchen, living room and street
and the fruit in `scenes/fruit-stand.json`; the drawn
`public/scenes/fruit-stand/basic.svg` has the same stand in the same order.
Best asked in the chat that made the street, so the look carries over; in a
new one, attach `art/street/realistic.png` with it. Paste the first part,
then the second in the same chat once the picture is right. Put the picture
and the answer to the second part in `-AmerLocal/`; they become
`art/fruit-stand/realistic.png` and `art/fruit-stand/realistic.boxes.json`.

## 1. The picture

> A realistic, photo-like picture of a fruit stand at an outdoor market on
> a sunny morning, seen straight on at eye level, landscape, 1536 × 1024.
> Warm natural daylight and soft shadows, in the same style as the kitchen,
> the living room and the street. A red-and-white striped awning across the
> top, and under it three tiers of wooden crates, eight crates to a tier,
> each crate heaped with one kind of fruit only. No people, no text, no
> price signs, labels or stickers, no logos, and no bags or baskets.
>
> The top tier, from left to right: pineapples standing upright; a whole
> watermelon and half of one cut open, its red side to the viewer; hands of
> yellow bananas; coconuts, one cracked open, white inside; cantaloupe
> melons, one cut in half, orange inside; red-and-yellow mangoes; papayas,
> one cut down its length, its black seeds showing; pomegranates, one cut
> open, its red seeds showing.
>
> The middle tier, from left to right: apricots; red apples; lemons; dark
> purple plums; oranges; green pears; tangerines, a few with a leaf;
> peaches.
>
> The bottom tier, from left to right: strawberries; blueberries; bunches
> of green grapes; dark red cherries with their stems; kiwis, a few cut in
> half, green inside; raspberries; purple figs, one cut in half; dates.
>
> Keep every crate's fruit clearly visible and easy to tell from its
> neighbours', heaped so it shows above the crate's rim, and add nothing
> else on the stand or around it.

## 2. The boxes

> Now list each crate with its bounding box in the picture, as JSON:
> `scene_id` "fruit-stand", the image's size, and `items`, each with `id`,
> `name`, `bbox` (x, y, width, height) and `bbox_xyxy`, in pixels from the
> top-left. A crate's box takes in its fruit and the crate together. Use
> these ids, the top tier first, each tier from left to right: pineapple,
> watermelon, banana, coconut, melon, mango, papaya, pomegranate, apricot,
> apple, lemon, plum, orange, pear, tangerine, peach, strawberry,
> blueberry, grape, cherry, kiwi, raspberry, fig, date.
