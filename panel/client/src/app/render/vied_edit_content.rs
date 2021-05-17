use vertigo::{VDomElement, computed::Computed};
use vertigo_html::html;

use crate::app::state::StateViewEditContent;



pub fn render(state: &Computed<StateViewEditContent>) -> VDomElement {

    let state = state.get_value();

    let on_click = {
        let state = state.clone();
        move || {
            state.redirect_to_index();
        }
    };

    let path = state.as_ref().path.as_slice().join("/");

    html!("
        <div>
            <div>edycja pliku - do zrobienia .... => {path}</div>
            <div onClick={on_click}>Wróć</div>
        </div>
    ")
}