#![feature(format_args_capture)]

#![allow(clippy::nonstandard_macro_braces)]

use wasm_bindgen::prelude::wasm_bindgen;

use vertigo::{
    start_app,
    VDomComponent,
};

use vertigo_browserdriver::DriverBrowser;

mod content;
mod state_data;
mod components;
mod app;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub async fn start_application() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());

    log::info!("Start rustowego modułu ...");

    let driver = DriverBrowser::new();
    let app_state = app::AppState::new(&driver);

    start_app(driver, VDomComponent::new(app_state, app::render)).await;
}
