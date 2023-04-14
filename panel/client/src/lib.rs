#![allow(clippy::ptr_arg)]
#![allow(clippy::vec_init_then_push)]
#![allow(clippy::needless_return)]
#![allow(clippy::len_zero)]
#![allow(clippy::inherent_to_string)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::module_inception)]
#![allow(clippy::match_like_matches_macro)]
#![allow(clippy::let_and_return)]
#![allow(clippy::vtable_address_comparisons)]               //TODO - do sprawdzenia, podobno bywa niebezpieczne
use vertigo::{main, DomNode};

mod content;
pub mod data;
mod components;
mod app;

#[main]
pub fn render() -> DomNode {
    let state = app::App::new();
    state.render()
}
