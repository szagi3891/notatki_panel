use std::rc::Rc;

use vertigo::{
    css, Css,
    dom, Computed, DomNode, dom_element, component
};
use crate::data::{Data, ListItem, ListItemType, RouterValue};
use crate::components::icon;

fn css_normal(is_select: bool, is_hover: bool, is_todo: bool) -> Css {
    let css = css!("
        display: flex;
        border-bottom: 1px solid #c0c0c0;
        padding: 3px 0;
        text-decoration: none;
        color: black;

        cursor: pointer;

        :visited {
            text-decoration: none;
        }
    ");

    if is_select {
        return css.push_str("
            background-color: #c0c0c0;
        ");
    }
    
    if is_hover {
        return css.push_str("
            background-color: #03fc7740;
        ");
    }

    if is_todo {
        return css.extend(css!("
            background: #00ff0080;
        "));
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

fn label_css(is_todo: bool, prirority: bool) -> Css {
    if is_todo {
        return css!("
            padding-left: 3px;
            color: red;
        ");
    }

    if prirority {
        return css!("
            padding-left: 3px;
            color: green;
        ");
    }

    css!("
        padding-left: 3px;
    ")
}

pub enum ItemDefaultOnClick {
    Link(Computed<RouterValue>),
    Click(Computed<Rc<dyn Fn() + 'static>>)
}

impl From<Computed<RouterValue>> for ItemDefaultOnClick {
    fn from(value: Computed<RouterValue>) -> Self {
        ItemDefaultOnClick::Link(value)
    }
}

impl From<Computed<Rc<dyn Fn() + 'static>>> for ItemDefaultOnClick {
    fn from(value: Computed<Rc<dyn Fn() + 'static>>) -> Self {
        ItemDefaultOnClick::Click(value)
    }
}

#[component]
pub fn ItemDefault(
    data: Data,
    item: ListItem,
    on_click: ItemDefaultOnClick,
    mouse_over_enter: Option<Rc<dyn Fn()>>,
    mouse_over_leave: Option<Rc<dyn Fn()>>,
) {
    let css_wrapper = Computed::from({
        let data = data.clone();
        let item = item.clone();
        move |context| {
            let is_hover = Some(item.name()) == data.tab.select_content_hover.get(context).map(|item| item.name());
            let is_select = Some(item.name()) == data.tab.select_content_current.get(context).map(|item| item.name());

            css_normal(is_select, is_hover, item.is_todo())
        }
    });

    let is_select = Computed::from({
        let data = data.clone();
        let item = item.clone();
        move |context| {
            if let Some(current_item) = data.tab.select_content_current.get(context) {
                item.name() == current_item.name()
            } else {
                false
            }
        }
    });

    let name = Computed::from({
        let item = item.clone();

        move |context| {
            let name = item.name_without_prefix();
            let todo = data.items.todo_only.get(context);

            if todo {
                if ListItemType::Dir == item.is_dir.get(context) {
                    let todo = item.count_todo.get(context);
                    format!("{name} ({todo})")
                } else {
                    name
                }
            } else {
                name
            }
        }
    });

    let element = match on_click {
        ItemDefaultOnClick::Link(link) => {
            let link = link.map(|inner| format!("#{}", inner.to_string()));

            dom_element!{
                <a
                    href={link}
                    css={css_wrapper}
                >
                    {icon_arrow(is_select)}
                    {icon::icon_render(&item)}
                    <span css={label_css(item.is_todo(), item.prirority())}>
                        {name}
                    </span>
                </a>
            }
        }
        ItemDefaultOnClick::Click(on_click) => {
            dom_element!{
                <div
                    on_click={on_click}
                    css={css_wrapper}
                >
                    {icon_arrow(is_select)}
                    {icon::icon_render(&item)}
                    <span css={label_css(item.is_todo(), item.prirority())}>
                        {name}
                    </span>
                </div>
            }
        }
    };

    let element = if let Some(mouse_over_enter) = mouse_over_enter {
        element.on_mouse_enter(mouse_over_enter)
    } else {
        element
    };

    let element = if let Some(mouse_over_leave) = mouse_over_leave {
        element.on_mouse_leave(mouse_over_leave)
    } else {
        element
    };

    element
}


#[component]
pub fn ItemDotHtml(on_click: Rc<dyn Fn()>) {
    dom!{
        <div
            on_click={on_click}
            css={css_normal(false, false, false)}
        >
            {icon_arrow_render(false)}
            {icon::icon_dir()}
            <span css={label_css(false, false)}>
                ".."
            </span>
        </div>
    }
}
