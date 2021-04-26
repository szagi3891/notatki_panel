use vertigo::{
    VDomElement,
    Css,
    computed::{
        Computed,
    }
};

use vertigo_html::{html, css};

use crate::app::state::State;
// use super::render_header::render_header;
// use super::render_list::render_list;
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
        flex-grow: 1;
        border-right: 1px solid black;
        padding: 5px;
    ")
}

fn css_content_content() -> Css {
    css!("
        flex-grow: 1;
        padding: 5px;
    ")
}

// pub fn render(state: &Computed<State>) -> VDomElement {
//     let reset: &str = "html, body {
//         margin: 0;
//         padding: 0;
//         border: 0;
//     }";

//     html! {"
//         <div css={css_wrapper()}>
//             <style>
//                 { reset }
//             </style>
//             <component {render_header} data={state.clone()} />
//             <div css={css_content()}>
//                 <div css={css_content_list()}>
//                     <component {render_list} data={state.clone()} />
//                 </div>
//                 <div css={css_content_content()}>
//                     content ...
//                 </div>
//             </div>
//             <component {render_footer} data={state.clone()} />
//         </div>
//     "}
// }


pub fn render(state: &Computed<State>) -> VDomElement {
    let reset: &str = "html, body {
        margin: 0;
        padding: 0;
        border: 0;
    }";

    let state = state.get_value();
    let current_hash = state.state_root.get_hash_view();


    html! {"
        <div css={css_wrapper()}>
            <style>
                { reset }
            </style>
            <div>TODO - header {current_hash}</div>
            <div css={css_content()}>
                <div css={css_content_list()}>
                    <div>TODO - list</div>
                </div>
                <div css={css_content_content()}>
                    content ...
                </div>
            </div>
            <div>TODO - footer</div>
        </div>
    "}
}
