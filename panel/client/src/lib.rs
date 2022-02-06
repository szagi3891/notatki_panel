#![allow(clippy::ptr_arg)]
#![allow(clippy::vec_init_then_push)]
#![allow(clippy::needless_return)]
#![allow(clippy::len_zero)]
#![allow(clippy::inherent_to_string)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::module_inception)]
use vertigo_browserdriver::{start_browser_app};

mod content;
mod data;
mod components;
mod app;

#[no_mangle]
pub fn start_application() {
    start_browser_app(app::App::component);
}
