use vertigo::{Css, Computed, dom, DomNode};
use vertigo::{css, bind};

use super::AppRenameitem;
use crate::data::ContentView;

fn css_wrapper() -> Css {
    css!("
        display: flex;
        flex-direction: column;
        border: 1px solid black;
        background-color: #e0e0e0;
        width: 100vw;
        height: 100vh;
    ")
}

fn css_header() -> Css {
    css!("
        display: flex;
        border-bottom: 1px solid black;
        padding: 5px;
    ")
}

fn css_input() -> Css {
    css!("
        border: 0;
        padding: 5px;
        margin: 5px;
        border: 1px solid blue;
        :focus {
            border: 0;
        }
    ")
}

fn css_textarea_wrapper() -> Css {
    css!("
        display: flex;
        flex-grow: 1;
    ")
}

fn css_textarea() -> Css {
    css!("
        flex-grow: 1;
        border: 0;
        padding: 5px;
        margin: 5px;
        border: 1px solid blue;
        background: #e0e0e010;
        :focus {
            border: 0;
        }
    ")
}

fn render_input(state: &AppRenameitem) -> DomNode {
    let state = state.clone();

    let content = Computed::from({
        let state = state.clone();
        move |context| state.new_name.get(context)
    });

    let on_input = bind!(state, |new_value: String| {
        state.on_input(new_value);
    });

    dom! {
        <input css={css_input()} on_input={on_input} value={content} autofocus="" />
    }
}

fn render_textarea(state: &AppRenameitem) -> DomNode {
    let state = state.clone();

    let content_computed = Computed::from(move |contetx| {
        state.item.get_content(contetx)
    });

    let render = content_computed.render_value_option(|content_inner| {
        match content_inner {
            Some(ContentView { content, .. }) => {
                let text = (*content).clone();

                Some(dom! {
                    <textarea css={css_textarea()} readonly="readonly" value={text} />
                })
            },
            None => None,
        }
    });

    dom! {
        <div css={css_textarea_wrapper()}>
            {render}
        </div>
    }
}

fn render_path(state: &AppRenameitem) -> DomNode {
    let path = state.item.to_string_path();

    dom! {
        <div css={css_header()}>
            "zmiana nazwy => "
            <text computed={path} />
        </div>
    }
}

pub fn app_renameitem_render(state: &AppRenameitem) -> DomNode {

    let view_path = render_path(state);
    let view_input = render_input(state);
    let view_textarea = render_textarea(state);
    let button_back = state.button_on_back();
    let button_save = state.button_on_save();

    dom! {
        <div css={css_wrapper()}>
            { view_path }
            <div css={css_header()}>
                { button_back }
                { button_save }
            </div>
            { view_input }
            { view_textarea }
        </div>
    }
}