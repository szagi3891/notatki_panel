use vertigo::{
    Css,
    VDomElement,
    computed::{
        Computed,
    }
};

use vertigo_html::{html, css};

use super::state::AppIndexState;

use super::render_list::render_list;
use super::render_header::render_header;
use super::render_content::render_content;
use super::render_menu::render_menu;
use super::alert::render_alert;

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
        background-color: #e8e8e8;
    ")
    //font-size: 20px;
}

pub fn render_index(state: &Computed<AppIndexState>) -> VDomElement {

    let state_value = state.get_value();

    let alert = state_value.alert.clone();

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
            <component {render_alert} data={alert} />
        </div>
    }
}


fn css_iframe_bg() -> Css {
    css!("
        position: fixed;
        left: 0;
        top: 0;
        right: 0;
        bottom: 0;
    ")
}

fn css_iframe() -> Css {
    css!("
        position: absolute;
        overflow-y: scroll;
        width: 90%;
        height: 96%;
        margin-top: 2%;
        margin-left: 5%;
    ")
}

fn css_iframe_close() -> Css {
    css!("
        position: absolute;
        top: 0;
        right: 0;
        height: 30px;

        background: red;
        color: white;
        line-height: 30px;
        padding: 0 20px;
        cursor: pointer;
    ")
}

pub fn render(state: &Computed<AppIndexState>) -> VDomElement {

    let app_index_state = state.get_value();

    let tabs = app_index_state.tabs_url.get_value();
    if let Some(src) = tabs.first() {
        let on_click = {
            let src = src.clone();

            move || {
                app_index_state.tabs_remove(src.clone());
            }
        };

        html! {
            <div css={css_iframe_bg()}>
                <iframe src={src} css={css_iframe()} />
                <div on_click={on_click} css={css_iframe_close()}>
                    "close"
                </div>
            </div>
        }
    } else {
        html! {
            <div>
                <component {render_index} data={state.clone()} />
            </div>
        }
    }
}