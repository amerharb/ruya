#!/usr/bin/env python3
"""Ruya's vegetable stand, drawn: writes public/scenes/vegetable-stand/basic.svg
and the basic style's regions in scenes/vegetable-stand.json.

A crate of each vegetable under a green-and-white awning: the big ones on
top, the ones sold by the piece in the middle, the small ones at the bottom.
The stand itself, the regions and the writing are tools/stand.py's, shared
with the fruit stand; this says which vegetable goes where and how each is
drawn.

Run it from anywhere, `python3 art/vegetable-stand/basic.py`, after changing
a vegetable or the layout; it needs nothing beyond Python.
"""
import math
import random
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[2] / "tools"))
from stand import draw, f, pyramid, quad  # noqa: E402


def along(s, c, t, width, n=16):
    """The outline of a long thing along the curve s-c-t, `width(u)` its
    width a fraction u of the way from s to t."""
    centre = quad(s, c, t, n)
    one, other = [], []
    for k, (x, y) in enumerate(centre):
        a, b = centre[max(k - 1, 0)], centre[min(k + 1, n)]
        dx, dy = b[0] - a[0], b[1] - a[1]
        ln = math.hypot(dx, dy)
        nx, ny = -dy / ln, dx / ln
        w = width(k / n) / 2
        one.append((x - nx * w, y - ny * w))
        other.append((x + nx * w, y + ny * w))
    return one + other[::-1]


def star(x, y, r_out, r_in, points, squash=1.0, turn=-90):
    """A star's points round (x, y), flattened by `squash` up and down."""
    out = []
    for k in range(points * 2):
        a = math.radians(turn + 180 * k / points)
        r = r_out if k % 2 == 0 else r_in
        out.append((x + r * math.cos(a), y + r * math.sin(a) * squash))
    return out


def turned(x, y, dx, dy, rot):
    """The point (dx, dy) from (x, y), turned by `rot` degrees about it."""
    a = math.radians(rot)
    return x + dx * math.cos(a) - dy * math.sin(a), y + dx * math.sin(a) + dy * math.cos(a)


# --- the top tier: the big ones -------------------------------------------


def pumpkin(p, x, y, rx, ry):
    top = y - ry
    p.poly([(x - 5, top + 6), (x - 4, top - 10), (x + 3, top - 18), (x + 8, top - 15), (x + 5, top + 6)],
           "#6b5a2a")
    p.ellipse(x - rx * 0.45, y, rx * 0.55, ry, "#e57d1c")
    p.ellipse(x + rx * 0.45, y, rx * 0.55, ry, "#e57d1c")
    p.ellipse(x, y + 1, rx * 0.5, ry * 1.02, "#f39a33")
    p.ellipse(x - rx * 0.14, y - ry * 0.3, rx * 0.12, ry * 0.38, "#f7b052", hull=False)


def pumpkins(p, cx, rim):
    pumpkin(p, cx - 18, rim - 60, 50, 40)
    pumpkin(p, cx - 36, rim - 14, 34, 27)
    pumpkin(p, cx + 34, rim - 20, 38, 31)


def cabbage(p, x, y, r):
    # a tight head, two outer leaves cupping it from below, veins fanning
    # out from where they meet
    p.circle(x, y, r, "#c3e09a")
    for side in (-1, 1):
        p.ellipse(x + side * r * 0.42, y + r * 0.3, r * 0.7, r * 0.6, ("#9cc86f", "#8dbd61")[side > 0],
                  rot=side * 28)
    base = (x, y + r * 0.8)
    for ang in (-150, -125, -100, -80, -55, -30):
        a = math.radians(ang)
        reach = r * (0.75 if ang in (-100, -80) else 0.95)
        end = (base[0] + reach * math.cos(a), base[1] + reach * math.sin(a))
        p.path(f"M{f(base[0])} {f(base[1])}L{f(end[0])} {f(end[1])}", stroke="#d9edc0", width=2.2)
    p.path(f"M{f(x - r * 0.55)} {f(y - r * 0.55)}Q{f(x)} {f(y - r * 0.9)} {f(x + r * 0.55)} {f(y - r * 0.55)}",
           stroke="#aed283", width=2.5)


def cabbages(p, cx, rim):
    cabbage(p, cx + 20, rim - 66, 32)
    cabbage(p, cx - 34, rim - 24, 33)
    cabbage(p, cx + 36, rim - 18, 31)


