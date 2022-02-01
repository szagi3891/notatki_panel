use vertigo::{Css, VDomComponent, css, html};

use super::{StateAppNewDir};
use crate::components::{button};

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

pub fn build_render(view_new_name: VDomComponent, state: StateAppNewDir) -> VDomComponent {
    VDomComponent::new(state, move |state| {
        let on_click = {
            let state = state.clone();
            move || {
                state.redirect_to_index();
            }
        };

        let parent_path = state.parent.as_slice().join("/");

        let mut buttons = vec!(button("Wróć", on_click));

        let save_enable = state.save_enable.get_value();

        if *save_enable {
            buttons.push(button("Zapisz", {
                let state = state.clone();
                move || {
                    state.on_save();
                }
            }));
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
                    "tworzenie katalogu => "
                    {parent_path}
                </div>
                <div css={css_header()}>
                    { ..buttons }
                </div>
                { view_new_name.clone() }
            </div>
        }
    })
}