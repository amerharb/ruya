#!/usr/bin/env python3
"""Ruya's fruit stand, drawn: writes public/scenes/fruit-stand/basic.svg
and the basic style's regions in scenes/fruit-stand.json.

A crate of each fruit under a red-and-white awning: big fruit on top, then
the tree fruit, then the berries. The stand itself, the regions and the
writing are tools/stand.py's, shared with the vegetable stand; this says
which fruit goes where and how each is drawn.

Run it from anywhere, `python3 art/fruit-stand/basic.py`, after changing a
fruit or the layout; it needs nothing beyond Python.
"""
import math
import random
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[2] / "tools"))
from stand import capped, draw, ellipse_chord, f, pyramid, quad  # noqa: E402


# --- the fruits, each drawn about its crate's centre x and its rim y ----


def pineapple(p, cx, rim):
    for (dx, dy) in [(0, -50), (-38, -34), (38, -34)]:
        x, y = cx + dx, rim + dy
        base = y - 44
        for k, a in enumerate([-38, -18, 0, 18, 38]):
            r = math.radians(a)
            length = 62 if a == 0 else 52 if abs(a) == 18 else 40
            w = 7
            tip = (x + length * math.sin(r), base - length * math.cos(r))
            bl = (x - w * math.cos(r) - 2, base + 6)
            br = (x + w * math.cos(r) + 2, base + 6)
            p.poly([bl, tip, br], "#5c9a6b" if k % 2 else "#6fb07f")
        p.ellipse(x, y, 30, 46, "#e0a84a")
        segs = []
        for ang in (55, -55):
            for off in range(-60, 61, 13):
                c = ellipse_chord(x, y, 30, 46, ang, off)
                if c:
                    segs.append(c)
        p.lines(segs, "#c08a34", 2.5)


def watermelon(p, cx, rim):
    x, y = cx - 14, rim - 46
    p.ellipse(x, y, 62, 44, "#3f7d4a")
    for k in (0.72, 0.4, 0.08):
        p.add(
            f'<ellipse cx="{f(x)}" cy="{f(y)}" rx="{f(60)}" ry="{f(44 * k)}" fill="none" '
            f'stroke="#6aa86b" stroke-width="6" />'
        )
    # a half, cut face to the viewer
    hx, hy = cx + 28, rim + 6
    for r, col in ((50, "#3f7d4a"), (45, "#eaf3d6"), (41, "#e2524a")):
        d = f"M{f(hx - r)} {f(hy)}A{f(r)} {f(r)} 0 0 1 {f(hx + r)} {f(hy)}Z"
        p.path(d, fill=col, pts=[(hx + r * math.cos(t), hy - r * math.sin(t)) for t in
                                 (math.pi * k / 16 for k in range(17))] if r == 50 else None)
    for rr, angs in ((28, (25, 55, 90, 125, 155)), (15, (60, 120))):
        for a in angs:
            t = math.radians(a)
            sx, sy = hx + rr * math.cos(t), hy - rr * math.sin(t)
            p.ellipse(sx, sy, 2.4, 4.2, "#2f2a26", rot=90 - a, hull=False)


def banana(p, s, c, t, width):
    """A banana along the curve s-c-t: fat in the middle, narrowing to its
    stalk at s and its dark tip at t, a darker band along its belly."""
    n = 16
    centre = quad(s, c, t, n)
    top, belly, band = [], [], []
    for k, (x, y) in enumerate(centre):
        a, b = centre[max(k - 1, 0)], centre[min(k + 1, n)]
        dx, dy = b[0] - a[0], b[1] - a[1]
        ln = math.hypot(dx, dy)
        nx, ny = -dy / ln, dx / ln
        w = width / 2 * (0.32 + 0.68 * math.sin(math.pi * k / n) ** 0.7)
        top.append((x - nx * w, y - ny * w))
        belly.append((x + nx * w, y + ny * w))
        band.append((x + nx * w * 0.35, y + ny * w * 0.35))
    p.poly(top + belly[::-1], "#f3d250")
    p.poly(band + belly[::-1], "#e2bc3e", hull=False)
    p.poly([(s[0] - 6, s[1] - 5), (s[0] + 6, s[1] - 5), (s[0] + 5, s[1] + 5), (s[0] - 5, s[1] + 5)],
           "#9a7a3a", hull=False)
    p.circle(t[0], t[1], 3.5, "#5e3e28", hull=False)


