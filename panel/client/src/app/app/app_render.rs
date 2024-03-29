use vertigo::{Css, dom, DomNode};
use vertigo::{css, bind};

use crate::app::App;
use crate::components::render_path;
use crate::data::ListItem;

use super::app_render_list::render_list;
use super::app_render_content::render_content;
use super::app_render_menu::MenuComponent;

fn css_wrapper() -> Css {
    css!("
        display: flex;
        flex-direction: column;
        border: 1px solid black;
        background-color: #e0e0e0;
        padding: 0;
        box-sizing: border-box;

        height: 100vh;
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
        width: 100%;
        flex-grow: 1;
        padding: 5px;
        overflow-y: scroll;

        display: flex;
        background-color: #e8e8e8;
    ")
}

pub fn app_index_render(app: &App) -> DomNode {
    let view_alert = app.alert.render();

    let view_menu = MenuComponent::component(app);

    let data = &app.data;
    let on_click_path = bind!(data, |node_id: ListItem| {
        data.tab.set_path(node_id);
    });
    
    let view_header = render_path(&app.data.tab.select_dir, on_click_path);

    let view_list = render_list(app);
    let view_content = render_content(app);

    let hook_keydown = bind!(app, |event: vertigo::KeyDownEvent| {
        app.keydown(event.code)
    });

    dom! {
        <div css={css_wrapper()} hook_key_down={hook_keydown}>
            { view_menu }
            { view_header }
            <div css={css_content()}>
                <div css={css_content_list()}>
                    { view_list }
                </div>
                <div css={css_content_content()}>
                    { view_content }
                </div>
            </div>
            { view_alert }
        </div>
    }
}
