use vertigo_browserdriver::{start_browser_app};

mod content;
mod data;
mod components;
mod app;

#[no_mangle]
pub fn start_application() {
    start_browser_app(app::StateApp::new);
}
