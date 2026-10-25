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
//! One stage shows one scene in one style; switching either builds a new one.

use crate::lang::Lang;
use crate::scene::{Rendering, Scene};
use crate::style::Style;
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
) -> impl IntoView {
    let (w, h) = (rendering.picture.width, rendering.picture.height);
    let name = move |i: usize, l: Lang| scene.thing(&rendering.regions[i].thing).name.get(l);

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
    // sideways; open it at the middle rather than the left edge.
    let viewport = NodeRef::<Div>::new();
    Effect::new(move |_| {
        if let Some(el) = viewport.get() {
            request_animation_frame(move || {
                let spare = el.scroll_width() - el.client_width();
                if spare > 0 {
                    el.set_scroll_left(spare / 2);
                }
            });
        }
    });

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
                    aria-label=move || name(i, lang.get()).to_string()
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
                    {name(i, l).to_string()}
                </div>
            }
        })
    };

    view! {
        <div class="viewport" node_ref=viewport on:click=on_click>
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
                />
                <svg
                    viewBox=format!("0 0 {w} {h}")
                    preserveAspectRatio="xMidYMid meet"
                    role="group"
                    aria-label=move || scene.name.get(lang.get()).to_string()
                >
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
                {label}
            </div>
        </div>
    }
}