def eggplant(p, x, y, rot):
    # the round end ahead, narrowing back to a green cap and its stalk
    p.ellipse(*turned(x, y, -4, 0, rot), 22, 13, "#4b2a5e", rot=rot)
    p.circle(*turned(x, y, 12, 1, rot), 15, "#4b2a5e")
    p.ellipse(*turned(x, y, 4, -6, rot), 12, 3.5, "#74518c", rot=rot, hull=False)
    cap_x, cap_y = turned(x, y, -24, 0, rot)
    p.poly(star(cap_x, cap_y, 10, 4, 5, 0.8, rot), "#5c8a3a", hull=False)
    sx, sy = turned(x, y, -38, -2, rot)
    p.path(f"M{f(cap_x)} {f(cap_y)}L{f(sx)} {f(sy)}", stroke="#5c7a3a", width=4, pts=[(sx, sy)])


def eggplants(p, cx, rim):
    for i, (dx, dy) in enumerate(pyramid([2, 2, 1], 68, 27, rim - 10)):
        eggplant(p, cx + dx + 6, dy, (24, -20, 20, -24, 12)[i])


def floret(p, x, y, r, parts, colours):
    for k, (dx, dy, s) in enumerate(parts):
        p.circle(x + dx * r, y + dy * r, s * r, colours[k % len(colours)])


CAULI = [(-0.62, 0.08, 0.38), (0.62, 0.08, 0.38), (-0.45, -0.32, 0.4), (0.45, -0.32, 0.4), (0, -0.52, 0.42),
         (-0.22, 0.02, 0.42), (0.22, 0.02, 0.42)]


def cauliflower(p, x, y, r):
    for ang, colour in ((200, "#6f9e4c"), (340, "#6f9e4c"), (240, "#7fae5a"), (300, "#7fae5a")):
        a = math.radians(ang)
        p.ellipse(x + r * 0.62 * math.cos(a), y + r * 0.35 - r * 0.3 * math.sin(a), r * 0.5, r * 0.24,
                  colour, rot=ang - 180 if ang < 270 else ang - 360)
    p.ellipse(x, y - r * 0.1, r * 0.9, r * 0.6, "#e6dcc4", hull=False)
    floret(p, x, y, r, CAULI, ("#f4eedd", "#ece3cc", "#f7f2e4"))


def cauliflowers(p, cx, rim):
    cauliflower(p, cx + 28, rim - 62, 33)
    cauliflower(p, cx - 30, rim - 22, 33)
    cauliflower(p, cx + 34, rim - 10, 30)


BROC = [(-0.6, 0.1, 0.36), (0.6, 0.1, 0.36), (-0.4, -0.3, 0.4), (0.4, -0.3, 0.4), (0, -0.5, 0.42),
        (-0.18, 0.05, 0.4), (0.2, 0.05, 0.4)]


def broccoli(p, x, y, r):
    p.poly([(x - 8, y + r * 0.2), (x - 11, y + r * 1.4), (x + 11, y + r * 1.4), (x + 8, y + r * 0.2)], "#93c46c")
    p.poly([(x - 8, y + r * 0.5), (x - 22, y + r * 0.35), (x - 9, y + r * 0.75)], "#7fb45a", hull=False)
    floret(p, x, y, r, BROC, ("#3f7f3a", "#4f9446", "#367234"))
    for dx, dy in ((-0.4, -0.35), (0.05, -0.55), (0.42, -0.32), (-0.15, 0.0), (0.3, 0.05)):
        p.circle(x + dx * r, y + dy * r, 2.4, "#63a85a", hull=False)


def broccolis(p, cx, rim):
    broccoli(p, cx + 2, rim - 82, 30)
    broccoli(p, cx - 38, rim - 40, 30)
    broccoli(p, cx + 38, rim - 36, 30)


