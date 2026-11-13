//! The choices in the bar: the scene, as its name; the style, as a little
//! picture of each; every name or one at a time; the forms each name shows;
//! readings over the names, in Japanese; the names said aloud, in a language
//! with a voice; and the language. The scene and the language are the
//! browser's own `<select>`, which every phone turns into its best native
//! chooser.

use crate::lang::{Forms, Lang, Number};
use crate::scene::{SCENES, Scene};
use crate::style::Style;
use leptos::prelude::*;

/// The scene's name, as the page's heading. With more than one scene the name
/// is a list of them, each named in the chosen language.
#[component]
pub fn ScenePicker(
    scene: ReadSignal<&'static Scene>,
    set_scene: WriteSignal<&'static Scene>,
    lang: ReadSignal<Lang>,
) -> impl IntoView {
    let picker = (SCENES.len() > 1).then(|| {
        view! {
            <select
                class="scene-picker"
                aria-label=move || Scene::label(lang.get())
                on:change=move |ev| {
                    if let Some(s) = Scene::find(&event_target_value(&ev)) {
                        set_scene.set(s);
                    }
                }
            >
                // as in the language picker, each option says whether it is
                // the one chosen, and its direction is its own
                {SCENES
                    .iter()
                    .map(|s| {
                        view! {
                            <option
                                value=s.id.clone()
                                dir=move || lang.get().dir()
                                selected=move || scene.get().id == s.id
                            >
                                {move || s.name.get(lang.get()).to_string()}
                            </option>
                        }
                    })
                    .collect_view()}
            </select>
        }
    });
    let alone = picker.is_none();
    // no `dir` on the heading: in Arabic or Hebrew it would carry the name,
    // and the list with it, to the bar's other end
    view! {
        <h1 class="scene-name" lang=move || lang.get().code()>
            {picker}
            {move || alone.then(|| scene.get().name.get(lang.get()).to_string())}
        </h1>
    }
}

/// A thumbnail per style the scene has; none at all when it has only one.
#[component]
pub fn StylePicker(
    scene: &'static Scene,
    style: ReadSignal<Style>,
    set_style: WriteSignal<Style>,
    lang: ReadSignal<Lang>,
) -> impl IntoView {
    // the style actually shown, which is not the one asked for when the scene
    // does not have it
    let shown = move || scene.rendering_or_first(style.get()).0;
    let options: Vec<_> = scene.renderings().collect();
    (options.len() > 1).then(|| {
        view! {
            <div class="styles" role="group" aria-label=move || Style::label(lang.get())>
                {options
                    .into_iter()
                    .map(|(s, rendering)| {
                        view! {
                            <button
                                class="style"
                                class:on=move || shown() == s
                                aria-pressed=move || (shown() == s).to_string()
                                aria-label=move || s.name(lang.get())
                                title=move || s.name(lang.get())
                                on:click=move |_| set_style.set(s)
                            >
                                <img
                                    src=rendering.picture.thumb.clone()
                                    alt=""
                                    width="48"
                                    height="32"
                                    draggable="false"
                                />
                            </button>
                        }
                    })
                    .collect_view()}
            </div>
        }
    })
}

/// Every name on the picture at once, or one at a time as you point.
#[component]
pub fn AllNames(
    all: ReadSignal<bool>,
    set_all: WriteSignal<bool>,
    lang: ReadSignal<Lang>,
) -> impl IntoView {
    let words = move || all_names(lang.get());
    view! {
        <button
            class="all-names"
            class:on=move || all.get()
            aria-pressed=move || all.get().to_string()
            aria-label=words
            title=words
            on:click=move |_| set_all.update(|on| *on = !*on)
        >
            // two name tags, as the picture shows them
            <svg viewBox="0 0 24 24" aria-hidden="true">
                <rect x="2" y="4" width="13" height="7" rx="2" />
                <rect x="9" y="14" width="13" height="7" rx="2" />
            </svg>
        </button>
    }
}

/// Readings over the names, or none: furigana over Japanese kanji. A button
/// only in a language that has readings.
#[component]
pub fn ReadingsButton(
    readings: ReadSignal<bool>,
    set_readings: WriteSignal<bool>,
    lang: ReadSignal<Lang>,
) -> impl IntoView {
    move || {
        lang.get().readings_name().map(|word| {
            view! {
                <button
                    class="readings"
                    class:on=move || readings.get()
                    aria-pressed=move || readings.get().to_string()
                    aria-label=word
                    title=word
                    on:click=move |_| set_readings.update(|on| *on = !*on)
                >
                    // a kanji with its reading over it, as the names get
                    <ruby lang="ja" aria-hidden="true">
                        "字"
                        <rt>"じ"</rt>
                    </ruby>
                </button>
            }
        })
    }
}

