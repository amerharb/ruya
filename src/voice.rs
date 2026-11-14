//! The voice: with the speaker on, a thing tapped says its name aloud, in
//! each form shown, one after another — in a language Ruya has recordings
//! for. The recordings are MP3s beside the scene's pictures, made by
//! `tools/voice.py`, each fetched the first time it is said.
//!
//! One `<audio>` for the page: a thing tapped while another is being said
//! cuts it short, and a phone that lets a page sound only from a tap lets
//! the same element go on from the singular to the plural.

use std::cell::RefCell;
use std::collections::VecDeque;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use wasm_bindgen_futures::JsFuture;

thread_local! {
    static PLAYER: RefCell<Option<web_sys::HtmlAudioElement>> = const { RefCell::new(None) };
    /// What is still to be said after the recording playing.
    static QUEUE: RefCell<VecDeque<String>> = const { RefCell::new(VecDeque::new()) };
}

/// Say these recordings in turn, cutting short whatever is being said.
pub fn say(clips: Vec<String>) {
    QUEUE.with_borrow_mut(|queue| *queue = clips.into());
    next();
}

/// The page's player, made on the first name said.
fn player() -> Option<web_sys::HtmlAudioElement> {
    PLAYER.with_borrow_mut(|player| {
        if player.is_none() {
            let audio = web_sys::HtmlAudioElement::new().ok()?;
            // on to the next form when one ends, or when one will not play
            let on = Closure::<dyn FnMut()>::new(next);
            for event in ["ended", "error"] {
                let _ = audio.add_event_listener_with_callback(event, on.as_ref().unchecked_ref());
            }
            // kept for the page's life, as the player is
            on.forget();
            *player = Some(audio);
        }
        player.clone()
    })
}

fn next() {
    let Some(src) = QUEUE.with_borrow_mut(VecDeque::pop_front) else {
        return;
    };
    let Some(audio) = player() else {
        return;
    };
    audio.set_src(&src);
    if let Ok(playing) = audio.play() {
        // awaited only so that a refusal — sound not allowed yet, or cut
        // short by the next name — is swallowed here rather than reported
        // as an unhandled rejection
        wasm_bindgen_futures::spawn_local(async move {
            let _ = JsFuture::from(playing).await;
        });
    }
}
