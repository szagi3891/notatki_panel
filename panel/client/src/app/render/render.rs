use vertigo::{VDomElement, computed::{
        Computed,
    }};

use super::view_index::render as view_index;

use crate::app::state::{State, View};


pub fn render(state: &Computed<State>) -> VDomElement {

    let state_value = state.get_value();
    let view = state_value.current_view.get_value();

    match view.as_ref() {
        View::Index => {
            view_index(&state_value.state_view_index)
        },
        // View::EditContent => {
        //     panic!("das");
        // }
    }
}

/*
onKeyDown={on_keydown} tabindex="0"
*/