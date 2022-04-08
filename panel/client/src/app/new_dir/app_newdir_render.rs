use vertigo::{Css, VDomComponent, css, html};

use super::{AppNewdir};
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

pub fn app_newdir_render(view_new_name: VDomComponent, state: AppNewdir) -> VDomComponent {
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
            buttons.push(button("Zapisz", state.bind_on_save()));
        }

        html! {
            <div css={css_wrapper()}>
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