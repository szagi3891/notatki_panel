use vertigo::{Css, VDomElement, VDomComponent, bind, Context};
use vertigo::{css, html};

use super::AppEditcontent;
use super::app_editcontent_state::EditContent;
use crate::app::App;
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

fn render_textarea(context: &Context, state: &AppEditcontent) -> VDomElement {
    let content = state.content_view.get(context);

    if let Some(EditContent { hash, content}) = content {
        if let Some(hash) = hash {
            let on_input = bind(state)
                .and(&hash)
                .call_param(|context, state, hash, new_value| {
                    state.on_input(context, new_value, hash.clone());
                });

            html! {
                <textarea css={css_body()} on_input={on_input} value={content} />
            }
        } else {

            html! {
                <textarea css={css_body()} value={content} />
            }
        }
    } else {
        html! {
            <div>
                "Ładowanie ..."
            </div>
        }
    }
}

pub fn app_editcontent_render(app: &App, state: &AppEditcontent) -> VDomComponent {
    let view_textares = VDomComponent::from_ref(state, render_textarea);
    let app = app.clone();

    VDomComponent::from_ref(state, move |context, state: &AppEditcontent| {
        let on_click = bind(&app).call(|_, app| {
            app.redirect_to_index();
        });

        let path = state.path.as_slice().join("/");

        let mut buttons = Vec::new();

        buttons.push(button("Wróć", on_click));

        let save_enable = state.save_enable.get(context);

        if save_enable {
            let on_save = state.on_save(&app, true);
            buttons.push(button("Zapisz", on_save));

            let on_save = state.on_save(&app, false);
            buttons.push(button("Zapisz i zostań", on_save));

            let on_reset = state.on_reset();
            buttons.push(button("Usuń naniesione zmiany", on_reset));
        }

        html! {
            <div css={css_wrapper()}>
                <div css={css_header()}>
                    "Edycja pliku => "
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