//! The language picker: the browser's own `<select>`, which every phone turns
//! into its best native chooser. Each language is listed in its own script.

use crate::lang::Lang;
use leptos::prelude::*;

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
