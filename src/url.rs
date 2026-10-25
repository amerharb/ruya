//! The language in the address: `?l=sv` opens Ruya in Swedish, and picking a
//! language writes it back, so a link shows what the person sharing it saw.

use crate::lang::Lang;
use wasm_bindgen::JsValue;

/// `?l=` from a query string, or None when it is absent or not a language
/// Ruya has — an unknown value is ignored rather than leaving a blank page.
pub fn lang_from_search(search: &str) -> Option<Lang> {
    search
        .trim_start_matches('?')
        .split('&')
        .find_map(|pair| pair.strip_prefix("l="))
        .and_then(Lang::from_tag)
}

/// The language to open in: the address, then the browser's own list, then
/// English.
pub fn initial_lang() -> Lang {
    let window = web_sys::window().expect("a browser window");
    if let Some(lang) = window
        .location()
        .search()
        .ok()
        .and_then(|s| lang_from_search(&s))
    {
        return lang;
    }
    window
        .navigator()
        .languages()
        .iter()
        .filter_map(|tag| tag.as_string())
        .find_map(|tag| Lang::from_tag(&tag))
        .unwrap_or(Lang::En)
}

/// Write `?l=` into the address without a new history entry: changing the
/// language is not a place to go back to.
pub fn write_lang(lang: Lang) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let location = window.location();
    let search = location.search().unwrap_or_default();
    let Ok(params) = web_sys::UrlSearchParams::new_with_str(&search) else {
        return;
    };
    params.set("l", lang.code());
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
}
