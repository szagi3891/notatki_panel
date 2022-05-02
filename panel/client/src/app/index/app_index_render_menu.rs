use vertigo::{
    VDomElement,
    Css,
    Computed, VDomComponent,
    bind,
};

use vertigo::{html, css};
use super::AppIndex;
use crate::app::App;
use crate::components::button;
use crate::data::CurrentContent;

fn css_footer() -> Css {
    css!("
        display: flex;
        flex-shrink: 0;
        line-height: 25px;
        border-bottom: 1px solid black;
    ")
}


fn create_avaible_delete_current(
    current_content: Computed<CurrentContent>
) -> Computed<bool> {

    Computed::from(move || -> bool {
        let current = current_content.get_value();

        match current.as_ref() {
            CurrentContent::None => false,
            CurrentContent::File { .. } => true,
            CurrentContent::Dir { list, ..} => list.len() == 0
        }
    })
}


pub fn render_menu_state(app: &App, app_index: &AppIndex) -> VDomComponent {
    let avaible_delete_button= create_avaible_delete_current(
        app_index.data.tab.current_content.clone()
    );

    let app = app.clone();
    let app_index = app_index.clone();

    VDomComponent::from_fn(move || -> VDomElement {
        render_menu(&app, &app_index, &avaible_delete_button)
    })
}

fn render_menu(app: &App, app_index: &AppIndex, avaible_delete_button: &Computed<bool>) -> VDomElement {
    let on_click = bind(app).call(|app|{
        app.current_edit();
    });

    let on_rename = bind(app).call(|app| {
        app.current_rename();
    });

    let on_create = bind(app).call(|app| {
        app.redirect_to_new_content();
    });

    let on_mkdir = bind(app).call(|app| {
        app.redirect_to_mkdir();
    });

    let mut out = Vec::new();

    out.push(button("Utwórz plik", on_create));
    out.push(button("Zmień nazwę", on_rename));
    out.push(button("Edycja pliku", on_click));
    out.push(button("Utwórz katalog", on_mkdir));
    
    let avaible_delete_button = avaible_delete_button.get_value();

    if *avaible_delete_button {
        let alert = app_index.alert.clone();
        let on_delete = bind(&alert).call(|alert| {
            let path = alert.data.tab.full_path.get_value();
            alert.delete(path);
        });

        out.push(button("Usuń", on_delete));
    }

    out.push(button("Wyszukaj", bind(&app_index.alert).call(|alert| {
        alert.redirect_to_search();
    })));

    out.push(button("Przenieś", bind(app_index).call(|app_index| {
        let current_path = app_index.data.tab.full_path.get_value();
        app_index.alert.move_current(current_path.clone());
    })));

    html! {
        <div css={css_footer()}>
            { ..out }
        </div>
    }
}
