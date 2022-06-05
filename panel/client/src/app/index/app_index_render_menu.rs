use std::rc::Rc;

use vertigo::{
    VDomElement,
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

pub struct MenuComponent {
    app: App,

    on_delete: VDomComponent,
    on_edit_file: VDomComponent,
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

        let on_delete = ButtonComponent::new({
            let app = app.clone();
            let is_current_content = is_current_content.clone();

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
            
                    ButtonState::Active {
                        label: "Usuń".into(),
                        action: Rc::new(on_delete),
                    }
                } else {
                    ButtonState::Disabled { label: "Usuń".into() }
                }
            }
        });

        let on_edit_file = ButtonComponent::new({
            let app = app.clone();
            let is_current_content = is_current_content.clone();

            move || {
                let is_current_content = is_current_content.get();

                if is_current_content {
                    let on_click = bind(&app).call(|app|{
                        app.current_edit();
                    });

                    ButtonState::Active {
                        label: "Edycja pliku".into(),
                        action: Rc::new(on_click),
                    }
                } else {
                    ButtonState::Disabled {
                        label: "Edycja pliku".into(),
                    }
                }
            }
        });

        let state = MenuComponent {
            app: app.clone(),
            on_delete,
            on_edit_file,
        };

        VDomComponent::from(state, render_menu)
    }
}

fn render_menu(state: &MenuComponent) -> VDomElement {
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
            {state.on_edit_file.clone()}
        </span>
    });
    out.push(button("Utwórz katalog", on_mkdir));    
    out.push(html! {
        <span>
            {state.on_delete.clone()}
        </span>
    });

    out.push(button("Wyszukaj", bind(&app.alert).call(|alert| {
        alert.redirect_to_search();
    })));

    out.push(button("Przenieś", bind(&app).call(|app| {
        let current_path = app.data.tab.full_path.get();
        app.alert.move_current(current_path);
    })));

    html! {
        <div css={css_footer()}>
            { ..out }
        </div>
    }
}
