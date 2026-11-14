#!/usr/bin/env python3
"""Ruya's farmyard, drawn: writes public/scenes/farmyard/basic.svg and the
basic style's regions in scenes/farmyard.json.

A farm's yard on a summer's day. Along the back the farmhouse with the cat
on its step, a beehive, the well with its bucket beside it, the red barn
with a pitchfork leaning on it, and the chicken coop; the pond on the left,
a duck and her ducklings swimming on it; in the middle the goose, the hen
and her chicks, the rooster and the turkey; in front the dog, the
wheelbarrow, a rabbit and a nest of eggs.

Each thing's region is the convex hull of its shapes; a thin one, the
pitchfork, is widened so a finger finds it. The things are drawn back to
front, and their regions are in the same order, so where a nearer thing
stands over one behind it, a tap there is the nearer one's.

Run it from anywhere, `python3 art/farmyard/basic.py`, after changing a
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

SCENE = "farmyard"

INK = "#2f3a44"
WOOD_DARK, WOOD, WOOD_LIGHT = "#8a6a4a", "#b98250", "#d9a66b"
ROOF, ROOF_EDGE = "#9b4a3a", "#86402f"
WHITE = "#ffffff"
BEAK, LEGS = "#e9a23b", "#e0a830"


def rect(p, x, y, w, h, fill, rx=0, hull=True):
    r = f' rx="{f(rx)}"' if rx else ""
    p.add(f'<rect x="{f(x)}" y="{f(y)}" width="{f(w)}" height="{f(h)}"{r} fill="{fill}" />')
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


def leg(p, top, foot, colour=LEGS, width=5):
    """A bird's leg and its three toes, forward to the right."""
    line(p, [top, foot], colour, width)
    x, y = foot
    p.lines([((x, y), (x + 11, y + 2)), ((x, y), (x + 6, y + 6)), ((x, y), (x - 7, y + 2))],
            colour, width - 1)


def eye(p, x, y, r=3):
    p.circle(x, y, r, INK, hull=False)


# ── along the back ──

def farmhouse(p):
    # the chimney first, behind the roof
    rect(p, 262, 228, 32, 80, WOOD_DARK)
    rect(p, 256, 222, 44, 12, "#6f5238", rx=2)
    rect(p, 60, 340, 280, 240, "#f3e3c3")
    p.poly([(36, 352), (200, 214), (364, 352)], ROOF)
    rect(p, 36, 346, 328, 12, ROOF_EDGE, rx=3)
    for wx in (84, 256):
        rect(p, wx, 392, 62, 56, WHITE, rx=3, hull=False)
        rect(p, wx + 6, 398, 50, 44, "#bfe3f5", hull=False)
        p.path(f"M{wx + 31} 398V442M{wx + 6} 420H{wx + 56}", stroke=WHITE, width=5)
    rect(p, 170, 470, 60, 110, "#8a5a3b", rx=3, hull=False)
    rect(p, 178, 480, 44, 36, "#9c6a48", rx=2, hull=False)
    p.circle(219, 528, 4, "#e0b050", hull=False)
    rect(p, 160, 578, 80, 10, "#c9bfae", rx=2)


def cat(p):
    # sitting on the step's right, facing out, its tail curled round
    line(p, quad((288, 582), (322, 584), (314, 552)), "#e0a15a", 9)
    p.ellipse(270, 560, 26, 28, "#e0a15a")
    p.path("M252 548Q262 552 268 546M252 562Q262 566 268 560", stroke="#c98a4a", width=3)
    p.circle(270, 522, 18, "#e0a15a")
    p.poly([(254, 514), (257, 494), (269, 507)], "#e0a15a")
    p.poly([(271, 507), (283, 494), (286, 514)], "#e0a15a")
    p.poly([(258, 510), (259, 499), (265, 507)], "#e8b4b4", hull=False)
    p.poly([(275, 507), (281, 499), (282, 510)], "#e8b4b4", hull=False)
    eye(p, 263, 520, 2.5)
    eye(p, 277, 520, 2.5)
    p.poly([(267, 527), (273, 527), (270, 531)], "#c97a6a", hull=False)


