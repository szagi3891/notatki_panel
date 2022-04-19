use vertigo::{Css, VDomElement, VDomComponent};
use vertigo::{css, html};

use super::AppRenameitem;
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
    let content = &state.new_name.get_value();

    let on_input = {
        let state = state.clone();

        move |new_value: String| {
            state.on_input(new_value);
        }
    };

    html! {
        <input css={css_input()} on_input={on_input} value={content.as_ref()} autofocus="" />
    }
}


fn render_textarea(state: &AppRenameitem) -> VDomElement {
    let prev_content = state.prev_content.clone();

    match prev_content {
        Some(text) => {
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


pub fn app_renameitem_render(state: AppRenameitem, app: App) -> VDomComponent {

    let view_input = VDomComponent::new(state.clone(), render_input);
    let view_textarea = VDomComponent::new(state.clone(), render_textarea);

    VDomComponent::new(state, move |state: &AppRenameitem| {
        let on_click = {
            let state = state.clone();
            let app = app.clone();

            move || {
                app.redirect_to_index();
            }
        };

        let path = state.get_full_path();

        let mut buttons = vec![
            button("Wróć", on_click)
        ];

        let save_enable = state.save_enable.get_value();

        if *save_enable {
            let on_save = state.bind_on_save(app.clone());
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