/// The names said aloud, or not: pressed, the speaker has a thing tapped
/// say its name. A button only in a language Ruya has a voice for; off
/// until pressed.
#[component]
pub fn VoiceButton(
    voice: ReadSignal<bool>,
    set_voice: WriteSignal<bool>,
    lang: ReadSignal<Lang>,
) -> impl IntoView {
    move || {
        lang.get().voice_name().map(|word| {
            view! {
                <button
                    class="voice"
                    class:on=move || voice.get()
                    aria-pressed=move || voice.get().to_string()
                    aria-label=word
                    title=word
                    on:click=move |_| set_voice.update(|on| *on = !*on)
                >
                    // a speaker and its sound
                    <svg viewBox="0 0 24 24" aria-hidden="true">
                        <path class="body" d="M4 9.5h3.5L12 5.5v13l-4.5-4H4z" />
                        <path d="M15.5 9a4 4 0 0 1 0 6" />
                        <path d="M18.5 6.5a7.5 7.5 0 0 1 0 11" />
                    </svg>
                </button>
            }
        })
    }
}

/// A button per form the language has — singular and plural, and in Arabic
/// the dual between them — each adding its form under every name shown, or
/// taking it away: any of them at once, never none. A language with one
/// form, as Japanese, has nothing to choose and shows none.
#[component]
pub fn FormButtons(
    forms: ReadSignal<Forms>,
    set_forms: WriteSignal<Forms>,
    lang: ReadSignal<Lang>,
) -> impl IntoView {
    // two buttons or three, so a new set with each language
    move || {
        let l = lang.get();
        (l.numbers().len() > 1).then(|| {
            view! {
                <div class="forms" role="group" aria-label=Forms::label(l)>
                    {l
                        .numbers()
                        .iter()
                        .map(|&n| {
                            let on = move || forms.get().shows(n, l);
                            view! {
                                <button
                                    class="form"
                                    class:on=on
                                    aria-pressed=move || on().to_string()
                                    aria-label=n.name(l)
                                    title=n.name(l)
                                    on:click=move |_| set_forms.update(|f| *f = f.toggled(n, l))
                                >
                                    {dots(n)}
                                </button>
                            }
                        })
                        .collect_view()}
                </div>
            }
        })
    }
}

/// A form's button: as many dots as it names — one, two, or three in a
/// triangle for many.
fn dots(n: Number) -> impl IntoView {
    let at: &[(&str, &str)] = match n {
        Number::Singular => &[("12", "12")],
        Number::Dual => &[("7", "12"), ("17", "12")],
        Number::Plural => &[("12", "7"), ("6.5", "16.5"), ("17.5", "16.5")],
    };
    view! {
        <svg viewBox="0 0 24 24" aria-hidden="true">
            {at.iter().map(|&(x, y)| view! { <circle cx=x cy=y r="3" /> }).collect_view()}
        </svg>
    }
}

/// The toggle's name, for a screen reader and a mouse's tooltip.
fn all_names(lang: Lang) -> &'static str {
    match lang {
        Lang::En => "All names",
        Lang::Ar => "كل الأسماء",
        Lang::De => "Alle Namen",
        Lang::Sv => "Alla namn",
        Lang::Es => "Todos los nombres",
        Lang::Uk => "Усі назви",
        Lang::He => "כל השמות",
        Lang::Ja => "すべての名前",
    }
}

/// Each language listed in its own script.
#[component]
pub fn Picker(lang: ReadSignal<Lang>, set_lang: WriteSignal<Lang>) -> impl IntoView {
    view! {
        <select
            class="picker"
            aria-label=move || lang.get().language_word()
            on:change=move |ev| {
                if let Some(l) = Lang::from_tag(&event_target_value(&ev)) {
                    set_lang.set(l);
                }
            }
        >
            // each option says whether it is the one chosen; setting the
            // select's value instead can run before the options exist
            {Lang::ALL
                .into_iter()
                .map(|l| {
                    view! {
                        <option
                            value=l.code()
                            lang=l.code()
                            dir=l.dir()
                            selected=move || lang.get() == l
                        >
                            {l.endonym()}
                        </option>
                    }
                })
                .collect_view()}
        </select>
    }
}
