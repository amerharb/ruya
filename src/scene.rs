//! A scene: a place, the things in it that have a name, and the styles it
//! can be shown in.
//!
//! Names belong to the things, once per scene. Each style — a flat drawing, a
//! realistic picture, a Ghibli-like painting — has its own picture and its own
//! regions over it, since the same fridge stands somewhere else in each; a
//! region says which thing it is. Two regions may be the same thing: the
//! realistic kitchen has two chairs.
//!
//! The picture is only ever shown as an image, and the regions are polygons in
//! the picture's own coordinates laid over it, so an SVG and a bitmap take the
//! same path through the app.
//!
//! Scene files are embedded at compile time: one less request before the
//! picture appears, and `cargo test` checks every file, so the parse below
//! cannot fail in a browser.

use crate::geometry::{self, Ring};
use crate::lang::{Duals, Lang, Names, Number, Plurals, Readings};
use crate::style::Style;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::sync::LazyLock;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Scene {
    pub id: String,
    pub name: Names,
    /// Every thing with a name, by id.
    pub things: BTreeMap<String, Thing>,
    /// Words painted in the pictures, by id: a sign's, in every language.
    /// The app shows them as patches (see [`Spot`]); `tools/signs.py` makes
    /// the patches from these words, and `cargo test` checks them.
    #[serde(default)]
    #[allow(dead_code)]
    pub signs: BTreeMap<String, Sign>,
    pub styles: Styles,
}

/// A sign painted in a scene's pictures — the bus's destination — its words
/// in every language.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(dead_code)]
pub struct Sign {
    pub text: Names,
    /// Why the words were chosen, as a thing's note.
    #[serde(default)]
    pub note: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Thing {
    /// Its name for one of it, the one shown.
    pub name: Names,
    /// For many of it.
    pub plural: Plurals,
    /// For two of it, in a language with a dual.
    pub dual: Duals,
    /// Its name for one with readings over it, where the language has them
    /// and the name needs them: Japanese with kanji.
    #[serde(default)]
    pub reading: Readings,
    /// Why a word was chosen, where that is not obvious. JSON has no comments.
    #[serde(default)]
    #[allow(dead_code)]
    pub note: Option<String>,
}

impl Thing {
    /// Its name for one, two or many of it, where the language has that
    /// number.
    pub fn name_for(&self, lang: Lang, number: Number) -> Option<&str> {
        match number {
            Number::Singular => Some(self.name.get(lang)),
            Number::Dual => self.dual.get(lang),
            Number::Plural => self.plural.get(lang),
        }
    }
}

/// One field per style, like the names: a style the app does not know fails
/// to parse. Any of them may be left out.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Styles {
    #[serde(default)]
    pub basic: Option<Rendering>,
    #[serde(default)]
    pub realistic: Option<Rendering>,
    #[serde(default)]
    pub ghibli: Option<Rendering>,
}

/// The scene in one style.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Rendering {
    pub picture: Picture,
    /// In paint order: where two overlap, the later one is the one tapped.
    pub regions: Vec<Region>,
    /// The signs painted in this picture, each followed into the chosen
    /// language by a patch.
    #[serde(default)]
    pub signs: Vec<Spot>,
}

/// Where a sign is in one style's picture, and the language the picture
/// shows it in. In any other language a patch is laid over it: a small
/// picture of the sign with that language's words, made by `tools/signs.py`
/// and named `<style>.<sign>.<lang>.webp` beside the picture.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Spot {
    /// The id of the sign this is.
    pub sign: String,
    /// The patch's place: left, top, right and bottom, in picture units.
    pub rect: [f64; 4],
    /// The language the picture shows the sign in, which needs no patch.
    pub lang: String,
}

