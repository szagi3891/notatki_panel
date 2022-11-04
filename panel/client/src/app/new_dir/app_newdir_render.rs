use vertigo::{Css, css, bind, dom, DomElement};

use super::AppNewdir;
use crate::components::button;

fn css_wrapper() -> Css {
    css!("
        display: flex;
        flex-direction: column;
        border: 1px solid black;
        background-color: #e0e0e0;
        width: 100vw;
        height: 100vh;
    ")
}

fn css_header() -> Css {
    css!("
        border-bottom: 1px solid black;
        padding: 5px;
    ")
}

pub fn app_newdir_render(state: AppNewdir) -> DomElement {
    let view_new_name = state.new_name.render(true);

    let parent_path = state.parent.as_slice().join("/");

    let app = &state.app;
    let button_back = button("Wróć", bind!(|app| {
        app.redirect_to_index();
    }));

    let button_save = state.save_enable.render_value_option({
        let state = state.clone();
        move |save_enable| {
            match save_enable {
                true => Some(button("Zapisz", state.bind_on_save(&state.app))),
                false => None
            }
        }
    });

    dom! {
        <div css={css_wrapper()}>
            <div css={css_header()}>
                "tworzenie katalogu => "
                {parent_path}
            </div>
            <div css={css_header()}>
                { button_back }
                { button_save }
            </div>
            { view_new_name }

            <div data-run-module="funkcjaJs">
            </div>
        </div>
    }
}
