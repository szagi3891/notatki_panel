use std::rc::Rc;
use vertigo::{
    VDomElement,
    Css,
    computed::{
        Computed,
    }
};
use vertigo_html::{html, css};
use crate::app::state::State;

fn css_header() -> Css {
    css!("
        flex-shrink: 0;
        display: flex;
        border-bottom: 1px solid black;
        line-height: 25px;
        padding: 0 5px;
        
    ")
}

fn css_separator() -> Css {
    css!("
        margin: 0 5px;
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
    let css = css!("
        color: blue;
    ");

    let css = css.push_str(css_active(is_active));

    css
}

fn css_item(is_active: bool) -> Css {
    let css = css!("
        color: black;
    ");

    let css = css.push_str(css_active(is_active));

    css
}

//let title = state.node_title(&node_id);

// let title: Rc<String> = match title {
//     Some(title) => title.clone(),
//     None => Rc::new("loading ...".into())
// };

// let title: String = match title {
//     Some(title) => (&*title.clone()).clone(),
//     None => "loading ...".into()
// };

fn create_link(state: &Rc<State>, title: String, node_id: Vec<String>, create_css: fn(bool) -> Css, is_active: bool) -> VDomElement {
    if is_active {
        let css = create_css(true);

        return html! {"
            <div css={css}>
                { title }
            </div>
        "};
    }

    let on_click = {
        let state = state.clone();
        let node_id = node_id.clone();
        move || {
            state.set_path(node_id.clone());
        }
    };

    let css = create_css(false);

    html! {"
        <div css={css} onClick={on_click}>
            { title }
        </div>
    "}
}

pub fn render_header(state: &Computed<State>) -> VDomElement {
    let state = state.get_value();

    let current_path = state.current_path.get_value();
    let all_items = current_path.len();

    let mut out: Vec<VDomElement> = Vec::new();

    let root_is_active = all_items == 0;
    out.push(create_link(&state, "root".into(), Vec::new(), css_root, root_is_active));

    let mut wsk_current_path = Vec::<String>::new();

    for (index, item) in current_path.iter().enumerate() {
        wsk_current_path.push(item.clone());

        let is_active = index == all_items - 1;
        out.push(html!{"<span css={css_separator()}>-</span>"});
        out.push(create_link(&state, item.clone(), wsk_current_path.clone(), css_item, is_active));
    }

    html! {"
        <div css={css_header()}>
            { ..out }
        </div>
    "}
}
