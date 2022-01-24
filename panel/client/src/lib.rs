use vertigo_browserdriver::{start_browser_app2};

mod content;
mod data;
mod components;
mod app;

#[no_mangle]
pub fn start_application() {
    // start_browser_app(app::StateApp::new, app::render);

    start_browser_app2(app::StateApp::new);
}

