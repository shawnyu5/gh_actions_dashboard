use wasm_bindgen::prelude::wasm_bindgen;
use leptos::*;

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <p>"Hello, World!"</p>
    }
}

#[wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    _ = console_log::init_with_level(log::Level::Debug);

    log!("hydrate mode - hydrating");

    leptos::mount_to_body(|cx| {
        view! { cx,  <App/> }
    });
}
