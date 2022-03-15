use vertigo::{
    Css,
    VDomComponent,
};

use vertigo::{html, css};

use crate::app::index::app_index_render_menu::AppIndexMenuState;

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

pub fn app_index_render(view_alert: VDomComponent, state_value: AppIndex) -> VDomComponent {
    let view_menu = AppIndexMenuState::component(&state_value);
    let view_header = VDomComponent::new(state_value.clone(), render_header);
    let view_list = VDomComponent::new(state_value.clone(), render_list);
    let view_content = VDomComponent::new(state_value.clone(), render_content);

    VDomComponent::new(state_value, move |state_value: &AppIndex| {
        let hook_keydown = {
            let state = state_value.clone();
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