def cob(p, x, y, rot):
    # husks behind, the cob, then its kernels row by row
    for side, colour in ((-1, "#7fb04f"), (1, "#93c260")):
        s = turned(x, y, 0, 40, rot)
        c = turned(x, y, side * 34, 18, rot)
        t = turned(x, y, side * 10, -30, rot)
        p.poly(along(s, c, t, lambda u: 18 * (1 - u) ** 0.8 + 1), colour)
    p.ellipse(x, y, 13, 44, "#f4cf3e", rot=rot)
    for row in range(-5, 6):
        for col in (-1, 0, 1):
            dx, dy = col * 7 + (3.5 if row % 2 else 0) - 1.75, row * 7.4
            if (dx / 11) ** 2 + (dy / 41) ** 2 < 1:
                p.circle(*turned(x, y, dx, dy, rot), 2.4, "#e2b22a", hull=False)


def corn(p, cx, rim):
    for dx, rot in ((-46, -22), (-15, -7), (15, 7), (46, 22)):
        cob(p, cx + dx, rim - 46, rot)


def artichoke(p, x, y, r):
    # an egg of overlapping bracts, the inner ones at the top drawn first,
    # each tipped with purple
    p.poly([(x - 5, y + r * 0.8), (x - 6, y + r * 1.5), (x + 6, y + r * 1.5), (x + 5, y + r * 0.8)], "#7fa35a")
    p.ellipse(x, y, r * 0.95, r * 1.1, "#4f7a3a")
    rows = [(-0.62, 2, 0.62), (-0.25, 3, 0.62), (0.15, 3, 0.66), (0.55, 4, 0.5)]
    for row, (dy, n, w) in enumerate(rows):
        for k in range(n):
            sx = x + (k - (n - 1) / 2) * w * r
            sy = y + dy * r
            half = w * r * 0.6
            p.poly([(sx - half, sy + r * 0.2), (sx - half * 0.85, sy - r * 0.12), (sx - half * 0.4, sy - r * 0.34),
                    (sx, sy - r * 0.42), (sx + half * 0.4, sy - r * 0.34), (sx + half * 0.85, sy - r * 0.12),
                    (sx + half, sy + r * 0.2)], ("#8fbf63", "#7aaf52")[(row + k) % 2], hull=False)
            p.poly([(sx - 4, sy - r * 0.3), (sx, sy - r * 0.46), (sx + 4, sy - r * 0.3)], "#8b5a8c", hull=False)


def artichokes(p, cx, rim):
    for (dx, dy) in pyramid([3, 2], 46, 40, rim - 16):
        artichoke(p, cx + dx, dy, 22)


def lettuce(p, x, y, r):
    p.circle(x, y, r * 0.95, "#86bd4f")
    for k in range(11):
        a = 2 * math.pi * k / 11
        p.circle(x + r * 0.78 * math.cos(a), y + r * 0.7 * math.sin(a), r * 0.34,
                 ("#9fd166", "#8cc458")[k % 2])
    p.circle(x, y - r * 0.08, r * 0.55, "#c4e38f")
    p.path(f"M{f(x)} {f(y + r * 0.45)}Q{f(x - r * 0.1)} {f(y)} {f(x - r * 0.05)} {f(y - r * 0.5)}",
           stroke="#e3f2c8", width=2.5)
    p.path(f"M{f(x - r * 0.45)} {f(y + r * 0.2)}Q{f(x - r * 0.2)} {f(y - r * 0.1)} {f(x - r * 0.05)} {f(y - r * 0.05)}",
           stroke="#e3f2c8", width=2)


def lettuces(p, cx, rim):
    lettuce(p, cx + 14, rim - 64, 31)
    lettuce(p, cx - 32, rim - 22, 32)
    lettuce(p, cx + 36, rim - 18, 31)


# --- the middle tier: sold by the piece -----------------------------------


def tomatoes(p, cx, rim):
    for (dx, dy) in pyramid([4, 3, 2, 1], 34, 30, rim - 8):
        x, y = cx + dx, dy
        p.circle(x, y, 20, "#e2402f")
        p.circle(x - 7, y - 6, 5, "#ee6c58", hull=False)
        p.poly(star(x + 1, y - 16, 10, 3.5, 5, 0.55), "#3f8a3a", hull=False)


