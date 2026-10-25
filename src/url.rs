//! The choices in the address: `?l=sv` opens Ruya in Swedish and `?s=basic`
//! on the drawing, and picking either writes it back, so a link shows what the
//! person sharing it saw.

use crate::lang::Lang;
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

/// Write a parameter into the address without a new history entry: changing
/// the language or the style is not a place to go back to.
pub fn write(key: &str, value: &str) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let location = window.location();
    let search = location.search().unwrap_or_default();
    let Ok(params) = web_sys::UrlSearchParams::new_with_str(&search) else {
        return;
    };
    params.set(key, value);
    let url = format!(
        "{}?{}{}",
        location.pathname().unwrap_or_default(),
        String::from(params.to_string()),
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
        assert_eq!(style_from_search("?s=photo"), None);
        assert_eq!(style_from_search("?l=sv"), None);
    }
}
