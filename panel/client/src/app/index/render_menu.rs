use vertigo::{
    VDomElement,
    Css,
    Computed,
};

use vertigo::{html, css};
use super::state::{AppIndexState};
use crate::components::button;

fn css_footer() -> Css {
    css!("
        display: flex;
        flex-shrink: 0;
        line-height: 25px;
        border-bottom: 1px solid black;
    ")
}

pub fn render_menu(state: &Computed<AppIndexState>) -> VDomElement {
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

    let on_mkdir = {
        let state = state.clone();

        move || {
            state.redirect_to_mkdir();
        }
    };

    let mut out = Vec::new();

    out.push(button("Utwórz plik", on_create));
    out.push(button("Zmień nazwę", on_rename));
    out.push(button("Edycja pliku", on_click));
    out.push(button("Utwórz katalog", on_mkdir));
    
    let avaible_delete_button = state.avaible_delete_button.get_value();

    if *avaible_delete_button {
        let alert = state.alert.get_value();

        let on_delete = {
            move || {
                alert.delete();
            }
        };

        out.push(button("Usuń", on_delete));
    }

    out.push(button("Wyszukaj", {
        let alert = state.alert.get_value();

        move || {
            alert.redirect_to_search();
        }
    }));

    html! {
        <div css={css_footer()}>
            { ..out }
        </div>
    }
}
