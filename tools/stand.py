"""A market stand, for Ruya's drawn pictures of the fruit and the vegetable
stand: three tiers of eight crates under a striped awning, one kind of
produce to a crate.

Each stand has its own script, `art/<scene>/basic.py`, which says what is in
its crates and how each is drawn, and calls `draw`: that draws the stand
around them, writes `public/scenes/<scene>/basic.svg`, and writes each
crate's region in the basic style of `scenes/<scene>.json`. A region is the
convex hull of the crate's front and its heap, so a tap on the crate names
what is in it.

The scatter of berries and the like is seeded, so a script draws the same
picture each time it runs. Python alone is all it needs.
"""
import math
import random
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

# the crates' size, the gap between two, the first one's left edge, and the
# top of each tier's crates, the top tier first
CRATE_W, CRATE_H, GAP, LEFT = 158, 60, 13, 90
RIMS = (388, 600, 812)


def f(v):
    """A number as SVG gets it: to a tenth, and no ".0"."""
    v = round(v, 1)
    if abs(v - round(v)) < 1e-9:
        return str(int(round(v)))
    return f"{v:.1f}"


def quad(p0, p1, p2, n=10):
    """Points along a quadratic Bézier curve, its ends included."""
    return [
        (
            (1 - t) ** 2 * p0[0] + 2 * (1 - t) * t * p1[0] + t * t * p2[0],
            (1 - t) ** 2 * p0[1] + 2 * (1 - t) * t * p1[1] + t * t * p2[1],
        )
        for t in (k / n for k in range(n + 1))
    ]


def ellipse_chord(cx, cy, rx, ry, angle, offset, shrink=0.88):
    """A line at `angle` degrees, `offset` from the centre, cut to the
    ellipse and shortened a little, as two points; None if it misses."""
    a = math.radians(angle)
    dx, dy = math.cos(a), math.sin(a)
    nx, ny = -dy, dx
    x0, y0 = cx + nx * offset, cy + ny * offset
    # ((x0 + t dx - cx)/rx)^2 + ((y0 + t dy - cy)/ry)^2 = 1
    ax, ay = (x0 - cx) / rx, (y0 - cy) / ry
    bx, by = dx / rx, dy / ry
    qa = bx * bx + by * by
    qb = 2 * (ax * bx + ay * by)
    qc = ax * ax + ay * ay - 1
    disc = qb * qb - 4 * qa * qc
    if disc <= 0:
        return None
    s = math.sqrt(disc)
    t0, t1 = (-qb - s) / (2 * qa), (-qb + s) / (2 * qa)
    mid, half = (t0 + t1) / 2, (t1 - t0) / 2 * shrink
    if half < 3:
        return None
    return (
        (x0 + (mid - half) * dx, y0 + (mid - half) * dy),
        (x0 + (mid + half) * dx, y0 + (mid + half) * dy),
    )


class Pile:
    """One crate's heap: its SVG elements in paint order, and the points of
    its outline, which its region is the hull of. A detail drawn inside a
    shape already counted passes `hull=False`."""

    def __init__(self):
        self.els = []
        self.pts = []

    def add(self, el):
        self.els.append(el)

    def circle(self, x, y, r, fill, hull=True):
        self.add(f'<circle cx="{f(x)}" cy="{f(y)}" r="{f(r)}" fill="{fill}" />')
        if hull:
            self.pts += [
                (x + r * math.cos(2 * math.pi * k / 32), y + r * math.sin(2 * math.pi * k / 32))
                for k in range(32)
            ]

    def ellipse(self, x, y, rx, ry, fill, rot=0, hull=True):
        t = f' transform="rotate({f(rot)} {f(x)} {f(y)})"' if rot else ""
        self.add(f'<ellipse cx="{f(x)}" cy="{f(y)}" rx="{f(rx)}" ry="{f(ry)}" fill="{fill}"{t} />')
        if hull:
            a = math.radians(rot)
            for k in range(32):
                th = 2 * math.pi * k / 32
                px, py = rx * math.cos(th), ry * math.sin(th)
                self.pts.append(
                    (x + px * math.cos(a) - py * math.sin(a), y + px * math.sin(a) + py * math.cos(a))
                )

    def poly(self, pts, fill, hull=True):
        d = "M" + "L".join(f"{f(x)} {f(y)}" for x, y in pts) + "Z"
        self.add(f'<path d="{d}" fill="{fill}" />')
        if hull:
            self.pts += pts

    def path(self, d, fill=None, stroke=None, width=None, pts=None):
        attrs = f'd="{d}"'
        attrs += f' fill="{fill}"' if fill else ' fill="none"'
        if stroke:
            attrs += f' stroke="{stroke}" stroke-width="{f(width)}" stroke-linecap="round"'
        self.add(f"<path {attrs} />")
        if pts:
            self.pts += pts

    def lines(self, segs, stroke, width):
        d = "".join(f"M{f(a[0])} {f(a[1])}L{f(b[0])} {f(b[1])}" for a, b in segs)
        self.path(d, stroke=stroke, width=width)