def bananas(p, cx, rim):
    # the hand behind: joined at the left, curving up to the right
    s = (cx - 58, rim - 34)
    for k in range(4):
        banana(p, (s[0], s[1] + 3 * k), (cx + 10, rim - 6 - 8 * k), (cx + 66, rim - 104 + 20 * k), 26)
    # the hand in front: joined at the right, curving up to the left
    s = (cx + 60, rim + 2)
    for k in range(3):
        banana(p, (s[0], s[1] + 3 * k), (cx - 4, rim + 22 - 6 * k), (cx - 66, rim - 58 + 20 * k), 26)


def coconuts(p, cx, rim):
    for (dx, dy) in [(0, -64), (-38, -16)]:
        x, y = cx + dx, rim + dy
        p.circle(x, y, 32, "#7a5236")
        segs = []
        for _ in range(9):
            a = random.uniform(0, 2 * math.pi)
            r0 = random.uniform(6, 22)
            ax, ay = x + r0 * math.cos(a), y + r0 * math.sin(a)
            b = a + random.uniform(-0.6, 0.6)
            segs.append(((ax, ay), (ax + 9 * math.cos(b), ay + 9 * math.sin(b))))
        p.lines(segs, "#5e3e28", 2.5)
    for ex, ey in ((-8, -80), (4, -84), (-2, -74)):
        p.circle(cx + ex, rim + ey, 3.4, "#4a3020", hull=False)
    # one cracked open, its white to the viewer
    x, y = cx + 38, rim - 18
    p.circle(x, y, 31, "#5e3e28")
    p.circle(x, y, 25, "#f7f3ea")
    p.circle(x, y, 15, "#e9e1cf")


def melons(p, cx, rim):
    for (dx, dy) in [(4, -62), (-36, -18)]:
        x, y = cx + dx, rim + dy
        p.ellipse(x, y, 40, 34, "#d9c48a")
        segs = []
        for ang in (50, -50):
            for off in range(-48, 49, 12):
                c = ellipse_chord(x, y, 40, 34, ang, off, 0.86)
                if c:
                    segs.append(c)
        p.lines(segs, "#bfa96c", 2)
    x, y = cx + 36, rim - 18
    p.ellipse(x, y, 40, 34, "#9cb86a")
    p.ellipse(x, y, 35, 29, "#f2a65a")
    p.ellipse(x, y, 17, 13, "#f6c47e")
    for sx, sy, rot in ((-9, -4, 30), (-2, -7, 0), (6, -5, -30), (-6, 5, -30), (2, 7, 0), (9, 4, 30)):
        p.ellipse(x + sx, y + sy, 2, 3.6, "#f6ead0", rot=rot, hull=False)


def mangoes(p, cx, rim):
    for i, (dx, dy) in enumerate(pyramid([3, 2], 48, 32, rim - 12)):
        x, y = cx + dx, dy
        rot = (-14, 12, -8, 16, -12)[i]
        a = math.radians(rot)
        p.ellipse(x, y, 25, 19, "#f5b63a", rot=rot)
        # red over the shoulder, green at the stalk
        d = (f"M{f(x - 25)} {f(y)}A25 19 0 0 1 {f(x + 25)} {f(y)}"
             f"Q{f(x + 4)} {f(y + 2)} {f(x - 25)} {f(y)}Z")
        p.add(f'<path d="{d}" fill="#e2524a" transform="rotate({f(rot)} {f(x)} {f(y)})" />')
        p.circle(x - 17 * math.cos(a), y - 17 * math.sin(a), 6, "#8fbf5a", hull=False)
        p.circle(x - 23 * math.cos(a), y - 23 * math.sin(a), 2.4, "#6b4a2f", hull=False)


