use vertigo::{Driver, Computed, VDomElement};
use vertigo_browserdriver::{start_browser_app2};

mod content;
mod data;
mod components;
mod app;

#[no_mangle]
pub fn start_application() {
    // start_browser_app(app::StateApp::new, app::render);

    start_browser_app2(|driver: &Driver| -> Computed<VDomElement> {
        app::StateApp::new(driver).render()
    });
}