def pyramid(rows, step_x, step_y, base_y):
    """Centres of a heap: `rows` counts from the bottom row up, each row
    centred, `step_y` higher than the last. Returned top row first, so the
    nearer, lower one is drawn over the one behind it."""
    out = []
    for i, n in enumerate(rows):
        y = base_y - i * step_y
        out.append([((k - (n - 1) / 2) * step_x, y) for k in range(n)])
    return [p for row in reversed(out) for p in row]


def capped(p, x, y, rx, ry, rot, fill, cap, cap_x):
    """An ellipse, and over its one end, from `cap_x` along its long axis
    (negative: the left), a cap of another colour bounded by a curve."""
    k = math.sqrt(max(0.0, 1 - (cap_x / rx) ** 2)) * ry
    end = -rx if cap_x < 0 else rx
    sweep = 1 if cap_x < 0 else 0
    bulge = cap_x + (8 if cap_x < 0 else -8)
    d = (f"M{f(x + end)} {f(y)}A{f(rx)} {f(ry)} 0 0 {sweep} {f(x + cap_x)} {f(y - k)}"
         f"Q{f(x + bulge)} {f(y)} {f(x + cap_x)} {f(y + k)}A{f(rx)} {f(ry)} 0 0 {sweep} {f(x + end)} {f(y)}Z")
    p.ellipse(x, y, rx, ry, fill, rot=rot)
    t = f' transform="rotate({f(rot)} {f(x)} {f(y)})"' if rot else ""
    p.add(f'<path d="{d}" fill="{cap}"{t} />')


def hull(points):
    """The convex hull of `points`, rounded to whole units, anticlockwise."""
    pts = sorted(set((round(x), round(y)) for x, y in points))
    if len(pts) < 3:
        return pts

    def cross(o, a, b):
        return (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])

    lower, upper = [], []
    for q in pts:
        while len(lower) >= 2 and cross(lower[-2], lower[-1], q) <= 0:
            lower.pop()
        lower.append(q)
    for q in reversed(pts):
        while len(upper) >= 2 and cross(upper[-2], upper[-1], q) <= 0:
            upper.pop()
        upper.append(q)
    return lower[:-1] + upper[:-1]


def simplify(ring, tolerance=40.0):
    """Drops the points whose triangle with their neighbours is smallest,
    while that area stays under `tolerance` square units."""
    ring = list(ring)
    while len(ring) > 8:
        best, best_i = None, None
        for i in range(len(ring)):
            a, b, c = ring[i - 1], ring[i], ring[(i + 1) % len(ring)]
            area = abs((b[0] - a[0]) * (c[1] - a[1]) - (c[0] - a[0]) * (b[1] - a[1])) / 2
            if best is None or area < best:
                best, best_i = area, i
        if best >= tolerance:
            break
        ring.pop(best_i)
    return ring


def svg_comment(text, indent="\t"):
    words, lines, line = text.split(), [], ""
    for w in words:
        if len(line) + len(w) + 1 > 66:
            lines.append(line)
            line = w
        else:
            line = f"{line} {w}" if line else w
    lines.append(line)
    out = f"{indent}<!-- {lines[0]}"
    for l in lines[1:]:
        out += f"\n{indent}     {l}"
    return out + " -->"


def crate(x0, rim):
    x1 = x0 + CRATE_W
    return [
        f'<rect x="{x0}" y="{rim}" width="{CRATE_W}" height="{CRATE_H}" fill="#b98250" />',
        f'<rect x="{x0}" y="{rim + 6}" width="{CRATE_W}" height="20" fill="#d9a66b" />',
        f'<rect x="{x0}" y="{rim + 32}" width="{CRATE_W}" height="22" fill="#d9a66b" />',
        f'<rect x="{x0}" y="{rim}" width="10" height="{CRATE_H}" fill="#a8743f" />',
        f'<rect x="{x1 - 10}" y="{rim}" width="10" height="{CRATE_H}" fill="#a8743f" />',
    ]