def beehive(p):
    rect(p, 392, 546, 10, 34, WOOD_DARK)
    rect(p, 448, 546, 10, 34, WOOD_DARK)
    rect(p, 384, 538, 82, 10, WOOD_DARK)
    rect(p, 388, 500, 74, 40, "#e9be5c")
    rect(p, 388, 468, 74, 34, "#f2cf73")
    rect(p, 410, 480, 30, 7, "#c79a3c", rx=3, hull=False)
    rect(p, 410, 512, 30, 7, "#c79a3c", rx=3, hull=False)
    rect(p, 402, 531, 46, 5, "#5a4030", hull=False)
    rect(p, 380, 456, 90, 14, "#a8743f", rx=2)
    # three bees about it, too small to count in its outline
    for bx, by in ((484, 476), (500, 500), (478, 446)):
        p.ellipse(bx, by - 5, 5, 4, "#ffffff", hull=False)
        p.ellipse(bx, by, 7, 5, "#f2c230", hull=False)
        p.path(f"M{bx - 1} {by - 5}V{by + 5}", stroke=INK, width=2.5)


def well(p):
    rect(p, 498, 398, 12, 100, WOOD_DARK)
    rect(p, 600, 398, 12, 100, WOOD_DARK)
    p.poly([(476, 412), (555, 352), (634, 412)], ROOF)
    rect(p, 476, 406, 158, 10, ROOF_EDGE, rx=3)
    rect(p, 504, 438, 104, 8, "#6f5238", hull=False)
    p.path("M612 442H624V462", stroke="#6f5238", width=6)
    p.path("M555 446V486", stroke="#c9a86a", width=3)
    rect(p, 490, 486, 130, 96, "#b8b0a2", rx=6)
    rect(p, 484, 480, 142, 14, "#a39b8d", rx=4)
    p.path("M490 520H620M490 552H620M530 494V520M580 494V520M510 520V552M560 520V552"
           "M605 520V552M535 552V582M585 552V582", stroke="#a39b8d", width=3)


def bucket(p):
    line(p, quad((646, 540), (675, 500), (704, 540)), "#5f6b73", 4)
    p.poly([(644, 540), (706, 540), (698, 594), (652, 594)], "#9aa7b0")
    p.path("M646 556H704M650 580H700", stroke="#7f8c95", width=4)
    p.ellipse(675, 540, 31, 7, "#c2ccd3")
    p.ellipse(675, 540, 25, 4, "#7f8c95", hull=False)


def barn(p):
    # its front, a gambrel, the roof's edge along the top of it
    p.poly([(744, 582), (744, 330), (790, 256), (940, 200), (1090, 256), (1136, 330), (1136, 582)],
           "#b9473b")
    line(p, [(730, 342), (782, 252), (940, 192), (1098, 252), (1150, 342)], "#7d3029", 22)
    p.path("M760 582V336L800 266L940 214L1080 266L1120 336V582", stroke="#f4efe4", width=6)
    # the hay loft, and the windows either side of the doors
    rect(p, 906, 268, 68, 62, "#5a2a24", hull=False)
    p.path("M910 330Q920 302 934 318Q946 296 958 316Q966 304 970 330Z", fill="#e6c35c")
    p.path("M906 268H974V330H906Z", stroke="#f4efe4", width=5)
    for wx in (786, 1046):
        rect(p, wx, 380, 48, 44, "#5a2a24", hull=False)
        p.path(f"M{wx} 380H{wx + 48}V424H{wx}ZM{wx + 24} 380V424", stroke="#f4efe4", width=5)
    # the big doors, each with its cross
    rect(p, 866, 430, 148, 152, "#9e3a30", hull=False)
    p.path("M866 582V430H1014V582M940 430V582M866 430L940 582M940 430L866 582"
           "M940 430L1014 582M1014 430L940 582", stroke="#f4efe4", width=5)


