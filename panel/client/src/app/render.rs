use vertigo::{
    VDomElement,
    Css,
    computed::{
        Computed,
    }
};

use vertigo_html::{html_component, Inline};

use super::state::State;

fn css_header() -> Css {
    Css::one("
        flex-shrink: 0;
        border-bottom: 1px solid black;
        line-height: 25px;
        padding: 0 5px;
    ")
}

fn css_empty_path() -> Css {
    Css::one("
        color: blue;
    ")
}

pub fn render_header(state: &Computed<State>) -> VDomElement {
    let state = state.get_value();
    let current_path = state.current_path.get_value();

    if current_path.len() == 0 {
        return html_component! {
            <div css={css_header()}>
                <div css={css_empty_path()}>
                    root
                </div>
            </div>
        };
    }

    let mut path_chunks: Vec<&str> = Vec::new();
    for path_item in current_path.iter() {
        path_chunks.push(path_item);
    }
    let path_for_view = path_chunks.join(" / ");

    html_component! {
        <div css={css_header()}>
            { path_for_view }
        </div>
    }
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
            <div css={css_content_content()} onClick={content_click}>
                content ...
            </div>
        </div>
    }
}

fn css_footer() -> Css {
    Css::one("
        flex-shrink: 0;
        line-height: 25px;
        padding: 0 5px;
    ")
}

pub fn render_footer(state: &Computed<State>) -> VDomElement {
    html_component! {
        <div css={css_footer()}>
            Lista plików które zostały zmodyfikowane ale nie zapisane
        </div>
    }
}

/*
    path    - cały wiersz
    files content   - dwie kolumny
    zmodyfikowane sciezki - stopka, pliki które są zmodyfikowane
*/

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

pub fn render(state: &Computed<State>) -> VDomElement {
    let reset: &str = "html, body {
        margin: 0;
        padding: 0;
        border: 0;
    }";

    html_component! {
        <div css={css_wrapper()}>
            <style>
                { reset }
            </style>
            <component {render_header} data={state.clone()} />
            <component {render_content} data={state.clone()} />
            <component {render_footer} data={state.clone()} />
        </div>
    }
}

