use vertigo::{
    Css,
    VDomElement,
    Computed,
};

use vertigo::{html, css};

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

        display: flex;
    ")
}

fn css_left() -> Css {
    css!("
        position: relative;
        overflow: hidden;
        flex-grow:1;
    ")
}

fn css_iframe(active: bool) -> Css {
    let style = css!("
        position: absolute;
        top: 0;
        right: 0;
        bottom: 0;
        left: 0;
        overflow-y: scroll;

        width: 100%;
        height: 100%;
        padding: 0;
        margin: 0;
        border: 0;
    ");

    let style = if active {
        style
    } else {
        style.push_str("visibility: hidden;")
    };

    style
}

fn css_right() -> Css {
    css!("
        width: 200px;
        flex-shrink: 0;
        border-left: 1px solid black;
    ")
}

fn css_button(active: bool) -> Css {
    let css = css!("
        line-height: 30px;
        padding: 0 5px;
        cursor: pointer;
        word-break: break-word;
    ");

    if active {
        css.push_str("
            background: red;
            color: white;
        ")
    } else {
        css.push_str("
            background: #e0e0e0;
            color: black;
        ")
    }
}

fn button(
    label: impl Into<String>,
    on_click: impl Fn() + 'static,
    on_close: Option<impl Fn() + 'static>,
    active: bool
) -> VDomElement {
    let label: String = label.into();

    let close = match on_close {
        Some(on_close) => html! {
            <div on_click={on_close}>
                "x"
            </div>
        },
        None => html! { <div/> }
    };

    html!{
        <div on_click={on_click} css={css_button(active)}>
            { label }
            { close }
        </div>
    }
}

pub fn render(state: &Computed<AppIndexState>) -> VDomElement {

    let app_index_state = state.get_value();

    let active = app_index_state.tabs_active.get_value();
    let tabs = app_index_state.tabs_url.get_value();

    if tabs.len() > 0 {
        let mut tabs_iframe = Vec::new();
        let mut tabs_menu = Vec::new();

    
        let is_select_default = active.is_none();

        tabs_menu.push({
            let app_index_state = app_index_state.clone();
            let on_click = move || {
                app_index_state.tabs_default();
            };

            button("default", on_click, None::<fn()>, is_select_default)
        });

        if is_select_default {
            tabs_iframe.push(html! {
                <div css={css_iframe(true)}>
                    <component {render_index} data={state.clone()} />
                </div>
            });
        }

        for tab_item in tabs.iter() {
            let tab_item = tab_item.clone();

            let is_select = match active.as_ref() {
                Some(active) => *active == *tab_item,
                None => false,
            };

            tabs_iframe.push(html! {
                <iframe src={tab_item.clone()} css={css_iframe(is_select)} />
            });

            let on_click = {
                let app_index_state = app_index_state.clone();
                let tab_item = tab_item.clone();
    
                move || {
                    app_index_state.tabs_set(tab_item.clone());
                }
            };

            let on_close = {
                let app_index_state = app_index_state.clone();
                let tab_item = tab_item.clone();
                move || {
                    app_index_state.tabs_remove(tab_item.clone());
                }
            };
    
            tabs_menu.push(button(tab_item, on_click, Some(on_close), is_select));
        }

        return html! {
            <div css={css_iframe_bg()}>
                <div css={css_left()}>
                    { ..tabs_iframe }
                </div>
                <div css={css_right()}>
                    { ..tabs_menu }
                </div>
            </div>
        };
    }

    html! {
        <div>
            <component {render_index} data={state.clone()} />
        </div>
    }
}