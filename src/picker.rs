//! The two choices in the bar: the style, as a little picture of each, and the
//! language, as the browser's own `<select>`, which every phone turns into its
//! best native chooser.

use crate::lang::Lang;
use crate::scene::Scene;
use crate::style::Style;
use leptos::prelude::*;

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
