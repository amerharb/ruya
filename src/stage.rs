//! The picture, and the things in it you can point at.
//!
//! Three layers in one box that has the picture's exact aspect ratio: the
//! picture as an `<img>`, an `<svg>` over it in the picture's own coordinates
//! holding one invisible region per thing, and the name as an HTML label —
//! HTML rather than SVG text, so Arabic and Hebrew shape and fall back to a
//! font that has them. The box keeps the three aligned at every size.
//!
//! What is shown: whatever the mouse is over, else whatever was last tapped,
//! clicked or reached with the keyboard. Tapping anywhere else, or Escape,
//! clears it.
//!
//! With every name asked for, each thing also carries a small tag with its
//! name, on the thing itself; pointing works as before, the thing pointed at
//! getting the big label while the other tags step back.
//!
//! A name is shown in each form chosen, a line each: the singular on top,
//! then the dual where the language has one, then the plural. With readings
//! asked for, a Japanese name carries furigana over its kanji.
//!
//! The picture's two edges go to the other scenes, round in a circle: the
//! right one to the next, the left to the one before. Pointed at, an edge
//! shades and shows its arrow. On a touch screen a swipe does the same — see
//! `swipe`.
//!
//! One stage shows one scene in one style; switching either builds a new one.
//! A stage an edge or a swipe stepped to slides in from that side, and on a
//! phone held upright opens at the end it was entered from.

use crate::lang::{Forms, Lang, Number};
use crate::reading::pieces;
use crate::scene::{Rendering, SCENES, Scene, Step};
use crate::spread::{Tag, spread};
use crate::style::Style;
use crate::swipe::Swipe;
use leptos::either::Either;
use leptos::ev;
use leptos::html::Div;
use leptos::prelude::*;
use wasm_bindgen::JsCast;

