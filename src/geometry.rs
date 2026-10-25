//! Polygon regions: parsing an SVG path's `d` and the little geometry the app
//! and the checks need.
//!
//! A region is polygons only — M, L, H, V and Z, absolute or relative, any
//! number of subpaths. That is all a hit area needs (a plate is a 16-sided
//! polygon), it is exactly what a traced mask comes out as, and it keeps every
//! check here plain arithmetic. Several subpaths fill even-odd, so a ring
//! inside another is a hole.

pub type Point = (f64, f64);
pub type Ring = Vec<Point>;

/// The rings of a polygon-only path, each without its closing point repeated.
pub fn parse_path(d: &str) -> Result<Vec<Ring>, String> {
    let tokens = tokenize(d)?;
    let mut rings: Vec<Ring> = Vec::new();
    let mut ring: Ring = Vec::new();
    let mut cur: Point = (0.0, 0.0);
    let mut start: Point = (0.0, 0.0);
    let mut cmd: Option<char> = None;
    let mut i = 0;

    let close = |ring: &mut Ring, rings: &mut Vec<Ring>| {
        if ring.len() > 1 && ring.first() == ring.last() {
            ring.pop();
        }
        if !ring.is_empty() {
            rings.push(std::mem::take(ring));
        }
    };

    while i < tokens.len() {
        if let Token::Cmd(c) = tokens[i] {
            if cmd.is_none() && c != 'M' && c != 'm' {
                return Err("a path must start with M".into());
            }
            cmd = Some(c);
            i += 1;
            if c == 'Z' || c == 'z' {
                close(&mut ring, &mut rings);
                cur = start;
                continue;
            }
        }
        let c = cmd.ok_or("a path must start with M")?;
        let num = |k: usize| -> Result<f64, String> {
            match tokens.get(k) {
                Some(Token::Num(n)) => Ok(*n),
                _ => Err(format!("'{c}' is missing a number")),
            }
        };
        match c {
            'M' | 'm' => {
                let (x, y) = (num(i)?, num(i + 1)?);
                i += 2;
                close(&mut ring, &mut rings);
                cur = if c == 'm' {
                    (cur.0 + x, cur.1 + y)
                } else {
                    (x, y)
                };
                start = cur;
                ring.push(cur);
                // pairs after a moveto are linetos of the same kind
                cmd = Some(if c == 'm' { 'l' } else { 'L' });
            }
            'L' | 'l' => {
                let (x, y) = (num(i)?, num(i + 1)?);
                i += 2;
                cur = if c == 'l' {
                    (cur.0 + x, cur.1 + y)
                } else {
                    (x, y)
                };
                ring.push(cur);
            }
            'H' | 'h' => {
                let x = num(i)?;
                i += 1;
                cur.0 = if c == 'h' { cur.0 + x } else { x };
                ring.push(cur);
            }
            'V' | 'v' => {
                let y = num(i)?;
                i += 1;
                cur.1 = if c == 'v' { cur.1 + y } else { y };
                ring.push(cur);
            }
            other => {
                return Err(format!(
                    "'{other}' is not allowed: a region is polygons only (M L H V Z)"
                ));
            }
        }
    }
    close(&mut ring, &mut rings);
    if rings.is_empty() {
        return Err("the path has no points".into());
    }
    Ok(rings)
}

enum Token {
    Cmd(char),
    Num(f64),
}

fn tokenize(d: &str) -> Result<Vec<Token>, String> {
    let mut out = Vec::new();
    let chars: Vec<char> = d.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c.is_whitespace() || c == ',' {
            i += 1;
        } else if c.is_ascii_alphabetic() && c != 'e' && c != 'E' {
            out.push(Token::Cmd(c));
            i += 1;
        } else {
            // a number: sign, digits, one point, an optional exponent
            let begin = i;
            if chars[i] == '-' || chars[i] == '+' {
                i += 1;
            }
            let mut seen_point = false;
            while i < chars.len() && (chars[i].is_ascii_digit() || (chars[i] == '.' && !seen_point))
            {
                seen_point |= chars[i] == '.';
                i += 1;
            }
            if i < chars.len() && (chars[i] == 'e' || chars[i] == 'E') {
                i += 1;
                if i < chars.len() && (chars[i] == '-' || chars[i] == '+') {
                    i += 1;
                }
                while i < chars.len() && chars[i].is_ascii_digit() {
                    i += 1;
                }
            }
            let text: String = chars[begin..i].iter().collect();
            let n = text
                .parse::<f64>()
                .map_err(|_| format!("'{text}' is not a number"))?;
            out.push(Token::Num(n));
        }
    }
    Ok(out)
}

