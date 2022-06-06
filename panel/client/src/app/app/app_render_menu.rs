use vertigo::{
    Css,
    Computed, VDomComponent,
    bind, Resource,
};

use vertigo::{html, css};
use crate::app::App;
use crate::components::{button, ButtonState, ButtonComponent};
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
    let button_delete = render_button_on_delete(state);
    let button_edit_file = render_button_on_edit_file(state);

    let button_move_item = render_button_move_item(state);
    
    let app = state.app.clone();

    let on_rename = bind(&app).call(|app| {
        app.current_rename();
    });

    let on_create = bind(&app).call(|app| {
        app.redirect_to_new_content();
    });

    let on_mkdir = bind(&app).call(|app| {
        app.redirect_to_mkdir();
    });

    let mut out = Vec::new();

    out.push(button("Utwórz plik", on_create));
    out.push(button("Zmień nazwę", on_rename));
    out.push(html!{
        <span>
            {button_edit_file}
        </span>
    });
    out.push(button("Utwórz katalog", on_mkdir));    
    out.push(html! {
        <span>
            {button_delete}
        </span>
    });

    out.push(button("Wyszukaj", bind(&app.alert).call(|alert| {
        alert.redirect_to_search();
    })));

    out.push(html! {
        <span>
            { button_move_item }
        </span>
    });

    VDomComponent::from_html(
        html! {
            <div css={css_footer()}>
                { ..out }
            </div>
        }
    )
}


fn render_button_on_delete(state: &MenuComponent) -> VDomComponent {
    ButtonComponent::new({
        let app = state.app.clone();
        let is_current_content = state.is_current_content.clone();

        move || {
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
        }
    })

}


fn render_button_on_edit_file(state: &MenuComponent) -> VDomComponent {
    ButtonComponent::new({
        let app = state.app.clone();
        let is_current_content = state.is_current_content.clone();

        move || {
            let is_current_content = is_current_content.get();

            if is_current_content {
                let on_click = bind(&app).call(|app|{
                    app.current_edit();
                });

                ButtonState::active("Edycja pliku", on_click)
            } else {
                ButtonState::disabled("Edycja pliku")
            }
        }
    })
}

fn render_button_move_item(state: &MenuComponent) -> VDomComponent {
    let state = state.clone();

    ButtonComponent::new(move || {
        let app = state.app.clone();
        let current_path = app.data.tab.full_path.get();

        let current_content = app.data.git.content_from_path(&current_path);

        if let Resource::Ready(current_content) = current_content {
            let hash = current_content.id;

            return ButtonState::active("Przenieś", move || {
                app.alert.move_current(&app, &current_path, &hash);
            });
        }

        ButtonState::disabled("Przenieś")
    })
}