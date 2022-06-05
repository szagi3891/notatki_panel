use vertigo::{
    Css,
    VDomComponent,
};

use vertigo::{html, css};

use crate::app::App;

use super::app_index_render_list::render_list;
use super::app_index_render_header::render_header;
use super::app_index_render_content::render_content;
use super::app_index_render_menu::MenuComponent;

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
    //font-size: 20px;
}

pub fn app_index_render(app: &App) -> VDomComponent {

    let view_alert = app.alert.render();
    let view_menu = MenuComponent::component(app);
    let view_header = VDomComponent::from_ref(app, render_header);
    let view_list = VDomComponent::from_ref(app, render_list);
    let view_content = VDomComponent::from_ref(app, render_content);

    VDomComponent::from_ref(app, move |app: &App| {
        let hook_keydown = {
            let state = app.clone();
            move |event: vertigo::KeyDownEvent| {
                state.keydown(event.code)
            }
        };

        html! {
            <div css={css_wrapper()} hook_key_down={hook_keydown}>
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