def long_ones(p, cx, rim, body, mark, end, specks):
    """Cucumbers or zucchini, lying in rows, an end of each paler."""
    for i, (dx, dy, rot) in enumerate([(-35, -10, 3), (35, -10, -4), (-34, -32, -5), (35, -33, 5),
                                       (-30, -54, 2), (30, -55, -3), (0, -76, 1)]):
        x, y = cx + dx, rim + dy
        p.ellipse(x, y, 34, 11, body, rot=rot)
        ex, ey = turned(x, y, (30 if i % 2 else -30), 0, rot)
        p.ellipse(ex, ey, 5, 8, end, rot=rot, hull=False)
        for k in range(specks):
            sx, sy = turned(x, y, -20 + 40 * k / max(specks - 1, 1), -4 + 3 * (k % 2), rot)
            p.ellipse(sx, sy, 2.2, 1.2, mark, rot=rot, hull=False)


def cucumbers(p, cx, rim):
    long_ones(p, cx, rim, "#5aa043", "#3c7a2e", "#a3cf72", 6)


def zucchini(p, cx, rim):
    long_ones(p, cx, rim, "#2f5e2a", "#7aa765", "#b9c27c", 5)


def carrots(p, cx, rim):
    for i, dx in enumerate((-56, -34, -12, 10, 32, 54)):
        top = (cx + dx, rim - 66 - (8 if i % 2 else 0))
        tip = (cx + dx * 0.55, rim + 40)
        mid = ((top[0] + tip[0]) / 2 + (4 if i % 2 else -4), (top[1] + tip[1]) / 2)
        for k, (lx, ly) in enumerate(((-16, -40), (-4, -50), (10, -42))):
            leaf = (top[0] + lx, top[1] + ly)
            p.poly(along(top, ((top[0] + leaf[0]) / 2 + 4, (top[1] + leaf[1]) / 2), leaf,
                         lambda u: 7 * math.sin(math.pi * min(u + 0.15, 1)) + 1),
                   ("#5c9a4a", "#6fb07f", "#4f8a40")[k])
        p.poly(along(top, mid, tip, lambda u: 19 * (1 - u) ** 0.85 + 1.5), "#ef8a2a")
        for k in range(3):
            y0 = top[1] + 14 + k * 13
            x0 = top[0] + (tip[0] - top[0]) * (y0 - top[1]) / (tip[1] - top[1])
            p.path(f"M{f(x0 - 4)} {f(y0)}L{f(x0 + 4)} {f(y0 + 1)}", stroke="#d6701c", width=2)


def potatoes(p, cx, rim):
    for (dx, dy) in pyramid([4, 3, 2, 1], 35, 27, rim - 6):
        x, y = cx + dx, dy
        rot = random.uniform(-25, 25)
        p.ellipse(x, y, 20, 16, "#c9a066", rot=rot)
        p.ellipse(x - 5, y - 6, 7, 3.5, "#dbb983", rot=rot, hull=False)
        for _ in range(3):
            ex, ey = x + random.uniform(-12, 12), y + random.uniform(-8, 8)
            p.ellipse(ex, ey, 2, 1.3, "#9a7442", rot=random.uniform(0, 180), hull=False)


def pepper(p, x, y, colour, shade, light):
    p.path(f"M{f(x + 1)} {f(y - 19)}Q{f(x + 2)} {f(y - 28)} {f(x + 9)} {f(y - 31)}", stroke="#4f7a2a",
           width=5, pts=[(x + 9, y - 31)])
    p.ellipse(x - 11, y + 2, 12, 19, shade)
    p.ellipse(x + 11, y + 2, 12, 19, shade)
    p.ellipse(x, y - 12, 18, 9, shade)
    p.ellipse(x, y + 3, 12, 20, colour)
    p.ellipse(x - 4, y - 3, 3, 8, light, hull=False)


def peppers(p, cx, rim):
    colours = [("#e0392b", "#bf2b20", "#ec6a5c"), ("#f5c73a", "#dcae27", "#f9dc78"),
               ("#4f9a3a", "#3e812e", "#76b55e")]
    for i, (dx, dy) in enumerate(pyramid([3, 2, 1], 46, 36, rim - 12)):
        pepper(p, cx + dx, dy, *colours[i % 3])


def onions(p, cx, rim):
    for (dx, dy) in pyramid([4, 3, 2, 1], 34, 30, rim - 8):
        x, y = cx + dx, dy
        p.poly([(x - 6, y - 14), (x, y - 30), (x + 6, y - 14)], "#b8803a")
        p.circle(x, y, 19, "#d39a48")
        p.path(f"M{f(x - 8)} {f(y - 15)}Q{f(x - 14)} {f(y)} {f(x - 6)} {f(y + 15)}", stroke="#b98236", width=2)
        p.path(f"M{f(x + 7)} {f(y - 16)}Q{f(x + 12)} {f(y)} {f(x + 6)} {f(y + 16)}", stroke="#b98236", width=2)
        p.circle(x - 3, y - 6, 4.5, "#e4b265", hull=False)


