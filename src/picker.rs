//! The choices in the bar: the scene, as its name; the style, as a little
//! picture of each; and the language. The scene and the language are the
//! browser's own `<select>`, which every phone turns into its best native
//! chooser.

use crate::lang::Lang;
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
                // the one chosen
                {SCENES
                    .iter()
                    .map(|s| {
                        view! {
                            <option value=s.id.clone() selected=move || scene.get().id == s.id>
                                {move || s.name.get(lang.get()).to_string()}
                            </option>
                        }
                    })
                    .collect_view()}
            </select>
        }
    });
    let alone = picker.is_none();
    view! {
        <h1 class="scene-name" lang=move || lang.get().code() dir=move || lang.get().dir()>
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
