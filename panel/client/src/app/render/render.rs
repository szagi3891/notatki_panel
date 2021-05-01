use vertigo::{
    VDomElement,
    Css,
    computed::{
        Computed,
    }
};

use vertigo_html::{html, css};

use crate::app::state::State;
use super::render_header::render_header;
use super::render_list::render_list;
use super::render_content::render_content;
// use super::render_footer::render_footer;

fn css_wrapper() -> Css {
    css!("
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

fn css_content() -> Css {
    css!("
        flex-grow: 1;
        display: flex;
        border-bottom: 1px solid black;
    ")
}

fn css_content_list() -> Css {
    css!("
        width: 300px;
        flex-grow: 0;
        flex-shrink: 0;
        border-right: 1px solid black;
        padding: 0 3px;
    ")
}

fn css_content_content() -> Css {
    css!("
        flex-grow: 1;
        padding: 5px;
    ")
}


//TODO ...
//             <component {render_footer} data={state.clone()} />


pub fn render(state: &Computed<State>) -> VDomElement {
    //let state = state.get_value();

    //current_content

    html! {"
        <div css={css_wrapper()}>
            <style>
                html, body {
                    margin: 0;
                    padding: 0;
                    border: 0;
                }
            </style>
            <component {render_header} data={state.clone()} />
            <div css={css_content()}>
                <div css={css_content_list()}>
                    <component {render_list} data={state.clone()} />
                </div>
                <div css={css_content_content()}>
                    <component {render_content} data={state.clone()} />
                </div>
            </div>
            <div>TODO - footer</div>
        </div>
    "}
}
