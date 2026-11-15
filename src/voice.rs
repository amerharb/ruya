//! The voice: with the speaker on, a thing tapped says its name aloud, in
//! each form shown, one after another — in a language Ruya has recordings
//! for. The recordings are MP3s beside the scene's pictures, made by
//! `tools/voice.py`, each fetched the first time it is said and kept in
//! the browser after, so it is said again without asking the server (see
//! `cache`).
//!
//! One `<audio>` for the page. Safari lets an element sound only from
//! inside a tap, and one that has sounded once may go on sounding; a
//! recording read from what is kept arrives after the tap is over. So the
//! first tap plays a moment of silence through it, and every recording
//! after plays through the same element, however long it took to come: a
//! thing tapped while another is being said cuts it short, and the
//! singular goes on to the plural.

use crate::cache;
use crate::lang::Lang;
use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use wasm_bindgen_futures::JsFuture;

/// A WAV with no sound in it: what the first tap plays, as sawt's apps do.
const SILENCE: &str =
    "data:audio/wav;base64,UklGRiQAAABXQVZFZm10IBAAAAABAAEARKwAAIhYAQACABAAZGF0YQAAAAA=";

thread_local! {
    static PLAYER: RefCell<Option<web_sys::HtmlAudioElement>> = const { RefCell::new(None) };
    /// What is still to be said after the recording playing.
    static QUEUE: RefCell<VecDeque<Clip>> = const { RefCell::new(VecDeque::new()) };
    /// Which tap's names are being said: a recording that comes after
    /// another tap is not said.
    static TURN: Cell<u32> = const { Cell::new(0) };
    /// The address of the recording playing, made for its Blob, let go when
    /// the next one plays.
    static PLAYING: RefCell<Option<String>> = const { RefCell::new(None) };
}

/// A recording to say: where it is, and what it is kept under in the
/// browser — its address, its voice and the name it says, so a thing
/// renamed or a new voice is fetched, not taken from what was kept.
pub struct Clip {
    src: String,
    key: String,
}

impl Clip {
    pub fn new(src: String, lang: Lang, name: &str) -> Clip {
        let key = format!("{src} {} {name}", lang.voice().unwrap_or_default());
        Clip { src, key }
    }
}

/// Say these recordings in turn, cutting short whatever is being said.
/// Called from inside the tap, which the first time lets the player sound.
pub fn say(clips: Vec<Clip>) {
    TURN.set(TURN.get().wrapping_add(1));
    QUEUE.with_borrow_mut(|queue| *queue = clips.into());
    if let Some(audio) = player() {
        if audio.src().is_empty() {
            audio.set_src(SILENCE);
            play(&audio);
        } else {
            let _ = audio.pause();
        }
    }
    next();
}

/// The page's player, made on the first name said.
fn player() -> Option<web_sys::HtmlAudioElement> {
    PLAYER.with_borrow_mut(|player| {
        if player.is_none() {
            let audio = web_sys::HtmlAudioElement::new().ok()?;
            // on to the next form when one ends, or when one will not play
            let on = Closure::<dyn FnMut()>::new(ended);
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

/// A recording ended, or would not play: on to the next. The silence that
/// let the player sound ends too, and is no recording.
fn ended() {
    let on = PLAYER.with_borrow(|player| player.as_ref().map(|audio| audio.src()));
    if on.is_some() && on == PLAYING.with_borrow(Clone::clone) {
        next();
    }
}

fn next() {
    let Some(clip) = QUEUE.with_borrow_mut(VecDeque::pop_front) else {
        return;
    };
    let turn = TURN.get();
    wasm_bindgen_futures::spawn_local(async move {
        let blob = cache::clip(&clip.src, &clip.key).await;
        if TURN.get() != turn {
            // tapped again since: that tap's names are said instead
            return;
        }
        let src = blob.and_then(|blob| web_sys::Url::create_object_url_with_blob(&blob).ok());
        let (Some(src), Some(audio)) = (src, player()) else {
            // not to be had, offline and never said: on to the next form
            next();
            return;
        };
        if let Some(old) = PLAYING.replace(Some(src.clone())) {
            let _ = web_sys::Url::revoke_object_url(&old);
        }
        audio.set_src(&src);
        play(&audio);
    });
}

fn play(audio: &web_sys::HtmlAudioElement) {
    if let Ok(playing) = audio.play() {
        // awaited only so that a refusal — sound not allowed, or cut short
        // by the next name — is swallowed here rather than reported as an
        // unhandled rejection
        wasm_bindgen_futures::spawn_local(async move {
            let _ = JsFuture::from(playing).await;
        });
    }
}
