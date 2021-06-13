use wasm_bindgen::prelude::wasm_bindgen;

use vertigo::{
    App,
    VDomComponent,
    computed::Dependencies,
};

use vertigo_browserdriver::DomDriverBrowser;

mod request;
mod content;
mod state_data;
mod components;
mod app;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub async fn start_app() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());

    log::info!("Start rustowego modułu ...");

    let root: Dependencies = Dependencies::default();
    let driver = DomDriverBrowser::new(&root);
    let app_state = app::State::new(&root, &driver);

    let app = App::new(driver.clone(), VDomComponent::new(app_state, app::render));

    app.start_app().await;
}
