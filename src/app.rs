use crate::lang::{Forms, Lang};
use crate::picker::{AllNames, FormButtons, Picker, ReadingsButton, ScenePicker, StylePicker};
use crate::scene::{Scene, Step};
use crate::stage::Stage;
use crate::style::Style;
use crate::url;
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let (scene, set_scene) = signal(url::initial_scene());
    let (lang, set_lang) = signal(url::initial_lang());
    let (style, set_style) = signal(url::initial_style());
    let (all, set_all) = signal(url::initial_all_names());
    let (forms, set_forms) = signal(url::initial_forms());
    let (readings, set_readings) = signal(url::initial_readings());
    // How the scene shown was reached, when an edge or a swipe stepped to
    // it: set as the step is taken, and taken back by the stage it builds,
    // which slides in from that side. Anything else that builds a stage — a
    // scene picked from the list, a style switched — finds it empty.
    let arrival = StoredValue::new(None::<Step>);

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
    // one at a time is what Ruya opens with, so it leaves the address
    Effect::watch(
        move || all.get(),
        |on: &bool, _, _| match on {
            true => url::write("n", "all"),
            false => url::remove("n"),
        },
        false,
    );
    // and so does the singular alone
    Effect::watch(
        move || forms.get(),
        |f: &Forms, _, _| match *f == Forms::SINGULAR {
            true => url::remove("f"),
            false => url::write("f", &f.code()),
        },
        false,
    );
    // and names without readings
    Effect::watch(
        move || readings.get(),
        |on: &bool, _, _| match on {
            true => url::write("r", "on"),
            false => url::remove("r"),
        },
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
            <AllNames all=all set_all=set_all lang=lang />
            <FormButtons forms=forms set_forms=set_forms lang=lang />
            <ReadingsButton readings=readings set_readings=set_readings lang=lang />
            <Picker lang=lang set_lang=set_lang />
        </header>
        <main class="main">
            // A new stage per scene and style: its own picture, regions and
            // state, and its own elements. Keyed, so a change builds it
            // afresh; a closure would have Leptos patch the last stage's
            // elements in place, keeping their scroll and a slide already
            // played.
            <For
                each=move || {
                    let scene = scene.get();
                    [(scene, scene.rendering_or_first(style.get()).0)]
                }
                key=|&(scene, style)| (scene.id.as_str(), style)
                children=move |(scene, style)| {
                    let (_, rendering) = scene.rendering_or_first(style);
                    let came = arrival.try_update_value(Option::take).flatten();
                    let turn = Callback::new(move |step: Step| {
                        arrival.set_value(Some(step));
                        set_scene.set(scene.step(step));
                    });
                    view! {
                        <Stage
                            scene=scene
                            style=style
                            rendering=rendering
                            lang=lang
                            all=all
                            forms=forms
                            readings=readings
                            came=came
                            turn=turn
                        />
                    }
                }
            />
        </main>
    }
}
