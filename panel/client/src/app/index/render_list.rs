use vertigo::{
    Css,
    NodeRefs,
    VDomElement,
    computed::{
        Computed,
    }
};
use vertigo_html::{css, html};

use super::state::AppIndexState;

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
        color: #54aeff;
        fill: currentColor;
    ")
}

fn icon_dir() -> VDomElement {
    html! {
        <svg
            viewBox="0 0 16 16"
            css={icon_dir_css()}
        >
            <path
                fill-rule="evenodd"
                d="M1.75 1A1.75 1.75 0 000 2.75v10.5C0 14.216.784 15 1.75 15h12.5A1.75 1.75 0 0016 13.25v-8.5A1.75 1.75 0 0014.25 3h-6.5a.25.25 0 01-.2-.1l-.9-1.2c-.33-.44-.85-.7-1.4-.7h-3.5z"
            />
        </svg>
    }
}

fn icon_file_css() -> Css {
    css!("
        flex-shrink: 0;
        width: 16px;
        height: 16px;
        color: #57606a;
        fill: currentColor;
    ")
}

fn icon_file() -> VDomElement {
    html! {
        <svg
            viewBox="0 0 16 16"
            css={icon_file_css()}
        >
            <path
                fill-rule="evenodd" 
                d="M3.75 1.5a.25.25 0 00-.25.25v11.5c0 .138.112.25.25.25h8.5a.25.25 0 00.25-.25V6H9.75A1.75 1.75 0 018 4.25V1.5H3.75zm5.75.56v2.19c0 .138.112.25.25.25h2.19L9.5 2.06zM2 1.75C2 .784 2.784 0 3.75 0h5.086c.464 0 .909.184 1.237.513l3.414 3.414c.329.328.513.773.513 1.237v8.086A1.75 1.75 0 0112.25 15h-8.5A1.75 1.75 0 012 13.25V1.75z"
            />
        </svg>
    }
}

fn icon_render(dir: bool) -> VDomElement {
    if dir {
        icon_dir()
    } else {
        icon_file()
    }
}

fn label_css(prirority: u8) -> Css {
    if prirority == 2 {
        return css!("
            padding-left: 3px;
            color: green;
        ");
    }

    if prirority == 1 {
        return css!("
            padding-left: 3px;
        ");
    }

    css!("
        padding-left: 3px;
        opacity: 0.5;
        text-decoration: line-through;
    ")
}


//Koryguj tylko wtedy gdy element aktywny nie jest widoczny
fn dom_apply(node_refs: &NodeRefs) {

    if let (
        Some(wrapper),
        Some(active)
    ) = (
        node_refs.expect_one("wrapper"),
        node_refs.expect_one("active")
    ) {
        // let wrapper_rect = wrapper.get_bounding_client_rect();
        // let active_rect = active.get_bounding_client_rect();

        let active_rect_y = active.get_bounding_client_rect_y();
        let active_rect_height = active.get_bounding_client_rect_height();

        let wrapper_rect_y = wrapper.get_bounding_client_rect_y();
        let wrapper_rect_height = wrapper.get_bounding_client_rect_height();

        if active_rect_y < wrapper_rect_y {
            let offset = wrapper_rect_y - active_rect_y;

            let scroll_top = wrapper.scroll_top();
            wrapper.set_scroll_top(scroll_top - offset as i32);
            return;
        }

        let wrapper_y2 = wrapper_rect_y + wrapper_rect_height;
        let active_y2 = active_rect_y + active_rect_height;

        if active_y2 > wrapper_y2 {
            let offset = active_y2 - wrapper_y2;

            let scroll_top = wrapper.scroll_top();
            wrapper.set_scroll_top(scroll_top + offset as i32);
            return;
        }
    }
}

fn remove_first(chars: &[char]) -> &[char] {
    if let Some((name_item, rest_path)) = chars.split_first() {
        if *name_item == '_' {
            return rest_path;
        }
    }

    chars
}

fn remove_prefix(name: &String) -> String {
    let chars = name.chars().collect::<Vec<char>>();

    let chars = remove_first(&chars);
    let chars = remove_first(chars);

    let mut out: String = String::new();

    for char in chars {
        out.push(*char);
    }

    out
}

pub fn render_list(state: &Computed<AppIndexState>) -> VDomElement {
    
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

        if is_select {
            out.push(html!{
                <div on_click={on_click} css={css_normal(is_select)} dom_ref="active">
                    {icon_arrow(is_select)}
                    {icon_render(item.dir)}
                    <span css={label_css(item.prirority)}>{remove_prefix(&item.name)}</span>
                </div>
            });
        } else {
            out.push(html!{
                <div on_click={on_click} css={css_normal(is_select)}>
                    {icon_arrow(is_select)}
                    {icon_render(item.dir)}
                    <span css={label_css(item.prirority)}>{remove_prefix(&item.name)}</span>
                </div>
            });
        }
    }

    html! {
        <div css={css_wrapper()} dom_ref="wrapper" dom_apply={dom_apply}>
            { ..out }
        </div>
    }
}



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
