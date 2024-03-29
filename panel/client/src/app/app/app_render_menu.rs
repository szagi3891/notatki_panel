use vertigo::{
    Css,
    Computed,
    bind, Resource, dom, DomNode,
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
            let tab = app.data.tab.clone();
            move |context| -> bool {
                if let Some(select_content) = tab.select_content.get(context) {
                    if let Resource::Ready(content) = select_content.get_content_type(context) {
                        return match content {
                            ContentType::Dir { item } => {
                                let len = match item.list.get(context) {
                                    Resource::Ready(list) => list.len(),
                                    _ => 0,
                                };

                                len == 0
                            },
                            _ => true
                        };
                    }
                }

                false
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
    let button_rename_name = state.app.render_current_rename();
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
                let Some(select_content) = alert.data.tab.select_content.get(context) else {
                    return ButtonState::disabled("Usuń");
                };

                let on_delete = bind!(alert, app, select_content, || {
                    alert.delete(app.clone(), select_content.clone());
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
                let select_content = app.data.tab.select_content.get(context);

                let Some(select_content) = select_content else {
                    return ButtonState::disabled("Edycja pliku");
                };

                let on_click = bind!(app, select_content, || {
                    app.redirect_to_edit_content(select_content.clone());
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
        let current_content = app.data.tab.select_content.get(context);

        let Some(current_content) = current_content else {
            return ButtonState::disabled("Przenieś");
        };

        let hash = current_content.id.get(context);

        let Resource::Ready(hash) = &hash else {
            return ButtonState::disabled("Przenieś");
        };

        let on_click = bind!(app, current_content, hash, || {
            app.alert.move_current(&app, current_content.clone(), &hash);
        });

        return ButtonState::active("Przenieś", on_click);
    }))
}
    
fn render_button_create_file(state: &MenuComponent) -> DomNode {
    ButtonState::render({
        let app = state.app.clone();

        Computed::from(move |context| {
            let select_dir = app.data.tab.select_dir.get(context);

            let on_click = bind!(app, select_dir, || {
                app.redirect_to_new_content(select_dir.clone());
            });

            ButtonState::active("Utwórz plik", on_click)
        })
    })
}

fn render_button_make_dir(state: &MenuComponent) -> DomNode {
    ButtonState::render({
        let app = state.app.clone();

        Computed::from(move |context| {
            let select_dir = app.data.tab.select_dir.get(context);

            let on_click = bind!(app, select_dir, || {
                app.redirect_to_mkdir(select_dir.clone());
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
        let app = state.app.clone();

        Computed::from(move |context| {
            let todo_only = app.data.tab.items.todo_only.clone();

            let todo = todo_only.get(context);

            let on_click = move || {
                todo_only.set(!todo);
            };

            let label = match todo {
                false => "Todo nieaktywne",
                true => "Todo aktywne",
            };

            ButtonState::active(label, on_click)
        })
    })
}