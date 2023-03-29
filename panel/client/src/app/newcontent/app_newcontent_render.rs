use vertigo::{Css, bind, dom, css, DomNode};

use crate::app::App;
use crate::components::{button};

use super::AppNewcontent;

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

fn css_input_content() -> Css {
    css!("
        flex-grow: 1;
        border: 0;
        padding: 5px;
        margin: 5px;
        border: 1px solid blue;
        :focus {
            border: 0;
        }
    ")
}

fn render_input_content(state: &AppNewcontent) -> DomNode {
    let content = state.content.to_computed();

    let on_input = {
        let state = state.clone();
        move |new_value: String| {
            state.on_input_content(new_value);
        }
    };

    dom! {
        <textarea css={css_input_content()} on_input={on_input} value={content} />
    }
}

pub fn app_newcontent_render(app: App, state: &AppNewcontent) -> DomNode {
    let view_input = render_input_content(state);
    let view_new_name = state.new_name.clone().render(true);

    let view_path = {
        let parent_path = state.parent.as_slice().join("/");

        dom! {
            <div css={css_header()}>
                "tworzenie pliku => "
                {parent_path}
            </div>
        }
    };

    let button_save = state.save_enable.render_value_option({
        let state = state.clone();

        move |show| {
            match show {
                true => {
                    Some(button("Zapisz", state.on_save()))
                },
                false => None
            }
        }
    });

    let on_click = bind!(app, || {
        app.redirect_to_index();
    });

    dom! {
        <div css={css_wrapper()}>
            { view_path }
            <div css={css_header()}>
                { button("Wróć", on_click) }
                { button_save }
            </div>
            { view_new_name }
            { view_input }
        </div>
    }
}