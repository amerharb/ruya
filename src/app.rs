use crate::lang::Lang;
use crate::picker::Picker;
use crate::scene::{KITCHEN, Scene};
use crate::stage::Stage;
use crate::url;
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let (lang, set_lang) = signal(url::initial_lang());

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

    // written only when someone picks; a language guessed from the browser
    // stays out of the address
    Effect::watch(
        move || lang.get(),
        |l: &Lang, _, _| url::write_lang(*l),
        false,
    );

    let scene: &'static Scene = &KITCHEN;

    view! {
        <header class="bar">
            <span class="brand">"Ruya"</span>
            <h1
                class="scene-name"
                lang=move || lang.get().code()
                dir=move || lang.get().dir()
            >
                {move || scene.name.get(lang.get()).to_string()}
            </h1>
            <Picker lang=lang set_lang=set_lang />
        </header>
        <main class="main">
            <Stage scene=scene lang=lang />
        </main>
    }
}
