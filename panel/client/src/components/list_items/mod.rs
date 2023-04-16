use std::rc::Rc;

use vertigo::{
    css, Css,
    Resource,
    bind, DomElement, dom, Computed, DomNode, dom_element
};
use crate::data::{Data, ListItem};
use crate::components::icon;


fn css_normal(is_select: bool, is_hover: bool) -> Css {
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
    } else if is_hover {
        return css.push_str("
            background-color: #03fc7740;
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

fn icon_arrow_render(show: bool) -> DomNode {
    if show {
        dom! {
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
        // create_node("div")
        //     .css(icon_arrow_wrapper())
        dom! {
            <div css={icon_arrow_wrapper()}></div>
        }
    }
}

fn icon_arrow(show: Computed<bool>) -> DomNode {
    show.render_value(|show| {
        icon_arrow_render(show)
    })
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
        color: red;
    ")
}

pub fn item_dot_html(on_click: impl Fn() + 'static) -> DomNode {
    dom!{
        <div
            on_click={on_click}
            css={css_normal(false, false)}
        >
            {icon_arrow_render(false)}
            {icon::icon_dir()}
            <span css={label_css(1)}>
                ".."
            </span>
        </div>
    }
}

pub fn item_default(data: &Data, item: &ListItem, on_click: Computed<Rc<dyn Fn() + 'static>>) -> DomElement {
    let css_wrapper = Computed::from({
        let data = data.clone();
        let item = item.clone();
        move |context| {
            let Some(current_item) = data.tab.select_content.get(context) else {
                return css!("");
            };

            let current_hover = data.tab.get_hover(context);

            let is_select = item.name() == current_item.name();

            let is_hover = if let Some(hover) = &current_hover {
                *hover == item.name()
            } else {
                false
            };

            css_normal(is_select, is_hover)
        }
    });

    let is_select = Computed::from({
        let data = data.clone();
        let item = item.clone();
        move |context| {
            if let Some(current_item) = data.tab.select_content.get(context) {
                item.name() == current_item.name()
            } else {
                false
            }
        }
    });

    dom_element!{
        <div
            on_click={on_click}
            css={css_wrapper}
        >
            {icon_arrow(is_select)}
            {icon::icon_render(item)}
            <span css={label_css(item.prirority())}>
                {item.name_without_prefix()}
            </span>
        </div>
    }
}


pub fn item_default_render(data: &Data, item: &ListItem, mouse_over_enable: bool) -> DomElement {
    let data = data.clone();
    let item = item.clone();

    let tab = &data.tab;

    let on_click = tab.build_redirect_to_item(item.clone());

    let element = item_default(&data, &item, on_click);

    let element = if mouse_over_enable {

        let mouse_over_enter = bind!(item, tab, || {
            tab.hover_on(&item.name());
        });

        let mouse_over_leave = bind!(item, tab, || {
            tab.hover_off(item.name().as_str());
        });

        element
            .on_mouse_enter(mouse_over_enter)
            .on_mouse_leave(mouse_over_leave)
    } else {
        element
    };

    element
}

fn css_image() -> Css {
    css!("
        width: 100px;
        margin: 5px;
        border:1px solid black;
        padding: 1px;
        cursor: pointer;
    ")
}

fn item_image_render(data: &Data, item: &ListItem, ext: &String) -> DomNode {
    let data = data.clone();
    let item = item.clone();

    let url = Computed::from(bind!(item, ext, |context| {
        let id = item.id.get(context);

        match id {
            Resource::Ready(id) => Some(format!("/image/{id}/{ext}")),
            _ => None
        }
    }));

    let tab = &data.tab;
    let on_click = tab.build_redirect_to_item(item);

    url.render_value(bind!(on_click, |url| {
        match url {
            Some(url) => {
                dom!{
                    <img
                        css={css_image()}
                        src={url}
                        on_click={on_click.clone()}
                    />
                }
            },
            None => {
                dom! { <span /> }
            }
        }
    }))

}

pub fn list_items_from_vec(data: &Data, list: Computed<Vec<ListItem>>, mouse_over_enable: bool) -> DomNode {
    let list_sorted = Computed::from({
        move |context| {
            let list = list.get(context);

            let mut out: Vec<(Option<String>, ListItem)> = Vec::new();
            let mut picture: Vec<(Option<String>, ListItem)> = Vec::new();
        
            for item in list.into_iter() {
                if mouse_over_enable {
                    out.push((None, item));
                } else if let Some(ext) = item.get_picture_ext() {
                    picture.push((Some(ext), item));
                } else {
                    out.push((None, item));
                }
            }
        
            out.extend(picture.into_iter());

            out
        }
    });

    list_sorted.render_list(
        |(_, item)| item.to_string_path(),
        {
            let data = data.clone();
            move |(picture, item)| {

                if mouse_over_enable {
                    item_default_render(&data, item, mouse_over_enable).into()
                } else if let Some(ext) = picture {
                    item_image_render(&data, item, ext)
                } else {
                    item_default_render(&data, item, mouse_over_enable).into()
                }
            }
        }
    )
}

pub fn list_items_from_dir(data: &Data, select_dir: &Computed<ListItem>, mouse_over_enable: bool) -> DomNode {
    let list = Computed::from({
        let select_dir = select_dir.clone();
        move |context| {
            let select_dir = select_dir.get(context);
            let current = select_dir.list.get(context);

            match current {
                Resource::Ready(list) => list,
                _ => {
                    return Vec::new();
                }
            }
        }
    });

    list_items_from_vec(data, list, mouse_over_enable)
}