def pitchfork(p):
    # leaning on the barn's corner, tines up
    base, top = (1198, 592), (1170, 398)
    ux, uy = top[0] - base[0], top[1] - base[1]
    ln = math.hypot(ux, uy)
    ux, uy = ux / ln, uy / ln
    nx, ny = -uy, ux
    line(p, [base, top], "#a8743f", 8)
    cx, cy = top[0] + ux * 6, top[1] + uy * 6
    line(p, [(cx - nx * 20, cy - ny * 20), (cx + nx * 20, cy + ny * 20)], "#7f8c95", 6)
    for k in (-18, 0, 18):
        sx, sy = cx + nx * k, cy + ny * k
        line(p, [(sx, sy), (sx + ux * 64, sy + uy * 64)], "#8d969c", 5)
    # widened, so a finger finds it
    p.pts += [(x + dx, y + dy) for x, y in list(p.pts) for dx, dy in ((-14, 0), (14, 0))]


def coop(p):
    rect(p, 1266, 552, 12, 52, WOOD_DARK)
    rect(p, 1452, 552, 12, 52, WOOD_DARK)
    rect(p, 1252, 438, 232, 120, WOOD_LIGHT)
    p.path("M1252 468H1484M1252 498H1484M1252 528H1484", stroke="#c4904f", width=3)
    p.poly([(1238, 450), (1368, 372), (1498, 450)], WOOD_DARK)
    rect(p, 1238, 444, 260, 10, "#76593c", rx=3)
    p.path("M1300 558V522Q1300 504 1318 504Q1336 504 1336 522V558Z", fill="#5a4030")
    rect(p, 1388, 478, 64, 46, "#5a4030", hull=False)
    p.path("M1404 478V524M1420 478V524M1436 478V524M1388 494H1452M1388 509H1452",
           stroke="#d9d2c4", width=2)
    p.poly([(1300, 556), (1336, 556), (1290, 606), (1254, 606)], WOOD)
    p.path("M1290 570H1322M1279 584H1311M1268 597H1298", stroke=WOOD_DARK, width=3)


# ── the pond ──

def pond(p):
    p.ellipse(285, 815, 250, 118, "#8fc8e0")
    p.ellipse(300, 802, 200, 84, "#a3d3e7", hull=False)
    p.path("M150 760Q180 752 210 760M380 880Q410 872 440 880M180 900Q205 894 230 900",
           stroke="#d4ecf5", width=3)
    # reeds at its left end, and a lily pad
    p.path("M70 800Q66 740 74 700M86 806Q90 752 104 716M58 812Q48 770 44 736",
           stroke="#6f9e4f", width=5)
    for cx, cy, a in ((74, 712, -4), (102, 728, 18), (45, 748, -14)):
        p.ellipse(cx, cy, 5, 14, "#8a5a3b", rot=a, hull=False)
    p.path("M470 858A22 12 0 1 1 492 870L470 858Z", fill="#7fb46a")


def duckling(p, x, y):
    """A duckling swimming right, its body's middle at (x, y)."""
    p.poly([(x - 30, y - 8), (x - 18, y - 2), (x - 22, y + 6)], "#f0c43c")
    p.ellipse(x, y, 26, 17, "#f5d24b")
    p.ellipse(x - 4, y - 2, 13, 8, "#ecc23a", hull=False)
    p.circle(x + 20, y - 21, 14, "#f5d24b")
    p.poly([(x + 31, y - 24), (x + 44, y - 19), (x + 31, y - 14)], BEAK)
    eye(p, x + 24, y - 25, 2.5)


