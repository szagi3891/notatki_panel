use vertigo::{Css, VDomElement, VDomComponent};
use vertigo::{css, html};

use super::AppEditcontent;
use crate::app::App;
use crate::components::button;
use crate::utils::bind;

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

pub fn app_editcontent_render(app: &App, state: AppEditcontent) -> VDomComponent {
    let view_textares = VDomComponent::new(state.clone(), render_textarea);
    let app = app.clone();

    VDomComponent::new(state, move |state: &AppEditcontent| {
        let on_click = bind(&app).exec_ref(|app| {
            app.redirect_to_index();
        });

        let path = state.path.as_slice().join("/");

        let mut buttons = Vec::new();

        buttons.push(button("Wróć", on_click));

        let save_enable = state.save_enable.get_value();

        if *save_enable {
            let on_save = state.on_save(&app);
            buttons.push(button("Zapisz", on_save));
        }

        html! {
            <div css={css_wrapper()}>
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