use vertigo::{Css, VDomElement, Computed};
use vertigo::{css, html};

use super::AppRenameItemState;
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

fn render_input(state: &Computed<AppRenameItemState>) -> VDomElement {
    let state = state.get_value();

    let content = &state.new_name.get_value();

    let on_input = move |new_value: String| {
        state.on_input(new_value);
    };

    html! {
        <input css={css_input()} on_input={on_input} value={content.as_ref()} />
    }
}


fn render_textarea(state: &Computed<AppRenameItemState>) -> VDomElement {
    let state = state.get_value();

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


pub fn render(state: &Computed<AppRenameItemState>) -> VDomElement {

    let state_value = state.get_value();

    let on_click = {
        let state = state_value.clone();
        move || {
            state.redirect_to_index();
        }
    };

    let path = state_value.get_full_path();

    let mut buttons = vec![
        button("Wróć", on_click)
    ];

    let save_enable = state_value.save_enable.get_value();

    if *save_enable {
        let on_save = {
            let state = state_value;
            move || {
                state.on_save();
            }
        };

        buttons.push(button("Zmień nazwę", on_save));
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
                "zmiana nazwy => "
                {path}
            </div>
            <div css={css_header()}>
                { ..buttons }
            </div>
            <component {render_input} data={state} />
            <component {render_textarea} data={state} />
        </div>
    }
}