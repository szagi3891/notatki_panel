use vertigo::{
    VDomElement,
    Css,
    Computed, VDomComponent,
    bind, Resource,
};

use vertigo::{html, css};
use super::AppIndex;
use crate::app::App;
use crate::components::button;
use crate::data::ContentType;

fn css_footer() -> Css {
    css!("
        display: flex;
        flex-shrink: 0;
        line-height: 25px;
        border-bottom: 1px solid black;
    ")
}


fn create_avaible_delete_current(
    current_content: &Computed<Resource<ContentType>>
) -> Computed<bool> {

    let current_content = current_content.clone();

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
}

pub struct MenuComponent {
    app: App,
    app_index: AppIndex,
    avaible_delete_button: Computed<bool>,
}

impl MenuComponent {
    pub fn component(app: &App, app_index: &AppIndex) -> VDomComponent {
        let avaible_delete_button= create_avaible_delete_current(
            &app_index.data.tab.current_content
        );

        let state = MenuComponent {
            app: app.clone(),
            app_index: app_index.clone(),
            avaible_delete_button
        };

        VDomComponent::from(state, render_menu)
    }
}

fn render_menu(state: &MenuComponent) -> VDomElement {
    let app = state.app.clone();
    let app_index = state.app_index.clone();

    let on_click = bind(&app).call(|app|{
        app.current_edit();
    });

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
    out.push(button("Edycja pliku", on_click));
    out.push(button("Utwórz katalog", on_mkdir));
    
    let avaible_delete_button = state.avaible_delete_button.get();

    if avaible_delete_button {
        let alert = app_index.alert.clone();
        let on_delete = bind(&alert).call(|alert| {
            let path = alert.data.tab.full_path.get();
            alert.delete(path);
        });

        out.push(button("Usuń", on_delete));
    }

    out.push(button("Wyszukaj", bind(&app_index.alert).call(|alert| {
        alert.redirect_to_search();
    })));

    out.push(button("Przenieś", bind(&app_index).call(|app_index| {
        let current_path = app_index.data.tab.full_path.get();
        app_index.alert.move_current(current_path);
    })));

    html! {
        <div css={css_footer()}>
            { ..out }
        </div>
    }
}
