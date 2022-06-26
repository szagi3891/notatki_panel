use vertigo::{
    Css,
    Computed, VDomComponent,
    bind, Resource, DomElement, dom,
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
    pub fn component(app: &App) -> VDomComponent {
        let is_current_content= {
            let current_content = app.data.tab.current_content.clone();
        
            Computed::from(move || -> bool {
                if let Resource::Ready(content) = current_content.get() {
                    match content {
                        ContentType::Dir { list } => list.len() == 0,
                        _ => true
                    }
                } else {
                    false
                }
            })
        };

        let state = MenuComponent {
            app: app.clone(),
            is_current_content,
        };

        render_menu(&state)
    }
}

fn render_menu(state: &MenuComponent) -> VDomComponent {
    let button_edit_file = render_button_edit_file(state);
    let button_create_file = render_button_create_file(state);
    let button_rename_name = render_button_rename_name(state);
    let button_make_dir = render_button_make_dir(state);
    let button_delete = render_button_on_delete(state);
    let button_search = render_button_search(state);
    let button_move_item = render_button_move_item(state);
    let button_todo = render_button_todo(state);

    VDomComponent::dom(dom! {
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
    })
}

fn render_button_on_delete(state: &MenuComponent) -> DomElement {
    ButtonState::render({
        let app = state.app.clone();
        let is_current_content = state.is_current_content.clone();

        Computed::from(move || {
            let is_current_content = is_current_content.get();

            if is_current_content {
                let alert = app.alert.clone();
                let on_delete = bind(&alert)
                    .and(&app)
                    .call(|alert, app| {
                        let path = alert.data.tab.full_path.get();
                        alert.delete(app.clone(), path);
                    });
        
                ButtonState::active("Usuń", on_delete)
            } else {
                ButtonState::disabled("Usuń")
            }
        })
    })

}

fn render_button_edit_file(state: &MenuComponent) -> DomElement {
    ButtonState::render({
        let app = state.app.clone();
        let is_current_content = state.is_current_content.clone();

        Computed::from(move || {
            let is_current_content = is_current_content.get();

            if is_current_content {
                let on_click = bind(&app).call(|app|{
                    app.current_edit();
                });

                ButtonState::active("Edycja pliku", on_click)
            } else {
                ButtonState::disabled("Edycja pliku")
            }
        })
    })
}

fn render_button_move_item(state: &MenuComponent) -> DomElement {
    let state = state.clone();
    let app = state.app.clone();

    ButtonState::render(Computed::from(move || {
        let current_path = app.data.tab.full_path.get();

        let current_content = app.data.git.content_from_path(&current_path);

        if let Resource::Ready(current_content) = current_content {
            let hash = current_content.id;

            let app = app.clone();
            return ButtonState::active("Przenieś", move || {
                app.alert.move_current(&app, &current_path, &hash);
            });
        }

        ButtonState::disabled("Przenieś")
    }))
}
    
fn render_button_create_file(state: &MenuComponent) -> DomElement {
    ButtonState::render({
        let app = state.app.clone();

        Computed::from(move || {
            let on_click = bind(&app).call(|app| {
                app.redirect_to_new_content();
            });

            ButtonState::active("Utwórz plik", on_click)
        })
    })
}

fn render_button_rename_name(state: &MenuComponent) -> DomElement {
    ButtonState::render({
        let app = state.app.clone();

        Computed::from(move || {
            let on_click = bind(&app).call(|app| {
                app.current_rename();
            });

            ButtonState::active("Zmień nazwę", on_click)
        })
    })
}

fn render_button_make_dir(state: &MenuComponent) -> DomElement {
    ButtonState::render({
        let app = state.app.clone();

        Computed::from(move || {
            let on_click = bind(&app).call(|app| {
                app.redirect_to_mkdir();
            });

            ButtonState::active("Utwórz katalog", on_click)
        })
    })
}

fn render_button_search(state: &MenuComponent) -> DomElement {
    ButtonState::render({
        let app = state.app.clone();

        Computed::from(move || {
            let on_click = bind(&app.alert).call(|alert| {
                alert.redirect_to_search();
            });

            ButtonState::active("Wyszukaj", on_click)
        })
    })
}

fn render_button_todo(state: &MenuComponent) -> DomElement {
    ButtonState::render({
        // let app = state.app.clone();

        Computed::from(move || {
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