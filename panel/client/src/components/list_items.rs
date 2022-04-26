use vertigo::{
    html, css, Css,
    VDomElement, Resource,
    bind
};
use crate::data::{Data, ListItem};
use crate::components::icon;


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

fn render_item(data: &Data, dir: &Vec<String>, current_item: &Option<String>, item: &ListItem, mouse_over_enable: bool) -> VDomElement {
    let on_click = {
        let mut path = dir.clone();
        path.push(item.name.clone());

        bind(&data.tab)
            .and(&item.dir)
            .and(&path)
            .call(|tab, is_dir, path| {
                if *is_dir {
                    tab.redirect_to_dir(&path);
                } else {
                    tab.redirect_to_file(&path);
                } 
            })
    };

    let is_select = {
        if let Some(list_pointer) = current_item.as_ref() {
            item.name == *list_pointer
        } else {
            false
        }
    };

    let mouse_over_enter = {
        bind(&item.name)
            .and(&mouse_over_enable)
            .and(&data.tab)
            .call(|item_name, mouse_over_enable, tab| {
                if *mouse_over_enable {
                    tab.set_item_select(item_name);
                }
            })
    };

    if is_select {
        html!{
            <div
                on_click={on_click}
                css={css_normal(is_select)}
                dom_ref="active"
                on_mouse_enter={mouse_over_enter}
            >
                {icon_arrow(is_select)}
                {icon::icon_render(item.dir)}
                <span css={label_css(item.prirority())}>
                    {remove_prefix(&item.name)}
                </span>
            </div>
        }
    } else {
        html!{
            <div
                on_click={on_click}
                css={css_normal(is_select)}
                on_mouse_enter={mouse_over_enter}
            >
                {icon_arrow(is_select)}
                {icon::icon_render(item.dir)}
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
    ")
}

pub fn list_items(data: &Data, dir: &Vec<String>, current_item: &Option<String>, mouse_over_enable: bool) -> Vec<VDomElement> {
    let current = data.git.dir_list(dir);

    let list = match current {
        Resource::Ready(list) => list.get_list(),
        _ => {
            return Vec::new();
        }
    };

    let mut out: Vec<VDomElement> = Vec::new();
    let mut picture: Vec<VDomElement> = Vec::new();

    for item in (*list).iter() {
        if mouse_over_enable {
            out.push(render_item(data, dir, current_item, item, mouse_over_enable));
        } else {
            if let Some(ext) = item.get_picture_ext() {
                let id = item.id.clone();
                let url = format!("/image/{id}/{ext}");

                picture.push(html! {
                    <img
                        css={css_image()}
                        src={url}
                    />
                });
            } else {
                out.push(render_item(data, dir, current_item, item, mouse_over_enable));
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