def duck(p):
    # white, swimming right, ahead of her ducklings
    p.poly([(330, 800), (348, 782), (358, 812)], "#e9e9e4")
    p.ellipse(395, 815, 58, 28, WHITE)
    p.ellipse(384, 812, 32, 15, "#e4e4de", hull=False)
    p.poly([(425, 805), (431, 772), (453, 774), (452, 808)], WHITE)
    p.circle(442, 768, 20, WHITE)
    p.poly([(458, 762), (484, 768), (482, 777), (458, 778)], BEAK)
    eye(p, 448, 762)


# ── the birds and beasts in the middle ──

def goose(p):
    leg(p, (600, 766), (597, 790), "#e8902a", 6)
    leg(p, (628, 766), (631, 790), "#e8902a", 6)
    p.poly([(538, 714), (566, 698), (568, 734)], "#8c867c")
    p.ellipse(610, 730, 64, 40, "#9c968c")
    p.ellipse(624, 748, 46, 22, "#efeae0", hull=False)
    p.ellipse(596, 722, 40, 22, "#857f75", hull=False)
    line(p, quad((646, 716), (676, 682), (660, 642)), "#9c968c", 24)
    p.circle(663, 632, 20, "#9c968c")
    p.poly([(679, 624), (706, 633), (679, 642)], "#e8902a")
    p.circle(680, 626, 5, "#e8902a", hull=False)
    eye(p, 668, 626)


def hen(p):
    leg(p, (782, 760), (780, 786))
    leg(p, (800, 760), (804, 786))
    p.poly([(724, 712), (734, 658), (758, 674), (762, 716)], "#8f5428")
    p.ellipse(790, 728, 58, 42, "#b9733f")
    p.ellipse(780, 726, 34, 22, "#a3612f", hull=False)
    p.poly([(818, 702), (826, 668), (852, 672), (846, 710)], "#b9733f")
    p.circle(838, 676, 22, "#b9733f")
    for cx, cy, r in ((831, 652, 7), (841, 649, 8), (851, 654, 7)):
        p.circle(cx, cy, r, "#d8323a")
    p.ellipse(855, 694, 5, 8, "#d8323a")
    p.poly([(857, 672), (872, 679), (857, 686)], LEGS)
    eye(p, 845, 670)


def chick(p, x):
    """A chick standing at x, facing right, by its mother."""
    leg(p, (x - 6, 776), (x - 7, 790), width=4)
    leg(p, (x + 6, 776), (x + 7, 790), width=4)
    p.circle(x, 762, 24, "#f6d34e")
    p.ellipse(x - 4, 764, 12, 9, "#ecc23a", hull=False)
    p.circle(x + 14, 732, 15, "#f6d34e")
    p.poly([(x + 27, 727), (x + 38, 733), (x + 27, 739)], BEAK)
    eye(p, x + 18, 728, 2.5)


def rooster(p):
    # his tail's long feathers arching up behind him
    for s, c, t, colour in (((1126, 716), (1070, 660), (1098, 604), "#2f5d4a"),
                            ((1124, 722), (1078, 690), (1082, 634), "#1f3d33"),
                            ((1130, 714), (1096, 646), (1124, 606), "#3f7a5f")):
        line(p, quad(s, c, t), colour, 14)
    leg(p, (1152, 752), (1148, 790), width=6)
    leg(p, (1172, 752), (1176, 790), width=6)
    p.ellipse(1163, 718, 52, 40, "#c85a28")
    p.ellipse(1155, 714, 30, 20, "#a8481f", hull=False)
    p.poly([(1181, 700), (1191, 650), (1219, 652), (1215, 702)], "#e8a33c")
    p.circle(1207, 638, 19, "#c85a28")
    for cx, cy, r in ((1195, 616, 8), (1206, 611, 9), (1217, 614, 8), (1225, 622, 6)):
        p.circle(cx, cy, r, "#d8323a")
    p.ellipse(1220, 662, 7, 11, "#d8323a")
    p.poly([(1224, 634), (1239, 640), (1224, 646)], LEGS)
    eye(p, 1212, 632)