def papayas(p, cx, rim):
    x, y = cx + 4, rim - 64
    capped(p, x, y, 52, 24, -8, "#f4b942", "#93b552", -26)
    a = math.radians(-8)
    p.circle(x - 52 * math.cos(a), y - 52 * math.sin(a), 3.5, "#6b4a2f", hull=False)
    x, y = cx - 4, rim - 20
    p.ellipse(x, y, 50, 25, "#9cb85a", rot=5)
    p.ellipse(x, y, 46, 21, "#f08a4b", rot=5)
    p.ellipse(x, y, 27, 9, "#f6b47a", rot=5, hull=False)
    a = math.radians(5)
    for k in range(-4, 5):
        for j in (-1, 1):
            ox, oy = k * 5.6, j * 3.2 + (2 if k % 2 else 0) - 1
            if abs(k) == 4 and j == 1:
                continue
            p.circle(x + ox * math.cos(a) - oy * math.sin(a), y + ox * math.sin(a) + oy * math.cos(a),
                     2.8, "#2f2a26", hull=False)


def pomegranates(p, cx, rim):
    for (dx, dy) in pyramid([3, 2, 1], 44, 34, rim - 14):
        x, y = cx + dx, dy
        p.poly([(x - 8, y - 19), (x - 9, y - 30), (x - 3, y - 25), (x, y - 33), (x + 3, y - 25),
                (x + 9, y - 30), (x + 8, y - 19)], "#8e2a20")
        p.circle(x, y, 23, "#c0392b")
        p.ellipse(x - 8, y - 8, 4, 7, "#d65a4a", rot=30, hull=False)
    # one cut, its seeds to the viewer
    x, y = cx + 2, rim - 8
    p.circle(x, y, 25, "#c0392b")
    p.circle(x, y, 21, "#f3d6c8")
    for row in range(-3, 4):
        for col in range(-3, 4):
            sx = col * 7.2 + (3.6 if row % 2 else 0)
            sy = row * 6.2
            if math.hypot(sx, sy) < 16:
                p.circle(x + sx, y + sy, 3.3, "#e74c5c", hull=False)


def blushed(p, x, y, r, rot, base, blush, crease, cap_x):
    """A round fruit blushed on one side, its crease along the blush's edge."""
    capped(p, x, y, r, r, rot, base, blush, cap_x)
    k = math.sqrt(1 - (cap_x / r) ** 2) * r
    d = f"M{f(x + cap_x)} {f(y - k)}Q{f(x + cap_x + 8)} {f(y)} {f(x + cap_x)} {f(y + k)}"
    p.add(f'<path d="{d}" fill="none" stroke="{crease}" stroke-width="2.5" stroke-linecap="round" '
          f'transform="rotate({f(rot)} {f(x)} {f(y)})" />')


def apricots(p, cx, rim):
    for i, (dx, dy) in enumerate(pyramid([5, 4, 3, 2, 1], 27, 23, rim - 6)):
        x, y = cx + dx, dy
        blushed(p, x, y, 16, (140, 150, 130)[i % 3], "#f9b347", "#f4a03f", "#e8963c", -9)


def apples(p, cx, rim):
    for i, (dx, dy) in enumerate(pyramid([4, 3, 2, 1], 34, 30, rim - 8)):
        x, y = cx + dx, dy
        p.circle(x, y, 20, "#d64541")
        p.circle(x - 8, y - 8, 5, "#e8726a", hull=False)
        p.path(f"M{f(x)} {f(y - 17)}Q{f(x + 1)} {f(y - 24)} {f(x + 4)} {f(y - 27)}", stroke="#6b4a2f",
               width=3.5)
        if i in (0, 4):
            p.ellipse(x + 11, y - 25, 8, 4, "#6fb07f", rot=-30, hull=False)


def lemons(p, cx, rim):
    for i, (dx, dy) in enumerate(pyramid([3, 3, 2, 1], 47, 26, rim - 8)):
        x = cx + dx
        y = dy
        rot = (-14, 10, -6, 14, -10, 6, -12, 8, 0)[i % 9]
        a = math.radians(rot)
        for sgn in (-1, 1):
            p.circle(x + sgn * 22 * math.cos(a), y + sgn * 22 * math.sin(a), 4, "#f6d44a")
        p.ellipse(x, y, 21, 15, "#f6d44a", rot=rot)
        p.ellipse(x - 7 * math.sin(a), y + 7 * math.cos(a), 16, 5.5, "#e6bf34", rot=rot, hull=False)