impl Spot {
    /// The patch showing this sign in `lang`, over the picture of scene
    /// `scene` in `style`; none in the language the picture already shows.
    pub fn patch(&self, scene: &str, style: Style, lang: Lang) -> Option<String> {
        (lang.code() != self.lang).then(|| {
            format!(
                "scenes/{scene}/{}.{}.{}.webp",
                style.code(),
                self.sign,
                lang.code()
            )
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Picture {
    /// Relative to the page, no leading slash, so Ruya also works under a
    /// sub-path. A redrawn picture gets a new name: it is not content-hashed.
    pub src: String,
    /// A small copy for the style switch, the same shape.
    pub thumb: String,
    /// The coordinate space every region is written in, and the picture's
    /// aspect ratio. The file may have more pixels than this, in proportion.
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Region {
    /// The id of the thing this is.
    pub thing: String,
    /// The region, polygons only — see [`geometry`].
    pub d: String,
    /// A point on the region whose x the name is centred on. Left out, it is
    /// the middle of the region's largest part; given where that middle falls
    /// off it, as it does for a chair seen between its legs.
    #[serde(default)]
    pub at: Option<[f64; 2]>,
    #[serde(skip)]
    pub anchor: (f64, f64),
    /// The region's highest point. The name is shown above it, so the label
    /// never hides the thing it names.
    #[serde(skip)]
    pub top: f64,
    /// The area of its largest part: of two chairs, the bigger one carries
    /// the name when every name is shown.
    #[serde(skip)]
    pub area: f64,
}

/// A step round the scenes, as the picture's edges and a swipe take it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Step {
    /// To the scene before: the left edge, or a swipe to the right.
    Previous,
    /// To the scene after: the right edge, or a swipe to the left.
    Next,
}

impl Step {
    /// An edge's `data-side`, and the `data-came` of a stage the step
    /// reached.
    pub fn code(self) -> &'static str {
        match self {
            Step::Previous => "previous",
            Step::Next => "next",
        }
    }
}

impl Scene {
    pub fn parse(json: &str) -> Result<Scene, String> {
        let mut scene: Scene = serde_json::from_str(json).map_err(|e| e.to_string())?;
        for style in Style::ALL {
            let Some(rendering) = scene.styles.get_mut(style) else {
                continue;
            };
            for region in &mut rendering.regions {
                let rings = geometry::parse_path(&region.d)
                    .map_err(|e| format!("{} {}: {e}", style.code(), region.thing))?;
                region.anchor = match region.at {
                    Some([x, y]) => (x, y),
                    None => largest(&rings)
                        .map(geometry::centroid)
                        .unwrap_or((0.0, 0.0)),
                };
                region.top = geometry::bounds(&rings).1;
                region.area = largest(&rings).map(geometry::area).unwrap_or(0.0);
            }
        }
        Ok(scene)
    }

    /// The styles this scene has, in the switch's order.
    pub fn renderings(&self) -> impl Iterator<Item = (Style, &Rendering)> {
        Style::ALL
            .into_iter()
            .filter_map(|s| self.styles.get(s).map(|r| (s, r)))
    }

    /// The scene in the style asked for, or in its first style when it does
    /// not have that one.
    pub fn rendering_or_first(&self, style: Style) -> (Style, &Rendering) {
        self.styles
            .get(style)
            .map(|r| (style, r))
            .or_else(|| self.renderings().next())
            .expect("every scene has a style — checked by cargo test")
    }

    pub fn thing(&self, id: &str) -> &Thing {
        self.things
            .get(id)
            .expect("every region names a thing — checked by cargo test")
    }

    pub fn find(id: &str) -> Option<&'static Scene> {
        SCENES.iter().find(|s| s.id == id.trim())
    }

    pub fn first() -> &'static Scene {
        &SCENES[0]
    }

    /// The scene after this one in the picker's order, the first after the
    /// last: where the picture's right edge goes.
    pub fn next(&self) -> &'static Scene {
        self.ahead(1)
    }

    /// The scene before this one, the last before the first: where the
    /// picture's left edge goes.
    pub fn previous(&self) -> &'static Scene {
        self.ahead(SCENES.len() - 1)
    }

    /// The scene a step from this one reaches.
    pub fn step(&self, step: Step) -> &'static Scene {
        match step {
            Step::Previous => self.previous(),
            Step::Next => self.next(),
        }
    }

    fn ahead(&self, by: usize) -> &'static Scene {
        let i = SCENES.iter().position(|s| s.id == self.id).unwrap_or(0);
        &SCENES[(i + by) % SCENES.len()]
    }

    /// The word for "place", the scene picker's accessible name.
    pub fn label(lang: Lang) -> &'static str {
        match lang {
            Lang::En => "Place",
            Lang::Ar => "المكان",
            Lang::De => "Ort",
            Lang::Sv => "Plats",
            Lang::Es => "Lugar",
            Lang::Uk => "Місце",
            Lang::He => "מקום",
            Lang::Ja => "場所",
        }
    }
}

impl Rendering {
    /// One region per thing, the biggest, in paint order: where every name is
    /// shown, each name once.
    pub fn one_per_thing(&self) -> Vec<usize> {
        let mut biggest = BTreeMap::<&str, usize>::new();
        for (i, region) in self.regions.iter().enumerate() {
            let best = biggest.entry(&region.thing).or_insert(i);
            if region.area > self.regions[*best].area {
                *best = i;
            }
        }
        let mut chosen: Vec<usize> = biggest.into_values().collect();
        chosen.sort_unstable();
        chosen
    }
}

