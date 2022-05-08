use vertigo::{
    html, css, Css,
    VDomElement, Resource,
    bind, VDomComponent
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

fn render_item(data: &Data, dir: &Vec<String>, item: &ListItem, mouse_over_enable: bool) -> VDomElement {
    let current_item = data.tab.current_item.get();
    let current_hover = data.tab.item_hover.get();

    let is_select = {
        if let Some(list_pointer) = &current_item {
            item.name == *list_pointer
        } else {
            false
        }
    };

    let is_hover = {
        if let Some(hover) = &current_hover {
            *hover == item.name
        } else {
            false
        }
    };

    let on_click = {
        let mut path = dir.clone();
        path.push(item.name.clone());

        bind(&data.tab)
            .and(&item.is_dir)
            .and(&path)
            .call(|tab, is_dir, path| {
                if *is_dir {
                    tab.redirect_to_dir(&path);
                } else {
                    tab.redirect_to_file(&path);
                } 
            })
    };

    let mouse_over_enter = bind(&item.name)
        .and(&mouse_over_enable)
        .and(&data.tab)
        .call(|item_name, mouse_over_enable, tab| {
            if *mouse_over_enable {
                tab.hover_on(item_name);
            }
        });

    let mouse_over_leave = bind(&item.name)
        .and(&mouse_over_enable)
        .and(&data.tab)
        .call(|item_name, mouse_over_enable, tab| {
            if *mouse_over_enable {
                tab.hover_off(item_name);
            }
        });

    if is_select {
        html!{
            <div
                on_click={on_click}
                css={css_normal(is_select, is_hover)}
                dom_ref="active"
                on_mouse_enter={mouse_over_enter}
                on_mouse_leave={mouse_over_leave}
            >
                {icon_arrow(is_select)}
                {icon::icon_render(item.is_dir)}
                <span css={label_css(item.prirority())}>
                    {remove_prefix(&item.name)}
                </span>
            </div>
        }
    } else {
        html!{
            <div
                on_click={on_click}
                css={css_normal(is_select, is_hover)}
                on_mouse_enter={mouse_over_enter}
                on_mouse_leave={mouse_over_leave}
            >
                {icon_arrow(is_select)}
                {icon::icon_render(item.is_dir)}
                <span css={label_css(item.prirority())}>
                    {remove_prefix(&item.name)}
                </span>
            </div>
        }
    }
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

#[derive(Clone)]
struct ItemImage {
    data: Data,
    item: ListItem,
    ext: String,
}

impl ItemImage {
    pub fn component(data: &Data, item: &ListItem, ext: String, ) -> VDomComponent {

        let state = ItemImage {
            data: data.clone(),
            item: item.clone(),
            ext
        };

        VDomComponent::from(state, render_image_item)
    }
}

fn render_image_item(state: &ItemImage) -> VDomElement {
    let id = state.item.id.clone();
    let url = format!("/image/{id}/{ext}", ext = state.ext);

    let full_path = state.item.full_path();

    let on_click = bind(&full_path)
        .and(&state.data.tab)
        .call(|full_path, tab| {
            tab.redirect_to_file(full_path);
        });

    html!{
        <img
            css={css_image()}
            src={&url}
            on_click={on_click}
        />
    }
}

pub fn list_items(data: &Data, dir: &Vec<String>, mouse_over_enable: bool) -> Vec<VDomComponent> {
    let current = data.git.dir_list(dir);

    let list = match current {
        Resource::Ready(list) => list.get_list(),
        _ => {
            return Vec::new();
        }
    };

    let mut out: Vec<VDomComponent> = Vec::new();
    let mut picture: Vec<VDomComponent> = Vec::new();

    for item in list.iter() {
        if mouse_over_enable {
            let data = data.clone();
            let dir = dir.clone();
            let item = item.clone();

            out.push(VDomComponent::from_fn(move || {
                render_item(&data, &dir, &item, mouse_over_enable)
            }));

        } else {
            if let Some(ext) = item.get_picture_ext() {
                picture.push(ItemImage::component(data, item, ext));
            } else {
                let data = data.clone();
                let dir = dir.clone();
                let item = item.clone();

                out.push(VDomComponent::from_fn(move || {
                    render_item(&data, &dir, &item, mouse_over_enable)
                }));
            }
        }
    }

    out.extend(picture.into_iter());

    out
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