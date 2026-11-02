//! Every name at once: each tag starts centred on its thing, and those that
//! would overlap, or run off the picture, are moved the least they need.
//!
//! Plain arithmetic on boxes, so `cargo test` can check it; the stage
//! measures the tags and applies what comes out.

/// A tag where it wants to be: its centre and its size.
#[derive(Clone, Copy, Debug)]
pub struct Tag {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

/// Room left between two tags, and between a tag and the picture's edge.
const GAP: f64 = 2.0;

/// How far to move each tag, in the same units as the tags, to keep them
/// apart and inside a `width` × `height` picture. Two that overlap are pushed
/// apart along whichever way frees them sooner, half each, so neither moves
/// far from its thing; a few rounds settle a cluster of three or four.
pub fn spread(tags: &[Tag], width: f64, height: f64) -> Vec<(f64, f64)> {
    let mut at: Vec<(f64, f64)> = tags.iter().map(|t| (t.x, t.y)).collect();
    let inside = |t: &Tag, (x, y): (f64, f64)| {
        let half = (t.w / 2.0 + GAP, t.h / 2.0 + GAP);
        (
            x.clamp(half.0, (width - half.0).max(half.0)),
            y.clamp(half.1, (height - half.1).max(half.1)),
        )
    };
    for (t, p) in tags.iter().zip(at.iter_mut()) {
        *p = inside(t, *p);
    }
    for _ in 0..40 {
        let mut moved = false;
        for a in 0..tags.len() {
            for b in a + 1..tags.len() {
                let (ta, tb) = (&tags[a], &tags[b]);
                let (pa, pb) = (at[a], at[b]);
                let over_x = (ta.w + tb.w) / 2.0 + GAP - (pa.0 - pb.0).abs();
                let over_y = (ta.h + tb.h) / 2.0 + GAP - (pa.1 - pb.1).abs();
                if over_x <= 0.0 || over_y <= 0.0 {
                    continue;
                }
                moved = true;
                // a hair more than half each, so the pair does come apart
                if over_y <= over_x {
                    let d = (over_y / 2.0 + 0.01) * if pa.1 <= pb.1 { 1.0 } else { -1.0 };
                    at[a].1 -= d;
                    at[b].1 += d;
                } else {
                    let d = (over_x / 2.0 + 0.01) * if pa.0 <= pb.0 { 1.0 } else { -1.0 };
                    at[a].0 -= d;
                    at[b].0 += d;
                }
                at[a] = inside(ta, at[a]);
                at[b] = inside(tb, at[b]);
            }
        }
        if !moved {
            break;
        }
    }
    tags.iter()
        .zip(at)
        .map(|(t, (x, y))| (x - t.x, y - t.y))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tag(x: f64, y: f64) -> Tag {
        Tag {
            x,
            y,
            w: 60.0,
            h: 20.0,
        }
    }

    fn apart(a: &Tag, b: &Tag, da: (f64, f64), db: (f64, f64)) -> bool {
        let (ax, ay, bx, by) = (a.x + da.0, a.y + da.1, b.x + db.0, b.y + db.1);
        (ax - bx).abs() >= (a.w + b.w) / 2.0 || (ay - by).abs() >= (a.h + b.h) / 2.0
    }

    #[test]
    fn tags_apart_and_inside_stay_put() {
        let moves = spread(&[tag(100.0, 100.0), tag(300.0, 100.0)], 800.0, 600.0);
        assert_eq!(moves, vec![(0.0, 0.0), (0.0, 0.0)]);
    }

    #[test]
    fn two_overlapping_tags_part_half_each() {
        let tags = [tag(100.0, 100.0), tag(110.0, 110.0)];
        let moves = spread(&tags, 800.0, 600.0);
        assert!(apart(&tags[0], &tags[1], moves[0], moves[1]));
        // up and down, the shorter way out
        assert!(moves[0].1 < 0.0 && moves[1].1 > 0.0);
        assert!((moves[0].1 + moves[1].1).abs() < 0.5);
    }

    #[test]
    fn a_tag_at_the_edge_comes_inside() {
        let moves = spread(&[tag(5.0, 300.0), tag(795.0, 598.0)], 800.0, 600.0);
        assert!(5.0 + moves[0].0 >= 30.0);
        assert!(795.0 + moves[1].0 <= 770.0);
        assert!(598.0 + moves[1].1 <= 590.0);
    }

    #[test]
    fn a_cluster_comes_apart() {
        let tags = [
            tag(400.0, 300.0),
            tag(405.0, 305.0),
            tag(395.0, 310.0),
            tag(410.0, 295.0),
        ];
        let moves = spread(&tags, 800.0, 600.0);
        for a in 0..tags.len() {
            for b in a + 1..tags.len() {
                assert!(apart(&tags[a], &tags[b], moves[a], moves[b]), "{a} and {b}");
            }
        }
    }
}
