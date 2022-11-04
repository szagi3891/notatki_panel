use vertigo::{Css, bind, dom, DomElement, DomCommentCreate, transaction};
use vertigo::{css};

use super::AppEditcontent;
use super::app_editcontent_state::EditContent;
use crate::app::App;
use crate::components::button;

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

fn css_body() -> Css {
    css!("
        flex-grow: 1;
        border: 0;
        padding: 5px;
        margin: 5px;
        border: 1px solid blue;
        :focus {
            border: 0;
        }
    ")
}

fn render_textarea(state: &AppEditcontent) -> DomCommentCreate {
    let show_textarea = state.content_view.clone().map(|content| {
        if let Some(EditContent { ..}) = content {
            true
        } else {
            false
        }
    });

    let content = state.content_view.clone().map(|content| {
        if let Some(EditContent { content, ..}) = content {
            content
        } else {
            "".into()
        }
    });

    show_textarea.render_value({
        let state = state.clone();
        let content = content;

        move |show| {
            match show {
                true => {
                    let on_input = bind!(|state, new_value: String| {
                        transaction(|context| {
                            if let Some(EditContent { hash: Some(hash), content: _}) = state.content_view.get(context) {
                                state.on_input(context, new_value, hash);
                            } else {
                                log::warn!("Ignore on_input");
                            }
                        });
                    });
            
                    dom! {
                        <textarea css={css_body()} on_input={on_input} value={content.clone()} />
                    }
                },
                false => {
                    dom! {
                        <div>
                            "Ładowanie ..."
                        </div>
                    }
                }
            }
        }
    })
}

pub fn app_editcontent_render(app: &App, state: &AppEditcontent) -> DomElement {

    let view_textares = render_textarea(state);

    let path_view = {
        let path = state.path.as_slice().join("/");

        dom! {
            <div css={css_header()}>
                "Edycja pliku => "
                {path}
            </div>
        }
    };


    let button_save = state.save_enable.render_value_option({
        let app = app.clone();
        let state = state.clone();
        move |save_enabled| {
            match save_enabled {
                true => {
                    let on_save = state.on_save(&app, true);
                    Some(button("Zapisz", on_save))
                },
                false => {
                    None
                }
            }
        }
    });

    let button_save_and_stay = state.save_enable.render_value_option({
        let app = app.clone();
        let state = state.clone();
        move |save_enabled| {
            match save_enabled {
                true => {
                    let on_save = state.on_save(&app, false);
                    Some(button("Zapisz i zostań", on_save))
                },
                false => {
                    None
                }
            }
        }
    });

    let button_reset = state.save_enable.render_value_option({
        let state = state.clone();
        move |save_enabled| {
            match save_enabled {
                true => {
                    let on_reset = state.on_reset();
                    Some(button("Usuń naniesione zmiany", on_reset))
                },
                false => {
                    None
                }
            }
        }
    });

    let app = app.clone();

    let button_back = {
        let on_click = bind!(|app| {
            app.redirect_to_index();
        });
        button("Wróć", on_click)
    };

    dom! {
        <div css={css_wrapper()}>
            { path_view }
            <div css={css_header()}>
                { button_back }
                { button_save }
                { button_save_and_stay }
                { button_reset }
            </div>
            { view_textares }
        </div>
    }
}