impl Styles {
    pub fn get(&self, style: Style) -> Option<&Rendering> {
        match style {
            Style::Basic => self.basic.as_ref(),
            Style::Realistic => self.realistic.as_ref(),
            Style::Ghibli => self.ghibli.as_ref(),
        }
    }

    fn get_mut(&mut self, style: Style) -> Option<&mut Rendering> {
        match style {
            Style::Basic => self.basic.as_mut(),
            Style::Realistic => self.realistic.as_mut(),
            Style::Ghibli => self.ghibli.as_mut(),
        }
    }
}

fn largest(rings: &[Ring]) -> Option<&[(f64, f64)]> {
    rings
        .iter()
        .max_by(|a, b| geometry::area(a).total_cmp(&geometry::area(b)))
        .map(Vec::as_slice)
}

/// Every scene, by id, as embedded, in the order the picker lists them; the
/// first is the one Ruya opens on. A file in `scenes/` that is not listed here
/// fails `cargo test`.
pub const SOURCES: &[(&str, &str)] = &[
    ("kitchen", include_str!("../scenes/kitchen.json")),
    ("living-room", include_str!("../scenes/living-room.json")),
    ("street", include_str!("../scenes/street.json")),
    ("fruit-stand", include_str!("../scenes/fruit-stand.json")),
    (
        "vegetable-stand",
        include_str!("../scenes/vegetable-stand.json"),
    ),
];

