use vertigo::{Css, KeyDownEvent, VDomElement, computed::{
    Computed,
}};

use vertigo_html::{html, css};

use super::state::State;

use super::render_list::render_list;
use super::render_header::render_header;
use super::render_content::render_content;
use super::render_menu::render_menu;

fn css_wrapper() -> Css {
    css!("
        display: flex;
        flex-direction: column;
        border: 1px solid black;
        background-color: #e0e0e0;
        padding: 0;
        width: 100vw;
        height: 100vh;
        box-sizing: border-box;
    ")
}

fn css_content() -> Css {
    css!("
        flex-grow: 1;
        display: flex;
        border-bottom: 1px solid black;
        overflow: hidden;
    ")
}

fn css_content_list() -> Css {
    css!("
        width: 300px;
        flex-grow: 0;
        flex-shrink: 0;
        border-right: 1px solid black;

        display: flex;
    ")
}

fn css_content_content() -> Css {
    css!("
        flex-grow: 1;
        padding: 5px;
        overflow-y: scroll;

        display: flex;
    ")
}

pub fn render(state: &Computed<State>) -> VDomElement {

    let state_value = state.get_value();

    let on_keydown = move |event: KeyDownEvent| -> bool {
        state_value.keydown(event.code)
    };

    html! {
        <div id="root" css={css_wrapper()} on_key_down={on_keydown}>
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
            <component {render_menu} data={state.clone()} />
            <component {render_header} data={state.clone()} />
            <div css={css_content()}>
                <div css={css_content_list()}>
                    <component {render_list} data={state.clone()} />
                </div>
                <div css={css_content_content()}>
                    <component {render_content} data={state.clone()} />
                </div>
            </div>
        </div>
    }
}