def plums(p, cx, rim):
    for (dx, dy) in pyramid([4, 3, 2, 1], 31, 28, rim - 8):
        x, y = cx + dx, dy
        p.ellipse(x, y, 16, 18, "#6b3d7a")
        p.ellipse(x - 6, y - 5, 4, 6, "#8e5a9e", hull=False)
        p.path(f"M{f(x + 2)} {f(y - 16)}Q{f(x + 6)} {f(y)} {f(x + 1)} {f(y + 15)}", stroke="#5a3068",
               width=2)


def oranges(p, cx, rim):
    for (dx, dy) in pyramid([4, 3, 2, 1], 34, 30, rim - 8):
        x, y = cx + dx, dy
        p.circle(x, y, 20, "#f39c32")
        p.circle(x - 7, y - 7, 5, "#f7b45a", hull=False)
        p.circle(x + 5, y - 14, 2.5, "#8a9a3a", hull=False)


def pears(p, cx, rim):
    for (dx, dy) in pyramid([4, 3, 2], 36, 36, rim - 6):
        x, y = cx + dx, dy
        p.path(f"M{f(x + 3)} {f(y - 30)}Q{f(x + 4)} {f(y - 37)} {f(x + 8)} {f(y - 40)}", stroke="#6b4a2f",
               width=3)
        p.circle(x + 2, y - 21, 11, "#b8cf5a")
        p.poly([(x - 15, y - 8), (x - 8, y - 24), (x + 12, y - 24), (x + 17, y - 8)], "#b8cf5a")
        p.circle(x, y, 18, "#b8cf5a")
        p.ellipse(x - 7, y - 4, 4, 7, "#cfe07a", rot=15, hull=False)


def tangerines(p, cx, rim):
    for i, (dx, dy) in enumerate(pyramid([4, 3, 2, 1], 32, 22, rim - 6)):
        x, y = cx + dx, dy
        p.ellipse(x, y, 17, 14, "#f28c28")
        p.circle(x - 6, y - 5, 4, "#f6a94e", hull=False)
        p.circle(x, y - 12, 2, "#5c7a3a", hull=False)
        if i in (0, 1, 3, 6):
            p.ellipse(x + 10, y - 16, 10, 4.5, "#5c9a6b", rot=-28, hull=i == 0)


def peaches(p, cx, rim):
    for i, (dx, dy) in enumerate(pyramid([4, 3, 2, 1], 35, 31, rim - 8)):
        x, y = cx + dx, dy
        blushed(p, x, y, 21, (150, 135, 160)[i % 3], "#fbc893", "#ef8466", "#e0735a", -2)
        if i in (0, 5):
            p.ellipse(x + 8, y - 22, 9, 4, "#6fb07f", rot=-25, hull=i == 0)


def strawberries(p, cx, rim):
    for (dx, dy) in pyramid([5, 4, 3, 2], 28, 23, rim - 4):
        x, y = cx + dx, dy
        s, c1, t, c2 = (x - 15, y - 9), (x - 17, y + 5), (x, y + 18), (x + 17, y + 5)
        e, c3 = (x + 15, y - 9), (x, y - 15)
        d = (f"M{f(s[0])} {f(s[1])}Q{f(c1[0])} {f(c1[1])} {f(t[0])} {f(t[1])}"
             f"Q{f(c2[0])} {f(c2[1])} {f(e[0])} {f(e[1])}Q{f(c3[0])} {f(c3[1])} {f(s[0])} {f(s[1])}Z")
        p.path(d, fill="#e0393e", pts=quad(s, c1, t) + quad(t, c2, e) + quad(e, c3, s))
        for sx, sy in ((-6, -2), (0, -4), (6, -2), (-3, 5), (3, 5), (0, 11)):
            p.circle(x + sx, y + sy, 1.6, "#f6d44a", hull=False)
        star = []
        for k in range(10):
            a = math.pi * k / 5 - math.pi / 2
            r = 10 if k % 2 == 0 else 3.5
            star.append((x + r * math.cos(a), y - 12 + r * math.sin(a) * 0.55))
        p.poly(star, "#5c9a6b", hull=False)


