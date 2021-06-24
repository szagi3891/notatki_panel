use vertigo::{
    VDomElement,
    Css,
    computed::{
        Computed,
    }
};

use vertigo_html::{html, css};
use super::state::{State};
use crate::components::button;

fn css_footer() -> Css {
    css!("
        display: flex;
        flex-shrink: 0;
        line-height: 25px;
        padding: 0 5px;
        border-bottom: 1px solid black;
    ")
}

pub fn render_menu(state: &Computed<State>) -> VDomElement {
    let state = state.get_value();

    let on_click = {
        let state = state.clone();
        
        move || {
            state.current_edit();
        }
    };

    let on_rename = {
        let state = state.clone();

        move || {
            state.current_rename();
        }
    };

    let on_create = {
        let state = state.clone();
        
        move || {
            state.create_file();
        }
    };

    let mut out = Vec::new();

    out.push(button("Utwórz plik", on_create));
    out.push(button("Zmień nazwę", on_rename));
    out.push(button("Edycja pliku", on_click));

    // let out = [
    //     button("Utwórz plik", || {}),
    //     button("Utwórz katalog", || {}),
    //     button("Zmień nazwę", || {}),
    //     button("Edycja pliku", on_click)
    // ];

    // <span css={css_item()}>"Utwórz plik"</span>
    // <span css={css_item()}>"Utwórz katalog"</span>
    // <span css={css_item()}>"Zmień nazwę"</span>
    // <span css={css_item()} onClick={on_click}>"Edycja pliku"</span>

    html! {
        <div css={css_footer()}>
            { ..out }
        </div>
    }
}
