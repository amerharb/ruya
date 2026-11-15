//! The recordings kept: each one said is kept in the browser, in IndexedDB,
//! so a name said again — later in the visit, or on another day — plays
//! from there, with no request to the server. The sawt apps keep their
//! sounds the same way (`packages/audio-cache` there), for the same
//! reasons: IndexedDB works in Safari's Lockdown Mode, keeps a Blob as it
//! is, and has room.
//!
//! Nothing kept is checked against the server again, and nothing expires.
//! A recording is kept by its address, its voice and the name it says (see
//! `voice::Clip`), so a thing renamed, or a language given a new voice, is
//! simply not kept yet, and fetched. A recording made again under the same
//! name in the same voice — a take redone, a word given something else to
//! say in `SAY` — is not seen that way: raise `VERSION`, which drops
//! everything kept, so each recording is fetched afresh the next time it
//! is said. `tools/voice.py` says when.
//!
//! All of it fails soft: with no IndexedDB, or one that will not open, a
//! recording comes from the server every time, as it did before.

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen::closure::Closure;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Blob, IdbDatabase, IdbRequest, IdbTransactionMode, RequestCache, RequestInit};

const NAME: &str = "ruya-voice";
const STORE: &str = "clips";

/// Raised when a recording is made again under the name and the voice it
/// already had. Opening at a higher version drops everything kept, as
/// there is no telling which recordings changed; the cost is one fetch of
/// each again, the next time it is said.
const VERSION: u32 = 1;

/// How long opening may take before IndexedDB is taken to be not
/// answering. Opening is all but instant, so this is no allowance for a
/// slow device: Safari, on an upgrade while another tab holds the old
/// version, can leave the request with no answer at all, and every name
/// would wait on it.
const OPEN_TIMEOUT_MS: i32 = 3000;

thread_local! {
    /// The database, opening or open, as a promise every caller awaits.
    static DB: RefCell<Option<js_sys::Promise>> = const { RefCell::new(None) };
}

/// The recording at `src`, kept as `key`: what is kept if it is there,
/// else from the server, then kept. None when neither has it — offline and
/// never said, or not there at all.
pub async fn clip(src: &str, key: &str) -> Option<Blob> {
    if let Some(blob) = kept(key).await {
        return Some(blob);
    }
    let blob = fetched(src).await?;
    // kept in the background: the name is said without waiting for it
    let (key, kept) = (key.to_owned(), blob.clone());
    wasm_bindgen_futures::spawn_local(async move {
        let _ = keep(&key, &kept).await;
    });
    Some(blob)
}

async fn kept(key: &str) -> Option<Blob> {
    let store = database()
        .await?
        .transaction_with_str(STORE)
        .ok()?
        .object_store(STORE)
        .ok()?;
    let blob = done(store.get(&key.into()).ok()?).await.ok()?;
    blob.dyn_into::<Blob>().ok()
}

async fn keep(key: &str, blob: &Blob) -> Option<()> {
    let store = database()
        .await?
        .transaction_with_str_and_mode(STORE, IdbTransactionMode::Readwrite)
        .ok()?
        .object_store(STORE)
        .ok()?;
    done(store.put_with_key(blob, &key.into()).ok()?)
        .await
        .ok()?;
    Some(())
}

/// The recording from the server, if it sends one.
async fn fetched(src: &str) -> Option<Blob> {
    let init = RequestInit::new();
    // asked of the server, not taken from the browser's own cache: what is
    // kept is kept for good, so it must not be a copy from before a release
    init.set_cache(RequestCache::NoCache);
    let sent = web_sys::window()?.fetch_with_str_and_init(src, &init);
    let response: web_sys::Response = JsFuture::from(sent).await.ok()?.dyn_into().ok()?;
    // A 200 is no proof of a recording: a server can answer a missing file
    // with a page, as `trunk serve` and a captive portal do, and a page
    // kept as a recording would never play. One that says it is HTML is
    // not kept; one that says nothing may still play.
    let html = response
        .headers()
        .get("content-type")
        .ok()
        .flatten()
        .is_some_and(|t| t.contains("text/html"));
    if !response.ok() || html {
        return None;
    }
    let blob: Blob = JsFuture::from(response.blob().ok()?)
        .await
        .ok()?
        .dyn_into()
        .ok()?;
    (blob.size() > 0.0).then_some(blob)
}