def blueberries(p, cx, rim):
    for (dx, dy) in pyramid([8, 7, 6, 5, 4, 3], 17, 13, rim - 4):
        x, y = cx + dx + random.uniform(-1, 1), dy + random.uniform(-1, 1)
        p.circle(x, y, 8.5, "#3b4f8f")
        p.circle(x - 3, y - 3, 2.2, "#6b7fbf", hull=False)
        a = random.uniform(0, 2 * math.pi)
        p.circle(x + 4 * math.cos(a), y + 4 * math.sin(a), 2.3, "#26355f", hull=False)


def grapes(p, cx, rim):
    for bx, top, lean in ((cx - 36, rim - 112, -6), (cx + 36, rim - 104, 6)):
        rows = [4, 4, 3, 3, 2, 2, 1]
        p.path(f"M{f(bx)} {f(top - 4)}Q{f(bx + 2)} {f(top - 14)} {f(bx + 9)} {f(top - 20)}", stroke="#6b4a2f",
               width=3.5)
        for i, n in enumerate(rows):
            for k in range(n):
                x = bx + (k - (n - 1) / 2) * 17 + lean * i / 6
                y = top + 8 + i * 15
                p.circle(x, y, 9.5, "#a9cf5a")
                p.circle(x - 3, y - 3, 2.5, "#cfe79a", hull=False)
    p.ellipse(cx - 56, rim - 116, 15, 9, "#6fb07f", rot=-25)


def cherries(p, cx, rim):
    rows = [6, 5, 4, 3]
    centres = pyramid(rows, 23, 19, rim - 4)
    top = centres[:3]
    for (dx, dy) in centres:
        x, y = cx + dx, dy
        p.circle(x, y, 11, "#a91d2c")
        p.circle(x - 4, y - 4, 3, "#d0485a", hull=False)
    for (dx, dy) in top:
        x, y = cx + dx, dy
        p.path(f"M{f(x)} {f(y - 10)}Q{f(x + 3)} {f(y - 26)} {f(x + 11)} {f(y - 34)}", stroke="#5c7a3a",
               width=2.5, pts=[(x + 11, y - 34)])


def kiwis(p, cx, rim):
    for (dx, dy) in pyramid([3, 2, 1], 46, 28, rim - 14):
        x, y = cx + dx, dy
        p.ellipse(x, y, 21, 16, "#8a6a3a")
        for _ in range(6):
            p.circle(x + random.uniform(-13, 13), y + random.uniform(-9, 9), 1.4, "#74582e", hull=False)
    for x, y in ((cx - 38, rim - 10), (cx + 36, rim - 8), (cx, rim - 2)):
        p.circle(x, y, 22, "#7a5c30")
        p.circle(x, y, 19, "#8cc63f")
        p.ellipse(x, y, 7, 6, "#e3efb5", hull=False)
        for k in range(14):
            a = 2 * math.pi * k / 14
            p.ellipse(x + 11 * math.cos(a), y + 11 * math.sin(a), 1.3, 2.4, "#2f2a26",
                      rot=math.degrees(a) + 90, hull=False)


def raspberries(p, cx, rim):
    for (dx, dy) in pyramid([7, 6, 5, 4, 3], 20, 16, rim - 4):
        x, y = cx + dx + random.uniform(-1, 1), dy + random.uniform(-1, 1)
        p.circle(x, y, 10, "#d93a5f")
        for sx, sy in ((-4, -4), (4, -4), (0, 1), (-5, 4), (5, 4)):
            p.circle(x + sx, y + sy, 2.3, "#ea6a84", hull=False)


def fig_shape(x, y, s):
    """A fig's outline, its round end down, `s` its scale."""
    pts = [(x - 12 * s, y - 9 * s), (x - 6 * s, y - 24 * s), (x - 2 * s, y - 30 * s), (x + 2 * s, y - 30 * s),
           (x + 6 * s, y - 24 * s), (x + 12 * s, y - 9 * s)]
    return pts