def sweet_potatoes(p, cx, rim):
    for (dx, dy, rot) in [(-35, -14, 6), (35, -15, -8), (-30, -40, -4), (32, -42, 8), (0, -66, 3)]:
        x, y = cx + dx, rim + dy
        s, t = turned(x, y, -33, 0, rot), turned(x, y, 33, 0, rot)
        for end, tail in ((s, turned(x, y, -40, 2, rot)), (t, turned(x, y, 41, -2, rot))):
            p.path(f"M{f(end[0])} {f(end[1])}L{f(tail[0])} {f(tail[1])}", stroke="#7e3038", width=2.5,
                   pts=[tail])
        p.poly(along(s, turned(x, y, 0, -2, rot), t, lambda u: 32 * math.sin(math.pi * u) ** 0.6 + 3),
               "#9c3f48")
        s2, t2 = turned(x, y, -18, -7, rot), turned(x, y, 16, -8, rot)
        p.poly(along(s2, turned(x, y, 0, -10, rot), t2, lambda u: 5 * math.sin(math.pi * u) + 0.5), "#bd5a62",
               hull=False)


# --- the bottom tier: the small ones --------------------------------------


def radishes(p, cx, rim):
    centres = pyramid([5, 4, 3], 27, 22, rim - 6)
    for i, (dx, dy) in enumerate(centres):
        x, y = cx + dx, dy
        if i < 3:
            for lx, rot in ((-6, -18), (5, 14)):
                p.ellipse(x + lx, y - 26, 6, 14, "#5c9a4a", rot=rot)
        p.ellipse(x, y + 12, 4, 6, "#f4f0ea", hull=False)
        p.circle(x, y, 13, "#d93a4f")
        p.circle(x - 4, y - 4, 3.5, "#ea6b7c", hull=False)


def peas(p, cx, rim):
    rows = [(-46, -8, 8), (0, -10, -6), (46, -8, 6), (-24, -26, -8), (24, -27, 7), (-46, -44, 5),
            (0, -45, -5), (46, -43, -7), (-22, -62, 6), (24, -63, -6)]
    for (dx, dy, rot) in rows:
        x, y = cx + dx, rim + dy
        s, t = turned(x, y, -25, 0, rot), turned(x, y, 25, 0, rot)
        c = turned(x, y, 0, 8, rot)
        p.poly(along(s, c, t, lambda u: 17 * math.sin(math.pi * u) ** 0.6 + 1), "#8cc95a")
        p.path(f"M{f(s[0])} {f(s[1])}Q{f(c[0])} {f(c[1] - 7)} {f(t[0])} {f(t[1])}", stroke="#6fa845", width=2)
    # one open, its peas in a row
    x, y = cx + 2, rim - 82
    s, t = (x - 30, y + 2), (x + 30, y - 2)
    p.poly(along(s, (x, y + 14), t, lambda u: 22 * math.sin(math.pi * u) ** 0.6 + 1), "#6fa845")
    for k in range(5):
        p.circle(x - 20 + 10 * k, y + 3 + (2 if k in (1, 3) else 0) - (2 if k == 2 else 0), 5.4, "#a8dc6a",
                 hull=False)


def mushrooms(p, cx, rim):
    for (dx, dy) in pyramid([4, 3, 2, 1], 34, 26, rim - 6):
        x, y = cx + dx, dy
        p.poly([(x - 7, y), (x - 8, y + 16), (x + 8, y + 16), (x + 7, y)], "#efe6d6")
        p.path(f"M{f(x - 19)} {f(y + 2)}A19 15 0 0 1 {f(x + 19)} {f(y + 2)}Z", fill="#b9875a",
               pts=[(x + 19 * math.cos(a), y + 2 - 15 * math.sin(a)) for a in
                    (math.pi * k / 12 for k in range(13))])
        p.path(f"M{f(x - 19)} {f(y + 2)}Q{f(x)} {f(y + 8)} {f(x + 19)} {f(y + 2)}Z", fill="#e3cfae")
        p.ellipse(x - 6, y - 7, 6, 3, "#cfa279", hull=False)


