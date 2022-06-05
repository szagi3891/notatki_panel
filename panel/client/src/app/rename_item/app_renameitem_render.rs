use vertigo::{Css, VDomComponent};
use vertigo::{css, html, bind};

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

fn render_input(state: &AppRenameitem) -> VDomComponent {
    VDomComponent::from_ref(state, |state| {
        let content = state.new_name.get();

        let on_input = bind(state).call_param(|state, new_value: String| {
            state.on_input(new_value);
        });

        html! {
            <input css={css_input()} on_input={on_input} value={content} autofocus="" />
        }
    })
}

fn render_textarea(state: &AppRenameitem) -> VDomComponent {
    VDomComponent::from_ref(state, |state| {
        let mut full_path = state.path.clone();
        full_path.push(state.prev_name.clone());
        let content = state.app.data.git.get_content(&full_path);

        match content {
            Some(ContentView { content, .. }) => {
                let text = content.as_str();
                html! {
                    <textarea css={css_textarea()} readonly="readonly" value={text} />
                }
            },
            None => {
                html!{
                    <div/>
                }
            }
        }
    })
}

fn render_path(state: &AppRenameitem) -> VDomComponent {
    VDomComponent::from_ref(state, |state| {
        let path = state.get_full_path();

        html! {
            <div css={css_header()}>
                "zmiana nazwy => "
                {path}
            </div>
        }
    })
}

pub fn app_renameitem_render(state: &AppRenameitem) -> VDomComponent {

    let view_path = render_path(state);
    let view_input = render_input(state);
    let view_textarea = render_textarea(state);
    let button_back = state.button_on_back();
    let button_save = state.button_on_save();

    VDomComponent::from_html(
        html! {
            <div css={css_wrapper()}>
                { view_path.clone() }
                <div css={css_header()}>
                    { button_back.clone() }
                    { button_save.clone() }
                </div>
                { view_input.clone() }
                { view_textarea.clone() }
            </div>
        }
    )
}