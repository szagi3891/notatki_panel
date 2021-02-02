use vertigo::{
    VDomElement,
    Css,
    computed::{
        Computed,
    }
};

use vertigo_html::{html_component, Inline};

use super::state::State;

pub fn render_list(state: &Computed<State>) -> VDomElement {
    let on_create = {
        let state = state.clone();
        move || {
            state.get_value().create_dir("Jakiś".into());
        }
    };

    html_component! {
        <div>
            <div onClick={on_create}>utwórz katalog</div>
            <div>lista plikow</div>
        </div>
    }
}