def draw(scene, about, tiers, drawers, comments, awning, seed=7):
    """Draws the stand of scene `scene` and writes its picture and regions.

    `about` is the picture's opening comment; `tiers` three lists of eight
    ids, the top tier first, each from left to right; `drawers` the function
    drawing each id's heap into a `Pile`, given its crate's centre and rim;
    `comments` a line about each; `awning` its colour's name, the stripes'
    colour and the valance's, the other stripes being white.
    """
    random.seed(seed)
    svg_out = ROOT / f"public/scenes/{scene}/basic.svg"
    scene_file = ROOT / f"scenes/{scene}.json"
    colour, stripe, valance = awning

    lines = []
    w = lines.append
    w('<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1536 1024" width="1536" height="1024">')
    w(svg_comment(about))
    w("")
    w(svg_comment("sky, and the paving the stand stands on"))
    w('\t<rect width="1536" height="1024" fill="#cfe8f5" />')
    w('\t<rect y="984" width="1536" height="40" fill="#d6d0c6" />')
    w("")
    w(svg_comment("the stand's back, between its posts, and the posts"))
    w('\t<rect x="68" y="448" width="1400" height="436" fill="#e3ebee" />')
    w('\t<rect x="44" y="60" width="24" height="940" fill="#8a6a4a" />')
    w('\t<rect x="1468" y="60" width="24" height="940" fill="#8a6a4a" />')
    w("")
    w(svg_comment(f"the awning: {colour} and white stripes, scalloped along the bottom"))
    coloured, white = [], []
    for k in range(24):
        x = 24 + 62 * k
        (coloured if k % 2 == 0 else white).append(f"M{x} 40H{x + 62}V150Q{x + 31} 178 {x} 150Z")
    w(f'\t<path d="{"".join(white)}" fill="#ffffff" />')
    w(f'\t<path d="{"".join(coloured)}" fill="{stripe}" />')
    w(f'\t<rect x="16" y="30" width="1504" height="16" rx="5" fill="{valance}" />')
    w("")
    w(svg_comment("the shelf under each tier, and the stand's front, planked"))
    for rim in RIMS:
        w(f'\t<rect x="68" y="{rim + CRATE_H}" width="1400" height="12" fill="#8a6a4a" />')
    w('\t<rect x="68" y="884" width="1400" height="100" fill="#b07f48" />')
    w('\t<path d="' + "".join(f"M{x} 884V984" for x in range(243, 1468, 175)) +
      '" stroke="#9a6c3c" stroke-width="4" />')

    regions = []
    for rim, ids in zip(RIMS, tiers):
        for i, thing in enumerate(ids):
            x0 = LEFT + (CRATE_W + GAP) * i
            cx = x0 + CRATE_W / 2
            p = Pile()
            drawers[thing](p, cx, rim)
            xs = [x for x, _ in p.pts]
            if min(xs) < x0 + 1 or max(xs) > x0 + CRATE_W - 1:
                print(f"warning: {thing} spills out of its crate: {min(xs):.0f}..{max(xs):.0f} "
                      f"vs {x0}..{x0 + CRATE_W}", file=sys.stderr)
            w("")
            w(svg_comment(comments[thing]))
            for el in p.els:
                w("\t" + el)
            for el in crate(x0, rim):
                w("\t" + el)
            ring = hull(p.pts + [(x0, rim), (x0 + CRATE_W, rim), (x0 + CRATE_W, rim + CRATE_H),
                                 (x0, rim + CRATE_H)])
            ring = simplify(ring)
            d = "M" + "L".join(f"{x} {y}" for x, y in ring) + "Z"
            regions.append({"thing": thing, "d": d})
    w("</svg>")

    svg_out.write_text("\n".join(lines) + "\n")

    # the basic style's regions, in the scene file's own layout, one a line
    text = scene_file.read_text()
    block = re.compile(r'("basic": \{\n\t\t\t"picture": [^\n]*\n\t\t\t"regions": \[\n)(.*?)(\t\t\t\])', re.S)
    if not block.search(text):
        sys.exit(f"no basic regions in {scene_file}")
    rows = ",\n".join(f'\t\t\t\t{{ "thing": "{r["thing"]}", "d": "{r["d"]}" }}' for r in regions)
    scene_file.write_text(block.sub(lambda m: m.group(1) + rows + "\n" + m.group(3), text, count=1))
    print(f"{svg_out.relative_to(ROOT)}: {svg_out.stat().st_size} bytes; {len(regions)} regions in "
          f"{scene_file.relative_to(ROOT)}")
