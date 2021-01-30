use vertigo::{
    VDomElement,
    node_attr,
    Css,
    computed::{
        Computed,
    }
};
use vertigo_html::{Inline, html_component, html_element};

use super::state::State;

const GLOBAL_RESET: &'static str = "html, body {
    margin: 0;
    padding: 0;
    border: 0;
}";

fn css_wrapper() -> Css {
    Css::one("
        display: flex;
        flex-direction: column;
        border: 1px solid black;
        background-color: #e0e0e0;
        padding: 0;
        width: 100vw;
        height: 100vh;
        box-sizing: border-box;
    ")
}

fn css_header() -> Css {
    Css::one("
        flex-shrink: 0;
        border-bottom: 1px solid black;
        line-height: 25px;
        padding: 0 5px;
    ")
}

pub fn render_header(state: &Computed<State>) -> VDomElement {
    let state = state.get_value();
    let current_path = state.current_path.get_value();

    let mut path_chunks: Vec<&str> = Vec::new();
    for path_item in current_path.iter() {
        path_chunks.push(path_item);
    }
    let path_for_view = path_chunks.join(" / ");

    use node_attr::{build_node, text, css};
    build_node("div", vec!(
        css(css_header()),
        text(path_for_view)
    ))
}

fn css_content() -> Css {
    Css::one("
        flex-grow: 1;
        display: flex;
        border-bottom: 1px solid black;
    ")
}

fn css_content_list() -> Css {
    Css::one("
        flex-grow: 1;
        border-right: 1px solid black;
        padding: 5px;
    ")
}

fn css_content_content() -> Css {
    Css::one("
        flex-grow: 1;
        padding: 5px;
    ")
}

pub fn render_content(state: &Computed<State>) -> VDomElement {
    // use node_attr::{build_node, text, css, node, on_click};

    let content_click = {
        let state = state.get_value();
        move || {
            state.push_path();
        }
    };

    html_component! {
        <div css={css_content()}>
            <div css={css_content_list()}>
                lista plikow
            </div>
            <div css={css_content_content()} on_click={content_click}>
                content ...
            </div>
        </div>
    }

    // build_node("div", vec!(
    //     css(css_content()),
    //     node("div", vec!(
    //         css(css_content_list()),
    //         text("lista plikow")
    //     )),
    //     node("div", vec!(
    //         css(css_content_content()),
    //         on_click(content_click),
    //         text("content ...")
    //     )),
    // ))
}

fn css_footer() -> Css {
    Css::one("
        flex-shrink: 0;
        line-height: 25px;
        padding: 0 5px;
    ")
}

pub fn render_footer(state: &Computed<State>) -> VDomElement {
    use node_attr::{build_node, text, css, node};
    build_node("div", vec!(
        css(css_footer()),
        text("lista plików które zostały zmodyfikowane ale nie zapisane")
    ))
}

/*
    path    - cały wiersz
    files content   - dwie kolumny
    zmodyfikowane sciezki - stopka, pliki które są zmodyfikowane
*/

pub fn render(state: &Computed<State>) -> VDomElement {
    use node_attr::{build_node, text, css, node, component};
    build_node("div", vec!(
        node("style", vec!(
            text(GLOBAL_RESET)
        )),
        css(css_wrapper()),
        component(state.clone(), render_header),
        component(state.clone(), render_content),
        component(state.clone(), render_footer),
    ))
}
