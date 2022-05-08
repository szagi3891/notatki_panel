use vertigo::{Css, VDomElement, VDomComponent};
use vertigo::{css, html, bind};

use super::AppRenameitem;
use crate::app::App;
use crate::components::button;
use crate::data::ContentView;

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

fn css_input() -> Css {
    css!("
        border: 0;
        padding: 5px;
        margin: 5px;
        border: 1px solid blue;
        :focus {
            border: 0;
        }
    ")
}

fn css_textarea() -> Css {
    css!("
        flex-grow: 1;
        border: 0;
        padding: 5px;
        margin: 5px;
        border: 1px solid blue;
        background: #e0e0e010;
        :focus {
            border: 0;
        }
    ")
}

fn render_input(state: &AppRenameitem) -> VDomElement {
    let content = state.new_name.get();

    let on_input = bind(state).call_param(|state, new_value: String| {
        state.on_input(new_value);
    });

    html! {
        <input css={css_input()} on_input={on_input} value={content} autofocus="" />
    }
}

fn render_textarea(state: &AppRenameitem) -> VDomElement {
    let mut full_path = state.path.clone();
    full_path.push(state.prev_name.clone());
    let content = state.data.git.get_content(&full_path);

    match content {
        Some(ContentView { content, .. }) => {
            let text = content.as_str();
            html! {
                <textarea css={css_textarea()} readonly="readonly" value={text} />
            }
        },
        None => {
            html!{
                <div/>
            }
        }
    }
}

pub fn app_renameitem_render(state: &AppRenameitem, app: App) -> VDomComponent {

    let view_input = VDomComponent::from_ref(state, render_input);
    let view_textarea = VDomComponent::from_ref(state, render_textarea);

    VDomComponent::from_ref(state, move |state: &AppRenameitem| {
        let path = state.get_full_path();

        let mut buttons = vec![
            button("Wróć", bind(&app).call(|app| {
                app.redirect_to_index();
            }))
        ];

        let save_enable = state.save_enable.get();

        if save_enable {
            let on_save = bind(state).and(&app).spawn(move |state, app| {
                state.on_save(app)
            });

            buttons.push(button("Zmień nazwę", on_save));
        }

        html! {
            <div css={css_wrapper()}>
                <div css={css_header()}>
                    "zmiana nazwy => "
                    {path}
                </div>
                <div css={css_header()}>
                    { ..buttons }
                </div>
                { view_input.clone() }
                { view_textarea.clone() }
            </div>
        }
    })
}