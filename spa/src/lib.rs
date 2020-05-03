#![recursion_limit = "512"]

mod components;
pub use components::SimpleComponent;

mod pages;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::initialize();
    let element = yew::utils::document()
        .query_selector("#app-container")
        .unwrap()
        .expect("Cannot find app-container element");

    yew::App::<pages::AppContainer>::new().mount(element);
    yew::run_loop();

    Ok(())
}
