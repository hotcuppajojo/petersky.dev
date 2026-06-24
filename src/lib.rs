pub mod app;

#[cfg(feature = "hydrate")]
// ensure web_sys symbols are linked so wasm-bindgen emits required browser intrinsics for deployed builds
use web_sys as _;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;

    // enable panic hook so browser-side failures are observable in production monitoring
    console_error_panic_hook::set_once();

    // hydrate into the server-rendered dom to attach client interactivity without re-rendering
    leptos::mount::hydrate_body(App);
}