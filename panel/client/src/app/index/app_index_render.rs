use vertigo::{
    Css,
    VDomElement, VDomComponent,
};

use vertigo::{html, css};

use crate::app::index::app_index_render_menu::AppIndexMenuState;
use crate::data::OpenLinks;

use super::AppIndex;

use super::app_index_render_list::render_list;
use super::app_index_render_header::render_header;
use super::app_index_render_content::render_content;

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

pub fn render_index(view_alert: VDomComponent, state_value: AppIndex) -> VDomComponent {
    let view_menu = AppIndexMenuState::component(&state_value);
    let view_header = VDomComponent::new(state_value.clone(), render_header);
    let view_list = VDomComponent::new(state_value.clone(), render_list);
    let view_content = VDomComponent::new(state_value.clone(), render_content);

    VDomComponent::new(state_value, move |_state_value: &AppIndex| {
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
                { view_menu.clone() }
                { view_header.clone() }
                <div css={css_content()}>
                    <div css={css_content_list()}>
                        { view_list.clone() }
                    </div>
                    <div css={css_content_content()}>
                        { view_content.clone() }
                    </div>
                </div>
                { view_alert.clone() }
            </div>
        }
    })
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

    if active {
        style
    } else {
        style.push_str("visibility: hidden;")
    }
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

pub fn open_links_render(open_links: OpenLinks, default_view: VDomComponent) -> VDomComponent {

    VDomComponent::new(open_links, move |open_links: &OpenLinks| {
        let active = open_links.tabs_active.get_value();
        let tabs = open_links.tabs_url.get_value();

        if tabs.len() > 0 {
            let mut tabs_iframe = Vec::new();
            let mut tabs_menu = Vec::new();

        
            let is_select_default = active.is_none();

            tabs_menu.push({
                let open_links = open_links.clone();
                let on_click = move || {
                    open_links.tabs_default();
                };

                button("default", on_click, None::<fn()>, is_select_default)
            });

            if is_select_default {
                tabs_iframe.push(html! {
                    <div css={css_iframe(true)}>
                        { default_view.clone() }
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
                    let open_links = open_links.clone();
                    let tab_item = tab_item.clone();
        
                    move || {
                        open_links.tabs_set(tab_item.clone());
                    }
                };

                let on_close = {
                    let open_links = open_links.clone();
                    let tab_item = tab_item.clone();
                    move || {
                        open_links.tabs_remove(tab_item.clone());
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
                { default_view.clone() }
            </div>
        }
    })
}

pub fn app_index_render(view_alert: VDomComponent, app_index_state: AppIndex) -> VDomComponent {
    let view_index = render_index(view_alert, app_index_state.clone());
    let open_links = app_index_state.data.tab.open_links.clone();

    open_links_render(open_links, view_index)
}