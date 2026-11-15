#!/usr/bin/env python3
"""Ruya's pasture, drawn: writes public/scenes/pasture/basic.svg and the
basic style's regions in scenes/pasture.json.

The farm's grazing land, beyond the farmyard. Along the back, a field of
grain with a scarecrow in it, the windmill, a tree, the stable and the
tractor; across the middle, the fence with its gate. Beyond it, the
smaller animals and things: the water trough and the donkey beside it, the goat and her
kid, a hay bale, the pig and her piglet, the sheep and her lamb; and in
front, the big ones: the horse and her foal, the cow and her calf, and the
bull.

Each thing's region is the convex hull of its shapes, the fence's two, one
either side of the gate. The things are drawn back to front, and their
regions are in the same order, so where a nearer thing stands over one
behind it, a tap there is the nearer one's.

The animals are drawn side on, facing right, and turned round with a
`Frame`, which is where each one stands, how big it is and which way it
faces.

Run it from anywhere, `python3 art/pasture/basic.py`, after changing a
thing or where it stands; it needs nothing beyond Python. The shapes and
the hull are tools/stand.py's.
"""
import math
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "tools"))
from stand import Pile, f, hull, quad, simplify, svg_comment  # noqa: E402

SCENE = "pasture"

INK = "#2f3a44"
WOOD_DARK, WOOD, WOOD_LIGHT = "#8a6a4a", "#b98250", "#d9a66b"
ROOF, ROOF_EDGE = "#9b4a3a", "#86402f"
WHITE = "#ffffff"
GOLD, GOLD_DARK = "#e6c35c", "#cfa640"

# the ground the animals beyond the fence stand on, and the ground in front
FAR, NEAR = 728, 986


class Piles(Pile):
    """A thing's shapes, its outline in one or more parts: `part()` closes
    one and starts the next, so the fence is two, either side of the gate."""

    def __init__(self):
        super().__init__()
        self.parts = []

    def part(self):
        self.parts.append(self.pts)
        self.pts = []

    def rings(self):
        return [simplify(hull(pts)) for pts in self.parts + [self.pts] if pts]


def rect(p, x, y, w, h, fill, rx=0, hull=True, opacity=None):
    if w < 0:
        x, w = x + w, -w
    r = f' rx="{f(rx)}"' if rx else ""
    o = f' fill-opacity="{opacity}"' if opacity else ""
    p.add(f'<rect x="{f(x)}" y="{f(y)}" width="{f(w)}" height="{f(h)}"{r} fill="{fill}"{o} />')
    if hull:
        p.pts += [(x, y), (x + w, y), (x + w, y + h), (x, y + h)]


def line(p, pts, colour, width, hull=True):
    """A stroke through `pts`, round at its ends; its hull points are the
    stroke's own edges."""
    d = "M" + "L".join(f"{f(x)} {f(y)}" for x, y in pts)
    r = width / 2
    edge = [(x + r * math.cos(a), y + r * math.sin(a))
            for x, y in pts for a in (k * math.pi / 4 for k in range(8))]
    p.path(d, stroke=colour, width=width, pts=edge if hull else None)


class Frame:
    """An animal's own coordinates: x forward, y up from its feet, at `s`
    times its size, standing at (x, y) and facing right (d=1) or left
    (d=-1). A y above the ground is negative, as in SVG."""

    def __init__(self, p, x, y, s=1.0, d=1):
        self.p, self.x, self.y, self.s, self.d = p, x, y, s, d

    def at(self, lx, ly):
        return (self.x + self.d * lx * self.s, self.y + ly * self.s)

    def ellipse(self, lx, ly, rx, ry, fill, rot=0, hull=True):
        x, y = self.at(lx, ly)
        self.p.ellipse(x, y, rx * self.s, ry * self.s, fill, rot=rot * self.d, hull=hull)

    def circle(self, lx, ly, r, fill, hull=True):
        x, y = self.at(lx, ly)
        self.p.circle(x, y, r * self.s, fill, hull=hull)

    def poly(self, pts, fill, hull=True):
        self.p.poly([self.at(*q) for q in pts], fill, hull=hull)

    def rect(self, lx, ly, w, h, fill, rx=0, hull=True, opacity=None):
        x, y = self.at(lx, ly)
        rect(self.p, x, y, self.d * w * self.s, h * self.s, fill, rx * self.s, hull, opacity)

    def shade(self, lx, ly, w, h):
        """A far leg's darkening, over the leg already drawn."""
        self.rect(lx, ly, w, h, INK, hull=False, opacity=0.18)

    def line(self, pts, colour, width, hull=True):
        line(self.p, [self.at(*q) for q in pts], colour, width * self.s, hull)

    def eye(self, lx, ly, r=3.5):
        x, y = self.at(lx, ly)
        self.p.circle(x, y, max(r * self.s, 2), INK, hull=False)


