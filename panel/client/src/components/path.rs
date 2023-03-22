use std::rc::Rc;

use vertigo::{
    Css,
    Computed, dom, DomElement, bind, DomFragment,
};
use vertigo::{css};

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
    title: String,
    node_id: Vec<String>,
    create_css: fn(bool) -> Css,
    is_active: bool,
    on_click: Rc<dyn Fn(Vec<String>) + 'static>,
) -> DomElement {
    if is_active {
        let css = create_css(true);

        return dom! {
            <div css={css}>
                { title }
            </div>
        };
    }

    let on_click = bind!(on_click, node_id, || {
        on_click(node_id.clone());
    });

    let css = create_css(false);

    dom! {
        <div css={css} on_click={on_click}>
            { title }
        </div>
    }
}

pub fn render_path(path: &Computed<Vec<String>>, on_click: impl Fn(Vec<String>) + 'static) -> DomFragment {
    let path = path.clone();
    let on_click = Rc::new(on_click);

    path.render_value({
        move |current_path| {
            let all_items = current_path.len();

            let result = dom! {
                <div css={css_header()} />
            };

            let home = '\u{1F3E0}';         //ikonka domu

            let root_is_active = all_items == 0;
            result.add_child(create_link(format!("{home} root"), Vec::new(), css_item, root_is_active, on_click.clone()));

            let mut wsk_current_path = Vec::<String>::new();

            for (index, item) in current_path.iter().enumerate() {
                wsk_current_path.push(item.clone());

                let is_active = index == all_items - 1;
                result.add_child(create_link(item.clone(), wsk_current_path.clone(), css_item, is_active, on_click.clone()));
            }

            result
        }
    })
}