#[component]
pub fn Stage(
    scene: &'static Scene,
    style: Style,
    rendering: &'static Rendering,
    lang: ReadSignal<Lang>,
    all: ReadSignal<bool>,
    forms: ReadSignal<Forms>,
    readings: ReadSignal<bool>,
    came: Option<Step>,
    turn: Callback<Step>,
) -> impl IntoView {
    let (w, h) = (rendering.picture.width, rendering.picture.height);
    // a thing's name in each form chosen: one, then two, then many
    let words = move |i: usize, l: Lang| -> Vec<&'static str> {
        let thing = scene.thing(&rendering.regions[i].thing);
        forms
            .get()
            .shown(l)
            .filter_map(|n| thing.name_for(l, n))
            .collect()
    };
    // the same, a line each, for the label and the tags; the name for one
    // with its readings over it, where they are asked for and it has some
    let lines = move |i: usize, l: Lang| {
        let thing = scene.thing(&rendering.regions[i].thing);
        let over = readings.get().then(|| thing.reading.get(l)).flatten();
        forms
            .get()
            .shown(l)
            .filter_map(|n| {
                let word = thing.name_for(l, n)?;
                Some(match over.filter(|_| n == Number::Singular) {
                    Some(annotated) => Either::Left(view! { <span>{ruby(annotated)}</span> }),
                    None => Either::Right(view! { <span>{word}</span> }),
                })
            })
            .collect_view()
    };

    // from a mouse or pen only: a finger has nothing to hover with, and its
    // enter and leave come as one with the tap
    let hovered = RwSignal::new(None::<usize>);
    // from a tap, a click or the keyboard
    let picked = RwSignal::new(None::<usize>);
    let shown = move || hovered.get().or(picked.get());

    // the last thing shown stays drawn while the label fades out, so moving
    // between things does not flicker through nothing
    let last = RwSignal::new(None::<usize>);
    Effect::new(move |_| {
        if let Some(i) = shown() {
            last.set(Some(i));
        }
    });

    // removed with the stage, so a stage left behind by a style switch does
    // not keep listening
    let escape = window_event_listener(ev::keydown, move |ev| {
        if ev.key() == "Escape" {
            hovered.set(None);
            picked.set(None);
        }
    });
    on_cleanup(move || escape.remove());

    // One handler for every tap: on a thing it picks it, anywhere else it
    // clears. `click`, not `pointerdown`, so a finger dragging the picture
    // sideways picks nothing.
    let on_click = move |ev: ev::MouseEvent| {
        let item = ev
            .target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
            .and_then(|el| el.closest("[data-item]").ok().flatten())
            .and_then(|el| el.get_attribute("data-item"))
            .and_then(|v| v.parse::<usize>().ok());
        picked.set(item);
    };

    // On a phone held upright the picture fills the height and scrolls
    // sideways. A scene stepped on to opens at its left end and one stepped
    // back to at its right, so swiping on the same way carries straight
    // across it; any other opens at the middle rather than the left edge.
    let viewport = NodeRef::<Div>::new();
    Effect::new(move |_| {
        if let Some(el) = viewport.get() {
            request_animation_frame(move || {
                let spare = overhang(&el);
                if spare > 0 {
                    el.set_scroll_left(match came {
                        Some(Step::Next) => 0,
                        Some(Step::Previous) => spare,
                        None => spare / 2,
                    });
                }
            });
        }
    });

    // A swipe on the picture turns the scene as the edges do — see `swipe`.
    // Listened for on the window, where a browser keeps touch listeners
    // passive, so the picture's own scrolling never waits on them; only a
    // touch beginning on this picture counts. Removed with the stage.
    let swipe = StoredValue::new(None::<Swipe>);
    let touch_start = window_event_listener(ev::touchstart, move |ev| {
        let begun = viewport.get_untracked().and_then(|el| begin(&ev, &el));
        // with one scene there is nowhere to turn to
        swipe.set_value(begun.filter(|_| SCENES.len() > 1));
    });
    let touch_end = window_event_listener(ev::touchend, move |ev| {
        let begun = swipe.try_update_value(Option::take).flatten();
        let Some((begun, touch)) = begun.zip(ev.changed_touches().get(0)) else {
            return;
        };
        let (x, y) = (touch.client_x().into(), touch.client_y().into());
        if let Some(step) = begun.end(x, y, ev.time_stamp()) {
            turn.run(step);
        }
    });
    let touch_cancel = window_event_listener(ev::touchcancel, move |_| swipe.set_value(None));
    on_cleanup(move || {
        touch_start.remove();
        touch_end.remove();
        touch_cancel.remove();
    });

    // A stage stepped to slides in once its picture is in, the CSS holding
    // it unseen till then, so what slides is the picture and not an empty
    // frame. The picture in, the scenes either side are fetched in this
    // style, so the next step finds its picture waiting.
    let ready = RwSignal::new(false);
    let fetch_neighbours = move || {
        if SCENES.len() > 1 {
            for step in [Step::Previous, Step::Next] {
                let (_, rendering) = scene.step(step).rendering_or_first(style);
                if let Ok(img) = web_sys::HtmlImageElement::new() {
                    img.set_src(&rendering.picture.src);
                }
            }
        }
    };

    // The signs painted in the picture, in the chosen language: each patch
    // is there from the start, hidden but the chosen language's, so a switch
    // shows its sign at once. Under the regions, so a tap on a sign is the
    // bus's, and under the dimming.
    let signs = rendering
        .signs
        .iter()
        .flat_map(|spot| {
            let [x0, y0, x1, y1] = spot.rect;
            Lang::ALL.into_iter().filter_map(move |l| {
                let href = spot.patch(&scene.id, style, l)?;
                Some(view! {
                    <image
                        class="sign"
                        href=href
                        x=x0.to_string()
                        y=y0.to_string()
                        width=(x1 - x0).to_string()
                        height=(y1 - y0).to_string()
                        preserveAspectRatio="none"
                        visibility=move || if lang.get() == l { "visible" } else { "hidden" }
                        aria-hidden="true"
                    />
                })
            })
        })
        .collect_view();

    let regions = rendering
        .regions
        .iter()
        .enumerate()
        .map(|(i, region)| {
            view! {
                <path
                    class="region"
                    d=region.d.clone()
                    data-item=i.to_string()
                    tabindex="0"
                    role="button"
                    aria-label=move || {
                        let l = lang.get();
                        words(i, l).join(l.comma())
                    }
                    on:pointerenter=move |ev: ev::PointerEvent| {
                        if ev.pointer_type() != "touch" {
                            hovered.set(Some(i));
                        }
                    }
                    on:pointerleave=move |_| {
                        if hovered.get_untracked() == Some(i) {
                            hovered.set(None);
                        }
                    }
                    on:focus=move |_| picked.set(Some(i))
                />
            }
        })
        .collect_view();

    let percent = move |(x, y): (f64, f64)| (x / w * 100.0, y / h * 100.0);

    // a tag per thing, on its biggest region, centred on the point its big
    // label is centred over
    let tags = rendering
        .one_per_thing()
        .into_iter()
        .map(|i| {
            let thing = &rendering.regions[i].thing;
            let (x, y) = percent(rendering.regions[i].anchor);
            // this thing, by any of its regions, has the big label
            let pointed = move || shown().is_some_and(|s| &rendering.regions[s].thing == thing);
            view! {
                <div
                    class="tag"
                    class:pointed=pointed
                    lang=move || lang.get().code()
                    dir=move || lang.get().dir()
                    style=format!("left:{x:.3}%;top:{y:.3}%")
                >
                    {move || lines(i, lang.get())}
                </div>
            }
        })
        .collect_view();

    // The tags measured where they stand and spread, so none covers another
    // or runs off the picture: when they appear, when the names change
    // language, forms or readings, and when the window changes size.
    let tag_box = NodeRef::<Div>::new();
    let place = move || {
        // nothing, for a stage already replaced: the frame it asked for can
        // come after it has gone, as in a tab not in view
        let Some(el) = tag_box.try_get_untracked().flatten() else {
            return;
        };
        let (sw, sh) = (f64::from(el.client_width()), f64::from(el.client_height()));
        let children = el.children();
        let nodes: Vec<web_sys::HtmlElement> = (0..children.length())
            .filter_map(|k| children.item(k)?.dyn_into().ok())
            .collect();
        let tags: Vec<Tag> = nodes
            .iter()
            .zip(rendering.one_per_thing())
            .map(|(node, i)| {
                let (x, y) = rendering.regions[i].anchor;
                Tag {
                    x: x / w * sw,
                    y: y / h * sh,
                    w: f64::from(node.offset_width()),
                    h: f64::from(node.offset_height()),
                }
            })
            .collect();
        for (node, (dx, dy)) in nodes.iter().zip(spread(&tags, sw, sh)) {
            // `translate`, which adds to the centring `transform` in the CSS
            let _ = node
                .style()
                .set_property("translate", &format!("{dx:.1}px {dy:.1}px"));
        }
    };
    Effect::new(move |_| {
        lang.track();
        forms.track();
        readings.track();
        if all.get() && tag_box.get().is_some() {
            // once the new names are laid out
            request_animation_frame(place);
        }
    });
    let resize = window_event_listener(ev::resize, move |_| {
        if all.get_untracked() {
            place();
        }
    });
    on_cleanup(move || resize.remove());

    // an edge, named for the scene it goes to
    let edge = move |step: Step| {
        let to = scene.step(step);
        let arrow = match step {
            Step::Previous => "M15 5L8 12L15 19",
            Step::Next => "M9 5L16 12L9 19",
        };
        view! {
            <button
                class="step"
                data-side=step.code()
                lang=move || lang.get().code()
                aria-label=move || to.name.get(lang.get()).to_string()
                title=move || to.name.get(lang.get()).to_string()
                on:click=move |_| turn.run(step)
            >
                <svg viewBox="0 0 24 24" aria-hidden="true">
                    <path d=arrow />
                </svg>
            </button>
        }
    };
    let edges = (SCENES.len() > 1).then(|| {
        view! {
            {edge(Step::Previous)}
            {edge(Step::Next)}
        }
    });

    let label = move || {
        last.get().map(|i| {
            let region = &rendering.regions[i];
            let l = lang.get();
            // above the region, at its anchor's x
            let (x, y) = percent((region.anchor.0, region.top));
            // near an edge the label turns inward instead of running off
            let edge = if y < 12.0 {
                "below"
            } else if x < 12.0 {
                "start"
            } else if x > 88.0 {
                "end"
            } else {
                "above"
            };
            view! {
                <div
                    class="label"
                    class:on=move || shown().is_some()
                    lang=l.code()
                    dir=l.dir()
                    data-edge=edge
                    style=format!("left:{x:.3}%;top:{y:.3}%")
                    aria-hidden="true"
                >
                    {lines(i, l)}
                </div>
            }
        })
    };

    view! {
        <div
            class="viewport"
            class:ready=move || ready.get()
            data-came=came.map(Step::code)
            node_ref=viewport
            on:click=on_click
        >
            <div
                class="stage"
                data-scene=scene.id.clone()
                data-style=style.code()
                style=format!("--w:{w};--h:{h}")
            >
                <img
                    src=rendering.picture.src.clone()
                    alt=""
                    draggable="false"
                    width=w.to_string()
                    height=h.to_string()
                    on:load=move |_| {
                        ready.set(true);
                        fetch_neighbours();
                    }
                    // a picture that will not load still lets its stage in
                    on:error=move |_| ready.set(true)
                />
                <svg
                    viewBox=format!("0 0 {w} {h}")
                    preserveAspectRatio="xMidYMid meet"
                    role="group"
                    aria-label=move || scene.name.get(lang.get()).to_string()
                >
                    {signs}
                    {regions}
                    // everything but the thing shown, dimmed: the picture's
                    // rectangle with the thing cut out of it, even-odd
                    <path
                        class="dim"
                        class:on=move || shown().is_some()
                        d=move || {
                            last.get()
                                .map(|i| format!("M0 0H{w}V{h}H0Z {}", rendering.regions[i].d))
                                .unwrap_or_default()
                        }
                    />
                    <path
                        class="outline"
                        class:on=move || shown().is_some()
                        d=move || {
                            last.get().map(|i| rendering.regions[i].d.clone()).unwrap_or_default()
                        }
                    />
                </svg>
                // after the regions, so over them, and reached by Tab after
                // them
                {edges}
                // the regions already carry the names for a screen reader
                <div
                    class="tags"
                    node_ref=tag_box
                    class:on=move || all.get()
                    class:aside=move || shown().is_some()
                    aria-hidden="true"
                >
                    {tags}
                </div>
                {label}
            </div>
        </div>
    }
}

