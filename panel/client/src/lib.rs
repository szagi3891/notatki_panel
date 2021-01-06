use wasm_bindgen::prelude::wasm_bindgen;

use vertigo::{
    App,
    VDomComponent,
    computed::Dependencies,
};

use browserdriver::DomDriverBrowser;

mod state;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub async fn start_app() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());

    log::info!("Start rustowego modułu ...");

    let driver = DomDriverBrowser::new();

    let root: Dependencies = Dependencies::default();
    let app_state = state::State::new(&root, &driver);

    let app = App::new(driver.clone(), VDomComponent::new(app_state, state::render));

    app.start_app().await;
}
