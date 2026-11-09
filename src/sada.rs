//! sada (صدى, the echo): the collector the sawt apps and the other sites
//! report to, at sada.sawt.info. Ruya tells it one anonymous thing, that it
//! was opened, once per load: `{ "site": "ruya.info", "kind": "open" }` is
//! the whole payload. No cookies, no identifiers; sada adds only the country
//! the request came from.
//!
//! It speaks only when served from ruya.info, so a local copy or a preview
//! stays silent — unless a `sada-url` entry in localStorage points it at a
//! collector, the switch for development, as on amerharb.com. Sending is
//! fire and forget: counting must never break the page.

use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

const SITE: &str = "ruya.info";
const COLLECTOR: &str = "https://sada.sawt.info";

/// Where to send, if anywhere: the localStorage override, else sada itself
/// when the page is served from ruya.info, else nowhere.
fn collector(hostname: &str, override_url: Option<&str>) -> Option<String> {
    let override_url = override_url
        .map(|u| u.trim().trim_end_matches('/'))
        .filter(|u| !u.is_empty());
    match override_url {
        Some(url) => Some(url.to_string()),
        None => {
            let home = hostname == SITE || hostname == format!("www.{SITE}");
            home.then(|| COLLECTOR.to_string())
        }
    }
}

/// The open event, as sada's events endpoint takes it.
fn open_event() -> String {
    format!(r#"{{"site":"{SITE}","kind":"open"}}"#)
}

/// Ruya was opened: once per load, from `main`.
pub fn post_open() {
    post("/v1/events", &open_event());
}

fn post(path: &str, body: &str) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let hostname = window.location().hostname().unwrap_or_default();
    // storage can be refused in a locked-down browser: no override then
    let override_url = window
        .local_storage()
        .ok()
        .flatten()
        .and_then(|storage| storage.get_item("sada-url").ok().flatten());
    let Some(base) = collector(&hostname, override_url.as_deref()) else {
        return;
    };
    let Ok(headers) = web_sys::Headers::new() else {
        return;
    };
    let _ = headers.set("content-type", "application/json");
    let init = web_sys::RequestInit::new();
    init.set_method("POST");
    init.set_headers(&headers);
    init.set_body(&JsValue::from_str(body));
    // carried through even if the page is left while it is on its way; set
    // by hand, as web-sys's RequestInit has no keepalive
    let _ = js_sys::Reflect::set(&init, &JsValue::from_str("keepalive"), &JsValue::TRUE);
    let sent = window.fetch_with_str_and_init(&format!("{base}{path}"), &init);
    // awaited only so that a refusal or a network error is swallowed here
    // rather than reported as an unhandled rejection
    wasm_bindgen_futures::spawn_local(async move {
        let _ = JsFuture::from(sent).await;
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn speaks_only_from_ruya_info() {
        let sada = Some("https://sada.sawt.info".to_string());
        assert_eq!(collector("ruya.info", None), sada);
        assert_eq!(collector("www.ruya.info", None), sada);
        assert_eq!(collector("127.0.0.1", None), None);
        assert_eq!(collector("localhost", None), None);
        assert_eq!(collector("amerharb.github.io", None), None);
        assert_eq!(collector("notruya.info", None), None);
    }

    #[test]
    fn the_override_points_anywhere_and_blank_is_none() {
        assert_eq!(
            collector("127.0.0.1", Some(" http://localhost:8080/ ")),
            Some("http://localhost:8080".to_string())
        );
        assert_eq!(
            collector("ruya.info", Some("http://localhost:8080")),
            Some("http://localhost:8080".to_string())
        );
        assert_eq!(collector("127.0.0.1", Some("  ")), None);
    }

    #[test]
    fn the_open_event_is_site_and_kind_alone() {
        let event: serde_json::Value = serde_json::from_str(&open_event()).unwrap();
        assert_eq!(
            event,
            serde_json::json!({ "site": "ruya.info", "kind": "open" })
        );
    }
}
