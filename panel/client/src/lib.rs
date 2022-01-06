// #![allow(clippy::nonstandard_macro_braces)]

use vertigo_browserdriver::prelude::*;

mod content;
mod state_data;
mod components;
mod app;

#[wasm_bindgen_derive(start)]
pub fn start_application() {
    log::info!("Start rustowego modułu ...");

    let driver = DriverBrowser::new();
    let app_state = app::AppState::new(&driver);

    start_browser_app(driver, app_state, app::render);
}
