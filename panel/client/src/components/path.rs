use std::rc::Rc;

use vertigo::{
    VDomElement,
    Css,
    bind, VDomComponent, Value,
};
use vertigo::{html, css};

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
) -> VDomElement {
    if is_active {
        let css = create_css(true);

        return html! {
            <div css={css}>
                { title }
            </div>
        };
    }

    let on_click = bind(&on_click)
        .and(&node_id)
        .call(|on_click, node_id| {
            on_click(node_id.clone());
        });

    let css = create_css(false);

    html! {
        <div css={css} on_click={on_click}>
            { title }
        </div>
    }
}

pub fn render_path(path: &Value<Vec<String>>, on_click: impl Fn(Vec<String>) + 'static) -> VDomComponent {
    let path = path.clone();
    let on_click = Rc::new(on_click);

    VDomComponent::from_fn(move || {
        let current_path = path.get();
        let all_items = current_path.len();

        let mut out: Vec<VDomElement> = Vec::new();

        let root_is_active = all_items == 0;
        out.push(create_link("root".into(), Vec::new(), css_item, root_is_active, on_click.clone()));

        let mut wsk_current_path = Vec::<String>::new();

        for (index, item) in current_path.iter().enumerate() {
            wsk_current_path.push(item.clone());

            let is_active = index == all_items - 1;
            out.push(create_link(item.clone(), wsk_current_path.clone(), css_item, is_active, on_click.clone()));
        }

        html! {
            <div css={css_header()}>
                { ..out }
            </div>
        }
    })
}

