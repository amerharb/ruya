//! The choices in the address: `?p=living-room` opens Ruya on the living
//! room, `?l=sv` in Swedish, `?s=basic` on the drawing, `?n=all` with every
//! name showing, `?f=one-many` with each name's plural under it, `?r=on`
//! with furigana over Japanese kanji and `?v=on` with the names said aloud,
//! and picking any of them writes it back, so a link shows what the person
//! sharing it saw. `p` is for place, as `s` was taken.

use crate::lang::{Forms, Lang};
use crate::scene::Scene;
use crate::style::Style;
use wasm_bindgen::JsValue;

/// A parameter's value from a query string.
fn param<'a>(search: &'a str, key: &str) -> Option<&'a str> {
    search
        .trim_start_matches('?')
        .split('&')
        .find_map(|pair| pair.strip_prefix(key)?.strip_prefix('='))
}

/// `?l=`, or None when it is absent or not a language Ruya has — an unknown
/// value is ignored rather than leaving a blank page.
pub fn lang_from_search(search: &str) -> Option<Lang> {
    param(search, "l").and_then(Lang::from_tag)
}

/// `?s=`, or None when it is absent or not a style Ruya has.
pub fn style_from_search(search: &str) -> Option<Style> {
    param(search, "s").and_then(Style::from_code)
}

/// `?p=`, or None when it is absent or not a scene Ruya has.
pub fn scene_from_search(search: &str) -> Option<&'static Scene> {
    param(search, "p").and_then(Scene::find)
}

/// `?n=all`: every name showing. Anything else, or nothing, is one at a time.
pub fn all_names_from_search(search: &str) -> bool {
    param(search, "n") == Some("all")
}

/// `?f=`, the forms each name shows, or None when it is absent or names
/// none.
pub fn forms_from_search(search: &str) -> Option<Forms> {
    param(search, "f").and_then(Forms::from_code)
}

/// `?r=on`: readings over the names, in a language that has them. Anything
/// else, or nothing, is none.
pub fn readings_from_search(search: &str) -> bool {
    param(search, "r") == Some("on")
}

/// `?v=on`: a thing tapped says its name, in a language Ruya has a voice
/// for. Anything else, or nothing, is silent.
pub fn voice_from_search(search: &str) -> bool {
    param(search, "v") == Some("on")
}

fn search() -> String {
    web_sys::window()
        .and_then(|w| w.location().search().ok())
        .unwrap_or_default()
}

/// The language to open in: the address, then the browser's own list, then
/// English.
pub fn initial_lang() -> Lang {
    if let Some(lang) = lang_from_search(&search()) {
        return lang;
    }
    web_sys::window()
        .expect("a browser window")
        .navigator()
        .languages()
        .iter()
        .filter_map(|tag| tag.as_string())
        .find_map(|tag| Lang::from_tag(&tag))
        .unwrap_or(Lang::En)
}

/// The style to open in: the address, else the default.
pub fn initial_style() -> Style {
    style_from_search(&search()).unwrap_or(Style::DEFAULT)
}

/// The scene to open on: the address, else the first.
pub fn initial_scene() -> &'static Scene {
    scene_from_search(&search()).unwrap_or_else(Scene::first)
}

/// Whether to open with every name showing.
pub fn initial_all_names() -> bool {
    all_names_from_search(&search())
}

/// The forms to open with: the address, else the singular alone.
pub fn initial_forms() -> Forms {
    forms_from_search(&search()).unwrap_or(Forms::SINGULAR)
}

/// Whether to open with readings over the names.
pub fn initial_readings() -> bool {
    readings_from_search(&search())
}

/// Whether to open with the names said aloud.
pub fn initial_voice() -> bool {
    voice_from_search(&search())
}

/// Write a parameter into the address without a new history entry: a
/// language, style or scene picked is a choice, not a page to go back to.
pub fn write(key: &str, value: &str) {
    edit(|params| params.set(key, value));
}

/// Take a parameter out of the address the same way, for a choice that is
/// back at what Ruya opens with anyway.
pub fn remove(key: &str) {
    edit(|params| params.delete(key));
}

fn edit(change: impl FnOnce(&web_sys::UrlSearchParams)) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let location = window.location();
    let search = location.search().unwrap_or_default();
    let Ok(params) = web_sys::UrlSearchParams::new_with_str(&search) else {
        return;
    };
    change(&params);
    let query = String::from(params.to_string());
    let url = format!(
        "{}{}{query}{}",
        location.pathname().unwrap_or_default(),
        if query.is_empty() { "" } else { "?" },
        location.hash().unwrap_or_default(),
    );
    if let Ok(history) = window.history() {
        let _ = history.replace_state_with_url(&JsValue::NULL, "", Some(&url));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_l_among_other_parameters() {
        assert_eq!(lang_from_search("?l=sv"), Some(Lang::Sv));
        assert_eq!(lang_from_search("?x=1&l=de"), Some(Lang::De));
        assert_eq!(lang_from_search("l=ar"), Some(Lang::Ar));
    }

    #[test]
    fn ignores_what_it_does_not_know() {
        assert_eq!(lang_from_search("?l=zz"), None);
        assert_eq!(lang_from_search("?l="), None);
        assert_eq!(lang_from_search(""), None);
        assert_eq!(lang_from_search("?lang=sv"), None);
    }

    #[test]
    fn reads_s_for_the_style() {
        assert_eq!(style_from_search("?l=sv&s=basic"), Some(Style::Basic));
        assert_eq!(style_from_search("?s=realistic"), Some(Style::Realistic));
        assert_eq!(style_from_search("?s=ghibli"), Some(Style::Ghibli));
        assert_eq!(style_from_search("?s=photo"), None);
        assert_eq!(style_from_search("?l=sv"), None);
    }

    #[test]
    fn reads_n_for_every_name() {
        assert!(all_names_from_search("?n=all"));
        assert!(all_names_from_search("?l=sv&n=all&s=basic"));
        assert!(!all_names_from_search("?n=one"));
        assert!(!all_names_from_search("?n="));
        assert!(!all_names_from_search("?l=sv"));
    }

    #[test]
    fn reads_f_for_the_forms() {
        let code = |search| forms_from_search(search).map(Forms::code);
        assert_eq!(code("?f=one-many").as_deref(), Some("one-many"));
        assert_eq!(code("?l=ar&f=two&n=all").as_deref(), Some("two"));
        assert_eq!(code("?f=lots"), None);
        assert_eq!(code("?f="), None);
        assert_eq!(code("?l=sv"), None);
    }

    #[test]
    fn reads_r_for_the_readings() {
        assert!(readings_from_search("?r=on"));
        assert!(readings_from_search("?l=ja&r=on&n=all"));
        assert!(!readings_from_search("?r=off"));
        assert!(!readings_from_search("?r="));
        assert!(!readings_from_search("?l=ja"));
    }

    #[test]
    fn reads_v_for_the_voice() {
        assert!(voice_from_search("?v=on"));
        assert!(voice_from_search("?p=street&v=on"));
        assert!(!voice_from_search("?v=off"));
        assert!(!voice_from_search("?v="));
        assert!(!voice_from_search("?l=en"));
    }

    #[test]
    fn reads_p_for_the_scene() {
        let id = |search| scene_from_search(search).map(|s| s.id.as_str());
        assert_eq!(id("?p=living-room&l=sv"), Some("living-room"));
        assert_eq!(id("?l=sv&p=kitchen"), Some("kitchen"));
        assert_eq!(id("?p=garage"), None);
        assert_eq!(id("?s=basic"), None);
    }
}
