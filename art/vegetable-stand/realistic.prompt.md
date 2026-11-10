# The vegetable stand, realistic: the prompt for ChatGPT

Written 2026-11-09, to match the realistic fruit stand and the vegetables
in `scenes/vegetable-stand.json`; the drawn
`public/scenes/vegetable-stand/basic.svg` has the same stand in the same
order. Best asked in the chat that made the fruit stand, so the stand and
the street carry over; in a new one, attach `art/fruit-stand/realistic.png`
with it. Paste the first part, then the second in the same chat once the
picture is right. Put the picture and the answer to the second part in
`-AmerLocal/`; they become `art/vegetable-stand/realistic.png` and
`art/vegetable-stand/realistic.boxes.json`.

## 1. The picture

> A realistic, photo-like picture of a vegetable stand at an outdoor market
> on a sunny morning: the same kind of stand as the fruit stand, on the
> same cobbled street and seen the same way, landscape, 1536 × 1024, in the
> same style, but its awning striped green and white. Three tiers of wooden
> crates, eight crates to a tier, each crate heaped with one kind of
> vegetable only. No people, no text, no price signs, labels or stickers,
> no logos, and no bags or baskets.
>
> The top tier, from left to right: pumpkins; round heads of cabbage; dark
> purple eggplants; white heads of cauliflower in their leaves; heads of
> broccoli; ears of corn, their husks pulled open; artichokes; heads of
> lettuce.
>
> The middle tier, from left to right: red tomatoes; cucumbers; carrots
> with their green tops; potatoes; red, yellow and green bell peppers; dark
> green zucchini; brown onions; sweet potatoes.
>
> The bottom tier, from left to right: red radishes with their leaves; pea
> pods, one open, its peas showing; button mushrooms; red chili peppers;
> bunches of green asparagus, tied; heads of garlic; okra; beets with their
> leaves.
>
> Keep every crate's vegetables clearly visible and easy to tell from its
> neighbours', heaped so they show above the crate's rim, and add nothing
> else on the stand or around it.

## 2. The boxes

> Now list each crate with its bounding box in the picture, as JSON:
> `scene_id` "vegetable-stand", the image's size, and `items`, each with
> `id`, `name`, `bbox` (x, y, width, height) and `bbox_xyxy`, in pixels
> from the top-left. A crate's box takes in its vegetables and the crate
> together. Use these ids, the top tier first, each tier from left to
> right: pumpkin, cabbage, eggplant, cauliflower, broccoli, corn,
> artichoke, lettuce, tomato, cucumber, carrot, potato, bell-pepper,
> zucchini, onion, sweet-potato, radish, pea, mushroom, chili-pepper,
> asparagus, garlic, okra, beet.