# ── along the back ──

def field(p):
    # grain on the far hillside, its rows running up it
    p.poly([(0, 382), (300, 368), (466, 392), (450, 482), (0, 490)], GOLD)
    p.path("".join(f"M{x0} 488L{x1} 374" for x0, x1 in
                   ((40, 70), (100, 130), (160, 190), (220, 250), (280, 310), (340, 362), (400, 412))),
           stroke=GOLD_DARK, width=4)
    p.path("M0 430Q230 418 458 436", stroke="#d8b450", width=3)


def scarecrow(p):
    # in the field, arms out on his crossbar
    rect(p, 244, 300, 12, 176, WOOD_DARK)
    rect(p, 192, 344, 116, 10, WOOD_DARK)
    for x0, x1 in ((180, 194), (306, 320)):
        p.poly([(x0, 350), (x1, 344), (x1, 356), (x0 + (x1 - x0) // 2, 362)], "#e6c35c")
    p.poly([(190, 340), (310, 340), (310, 362), (190, 362)], "#c0392b")
    p.poly([(214, 340), (286, 340), (294, 414), (206, 414)], "#c0392b")
    p.path("M230 340V414M270 340V414M210 372H290M208 396H292", stroke="#9b2c22", width=3)
    p.poly([(208, 412), (292, 412), (286, 428), (272, 418), (260, 432), (246, 418), (232, 430),
            (222, 418), (212, 428)], "#e6c35c")
    p.circle(250, 316, 22, "#e9d3a0")
    p.circle(243, 313, 3, INK, hull=False)
    p.circle(257, 313, 3, INK, hull=False)
    p.path("M240 326L244 323L248 326L252 323L256 326L260 323", stroke=INK, width=2)
    p.poly([(212, 300), (288, 300), (284, 292), (216, 292)], "#8a6a4a")
    p.poly([(228, 294), (272, 294), (266, 266), (234, 266)], "#8a6a4a")
    rect(p, 230, 284, 40, 6, "#c0392b", hull=False)


def windmill(p):
    hub = (620, 278)
    # the tower, white, its door and window
    p.poly([(560, 522), (582, 300), (658, 300), (680, 522)], "#f2ece0")
    p.path("M604 522V494Q604 478 620 478Q636 478 636 494V522Z", fill="#8a5a3b")
    p.path("M612 380H628V404H612Z", fill="#7fb7d3")
    p.path("M560 522H680", stroke="#d8d0c0", width=4)
    p.poly([(570, 306), (620, 254), (670, 306)], "#8a5a3b")
    # four sails, crossed like an X, each a lattice on one side of its arm
    for deg in (45, 135, 225, 315):
        a = math.radians(deg)
        ux, uy = math.cos(a), math.sin(a)
        nx, ny = -uy, ux
        r0, r1 = 34, 196
        line(p, [(hub[0] + ux * 12, hub[1] + uy * 12), (hub[0] + ux * r1, hub[1] + uy * r1)],
             WOOD_DARK, 9)
        corners = [(hub[0] + ux * r + nx * w, hub[1] + uy * r + ny * w)
                   for r, w in ((r0, 5), (r1, 5), (r1, 40), (r0, 40))]
        p.poly(corners, "#f6f1e6")
        bars = "".join(
            f"M{f(hub[0] + ux * r + nx * 5)} {f(hub[1] + uy * r + ny * 5)}"
            f"L{f(hub[0] + ux * r + nx * 40)} {f(hub[1] + uy * r + ny * 40)}"
            for r in range(r0, r1 + 1, 27))
        mid = (f"M{f(hub[0] + ux * r0 + nx * 22)} {f(hub[1] + uy * r0 + ny * 22)}"
               f"L{f(hub[0] + ux * r1 + nx * 22)} {f(hub[1] + uy * r1 + ny * 22)}")
        p.path(bars + mid, stroke="#b8a888", width=3)
    p.circle(*hub, 12, INK)


def tree(p):
    rect(p, 866, 360, 30, 162, "#7a5638")
    for cx, cy, r, c in ((815, 326, 56, "#4f8a43"), (947, 326, 56, "#4f8a43"),
                         (881, 300, 86, "#5f9e4f"), (881, 236, 56, "#5f9e4f")):
        p.circle(cx, cy, r, c)
    p.path("M846 300Q860 286 876 296M904 262Q918 250 930 262M830 336Q842 326 856 334",
           stroke="#4f8a43", width=4)


def stable(p):
    # long and low, a half door to each horse's stall
    rect(p, 1040, 404, 310, 118, "#c98a55")
    p.path("M1040 434H1350M1040 464H1350M1040 494H1350", stroke="#b47a48", width=2)
    p.poly([(1018, 412), (1062, 336), (1326, 336), (1370, 412)], ROOF)
    rect(p, 1016, 406, 356, 10, ROOF_EDGE, rx=3)
    for x in (1072, 1166, 1260):
        rect(p, x, 428, 62, 40, "#4a3020", hull=False)
        rect(p, x, 468, 62, 54, "#a8743f", hull=False)
        p.path(f"M{x} 468H{x + 62}V522H{x}ZM{x} 468L{x + 62} 522M{x + 62} 468L{x} 522",
               stroke="#7a5236", width=4)


def tractor(p):
    # red, facing left, its big wheel behind
    p.circle(1478, 480, 48, "#2e2e2e")
    p.circle(1478, 480, 22, "#e9c03a", hull=False)
    p.circle(1478, 480, 7, "#2e2e2e", hull=False)
    rect(p, 1444, 384, 84, 10, "#8f2a20", rx=3)
    rect(p, 1452, 392, 66, 76, "#c0392b")
    rect(p, 1462, 400, 46, 34, "#bfe3f5", hull=False)
    rect(p, 1420, 404, 8, 34, INK)
    p.poly([(1390, 474), (1396, 438), (1456, 434), (1458, 474)], "#c0392b")
    p.path("M1402 450H1450M1402 460H1450", stroke="#9b2c22", width=3)
    p.circle(1410, 498, 30, "#2e2e2e")
    p.circle(1410, 498, 13, "#e9c03a", hull=False)


# ── across the middle ──

def fence(p):
    # posts and two rails, left of the gate and right of it
    for x0, x1 in ((0, 640), (780, 1536)):
        rect(p, x0, 532, x1 - x0, 10, WOOD)
        rect(p, x0, 566, x1 - x0, 10, WOOD)
        for x in range(x0 + 40, x1 - 10, 100):
            rect(p, x - 7, 514, 14, 88, WOOD_DARK)
        if x0 == 0:
            p.part()


def gate(p):
    # five bars and a brace, hung between two posts
    for k in range(5):
        rect(p, 654, 520 + 17 * k, 112, 8, WOOD_LIGHT)
    line(p, [(660, 592), (760, 524)], WOOD_LIGHT, 8)
    rect(p, 640, 506, 16, 98, WOOD_DARK)
    rect(p, 764, 506, 16, 98, WOOD_DARK)


# ── the animals: horses and the donkey ──

def equine(fr, body, dark, light=None, leg=1.0, ears="short", tuft=False):
    """A horse, a foal or a donkey, side on: `body` its coat, `dark` its
    mane, tail and hooves, `light` its muzzle and belly, if lighter."""
    top = -160 * leg
    cy = top - 26
    if tuft:
        fr.line([(-102, cy - 14), (-114, cy + 30), (-112, cy + 70)], body, 7)
        fr.ellipse(-112, cy + 82, 9, 16, dark)
    else:
        fr.line(quad((-100, cy - 16), (-134, cy + 20), (-118, cy + 88)), dark, 18)
    for lx in (-64, 60):
        fr.rect(lx, top, 18, -top, body)
        fr.shade(lx, top, 18, -top)
        fr.rect(lx, -14, 18, 14, dark, hull=False)
    fr.ellipse(0, cy, 108, 52, body)
    fr.ellipse(-70, cy + 12, 40, 46, body, hull=False)
    if light:
        fr.ellipse(4, cy + 30, 70, 18, light, hull=False)
    for lx in (-88, 82):
        fr.rect(lx, top + 8, 20, -top - 8, body)
        fr.rect(lx, -14, 20, 14, dark)
    fr.poly([(60, cy - 32), (94, cy - 106), (126, cy - 100), (114, cy + 8)], body)
    if ears == "long":
        fr.ellipse(104, cy - 146, 9, 30, body, rot=-14)
        fr.ellipse(122, cy - 142, 9, 30, body, rot=12)
        fr.ellipse(104, cy - 146, 4, 22, dark, rot=-14, hull=False)
        fr.poly([(84, cy - 104), (104, cy - 112), (78, cy - 30), (64, cy - 34)], dark)
    else:
        fr.poly([(108, cy - 116), (114, cy - 142), (126, cy - 114)], body)
        fr.poly([(52, cy - 26), (88, cy - 110), (104, cy - 116), (72, cy - 22)], dark)
    # the head long and sloping down to the muzzle
    fr.ellipse(146, cy - 72, 54, 23, body, rot=58)
    fr.ellipse(169, cy - 36, 17, 15, light or dark, rot=58)
    fr.eye(130, cy - 94)
    fr.circle(176, cy - 30, 2.5, INK, hull=False)


def horse(p):
    equine(Frame(p, 200, NEAR, 1.0, 1), "#a0623a", "#4a2c1a")


def foal(p):
    equine(Frame(p, 510, NEAR, 0.6, -1), "#c58b5a", "#7a5236", leg=1.3)


def donkey(p):
    equine(Frame(p, 300, FAR, 0.66, -1), "#9b958c", "#5a5650", light="#ece8e1",
           ears="long", tuft=True)


# ── the cattle ──

def bovine(fr, body, patch=None, udder=False, horns=None, hump=False, ring=False,
           leg=1.0, face=None):
    """A cow, a calf or a bull, side on: boxy, its head low in front."""
    top = -126 * leg
    y0 = top - 98
    fr.line([(-122, y0 + 26), (-130, y0 + 120)], body, 7)
    fr.ellipse(-130, y0 + 132, 8, 16, INK)
    for lx in (-74, 66):
        fr.rect(lx, top, 20, -top, body)
        fr.shade(lx, top, 20, -top)
        fr.rect(lx, -14, 20, 14, INK, hull=False)
    fr.rect(-126, y0, 252, 112, body, rx=44)
    if hump:
        fr.ellipse(70, y0 + 12, 48, 24, body)
    if patch:
        for lx, ly, rx, ry in ((-62, 30, 34, 26), (28, 62, 38, 22), (84, 26, 22, 16), (-104, 70, 14, 18)):
            fr.ellipse(lx, y0 + ly, rx, ry, patch, hull=False)
    if udder:
        fr.ellipse(-46, top + 4, 24, 14, "#f2b8b8")
        for lx in (-58, -46, -34):
            fr.rect(lx - 2.5, top + 12, 5, 10, "#e9a0a0", hull=False)
    for lx in (-100, 90):
        fr.rect(lx, top + 8, 22, -top - 8, body)
        fr.rect(lx, -14, 22, 14, INK)
    fr.ellipse(120, y0 + 10, 18, 9, body, rot=-25)
    if horns:
        fr.line(quad((132, y0 - 6), (128, y0 - 36), (110, y0 - 44)), horns, 9)
        fr.line(quad((150, y0 - 4), (158, y0 - 34), (176, y0 - 42)), horns, 9)
    fr.ellipse(146, y0 + 36, 32, 44, face or body, rot=-18)
    if patch and not face:
        fr.ellipse(136, y0 + 16, 16, 14, patch, hull=False)
    fr.ellipse(160, y0 + 72, 26, 18, "#f2b8b8")
    fr.circle(152, y0 + 72, 3, "#9a5a5a", hull=False)
    fr.circle(168, y0 + 72, 3, "#9a5a5a", hull=False)
    if ring:
        fr.p.add(f'<circle cx="{f(fr.at(160, y0 + 86)[0])}" cy="{f(fr.at(160, y0 + 86)[1])}" '
                 f'r="{f(9 * fr.s)}" fill="none" stroke="#d8b04a" stroke-width="{f(4 * fr.s)}" />')
    fr.eye(152, y0 + 26)


def cow(p):
    bovine(Frame(p, 810, NEAR, 1.0, -1), WHITE, patch=INK, udder=True, horns="#e8dcc0")


def calf(p):
    bovine(Frame(p, 1040, NEAR, 0.56, -1), "#b07a4a", patch=WHITE, leg=1.25)


def bull(p):
    bovine(Frame(p, 1330, NEAR, 1.06, -1), "#6b3f26", horns="#f1ead8", hump=True, ring=True)


# ── the goats, the sheep and the pigs ──

def goat(p, fr=None, kid=False):
    fr = fr or Frame(p, 500, FAR, 0.8, 1)
    body = "#c69c6d" if kid else "#f1ece2"
    leg = 1.25 if kid else 1.0
    top = -92 * leg
    cy = top - 24
    fr.poly([(-60, cy - 6), (-76, cy - 30), (-56, cy - 18)], body)
    for lx in (-44, 40):
        fr.rect(lx, top, 10, -top, body)
        fr.shade(lx, top, 10, -top)
        fr.rect(lx, -8, 10, 8, INK, hull=False)
    fr.ellipse(0, cy, 66, 32, body)
    if kid:
        fr.ellipse(-8, cy - 4, 30, 18, "#f1ece2", hull=False)
    for lx in (-56, 50):
        fr.rect(lx, top + 6, 11, -top - 6, body)
        fr.rect(lx, -8, 11, 8, INK)
    fr.poly([(36, cy - 16), (56, cy - 60), (78, cy - 56), (64, cy + 6)], body)
    if not kid:
        fr.line(quad((70, cy - 72), (62, cy - 98), (42, cy - 100)), "#8a7a6a", 6)
        fr.poly([(94, cy - 30), (92, cy - 4), (106, cy - 24)], "#b8b0a2")
    fr.ellipse(64, cy - 56, 15, 6, body, rot=24)
    fr.ellipse(86, cy - 46, 28, 13, body, rot=55)
    fr.eye(80, cy - 56, 3)


def kid(p):
    goat(p, Frame(p, 650, FAR, 0.5, -1), kid=True)


def sheep(p, fr=None, lamb=False):
    fr = fr or Frame(p, 1310, FAR, 0.85, -1)
    leg = 1.2 if lamb else 1.0
    top = -64 * leg
    cy = top - 34
    face = "#efe6dc" if lamb else INK
    wool, shade = "#f4f1ea", "#e2ddd2"
    for lx in (-40, 34):
        fr.rect(lx, top, 10, -top, INK)
    for k in range(10):
        a = 2 * math.pi * k / 10
        fr.circle(56 * math.cos(a), cy + 30 * math.sin(a), 22, shade)
    fr.ellipse(0, cy, 64, 36, wool)
    for k in range(10):
        a = 2 * math.pi * (k + 0.5) / 10
        fr.circle(48 * math.cos(a), cy + 24 * math.sin(a), 18, wool, hull=False)
    for lx in (-52, 44):
        fr.rect(lx, top + 6, 11, -top - 6, INK)
    fr.ellipse(64, cy - 26, 15, 7, "#e8b4b4" if lamb else INK, rot=-10)
    fr.ellipse(80, cy - 16, 20, 26, face, rot=-24)
    fr.circle(70, cy - 40, 13, wool)
    fr.eye(86, cy - 22, 3)


def lamb(p):
    sheep(p, Frame(p, 1440, FAR, 0.52, -1), lamb=True)


def pig(p, fr=None):
    fr = fr or Frame(p, 960, FAR, 0.8, 1)
    body, deep = "#f2b3b3", "#e49c9c"
    fr.line([(-88, -92), (-102, -100), (-104, -112), (-92, -112), (-94, -102)], deep, 4)
    for lx in (-50, 38):
        fr.rect(lx, -60, 16, 60, deep)
    fr.ellipse(0, -82, 90, 48, body)
    for lx in (-64, 50):
        fr.rect(lx, -64, 18, 64, body)
        fr.rect(lx, -8, 18, 8, "#b06a6a")
    fr.circle(84, -94, 32, body)
    fr.ellipse(116, -88, 10, 16, deep)
    fr.ellipse(118, -94, 2.5, 4, "#9a5a5a", hull=False)
    fr.ellipse(118, -82, 2.5, 4, "#9a5a5a", hull=False)
    fr.poly([(66, -118), (80, -142), (94, -118)], deep)
    fr.eye(96, -104)


def piglet(p):
    pig(p, Frame(p, 1118, FAR, 0.42, -1))


# ── the things beyond the fence ──

def trough(p):
    rect(p, 50, 716, 10, 18, WOOD_DARK)
    rect(p, 170, 716, 10, 18, WOOD_DARK)
    p.poly([(32, 674), (198, 674), (186, 722), (44, 722)], "#8f9aa3")
    p.path("M38 690H192M42 706H188", stroke="#7f8c95", width=3)
    rect(p, 28, 668, 174, 10, "#7f8c95", rx=3)
    rect(p, 38, 672, 154, 6, "#8fd0ea", hull=False)


def hay_bale(p):
    p.poly([(722, 676), (742, 660), (842, 660), (822, 676)], "#f0d277")
    p.poly([(822, 676), (842, 660), (842, 712), (822, 728)], GOLD_DARK)
    rect(p, 722, 676, 100, 52, GOLD)
    p.path("M726 689H818M726 702H818M726 715H818", stroke="#d4ab45", width=2)
    p.path("M750 676V728M794 676V728M750 676L770 660M794 676L814 660", stroke="#8a6a4a", width=3)


# back to front, which is also the order of the regions
THINGS = [
    ("field", field, "the field of grain on the far hillside"),
    ("scarecrow", scarecrow, "the scarecrow, standing in the field"),
    ("windmill", windmill, "the windmill, its sails crossed"),
    ("tree", tree, "a tree"),
    ("stable", stable, "the stable, a half door to each stall"),
    ("tractor", tractor, "the tractor, red, facing left"),
    ("fence", fence, "the fence, posts and rails either side of the gate"),
    ("gate", gate, "the gate, shut"),
    ("water-trough", trough, "the water trough"),
    ("donkey", donkey, "the donkey, grey, beside the trough"),
    ("goat", goat, "the goat, white, bearded"),
    ("kid", kid, "her kid, facing her"),
    ("hay-bale", hay_bale, "a hay bale, tied twice"),
    ("pig", pig, "the pig"),
    ("piglet", piglet, "her piglet, facing her"),
    ("sheep", sheep, "the sheep, black-faced"),
    ("lamb", lamb, "her lamb"),
    ("horse", horse, "the horse, a bay, facing right"),
    ("foal", foal, "her foal, facing her"),
    ("cow", cow, "the cow, black and white"),
    ("calf", calf, "her calf, behind her"),
    ("bull", bull, "the bull, a ring in his nose"),
]


def main():
    svg_out = ROOT / f"public/scenes/{SCENE}/basic.svg"
    scene_file = ROOT / f"scenes/{SCENE}.json"
    svg_out.parent.mkdir(parents=True, exist_ok=True)

    lines = []
    w = lines.append
    w('<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1536 1024" width="1536" height="1024">')
    w(svg_comment("Ruya's pasture, drawn by art/pasture/basic.py: change the script and "
                  "run it again, rather than this file."))
    w("")
    w(svg_comment("the sky, the far hills, the pasture and the grass in front"))
    w('\t<rect width="1536" height="1024" fill="#cfe8f5" />')
    w('\t<path d="M0 400Q200 330 420 370T860 350T1260 340T1536 360V560H0Z" fill="#bcdca4" />')
    w('\t<path d="M0 472Q400 452 800 466T1536 456V1024H0Z" fill="#a8cf8a" />')
    w('\t<path d="M0 790Q400 770 800 784T1536 776V1024H0Z" fill="#b6d898" />')

    regions = []
    for thing, draw, comment in THINGS:
        p = Piles()
        draw(p)
        w("")
        w(svg_comment(comment))
        for el in p.els:
            w("\t" + el)
        d = "".join("M" + "L".join(f"{x} {y}" for x, y in ring) + "Z" for ring in p.rings())
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


if __name__ == "__main__":
    main()