def turkey(p):
    # his tail spread in a fan behind him
    p.circle(1350, 698, 88, "#6b4430")
    p.circle(1350, 698, 76, "#c9a06b", hull=False)
    p.circle(1350, 698, 64, "#8a5a3b", hull=False)
    p.path("".join(f"M1350 698L{f(1350 + 64 * math.cos(a))} {f(698 + 64 * math.sin(a))}"
                   for a in (math.radians(d) for d in range(190, 360, 20))),
           stroke="#6b4430", width=3)
    line(p, [(1340, 784), (1336, 806)], "#c9a08a", 6)
    line(p, [(1362, 784), (1366, 806)], "#c9a08a", 6)
    p.ellipse(1350, 742, 50, 46, "#4a3628")
    p.ellipse(1340, 742, 30, 24, "#3a2a20", hull=False)
    p.poly([(1374, 724), (1384, 676), (1402, 680), (1398, 728)], "#4a3628")
    p.circle(1396, 668, 14, "#a7c0d0")
    line(p, [(1404, 660), (1414, 690)], "#d33a3a", 6)
    p.ellipse(1398, 690, 7, 12, "#d33a3a")
    p.poly([(1408, 664), (1420, 669), (1408, 674)], "#e0c080")
    eye(p, 1400, 664, 2.5)


# ── in front ──

def dog(p):
    # a sheepdog, black and white, standing, facing right
    line(p, quad((618, 890), (584, 874), (590, 842)), INK, 14)
    p.circle(590, 840, 8, "#f4f1ea")
    # the hind legs black with white socks, the forelegs white
    for lx in (636, 662):
        rect(p, lx, 908, 16, 70, INK, rx=5)
        rect(p, lx, 952, 16, 26, "#f4f1ea", rx=5, hull=False)
    for lx in (716, 742):
        rect(p, lx, 908, 16, 70, "#f4f1ea", rx=5)
    p.ellipse(690, 900, 70, 32, INK)
    p.ellipse(746, 906, 22, 26, "#f4f1ea")
    p.poly([(748, 842), (752, 812), (768, 834)], INK)
    p.poly([(772, 832), (784, 808), (792, 834)], INK)
    p.circle(770, 856, 28, INK)
    p.poly([(771, 830), (779, 830), (786, 862), (768, 862)], "#f4f1ea", hull=False)
    p.ellipse(792, 868, 20, 12, "#f4f1ea")
    p.circle(810, 864, 6, "#1f2a30")
    p.circle(782, 848, 5, "#f4f1ea", hull=False)
    p.circle(783, 848, 3, "#1f2a30", hull=False)


def wheelbarrow(p):
    line(p, [(860, 898), (950, 934)], WOOD_DARK, 9)
    line(p, [(930, 944), (924, 980)], "#6b5640", 8)
    p.poly([(900, 880), (1080, 880), (1050, 950), (930, 950)], "#4f8a5b")
    rect(p, 894, 874, 192, 10, "#3f7049", rx=4)
    p.path("M1044 946L1066 960", stroke="#3f7049", width=7)
    p.circle(1066, 960, 28, "#3a3a3a")
    p.circle(1066, 960, 14, "#c0392b", hull=False)
    p.circle(1066, 960, 5, "#3a3a3a", hull=False)


def rabbit(p):
    p.circle(1156, 950, 12, WHITE)
    p.ellipse(1196, 950, 44, 32, "#a8998a")
    p.ellipse(1184, 958, 26, 22, "#9a8b7c", hull=False)
    p.ellipse(1226, 868, 8, 28, "#a8998a", rot=-10)
    p.ellipse(1246, 870, 8, 28, "#a8998a", rot=12)
    p.ellipse(1226, 870, 4, 19, "#e8b4b4", rot=-10, hull=False)
    p.ellipse(1246, 872, 4, 19, "#e8b4b4", rot=12, hull=False)
    p.circle(1236, 910, 22, "#a8998a")
    p.ellipse(1234, 978, 12, 6, "#b9ab9c")
    eye(p, 1244, 904, 3.5)
    p.circle(1257, 914, 3.5, "#e8a0a0", hull=False)