def chilies(p, cx, rim):
    for row, (y0, n) in enumerate(((-8, 3), (-24, 3), (-40, 3), (-56, 2), (-72, 2))):
        for k in range(n):
            x = cx + (k - (n - 1) / 2) * 43 + random.uniform(-3, 3)
            y = rim + y0 + random.uniform(-3, 3)
            rot = random.uniform(-25, 25) + (180 if (row + k) % 2 else 0)
            s, t = turned(x, y, -24, 0, rot), turned(x, y, 24, 0, rot)
            c = turned(x, y, 0, 9, rot)
            p.poly(along(s, c, t, lambda u: 11 * (1 - u ** 1.6) + 1.2), ("#d42a2a", "#b81f1f")[(row + k) % 2])
            st = turned(x, y, -33, -3, rot)
            p.path(f"M{f(s[0])} {f(s[1])}L{f(st[0])} {f(st[1])}", stroke="#4f8a3a", width=4, pts=[st])


def asparagus(p, cx, rim):
    for bx in (-44, 0, 44):
        x0 = cx + bx
        for k in range(6):
            off = (k - 2.5) * 6.2
            base = (x0 + off * 0.9, rim + 30)
            tip = (x0 + off * 1.25, rim - 102 + abs(k - 2.5) * 5)
            p.poly([(base[0] - 3.4, base[1]), (tip[0] - 3.4, tip[1] + 8), (tip[0] + 3.4, tip[1] + 8),
                    (base[0] + 3.4, base[1])], ("#78ad52", "#6a9f47")[k % 2])
            p.ellipse(tip[0], tip[1] + 4, 4.2, 8.5, "#5a8a3e")
            p.ellipse(tip[0], tip[1] + 1, 2.4, 4, "#7a5a7a", hull=False)
            for j in range(2):
                sy = tip[1] + 30 + j * 26
                sx = tip[0] + (base[0] - tip[0]) * (sy - tip[1]) / (base[1] - tip[1])
                p.poly([(sx - 3.4, sy), (sx, sy - 6), (sx + 3.4, sy)], "#5a8a3e", hull=False)
        p.poly([(x0 - 20, rim - 34), (x0 + 20, rim - 34), (x0 + 20, rim - 24), (x0 - 20, rim - 24)], "#c94a3a",
               hull=False)


def garlic(p, cx, rim):
    for (dx, dy) in pyramid([4, 3, 2, 1], 34, 28, rim - 6):
        x, y = cx + dx, dy
        p.poly([(x - 5, y - 15), (x, y - 30), (x + 5, y - 15)], "#e3d9c8")
        p.circle(x, y, 19, "#c9bba6")
        p.circle(x, y, 17.5, "#f4efe6")
        for sx in (-8, 0, 8):
            p.path(f"M{f(x + sx * 0.6)} {f(y - 15)}Q{f(x + sx * 1.6)} {f(y)} {f(x + sx * 0.7)} {f(y + 15)}",
                   stroke="#ddd2c2", width=2)
        p.path(f"M{f(x - 4)} {f(y - 12)}Q{f(x - 9)} {f(y)} {f(x - 4)} {f(y + 12)}", stroke="#c9a6c4", width=2.5)


def okra(p, cx, rim):
    for row, (y0, n) in enumerate(((-8, 3), (-25, 3), (-42, 3), (-59, 2), (-76, 2))):
        for k in range(n):
            x = cx + (k - (n - 1) / 2) * 43 + random.uniform(-3, 3)
            y = rim + y0 + random.uniform(-3, 3)
            rot = random.uniform(-30, 30) + (180 if (row + k) % 2 else 0)
            s, t = turned(x, y, -24, 0, rot), turned(x, y, 24, 0, rot)
            c = turned(x, y, 0, 3, rot)
            p.poly(along(s, c, t, lambda u: 15 * (1 - u) ** 0.8 + 1.5), "#5f9a3c")
            r1, r2 = turned(x, y, -16, 0, rot), turned(x, y, 18, 1, rot)
            p.path(f"M{f(r1[0])} {f(r1[1])}L{f(r2[0])} {f(r2[1])}", stroke="#4a7f2e", width=2)
            p.ellipse(*turned(x, y, -25, 0, rot), 5, 7.5, "#7fae55", rot=rot, hull=False)
            st = turned(x, y, -33, 0, rot)
            p.path(f"M{f(s[0])} {f(s[1])}L{f(st[0])} {f(st[1])}", stroke="#6b8a3a", width=4, pts=[st])