pub static SCENES: LazyLock<Vec<Scene>> = LazyLock::new(|| {
    SOURCES
        .iter()
        .map(|(id, json)| {
            Scene::parse(json)
                .unwrap_or_else(|e| panic!("scenes/{id}.json is checked by cargo test: {e}"))
        })
        .collect()
});

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::{bounds, contains, parse_path};
    use crate::reading::{is_kanji, pieces};
    use std::collections::HashSet;
    use std::path::Path;

    /// The shortest side a region may have, in picture units. On a phone held
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

    /// Every (file and style, rendering) pair, for the checks that run per
    /// picture.
    fn renderings(scenes: &[(String, Scene)]) -> Vec<(String, &Rendering)> {
        scenes
            .iter()
            .flat_map(|(file, scene)| {
                scene
                    .renderings()
                    .map(move |(s, r)| (format!("{file} {}", s.code()), r))
            })
            .collect()
    }

    #[test]
    fn every_file_is_embedded_and_every_embedded_scene_has_a_file() {
        for (file, scene) in scenes() {
            assert_eq!(
                file,
                format!("{}.json", scene.id),
                "a scene's file is named by its id"
            );
        }
        let files: HashSet<String> = scenes().into_iter().map(|(_, s)| s.id).collect();
        let listed: HashSet<String> = SOURCES.iter().map(|(id, _)| id.to_string()).collect();
        assert_eq!(files, listed);
        for (id, json) in SOURCES {
            assert_eq!(&Scene::parse(json).unwrap().id, id);
        }
    }

    #[test]
    fn the_edges_go_round_the_scenes() {
        let first = Scene::first();
        let last = SCENES.last().unwrap();
        assert_eq!(last.next().id, first.id);
        assert_eq!(first.previous().id, last.id);
        for s in SCENES.iter() {
            assert_eq!(s.next().previous().id, s.id);
            assert_eq!(s.step(Step::Next).id, s.next().id);
            assert_eq!(s.step(Step::Previous).id, s.previous().id);
        }
    }

    #[test]
    fn every_scene_has_a_style() {
        for (file, scene) in scenes() {
            assert!(scene.renderings().next().is_some(), "{file}: no style");
        }
    }

    #[test]
    fn regions_name_things_and_every_thing_is_somewhere() {
        for (file, scene) in scenes() {
            let mut used = HashSet::new();
            for (style, rendering) in scene.renderings() {
                for region in &rendering.regions {
                    assert!(
                        scene.things.contains_key(&region.thing),
                        "{file} {}: '{}' is not a thing in the scene",
                        style.code(),
                        region.thing
                    );
                    used.insert(region.thing.as_str());
                }
            }
            for id in scene.things.keys() {
                assert!(
                    id.chars()
                        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
                        && !id.is_empty(),
                    "{file}: id '{id}' is not [a-z0-9-]"
                );
                assert!(
                    used.contains(id.as_str()),
                    "{file}: '{id}' has names but no region in any style"
                );
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
                // hiragana and katakana, the long-vowel mark ー among them,
                // and kanji
                Lang::Ja => (0x3040..=0x30FF).contains(&u) || is_kanji(c),
                // Latin, with the accents Spanish, German and Swedish use
                Lang::En | Lang::De | Lang::Sv | Lang::Es => {
                    c.is_ascii_alphabetic() || (0x00C0..=0x017F).contains(&u)
                }
            }
        })
    }

    fn check_name(file: &str, what: &str, lang: Lang, word: &str) {
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

    #[test]
    fn every_name_is_there_and_in_its_script() {
        for (file, scene) in scenes() {
            for lang in Lang::ALL {
                check_name(&file, "the scene", lang, scene.name.get(lang));
            }
        }
    }

    #[test]
    fn every_kanji_has_its_reading_over_it() {
        let hiragana = |c: char| ('\u{3041}'..='\u{309F}').contains(&c);
        for (file, scene) in scenes() {
            for (id, thing) in &scene.things {
                for lang in Lang::ALL {
                    let name = thing.name.get(lang);
                    let reading = thing.reading.get(lang);
                    let needs = lang.readings_name().is_some() && name.chars().any(is_kanji);
                    assert_eq!(
                        reading.is_some(),
                        needs,
                        "{file}: {id} has readings in {} if and only if its name has kanji",
                        lang.code()
                    );
                    let Some(reading) = reading else {
                        continue;
                    };
                    let pieces = pieces(reading);
                    let bare: String = pieces.iter().map(|&(text, _)| text).collect();
                    assert_eq!(
                        bare, name,
                        "{file}: {id}'s reading {reading} is not its name"
                    );
                    for (text, over) in pieces {
                        match over {
                            Some(kana) => assert!(
                                !text.is_empty() && !kana.is_empty() && kana.chars().all(hiragana),
                                "{file}: {id}: [{kana}] is not in hiragana over a kanji"
                            ),
                            None => assert!(
                                !text.chars().any(is_kanji),
                                "{file}: {id}: {text} has a kanji with no reading"
                            ),
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn every_thing_has_a_name_for_each_number_its_language_has() {
        for (file, scene) in scenes() {
            for (id, thing) in &scene.things {
                for lang in Lang::ALL {
                    for number in [Number::Singular, Number::Dual, Number::Plural] {
                        let word = thing.name_for(lang, number);
                        assert_eq!(
                            word.is_some(),
                            lang.numbers().contains(&number),
                            "{file}: {id} has a {number:?} in {} if and only if the \
                             language has one",
                            lang.code()
                        );
                        if let Some(word) = word {
                            check_name(&file, &format!("{id} ({number:?})"), lang, word);
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn regions_are_inside_the_picture_and_big_enough() {
        let scenes = scenes();
        for (what, r) in renderings(&scenes) {
            let (w, h) = (r.picture.width, r.picture.height);
            for region in &r.regions {
                let rings = parse_path(&region.d).unwrap();
                for ring in &rings {
                    assert!(
                        ring.len() >= 3,
                        "{what}: {} has a ring of {} points",
                        region.thing,
                        ring.len()
                    );
                }
                let (x0, y0, x1, y1) = bounds(&rings);
                assert!(
                    x0 >= 0.0 && y0 >= 0.0 && x1 <= w && y1 <= h,
                    "{what}: {} reaches outside the {w}×{h} picture",
                    region.thing
                );
                let side = (x1 - x0).min(y1 - y0);
                assert!(
                    side >= MIN_SIDE,
                    "{what}: {} is {side} across at its narrowest, under {MIN_SIDE}",
                    region.thing
                );
            }
        }
    }

    #[test]
    fn every_label_point_is_on_its_region() {
        let scenes = scenes();
        for (what, r) in renderings(&scenes) {
            for region in &r.regions {
                let rings = parse_path(&region.d).unwrap();
                assert!(
                    contains(&rings, region.anchor),
                    "{what}: {}'s label point {:?} is off it — give it an \"at\"",
                    region.thing,
                    region.anchor
                );
            }
        }
    }

    #[test]
    fn every_name_shown_once_when_all_are() {
        let scenes = scenes();
        for (what, r) in renderings(&scenes) {
            let chosen = r.one_per_thing();
            let things: HashSet<&str> = r.regions.iter().map(|g| g.thing.as_str()).collect();
            let named: HashSet<&str> = chosen
                .iter()
                .map(|&i| r.regions[i].thing.as_str())
                .collect();
            assert_eq!(chosen.len(), named.len(), "{what}: a thing named twice");
            assert_eq!(named, things, "{what}: a thing left without its name");
            for &i in &chosen {
                let it = &r.regions[i];
                assert!(
                    r.regions
                        .iter()
                        .all(|g| g.thing != it.thing || g.area <= it.area),
                    "{what}: {}'s name is not on its biggest region",
                    it.thing
                );
            }
        }
    }

    #[test]
    fn a_thing_lying_on_another_comes_after_it() {
        // the plate on the table: drawn after it, so a tap on the plate is the
        // plate's, not the table's
        let scenes = scenes();
        for (what, r) in renderings(&scenes) {
            let rings: Vec<_> = r
                .regions
                .iter()
                .map(|g| parse_path(&g.d).unwrap())
                .collect();
            for (a, outer) in rings.iter().enumerate() {
                for (b, inner) in rings.iter().enumerate() {
                    if a == b {
                        continue;
                    }
                    let within = inner.iter().flatten().all(|&p| contains(outer, p));
                    assert!(
                        !within || b > a,
                        "{what}: {} lies on {} but comes before it",
                        r.regions[b].thing,
                        r.regions[a].thing
                    );
                }
            }
        }
    }

    /// The file is there and has the scene's shape: an SVG with the scene's
    /// viewBox, or a bitmap of the same proportions and at least its size —
    /// more pixels for big screens is fine, a different shape would move every
    /// region off its thing.
    fn check_file(what: &str, src: &str, w: f64, h: f64, at_least_full_size: bool) {
        assert!(!src.starts_with('/'), "{what}: {src} must be relative");
        let path = root().join("public").join(src);
        assert!(path.exists(), "{what}: no file at {}", path.display());
        if src.ends_with(".svg") {
            let svg = std::fs::read_to_string(&path).unwrap();
            let view_box = format!("viewBox=\"0 0 {w} {h}\"");
            assert!(svg.contains(&view_box), "{what}: {src} has no {view_box}");
        } else {
            let size = imagesize::size(&path).unwrap();
            let (pw, ph) = (size.width as f64, size.height as f64);
            assert!(
                (pw / ph - w / h).abs() < 0.002,
                "{what}: {src} is {pw}×{ph}, not the shape of {w}×{h}"
            );
            assert!(
                !at_least_full_size || pw >= w,
                "{what}: {src} is smaller than {w}×{h}"
            );
        }
    }

    #[test]
    fn pictures_and_thumbnails_exist_and_match_the_scene() {
        let scenes = scenes();
        for (what, r) in renderings(&scenes) {
            let p = &r.picture;
            check_file(&what, &p.src, p.width, p.height, true);
            check_file(&what, &p.thumb, p.width, p.height, false);
        }
    }

    #[test]
    fn every_sign_has_its_words_and_its_patches() {
        for (file, scene) in scenes() {
            for (id, sign) in &scene.signs {
                for lang in Lang::ALL {
                    check_name(&file, &format!("sign {id}"), lang, sign.text.get(lang));
                }
                assert!(
                    scene
                        .renderings()
                        .any(|(_, r)| r.signs.iter().any(|spot| &spot.sign == id)),
                    "{file}: sign {id} is in no picture"
                );
            }
            for (style, r) in scene.renderings() {
                let what = format!("{file} {}", style.code());
                for spot in &r.signs {
                    let name = &spot.sign;
                    assert!(
                        scene.signs.contains_key(name),
                        "{what}: {name} is not a sign of the scene"
                    );
                    assert!(
                        Lang::from_tag(&spot.lang).is_some_and(|l| l.code() == spot.lang),
                        "{what}: {name}'s {} is not a language Ruya has",
                        spot.lang
                    );
                    let [x0, y0, x1, y1] = spot.rect;
                    assert!(
                        0.0 <= x0 && x0 < x1 && x1 <= r.picture.width,
                        "{what}: {name} is not across the picture"
                    );
                    assert!(
                        0.0 <= y0 && y0 < y1 && y1 <= r.picture.height,
                        "{what}: {name} is not down the picture"
                    );
                    for lang in Lang::ALL {
                        if let Some(src) = spot.patch(&scene.id, style, lang) {
                            let what = format!("{what} {name} {}", lang.code());
                            check_file(&what, &src, x1 - x0, y1 - y0, true);
                        }
                    }
                }
            }
        }
    }
}