def nest(p):
    p.ellipse(1395, 950, 72, 22, "#b8903f")
    for ex, ey, rx, ry, c in ((1366, 936, 15, 20, "#fbf3e2"), (1396, 930, 16, 21, "#f4e6c8"),
                              (1426, 938, 15, 20, "#fbf3e2")):
        p.ellipse(ex, ey, rx, ry, c)
    p.ellipse(1395, 962, 74, 18, "#c9a050")
    p.path("M1336 958L1366 966M1380 970L1412 962M1426 968L1456 958M1350 970L1372 974",
           stroke="#a37a2e", width=3)


# back to front, which is also the order of the regions
THINGS = [
    ("farmhouse", farmhouse, "the farmhouse, its chimney behind the roof"),
    ("cat", cat, "the cat, sitting on the step beside the door"),
    ("beehive", beehive, "a beehive on its stand, its bees about it"),
    ("well", well, "the well, its roof on two posts and a rope to the water"),
    ("bucket", bucket, "a bucket beside the well"),
    ("barn", barn, "the red barn, hay in its loft"),
    ("pitchfork", pitchfork, "a pitchfork leaning on the barn's corner"),
    ("chicken-coop", coop, "the chicken coop on its legs, a ramp to its door"),
    ("pond", pond, "the pond, reeds at its end and a lily pad"),
    ("duckling", lambda p: duckling(p, 110, 846), "a duckling, the last of three"),
    ("duckling", lambda p: duckling(p, 190, 838), "a duckling"),
    ("duckling", lambda p: duckling(p, 270, 846), "a duckling, close behind its mother"),
    ("duck", duck, "the duck, white, swimming ahead of her ducklings"),
    ("goose", goose, "the goose, on the pond's bank"),
    ("hen", hen, "the hen, her chicks beside her"),
    ("chick", lambda p: chick(p, 900), "a chick"),
    ("chick", lambda p: chick(p, 965), "a chick"),
    ("chick", lambda p: chick(p, 1030), "a chick"),
    ("rooster", rooster, "the rooster, his tail arching behind him"),
    ("turkey", turkey, "the turkey, his tail spread"),
    ("dog", dog, "the dog, a black-and-white sheepdog"),
    ("wheelbarrow", wheelbarrow, "the wheelbarrow, its wheel in front"),
    ("rabbit", rabbit, "a rabbit, sitting up"),
    ("nest", nest, "a nest of three eggs, laid on the ground"),
]


def main():
    svg_out = ROOT / f"public/scenes/{SCENE}/basic.svg"
    scene_file = ROOT / f"scenes/{SCENE}.json"
    svg_out.parent.mkdir(parents=True, exist_ok=True)

    lines = []
    w = lines.append
    w('<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1536 1024" width="1536" height="1024">')
    w(svg_comment("Ruya's farmyard, drawn by art/farmyard/basic.py: change the script and "
                  "run it again, rather than this file."))
    w("")
    w(svg_comment("the sky, the hills, the grass behind the buildings and the yard"))
    w('\t<rect width="1536" height="1024" fill="#cfe8f5" />')
    w('\t<path d="M0 430Q180 360 380 410T760 400T1140 390T1536 410V600H0Z" fill="#bcdca4" />')
    w('\t<path d="M0 520Q300 490 640 515T1536 505V640H0Z" fill="#a8cf8a" />')
    w('\t<path d="M0 610Q400 585 800 600T1536 595V1024H0Z" fill="#e7d6aa" />')
    w('\t<ellipse cx="285" cy="818" rx="282" ry="146" fill="#a8cf8a" />')

    regions = []
    for thing, draw, comment in THINGS:
        p = Pile()
        draw(p)
        w("")
        w(svg_comment(comment))
        for el in p.els:
            w("\t" + el)
        ring = simplify(hull(p.pts))
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


if __name__ == "__main__":
    main()
