use std::sync::Arc;
use std::rc::Rc;
use vertigo::{
    VDomElement,
    Css,
    node_attr::NodeAttr,
    computed::{
        Computed,
    }
};
use vertigo_html::{html_component, Inline, html_element};
use super::state::State;

fn css_header() -> Css {
    Css::one("
        flex-shrink: 0;
        display: flex;
        border-bottom: 1px solid black;
        line-height: 25px;
        padding: 0 5px;
        
    ")
}

fn css_active(is_active: bool) -> &'static str {
    if is_active {
        "
        text-decoration: underline;
        cursor: default;
        "
    } else {
        "cursor: pointer;"
    }
}

fn css_root(is_active: bool) -> Css {
    let mut css = Css::one("
        color: blue;
        margin-right: 5px;
    ");

    css.str(css_active(is_active));

    css
}

fn css_item(is_active: bool) -> Css {
    let mut css = Css::one("
        color: black;
        margin-right: 5px;
    ");

    css.str(css_active(is_active));

    css
}

fn create_link(state: &Rc<State>, name: &str, create_css: fn(bool) -> Css, path: Vec<Arc<String>>, is_active: bool) -> NodeAttr {
    if is_active {
        let css = create_css(true);

        return html_element! {
            <div css={css}>
                { name }
            </div>
        };
    }

    let on_click = {
        let state = state.clone();
        move || {
            state.set_path(&path);
        }
    };

    let css = create_css(false);

    html_element! {
        <div css={css} onClick={on_click}>
            { name }
        </div>
    }
}

pub fn render_header(state: &Computed<State>) -> VDomElement {
    let state = state.get_value();

    let current_path = state.current_path.get_value();
    let all_items = current_path.len();

    let mut out: Vec<NodeAttr> = Vec::new();

    let root_is_active = all_items == 0;
    out.push(create_link(&state, "root", css_root, Vec::new(), root_is_active));


    let mut path_redirect = Vec::<Arc<String>>::new();

    for (index, item) in current_path.iter().enumerate() {
        path_redirect.push(item.clone());

        let is_active = index == all_items - 1;
        out.push(create_link(&state, item.as_str(), css_item, path_redirect.clone(), is_active));
    }

    html_component! {
        <div css={css_header()}>
            { ..out }
        </div>
    }
}
