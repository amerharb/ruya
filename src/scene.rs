//! A scene: one picture, and the things in it that have a name.
//!
//! The picture is only ever shown as an image — an SVG drawn by hand today,
//! a bitmap later — and the things are separate polygons in the picture's own
//! coordinates, laid over it. So a drawn scene and a painted one take the same
//! path through the app.
//!
//! Scene files are embedded at compile time: one less request before the
//! picture appears, and `cargo test` checks every file, so the parse below
//! cannot fail in a browser.

use crate::geometry::{self, Ring};
use crate::lang::Names;
use serde::Deserialize;
use std::sync::LazyLock;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Scene {
    pub id: String,
    pub name: Names,
    pub picture: Picture,
    /// In paint order: where two overlap, the later one is the one tapped.
    pub items: Vec<Item>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Picture {
    /// Relative to the page, no leading slash, so Ruya also works under a
    /// sub-path. A redrawn picture gets a new name: it is not content-hashed.
    pub src: String,
    /// The coordinate space every region is written in, and the picture's
    /// aspect ratio. The file may have more pixels than this, in proportion.
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Item {
    pub id: String,
    /// The region, polygons only — see [`geometry`].
    pub d: String,
    /// A point on the item the name belongs to. Left out, it is the middle of
    /// the item's largest part; given where that middle falls off the item,
    /// as it does for a chair seen between its legs.
    #[serde(default)]
    pub at: Option<[f64; 2]>,
    pub name: Names,
    /// Why a word was chosen, where that is not obvious. JSON has no comments.
    #[serde(default)]
    #[allow(dead_code)]
    pub note: Option<String>,
    #[serde(skip)]
    pub anchor: (f64, f64),
    /// The item's highest point. The name is shown above it, at the
    /// anchor's x, so the label never hides the thing it names.
    #[serde(skip)]
    pub top: f64,
}

impl Scene {
    pub fn parse(json: &str) -> Result<Scene, String> {
        let mut scene: Scene = serde_json::from_str(json).map_err(|e| e.to_string())?;
        for item in &mut scene.items {
            let rings = geometry::parse_path(&item.d).map_err(|e| format!("{}: {e}", item.id))?;
            item.anchor = match item.at {
                Some([x, y]) => (x, y),
                None => largest(&rings)
                    .map(geometry::centroid)
                    .unwrap_or((0.0, 0.0)),
            };
            item.top = geometry::bounds(&rings).1;
        }
        Ok(scene)
    }
}

fn largest(rings: &[Ring]) -> Option<&[(f64, f64)]> {
    rings
        .iter()
        .max_by(|a, b| geometry::area(a).total_cmp(&geometry::area(b)))
        .map(Vec::as_slice)
}

/// Every scene, by id, as embedded. A file in `scenes/` that is not listed
/// here fails `cargo test`.
pub const SOURCES: &[(&str, &str)] = &[("kitchen", include_str!("../scenes/kitchen.json"))];

pub static KITCHEN: LazyLock<Scene> = LazyLock::new(|| {
    Scene::parse(SOURCES[0].1).expect("scenes/kitchen.json is checked by cargo test")
});

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::{bounds, contains, parse_path};
    use crate::lang::Lang;
    use std::collections::HashSet;
    use std::path::Path;

    /// The shortest side an item may have, in picture units. On a phone held
    /// upright the picture fills the screen's height and scrolls sideways, at
    /// about three quarters of picture size, so 48 is a target of about 36 CSS
    /// pixels — a fingertip, roughly.
    const MIN_SIDE: f64 = 48.0;

    fn root() -> &'static Path {
        Path::new(env!("CARGO_MANIFEST_DIR"))
    }

    /// Every scene file on disk, parsed, with its name.
    fn scenes() -> Vec<(String, Scene)> {
        let mut out = Vec::new();
        for entry in std::fs::read_dir(root().join("scenes")).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            let name = path.file_name().unwrap().to_string_lossy().into_owned();
            let json = std::fs::read_to_string(&path).unwrap();
            let scene = Scene::parse(&json).unwrap_or_else(|e| panic!("{name}: {e}"));
            out.push((name, scene));
        }
        assert!(!out.is_empty(), "no scene files");
        out
    }

    #[test]
    fn every_file_is_embedded_and_every_embedded_scene_has_a_file() {
        let files: HashSet<String> = scenes().into_iter().map(|(_, s)| s.id).collect();
        let listed: HashSet<String> = SOURCES.iter().map(|(id, _)| id.to_string()).collect();
        assert_eq!(files, listed);
        for (id, json) in SOURCES {
            assert_eq!(&Scene::parse(json).unwrap().id, id);
        }
    }

    #[test]
    fn ids_are_unique_and_plain() {
        for (file, scene) in scenes() {
            let mut seen = HashSet::new();
            for item in &scene.items {
                assert!(
                    !item.id.is_empty()
                        && item
                            .id
                            .chars()
                            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-'),
                    "{file}: id '{}' is not [a-z0-9-]",
                    item.id
                );
                assert!(seen.insert(&item.id), "{file}: '{}' twice", item.id);
            }
        }
    }

    /// Whether a name is written in its language's own script — which catches
    /// a word pasted into the wrong language's field.
    fn in_script(lang: Lang, word: &str) -> bool {
        word.chars().filter(|c| c.is_alphabetic()).all(|c| {
            let u = c as u32;
            match lang {
                Lang::Ar => (0x0600..=0x06FF).contains(&u),
                Lang::He => (0x0590..=0x05FF).contains(&u),
                Lang::Uk => (0x0400..=0x04FF).contains(&u),
                // Latin, with the accents Spanish, German and Swedish use
                _ => c.is_ascii_alphabetic() || (0x00C0..=0x017F).contains(&u),
            }
        })
    }

    fn check_names(file: &str, what: &str, names: &Names) {
        for lang in Lang::ALL {
            let word = names.get(lang);
            assert!(
                !word.trim().is_empty(),
                "{file}: {what} has no {} name",
                lang.code()
            );
            assert_eq!(
                word,
                word.trim(),
                "{file}: {what}'s {} name has spaces at an end",
                lang.code()
            );
            assert!(
                in_script(lang, word),
                "{file}: {what}'s {} name '{word}' is not in that language's script",
                lang.code()
            );
        }
    }

    #[test]
    fn every_name_is_there_and_in_its_script() {
        for (file, scene) in scenes() {
            check_names(&file, "the scene", &scene.name);
            for item in &scene.items {
                check_names(&file, &item.id, &item.name);
            }
        }
    }

    #[test]
    fn shapes_are_inside_the_picture_and_big_enough() {
        for (file, scene) in scenes() {
            let (w, h) = (scene.picture.width, scene.picture.height);
            for item in &scene.items {
                let rings = parse_path(&item.d).unwrap();
                for ring in &rings {
                    assert!(
                        ring.len() >= 3,
                        "{file}: {} has a ring of {} points",
                        item.id,
                        ring.len()
                    );
                }
                let (x0, y0, x1, y1) = bounds(&rings);
                assert!(
                    x0 >= 0.0 && y0 >= 0.0 && x1 <= w && y1 <= h,
                    "{file}: {} reaches outside the {w}×{h} picture",
                    item.id
                );
                let side = (x1 - x0).min(y1 - y0);
                assert!(
                    side >= MIN_SIDE,
                    "{file}: {} is {side} across at its narrowest, under {MIN_SIDE}",
                    item.id
                );
            }
        }
    }

    #[test]
    fn every_label_point_is_on_its_item() {
        for (file, scene) in scenes() {
            for item in &scene.items {
                let rings = parse_path(&item.d).unwrap();
                assert!(
                    contains(&rings, item.anchor),
                    "{file}: {}'s label point {:?} is off the item — give it an \"at\"",
                    item.id,
                    item.anchor
                );
            }
        }
    }

    #[test]
    fn a_thing_lying_on_another_comes_after_it() {
        // the plate on the table: drawn after it, so a tap on the plate is the
        // plate's, not the table's
        for (file, scene) in scenes() {
            let rings: Vec<_> = scene
                .items
                .iter()
                .map(|i| parse_path(&i.d).unwrap())
                .collect();
            for (a, outer) in rings.iter().enumerate() {
                for (b, inner) in rings.iter().enumerate() {
                    if a == b {
                        continue;
                    }
                    let within = inner.iter().flatten().all(|&p| contains(outer, p));
                    assert!(
                        !within || b > a,
                        "{file}: {} lies on {} but comes before it",
                        scene.items[b].id,
                        scene.items[a].id
                    );
                }
            }
        }
    }

    #[test]
    fn the_picture_exists_and_matches_the_scene() {
        for (file, scene) in scenes() {
            let pic = &scene.picture;
            assert!(
                !pic.src.starts_with('/'),
                "{file}: picture src must be relative"
            );
            let path = root().join("public").join(&pic.src);
            assert!(path.exists(), "{file}: no picture at {}", path.display());
            if pic.src.ends_with(".svg") {
                let svg = std::fs::read_to_string(&path).unwrap();
                let view_box = format!("viewBox=\"0 0 {} {}\"", pic.width, pic.height);
                assert!(
                    svg.contains(&view_box),
                    "{file}: {} has no {view_box}",
                    pic.src
                );
            }
        }
    }
}
