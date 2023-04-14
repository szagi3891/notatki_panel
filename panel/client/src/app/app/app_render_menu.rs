use vertigo::{
    Css,
    Computed,
    bind, Resource, dom, transaction, DomNode,
};

use vertigo::{css};
use crate::app::App;
use crate::components::{ButtonState};
use crate::data::ContentType;

fn css_footer() -> Css {
    css!("
        display: flex;
        flex-shrink: 0;
        line-height: 25px;
        border-bottom: 1px solid black;
    ")
}

#[derive(Clone)]
pub struct MenuComponent {
    app: App,
    is_current_content: Computed<bool>,
}

impl MenuComponent {
    pub fn component(app: &App) -> DomNode {
        let is_current_content= Computed::from({
            let current_content = app.data.tab.current_content.clone();
            move |context| -> bool {
                if let Resource::Ready(content) = current_content.get(context) {
                    match content {
                        ContentType::Dir { list } => list.len() == 0,
                        _ => true
                    }
                } else {
                    false
                }
            }
        });

        let state = MenuComponent {
            app: app.clone(),
            is_current_content,
        };

        render_menu(&state)
    }
}

fn render_menu(state: &MenuComponent) -> DomNode {
    let button_edit_file = render_button_edit_file(state);
    let button_create_file = render_button_create_file(state);
    let button_rename_name = render_button_rename_name(state);
    let button_make_dir = render_button_make_dir(state);
    let button_delete = render_button_on_delete(state);
    let button_search = render_button_search(state);
    let button_move_item = render_button_move_item(state);
    let button_todo = render_button_todo(state);

    dom! {
        <div css={css_footer()}>
            { button_edit_file }
            { button_create_file }
            { button_rename_name }
            { button_make_dir }
            { button_delete }
            { button_search }
            { button_move_item }
            { button_todo}
        </div>
    }
}

fn render_button_on_delete(state: &MenuComponent) -> DomNode {
    ButtonState::render({
        let app = state.app.clone();
        let is_current_content = state.is_current_content.clone();

        Computed::from(move |context| {
            let is_current_content = is_current_content.get(context);

            if is_current_content {
                let alert = app.alert.clone();
                let on_delete = bind!(alert, app, || {
                    let path = transaction(|context| alert.data.tab.full_path.get(context));
                    alert.delete(app.clone(), path);
                });
        
                ButtonState::active("Usuń", on_delete)
            } else {
                ButtonState::disabled("Usuń")
            }
        })
    })

}

fn render_button_edit_file(state: &MenuComponent) -> DomNode {
    ButtonState::render({
        let app = state.app.clone();
        let is_current_content = state.is_current_content.clone();

        Computed::from(move |context| {
            let is_current_content = is_current_content.get(context);

            if is_current_content {
                let on_click = bind!(app, ||{
                    app.current_edit();
                });

                ButtonState::active("Edycja pliku", on_click)
            } else {
                ButtonState::disabled("Edycja pliku")
            }
        })
    })
}

fn render_button_move_item(state: &MenuComponent) -> DomNode {
    let state = state.clone();
    let app = state.app;

    ButtonState::render(Computed::from(move |context| {
        let current_path = app.data.tab.full_path.get(context);

        let current_content = app.data.git.content_from_path(context, &current_path);

        if let Resource::Ready(current_content) = current_content {
            let hash = current_content.id.get(context);

            let on_click = bind!(app, current_path, hash, || {
                if let Resource::Ready(hash) = &hash {
                    app.alert.move_current(&app, &current_path, &hash);
                } else {
                    log::error!("error move item");
                }
            });

            return ButtonState::active("Przenieś", on_click);
        }

        ButtonState::disabled("Przenieś")
    }))
}
    
fn render_button_create_file(state: &MenuComponent) -> DomNode {
    ButtonState::render({
        let app = state.app.clone();

        Computed::from(move |_| {
            let on_click = bind!(app, || {
                app.redirect_to_new_content();
            });

            ButtonState::active("Utwórz plik", on_click)
        })
    })
}

fn render_button_rename_name(state: &MenuComponent) -> DomNode {
    ButtonState::render({
        let app = state.app.clone();

        Computed::from(move |_| {
            let on_click = bind!(app, || {
                app.current_rename();
            });

            ButtonState::active("Zmień nazwę", on_click)
        })
    })
}

fn render_button_make_dir(state: &MenuComponent) -> DomNode {
    ButtonState::render({
        let app = state.app.clone();

        Computed::from(move |_| {
            let on_click = bind!(app, || {
                app.redirect_to_mkdir();
            });

            ButtonState::active("Utwórz katalog", on_click)
        })
    })
}

fn render_button_search(state: &MenuComponent) -> DomNode {
    ButtonState::render({
        let app = state.app.clone();

        Computed::from(move |_| {
            let alert = &app.alert;
            let on_click = bind!(alert, || {
                alert.redirect_to_search();
            });

            ButtonState::active("Wyszukaj", on_click)
        })
    })
}

fn render_button_todo(state: &MenuComponent) -> DomNode {
    ButtonState::render({
        // let app = state.app.clone();

        Computed::from(move |_| {
            let on_click = || {
                log::info!("todo --- ....");
            };
            // let on_click = bind(&app.alert).call(|alert| {
            //     alert.redirect_to_search();
            // });

            ButtonState::active("Todo", on_click)
        })
    })
}