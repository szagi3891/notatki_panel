use vertigo::{Css, VDomElement, computed::Computed};
use vertigo_html::{css, html};

use crate::state::StateViewEditContent;


fn css_wrapper() -> Css {
    css!("
        display: flex;
        flex-direction: column;
        width: 100%;
        height: 100%;
    ")
}

fn css_header() -> Css {
    css!("
        border-bottom: 1px solid black;
    ")
}

fn css_body() -> Css {
    css!("
        flex-grow: 1;
        border: 0;
        padding: 5px;
        :focus {
            border: 0;
        }
    ")
}

fn render_textarea(state: &Computed<StateViewEditContent>) -> VDomElement {
    let state = state.get_value();

    let content = &state.as_ref().content;

    html! {
        <textarea css={css_body()}>
            {content}
        </textarea>
    }
}

pub fn render(state: &Computed<StateViewEditContent>) -> VDomElement {

    let state_value = state.get_value();

    let on_click = {
        let state = state_value.clone();
        move || {
            state.redirect_to_index();
        }
    };

    let path = state_value.as_ref().path.as_slice().join("/");

    html! {
        <div id="root" css={css_wrapper()}>
            <style>
                "
                html, body {
                    width: 100%;
                    height: 100%;
                    margin: 0;
                    padding: 0;
                    border: 0;
                }
                "
            </style>
            <div css={css_header()}>
                <div>
                    "edycja pliku - do zrobienia .... => "
                    {path}
                </div>
                <div onClick={on_click}>"Wróć"</div>
            </div>
            <component {render_textarea} data={state} />
        </div>
    }
}