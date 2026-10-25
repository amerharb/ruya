use crate::lang::Lang;
use crate::picker::{Picker, ScenePicker, StylePicker};
use crate::scene::Scene;
use crate::stage::Stage;
use crate::style::Style;
use crate::url;
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let (scene, set_scene) = signal(url::initial_scene());
    let (lang, set_lang) = signal(url::initial_lang());
    let (style, set_style) = signal(url::initial_style());

    // <html lang> follows the language, so a screen reader reads the names in
    // the right voice and a browser does not offer to translate them. The
    // direction is not set page-wide: the layout stays put, and what reads in
    // the language — the label, each option — carries its own.
    Effect::new(move |_| {
        let code = lang.get().code();
        if let Some(html) = document().document_element() {
            let _ = html.set_attribute("lang", code);
        }
    });

    // written only when someone picks; a language guessed from the browser,
    // or the first scene or the default style, stays out of the address
    Effect::watch(
        move || scene.get(),
        |s: &&'static Scene, _, _| url::write("p", &s.id),
        false,
    );
    Effect::watch(
        move || lang.get(),
        |l: &Lang, _, _| url::write("l", l.code()),
        false,
    );
    Effect::watch(
        move || style.get(),
        |s: &Style, _, _| url::write("s", s.code()),
        false,
    );

    view! {
        <header class="bar">
            <span class="brand">"Ruya"</span>
            <ScenePicker scene=scene set_scene=set_scene lang=lang />
            // the scene's own styles
            {move || {
                view! {
                    <StylePicker scene=scene.get() style=style set_style=set_style lang=lang />
                }
            }}
            <Picker lang=lang set_lang=set_lang />
        </header>
        <main class="main">
            // a new stage per scene and style: its own picture, regions and
            // state
            {move || {
                let scene = scene.get();
                let (s, rendering) = scene.rendering_or_first(style.get());
                view! { <Stage scene=scene style=s rendering=rendering lang=lang /> }
            }}
        </main>
    }
}