/// The shoelace area, positive or negative by winding.
fn signed_area(ring: &[Point]) -> f64 {
    let n = ring.len();
    (0..n)
        .map(|k| {
            let (a, b) = (ring[k], ring[(k + 1) % n]);
            a.0 * b.1 - b.0 * a.1
        })
        .sum::<f64>()
        / 2.0
}

pub fn area(ring: &[Point]) -> f64 {
    signed_area(ring).abs()
}

/// The centre of mass of a ring; the plain average where it has no area.
pub fn centroid(ring: &[Point]) -> Point {
    let a = signed_area(ring);
    let n = ring.len();
    if a.abs() < 1e-9 {
        let (sx, sy) = ring.iter().fold((0.0, 0.0), |s, p| (s.0 + p.0, s.1 + p.1));
        return (sx / n as f64, sy / n as f64);
    }
    let (mut cx, mut cy) = (0.0, 0.0);
    for k in 0..n {
        let (p, q) = (ring[k], ring[(k + 1) % n]);
        let cross = p.0 * q.1 - q.0 * p.1;
        cx += (p.0 + q.0) * cross;
        cy += (p.1 + q.1) * cross;
    }
    (cx / (6.0 * a), cy / (6.0 * a))
}

/// Inside by the even-odd rule, the same rule the region is filled with.
/// Only the checks need it: the browser does its own hit testing.
#[cfg(test)]
pub fn contains(rings: &[Ring], p: Point) -> bool {
    let mut inside = false;
    for ring in rings {
        let n = ring.len();
        for k in 0..n {
            let (a, b) = (ring[k], ring[(k + 1) % n]);
            if (a.1 > p.1) != (b.1 > p.1) {
                let x = a.0 + (p.1 - a.1) * (b.0 - a.0) / (b.1 - a.1);
                if p.0 < x {
                    inside = !inside;
                }
            }
        }
    }
    inside
}

/// (min x, min y, max x, max y) over every ring.
pub fn bounds(rings: &[Ring]) -> (f64, f64, f64, f64) {
    rings.iter().flatten().fold(
        (f64::MAX, f64::MAX, f64::MIN, f64::MIN),
        |(x0, y0, x1, y1), &(x, y)| (x0.min(x), y0.min(y), x1.max(x), y1.max(y)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn absolute_rectangle() {
        let rings = parse_path("M10 20H110V70H10Z").unwrap();
        assert_eq!(
            rings,
            vec![vec![
                (10.0, 20.0),
                (110.0, 20.0),
                (110.0, 70.0),
                (10.0, 70.0)
            ]]
        );
        assert_eq!(area(&rings[0]), 5000.0);
        assert_eq!(centroid(&rings[0]), (60.0, 45.0));
    }

    #[test]
    fn relative_commands_and_implied_lineto() {
        // after m, further pairs are relative linetos
        let rings = parse_path("m10,10 100,0 0,50 -100,0z").unwrap();
        assert_eq!(
            rings[0],
            vec![(10.0, 10.0), (110.0, 10.0), (110.0, 60.0), (10.0, 60.0)]
        );
        let rings = parse_path("M0 0 L4 0 4 3 Z").unwrap();
        assert_eq!(rings[0], vec![(0.0, 0.0), (4.0, 0.0), (4.0, 3.0)]);
        let rings = parse_path("M0 0h10v10h-10z").unwrap();
        assert_eq!(area(&rings[0]), 100.0);
    }

    #[test]
    fn numbers_without_separators() {
        let rings = parse_path("M1-2L3.5.5L-1e1 4Z").unwrap();
        assert_eq!(rings[0], vec![(1.0, -2.0), (3.5, 0.5), (-10.0, 4.0)]);
    }

    #[test]
    fn curves_are_refused() {
        let err = parse_path("M0 0C1 1 2 2 3 3Z").unwrap_err();
        assert!(err.contains("polygons only"), "{err}");
        assert!(parse_path("M0 0A5 5 0 0 1 10 0Z").is_err());
        assert!(parse_path("").is_err());
        assert!(parse_path("L1 1").is_err());
    }

    #[test]
    fn several_subpaths_and_holes() {
        // a 10×10 square with a 4×4 hole in it
        let rings = parse_path("M0 0H10V10H0Z M3 3H7V7H3Z").unwrap();
        assert_eq!(rings.len(), 2);
        assert!(contains(&rings, (1.0, 1.0)));
        assert!(!contains(&rings, (5.0, 5.0)));
        assert!(!contains(&rings, (11.0, 5.0)));
        assert_eq!(bounds(&rings), (0.0, 0.0, 10.0, 10.0));
    }

    #[test]
    fn the_closing_point_is_not_repeated() {
        let rings = parse_path("M0 0L4 0L4 3L0 0Z").unwrap();
        assert_eq!(rings[0].len(), 3);
    }
}
