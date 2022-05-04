use vertigo::{Css, VDomElement, VDomComponent, bind};
use vertigo::{css, html};

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

fn render_input_content(state: &AppNewcontent) -> VDomElement {
    let content = &state.content.get();

    let on_input = {
        let state = state.clone();
        move |new_value: String| {
            state.on_input_content(new_value);
        }
    };

    html! {
        <textarea css={css_input_content()} on_input={on_input} value={content.as_ref()} />
    }
}

pub fn app_newcontent_render(state: &AppNewcontent, app: App) -> VDomComponent {
    let view_input = VDomComponent::new(state, render_input_content);
    let view_new_name = state.new_name.clone().render(true);

    VDomComponent::new(state, move |state: &AppNewcontent| -> VDomElement {
        let on_click = bind(&app).call(|app| {
            app.redirect_to_index();
        });

        let parent_path = state.parent.as_slice().join("/");

        let mut buttons = vec!(button("Wróć", on_click));

        let save_enable = state.save_enable.get();

        if *save_enable {
            buttons.push(button("Zapisz", state.on_save(&app)));
        }

        html! {
            <div css={css_wrapper()}>
                <div css={css_header()}>
                    "tworzenie pliku => "
                    {parent_path}
                </div>
                <div css={css_header()}>
                    { ..buttons }
                </div>
                { view_new_name.clone() }
                { view_input.clone() }
            </div>
        }
    })
}