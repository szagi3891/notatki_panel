use vertigo::{Css, VDomElement, VDomComponent};
use vertigo::{css, html};

use super::AppEditcontent;
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

fn css_body() -> Css {
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

fn render_textarea(state: &AppEditcontent) -> VDomElement {
    let content = &state.edit_content.get_value();

    let on_input = {
        let state = state.clone();
        
        move |new_value: String| {
            state.on_input(new_value);
        }
    };

    html! {
        <textarea css={css_body()} on_input={on_input} value={content.as_ref()} />
    }
}

pub fn app_editcontent_render(state: AppEditcontent) -> VDomComponent {
    let view_textares = VDomComponent::new(state.clone(), render_textarea);

    VDomComponent::new(state, move |state: &AppEditcontent| {
        let on_click = {
            let state = state.clone();
            move || {
                state.redirect_to_index();
            }
        };

        let path = state.path.as_slice().join("/");

        let mut buttons = Vec::new();

        buttons.push(button("Wróć", on_click));

        let save_enable = state.save_enable.get_value();

        if *save_enable {
            let on_save = {
                let state = state.clone();
                move || {
                    state.on_save();
                }
            };

            buttons.push(button("Zapisz", on_save));
        }

        html! {
            <div css={css_wrapper()}>
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
                    "edycja pliku => "
                    {path}
                </div>
                <div css={css_header()}>
                    { ..buttons }
                </div>
                { view_textares.clone() }
            </div>
        }
    })
}