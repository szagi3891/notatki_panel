use std::rc::Rc;

use vertigo::{
    Css,
    Computed, dom, bind, DomNode, dom_element,
};
use vertigo::{css};

use crate::data::ListItem;

fn css_header() -> Css {
    css!("
        flex-shrink: 0;
        display: flex;
        border-bottom: 1px solid black;
        line-height: 25px;
    ")
}

fn css_active(is_active: bool) -> &'static str {
    if is_active {
        "
        background-color: #51803660;
        cursor: default;
        "
    } else {
        "
        cursor: pointer;
        :hover {
            background-color: #518036a0;
        }
        "
    }
}

fn css_item(is_active: bool) -> Css {
    let css = css!("
        color: black;
        padding: 5px 10px;
        border-right: 1px solid black;
    ");

    let css = css.push_str(css_active(is_active));

    css
}

fn create_link(
    item: ListItem,
    create_css: fn(bool) -> Css,
    is_active: bool,
    on_click: Rc<dyn Fn(ListItem) + 'static>,
) -> DomNode {
    let title = Computed::from({
        let item = item.clone();

        move |context| {
            let title = if item.is_root() {
                let home = '\u{1F3E0}'; 
                format!("{home} root")
            } else {
                item.name()
            };

            let todo_only = item.todo_only.get(context);

            if todo_only {
                let count = item.count_todo.get(context);
                format!("{title} ({count})")
            } else {
                title
            }
        }
    });

    if is_active {
        let css = create_css(true);

        return dom! {
            <div css={css}>
                { title }
            </div>
        };
    }

    let on_click = bind!(on_click, item, || {
        on_click(item.clone());
    });

    let css = create_css(false);

    dom! {
        <div css={css} on_click={on_click}>
            { title }
        </div>
    }
}

pub fn render_path(path: &Computed<ListItem>, on_click: impl Fn(ListItem) + 'static) -> DomNode {
    let on_click = Rc::new(on_click);

    path.render_value({
        move |current_path| {
            let result = dom_element! {
                <div css={css_header()} />
            };

            for item in current_path.get_all_items() {
                let is_active = item == current_path;

                let link = create_link(item, css_item, is_active, on_click.clone());
                result.add_child(link);
            }

            result.into()
        }
    })
}

