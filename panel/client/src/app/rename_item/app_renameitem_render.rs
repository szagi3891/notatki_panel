use vertigo::{Css, VDomComponent, DomElement, create_node, Computed};
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

fn render_input(state: &AppRenameitem) -> DomElement {
    let state = state.clone();

    let content = Computed::from({
        let state = state.clone();
        move || state.new_name.get()
    });

    let on_input = bind(&state).call_param(|state, new_value: String| {
        state.on_input(new_value);
    });

    create_node("input")
        .css(css_input())
        .on_input(on_input)
        .attr_computed("value", content)
        .attr("autofocus", "")
    
    // VDomComponent::from_ref(state, |state| {
    //     let content = state.new_name.get();

    //     let on_input = bind(state).call_param(|state, new_value: String| {
    //         state.on_input(new_value);
    //     });

    //     html! {
    //         <input css={css_input()} on_input={on_input} value={content} autofocus="" />
    //     }
    // })
}

fn render_textarea(state: &AppRenameitem) -> DomElement {
    let state = state.clone();

    let content_computed = Computed::from(move || {
        let mut full_path = state.path.clone();
        full_path.push(state.prev_name.clone());
        state.app.data.git.get_content(&full_path)
    });

    create_node("div")
        .value(content_computed, |content_inner| {
            match content_inner {
                Some(ContentView { content, .. }) => {
                    let text = (*content).clone();

                    create_node("textarea")
                        .css(css_textarea())
                        .attr("readonly", "readonly")
                        .attr("value", text)
                    // html! {
                    //     <textarea css={css_textarea()} readonly="readonly" value={text} />
                    // }
                },
                None => {
                    create_node("div")
                    // html!{
                    //     <div/>
                    // }
                }
            }
        })

    // VDomComponent::from_ref(state, |state| {
    //     let mut full_path = state.path.clone();
    //     full_path.push(state.prev_name.clone());
    //     let content = state.app.data.git.get_content(&full_path);

    //     match content {
    //         Some(ContentView { content, .. }) => {
    //             let text = content.as_str();
    //             html! {
    //                 <textarea css={css_textarea()} readonly="readonly" value={text} />
    //             }
    //         },
    //         None => {
    //             html!{
    //                 <div/>
    //             }
    //         }
    //     }
    // })
}

fn render_path(state: &AppRenameitem) -> DomElement {
    let state = state.clone();
    let path = Computed::from(move || state.get_full_path());

    create_node("div")
        .css(css_header())
        .text("zmiana nazwy => ")
        .text_computed(path)
    // html! {
    //     <div css={css_header()}>
    //         "zmiana nazwy => "
    //         {path}
    //     </div>
    // }
}

pub fn app_renameitem_render(state: &AppRenameitem) -> VDomComponent {

    let view_path = render_path(state);
    let view_input = render_input(state);
    let view_textarea = render_textarea(state);
    let button_back = state.button_on_back();
    let button_save = state.button_on_save();

    VDomComponent::dom(
        create_node("div")
            .css(css_wrapper())
            .child(view_path)
            .child(
                create_node("div")
                .css(css_header())
                .child(button_back)
                .child(button_save)
            )
            .child(view_input)
            .child(view_textarea)
    )
}