def figs(p, cx, rim):
    for (dx, dy) in pyramid([4, 3, 2], 36, 30, rim - 8):
        x, y = cx + dx, dy
        p.path(f"M{f(x)} {f(y - 29)}L{f(x + 2)} {f(y - 36)}", stroke="#6b5a2a", width=3,
               pts=[(x + 2, y - 36)])
        p.poly(fig_shape(x, y, 1), "#5e3a5e")
        p.circle(x, y, 16, "#5e3a5e")
        p.ellipse(x - 6, y - 4, 3, 6, "#7a4f78", rot=10, hull=False)
    # one cut in half, its red to the viewer
    x, y = cx + 2, rim - 2
    p.poly(fig_shape(x, y, 1.1), "#5e3a5e")
    p.circle(x, y, 17.6, "#5e3a5e")
    p.poly(fig_shape(x, y, 0.92), "#f0d8b8")
    p.circle(x, y, 14.6, "#f0d8b8")
    p.ellipse(x, y - 4, 9, 13, "#e05a6a")
    for sx, sy in ((-3, -10), (3, -8), (-4, -2), (2, -1), (-2, 5), (4, 4)):
        p.circle(x + sx, y + sy, 1.2, "#f8e6c8", hull=False)


def dates(p, cx, rim):
    rows = [5, 4, 5, 4, 3, 2]
    for (dx, dy) in pyramid(rows, 27, 13, rim - 2):
        x, y = cx + dx + random.uniform(-2, 2), dy
        rot = random.uniform(-28, 28)
        p.ellipse(x, y, 15, 8, "#7a3f22", rot=rot)
        a = math.radians(rot)
        p.ellipse(x - 3 * math.sin(a) - 2 * math.cos(a), y - 3 * math.cos(a), 7, 2.4, "#9a5a32", rot=rot,
                  hull=False)


DRAW = {
    "pineapple": pineapple, "watermelon": watermelon, "banana": bananas, "coconut": coconuts,
    "melon": melons, "mango": mangoes, "papaya": papayas, "pomegranate": pomegranates,
    "apricot": apricots, "apple": apples, "lemon": lemons, "plum": plums, "orange": oranges,
    "pear": pears, "tangerine": tangerines, "peach": peaches,
    "strawberry": strawberries, "blueberry": blueberries, "grape": grapes, "cherry": cherries,
    "kiwi": kiwis, "raspberry": raspberries, "fig": figs, "date": dates,
}

COMMENTS = {
    "pineapple": "pineapples: three, standing, crowned with leaves",
    "watermelon": "watermelons: one whole, striped, and half of one, cut",
    "banana": "bananas: two hands, each joined at its stem",
    "coconut": "coconuts: two whole, one cracked open",
    "melon": "melons: two whole, netted, and half of one, cut",
    "mango": "mangoes: five, red over yellow, green at the stalk",
    "papaya": "papayas: one whole, one cut down its length, its seeds black",
    "pomegranate": "pomegranates: six crowned, and one cut, its seeds showing",
    "apricot": "apricots: fifteen, small, each with its crease",
    "apple": "apples: ten, a leaf on two",
    "lemon": "lemons: nine, pointed at the ends",
    "plum": "plums: ten, dark purple",
    "orange": "oranges: ten",
    "pear": "pears: nine, each with its stalk",
    "tangerine": "tangerines: ten, small and flat, some with a leaf",
    "peach": "peaches: ten, blushed red, a leaf on two",
    "strawberry": "strawberries: fourteen, seeded, green at the top",
    "blueberry": "blueberries: a heap of them",
    "grape": "grapes: two green bunches and a vine leaf",
    "cherry": "cherries: eighteen, three with their stalks",
    "kiwi": "kiwis: six whole, brown, and three halves, green inside",
    "raspberry": "raspberries: a heap of them",
    "fig": "figs: nine, and one cut in half",
    "date": "dates: a heap of them",
}

TIERS = [
    ["pineapple", "watermelon", "banana", "coconut", "melon", "mango", "papaya", "pomegranate"],
    ["apricot", "apple", "lemon", "plum", "orange", "pear", "tangerine", "peach"],
    ["strawberry", "blueberry", "grape", "cherry", "kiwi", "raspberry", "fig", "date"],
]

draw(
    "fruit-stand",
    "Drawn for Ruya: a fruit stand at a market, seen straight on: three tiers of crates under a "
    "striped awning, one fruit to a crate, big fruit at the top and berries at the bottom. No text "
    "in it, so it serves every language. Made by art/fruit-stand/basic.py, which also writes each "
    "fruit's region in fruit-stand.json: change that and run it, rather than this file.",
    TIERS,
    DRAW,
    COMMENTS,
    ("red", "#e26d5a", "#c9553f"),
)