/// A name with its readings, each over its kanji, as HTML ruby.
fn ruby(annotated: &'static str) -> impl IntoView {
    pieces(annotated)
        .into_iter()
        .map(|(text, reading)| match reading {
            Some(reading) => Either::Left(view! {
                <ruby>
                    {text}
                    <rt>{reading}</rt>
                </ruby>
            }),
            None => Either::Right(text),
        })
        .collect_view()
}

/// A touch beginning, as the swipe it may become: none for a touch off the
/// picture, a second finger, which makes a pinch, or a page zoomed in, which
/// a finger pans.
fn begin(ev: &web_sys::TouchEvent, viewport: &web_sys::HtmlDivElement) -> Option<Swipe> {
    let target: web_sys::Node = ev.target()?.dyn_into().ok()?;
    let zoomed = window().visual_viewport().is_some_and(|v| v.scale() > 1.01);
    if !viewport.contains(Some(&target)) || ev.touches().length() != 1 || zoomed {
        return None;
    }
    let touch = ev.touches().get(0)?;
    let (left, spare) = (viewport.scroll_left(), overhang(viewport));
    Some(Swipe {
        x: touch.client_x().into(),
        y: touch.client_y().into(),
        time: ev.time_stamp(),
        // all of it showing, it is at both ends; a pixel's leeway, for a
        // scroll that stops a fraction short
        at_left: spare <= 0 || left <= 1,
        at_right: left >= spare - 1,
    })
}

/// How much wider the picture is than the box it scrolls in, in CSS pixels:
/// none wherever it all shows. Measured on the picture itself, as a tag
/// reaching past its edge, unseen or not, widens what the box could scroll.
fn overhang(viewport: &web_sys::HtmlDivElement) -> i32 {
    viewport
        .first_element_child()
        .and_then(|stage| stage.dyn_into::<web_sys::HtmlElement>().ok())
        .map_or(0, |stage| stage.offset_width() - viewport.client_width())
}
