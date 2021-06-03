use vertigo::{Css, NodeRefs, VDomElement, computed::{
        Computed,
    }};
use vertigo_html::{css, html};

use crate::state::StateViewIndex;

fn css_wrapper() -> Css {
    css!("
        flex-grow: 1;
        overflow-y: scroll;
    ")
}

fn css_normal(is_select: bool) -> Css {
    let css = css!("
        display: flex;
        border-bottom: 1px solid #c0c0c0;
        padding: 3px 0;

        cursor: pointer;
    ");

    if is_select {
        return css.push_str("
            color: green;
            background-color: #c0c0c0;
        ");
    }

    css
}

fn icon_arrow_wrapper() -> Css {
    css!("
        flex-shrink: 0;
        width: 8px;
        height: 16px;
        position: relative;
    ")
}

fn icon_wrapper_svg() -> Css {
    css!("
        flex-shrink: 0;
        width: 16px;
        height: 16px;
        position: relative;
        left: -4px;
    ")
}

//https://css.gg/play-button

fn icon_arrow(show: bool) -> VDomElement {
    if show {
        html! {
            <div css={icon_arrow_wrapper()}>
                <svg
                    width="24"
                    height="24"
                    viewBox="0 0 24 24"
                    fill="none"
                    xmlns="http://www.w3.org/2000/svg"
                    css={icon_wrapper_svg()}
                >
                    <path d="M15 12.3301L9 16.6603L9 8L15 12.3301Z" fill="currentColor" />
                </svg>
            </div>
        }
    } else {
        html! {
            <div css={icon_arrow_wrapper()}></div>
        }
    }
}

fn icon_dir_css() -> Css {
    css!("
        flex-shrink: 0;
        width: 16px;
        height: 16px;
    ")
}

fn icon_dir() -> VDomElement {
    html! {
        <svg
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
            css={icon_dir_css()}
        >
            <path
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M4 1.5C2.89543 1.5 2 2.39543 2 3.5V4.5C2 4.55666 2.00236 4.61278 2.00698 4.66825C0.838141 5.07811 0 6.19118 0 7.5V19.5C0 21.1569 1.34315 22.5 3 22.5H21C22.6569 22.5 24 21.1569 24 19.5V7.5C24 5.84315 22.6569 4.5 21 4.5H11.874C11.4299 2.77477 9.86384 1.5 8 1.5H4ZM9.73244 4.5C9.38663 3.9022 8.74028 3.5 8 3.5H4V4.5H9.73244ZM3 6.5C2.44772 6.5 2 6.94772 2 7.5V19.5C2 20.0523 2.44772 20.5 3 20.5H21C21.5523 20.5 22 20.0523 22 19.5V7.5C22 6.94772 21.5523 6.5 21 6.5H3Z"
                fill="currentColor"
            />
        </svg>
    }
}

fn icon_file_css() -> Css {
    css!("
        flex-shrink: 0;
        width: 16px;
        height: 16px;
    ")
}

fn icon_file() -> VDomElement {
    html! {
        <svg
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
            css={icon_file_css()}
        >
            <path
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M3 5C3 3.34315 4.34315 2 6 2H14C17.866 2 21 5.13401 21 9V19C21 20.6569 19.6569 22 18 22H6C4.34315 22 3 20.6569 3 19V5ZM13 4H6C5.44772 4 5 4.44772 5 5V19C5 19.5523 5.44772 20 6 20H18C18.5523 20 19 19.5523 19 19V9H13V4ZM18.584 7C17.9413 5.52906 16.6113 4.4271 15 4.10002V7H18.584Z"
                fill="currentColor"
            />
        </svg>
    }
}

fn label_css(green_label: bool) -> Css {
    let out = css!("
        padding-left: 3px;
    ");

    if green_label {
        return out.push_str("
            color: green;
        ");
    }

    out
}

pub fn render_list(state: &Computed<StateViewIndex>) -> VDomElement {
    
    let mut out: Vec<VDomElement> = Vec::new();

    let state = state.get_value();
    let list = state.list.get_value();
    let current_item = state.list_current_item.get_value();

    for item in (*list).iter() {
        let on_click = {
            let state = state.clone();
            let item = item.clone();

            move || {
                state.click_list_item(item.name.clone());
            }
        };

        let is_select = {
            if let Some(list_pointer) = current_item.as_ref() {
                item.name == *list_pointer
            } else {
                false
            }
        };

        let icon = if item.dir {
            icon_dir()
        } else {
            icon_file()
        };

        let green_label = item.name.get(0..1) == Some("_");

        if is_select {
            out.push(html!{
                <div onClick={on_click} css={css_normal(true)} dom_ref="active">
                    {icon_arrow(true)}
                    {icon}
                    <span css={label_css(green_label)}>{&item.name}</span>
                </div>
            });
        } else {
            out.push(html!{
                <div onClick={on_click} css={css_normal(false)}>
                    {icon_arrow(false)}
                    {icon}
                    <span css={label_css(green_label)}>{&item.name}</span>
                </div>
            });
        }
    }

    //Koryguj tylko wtedy gdy element aktywny nie jest widoczny
    let dom_apply = |node_refs: &NodeRefs| {

        if let (Some(wrapper), Some(active)) = (node_refs.expect_one("wrapper"), node_refs.expect_one("active")) {
            let wrapper_rect = wrapper.get_bounding_client_rect();
            let active_rect = active.get_bounding_client_rect();

            if active_rect.y < wrapper_rect.y {
                let offset = wrapper_rect.y- active_rect.y;

                let scroll_top = wrapper.scroll_top();
                wrapper.set_scroll_top(scroll_top - offset as i32);
                return;
            }

            let wrapper_y2 = wrapper_rect.y + wrapper_rect.height;
            let active_y2 = active_rect.y + active_rect.height;

            if active_y2 > wrapper_y2 {
                let offset = active_y2 - wrapper_y2;

                let scroll_top = wrapper.scroll_top();
                wrapper.set_scroll_top(scroll_top + offset as i32);
                return;
            }
        }
    };

    //Centrowanie na środku zawsze
    // let dom_apply = |node_refs: &NodeRefs| {

    //     if let (Some(wrapper), Some(active)) = (node_refs.expect_one("wrapper"), node_refs.expect_one("active")) {
    //         let wrapper_rect = wrapper.get_bounding_client_rect();
    //         let active_rect = active.get_bounding_client_rect();
    //         let scroll_top = wrapper.scroll_top();

    //         let active_offset_from_wrapper = active_rect.y as i32 + scroll_top - wrapper_rect.y as i32;
    //         let target_offset_from_wrapper = (wrapper_rect.height as i32 - active_rect.height as i32) / 2;

    //         let offset = active_offset_from_wrapper - target_offset_from_wrapper;

    //         wrapper.set_scroll_top(offset);
    //     }
    // };

    html! {
        <div css={css_wrapper()} dom_ref="wrapper" dom_apply={dom_apply}>
            { ..out }
        </div>
    }
}
