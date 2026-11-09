mod app;
mod geometry;
mod lang;
mod picker;
mod reading;
mod sada;
mod scene;
mod spread;
mod stage;
mod style;
mod swipe;
mod url;

fn main() {
    // a panic shows in the browser's console with its message, not as
    // "unreachable executed"
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(app::App);
    // once the picture is on its way: Ruya was opened
    sada::post_open();
}