/// The open database, or None when there is none to be had.
async fn database() -> Option<IdbDatabase> {
    let opening = DB.with_borrow_mut(|db| db.get_or_insert_with(open).clone());
    JsFuture::from(opening).await.ok()?.dyn_into().ok()
}

/// Open the database at `VERSION`. The first of its answer and the deadline
/// settles it. One that fails is asked again on the next name said, unless
/// it ran out of time, which would only happen again.
fn open() -> js_sys::Promise {
    js_sys::Promise::new(&mut |resolve, reject| {
        let request = web_sys::window()
            .and_then(|window| window.indexed_db().ok().flatten())
            .and_then(|idb| idb.open_with_u32(NAME, VERSION).ok());
        let Some(request) = request else {
            // no IndexedDB here, private browsing in some browsers: none later
            let _ = reject.call1(&JsValue::NULL, &"no IndexedDB".into());
            return;
        };
        let settled = Rc::new(Cell::new(false));

        // a new version: nothing kept under the old one can be trusted, so
        // the store is made anew
        let upgrade = Closure::<dyn FnMut()>::new({
            let request = request.clone();
            move || {
                let Ok(db) = request.result().and_then(|db| db.dyn_into::<IdbDatabase>()) else {
                    return;
                };
                if db.object_store_names().contains(STORE) {
                    let _ = db.delete_object_store(STORE);
                }
                let _ = db.create_object_store(STORE);
            }
        });
        request.set_onupgradeneeded(Some(upgrade.as_ref().unchecked_ref()));

        let success = Closure::<dyn FnMut()>::new({
            let (request, settled) = (request.clone(), settled.clone());
            move || {
                let Ok(db) = request.result().and_then(|db| db.dyn_into::<IdbDatabase>()) else {
                    return;
                };
                if settled.replace(true) {
                    // too late: given up on already, so let it go
                    db.close();
                    return;
                }
                // Another tab opening a newer version: let go, so its upgrade
                // can run, and open again on the next name said.
                let close = Closure::<dyn FnMut()>::new({
                    let db = db.clone();
                    move || {
                        db.close();
                        DB.set(None);
                    }
                });
                db.set_onversionchange(Some(close.as_ref().unchecked_ref()));
                close.forget();
                let _ = resolve.call1(&JsValue::NULL, &db);
            }
        });
        request.set_onsuccess(Some(success.as_ref().unchecked_ref()));

        // An error, or an older tab holding the old version so the upgrade
        // cannot run: the server it is for now, and another try next time,
        // when that tab may be gone.
        let failed = Closure::<dyn FnMut()>::new({
            let (reject, settled) = (reject.clone(), settled.clone());
            move || {
                if !settled.replace(true) {
                    DB.set(None);
                    let _ = reject.call1(&JsValue::NULL, &"IndexedDB did not open".into());
                }
            }
        });
        request.set_onerror(Some(failed.as_ref().unchecked_ref()));
        request.set_onblocked(Some(failed.as_ref().unchecked_ref()));

        let deadline = Closure::once_into_js(move || {
            if !settled.replace(true) {
                let _ = reject.call1(&JsValue::NULL, &"IndexedDB did not answer".into());
            }
        });
        if let Some(window) = web_sys::window() {
            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                deadline.unchecked_ref(),
                OPEN_TIMEOUT_MS,
            );
        }

        // kept for as long as the request may answer: a database is opened
        // once a page, or again after another tab's upgrade, so they are few
        upgrade.forget();
        success.forget();
        failed.forget();
    })
}

/// An IndexedDB request's result, once it has one.
async fn done(request: IdbRequest) -> Result<JsValue, JsValue> {
    let answered = js_sys::Promise::new(&mut |resolve, reject| {
        // one closure for both events, so whichever comes frees it
        let on = Closure::once_into_js({
            let request = request.clone();
            move |ev: web_sys::Event| {
                let _ = if ev.type_() == "success" {
                    resolve.call1(&JsValue::NULL, &request.result().unwrap_or_default())
                } else {
                    reject.call1(&JsValue::NULL, &"IndexedDB request failed".into())
                };
            }
        });
        request.set_onsuccess(Some(on.unchecked_ref()));
        request.set_onerror(Some(on.unchecked_ref()));
    });
    JsFuture::from(answered).await
}