def beets(p, cx, rim):
    centres = pyramid([4, 3, 2, 1], 34, 28, rim - 8)
    for i, (dx, dy) in enumerate(centres):
        x, y = cx + dx, dy
        if i < 3:
            for lx, rot, h in ((-5, -16, 18), (6, 18, 16)):
                stem_top = (x + lx * 2.2, y - 18 - h)
                p.path(f"M{f(x)} {f(y - 14)}L{f(stem_top[0])} {f(stem_top[1])}", stroke="#b0304f", width=3)
                p.ellipse(stem_top[0], stem_top[1] - 6, 6, 11, "#4f8a3a", rot=rot)
        p.poly([(x - 4, y + 14), (x + 2, y + 30), (x + 4, y + 14)], "#6a1a35", hull=False)
        p.circle(x, y, 18, "#7a1f3d")
        p.circle(x - 5, y - 6, 4.5, "#9a3a5a", hull=False)


DRAW = {
    "pumpkin": pumpkins, "cabbage": cabbages, "eggplant": eggplants, "cauliflower": cauliflowers,
    "broccoli": broccolis, "corn": corn, "artichoke": artichokes, "lettuce": lettuces,
    "tomato": tomatoes, "cucumber": cucumbers, "carrot": carrots, "potato": potatoes,
    "bell-pepper": peppers, "zucchini": zucchini, "onion": onions, "sweet-potato": sweet_potatoes,
    "radish": radishes, "pea": peas, "mushroom": mushrooms, "chili-pepper": chilies,
    "asparagus": asparagus, "garlic": garlic, "okra": okra, "beet": beets,
}

COMMENTS = {
    "pumpkin": "pumpkins: three, ribbed, each with its stalk",
    "cabbage": "cabbages: three heads, wrapped in their outer leaves",
    "eggplant": "eggplants: five, dark purple, green at the stalk",
    "cauliflower": "cauliflowers: three white heads, leaves round them",
    "broccoli": "broccoli: three heads on their stalks",
    "corn": "corn: four ears, their husks open",
    "artichoke": "artichokes: five, scaled, purple at the tips",
    "lettuce": "lettuces: three heads, ruffled",
    "tomato": "tomatoes: ten, a green star on each",
    "cucumber": "cucumbers: seven, lying in rows",
    "carrot": "carrots: six, standing, their leaves up",
    "potato": "potatoes: ten",
    "bell-pepper": "bell peppers: six, red, yellow and green",
    "zucchini": "zucchini: seven, dark green, lying in rows",
    "onion": "onions: ten, golden",
    "sweet-potato": "sweet potatoes: five, long and red",
    "radish": "radishes: twelve, leaves on the top three",
    "pea": "peas: ten pods, and one open on top",
    "mushroom": "mushrooms: ten",
    "chili-pepper": "chili peppers: a heap of them",
    "asparagus": "asparagus: three bunches, tied",
    "garlic": "garlic: ten heads",
    "okra": "okra: a heap of them",
    "beet": "beets: ten, leaves on the top three",
}

TIERS = [
    ["pumpkin", "cabbage", "eggplant", "cauliflower", "broccoli", "corn", "artichoke", "lettuce"],
    ["tomato", "cucumber", "carrot", "potato", "bell-pepper", "zucchini", "onion", "sweet-potato"],
    ["radish", "pea", "mushroom", "chili-pepper", "asparagus", "garlic", "okra", "beet"],
]

draw(
    "vegetable-stand",
    "Drawn for Ruya: a vegetable stand at a market, seen straight on: three tiers of crates under a "
    "striped awning, one vegetable to a crate, big ones at the top and small ones at the bottom. No "
    "text in it, so it serves every language. Made by art/vegetable-stand/basic.py, which also writes "
    "each vegetable's region in vegetable-stand.json: change that and run it, rather than this file.",
    TIERS,
    DRAW,
    COMMENTS,
    ("green", "#79b363", "#4f8a44"),
)
