use vertigo::{
    VDomElement,
    Css,
    Computed, VDomComponent, Driver,
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
    driver: &Driver,
    current_content: Computed<CurrentContent>
) -> Computed<bool> {

    driver.from(move || -> bool {
        let current = current_content.get_value();

        match current.as_ref() {
            CurrentContent::None => false,
            CurrentContent::File { .. } => true,
            CurrentContent::Dir { list, ..} => list.len() == 0
        }
    })
}

#[derive(Clone)]
pub struct AppIndexMenuState {
    app_index: AppIndex,

    //true - jeśli aktualnie podświetlony element jest mozliwy do usuniecia
    pub avaible_delete_button: Computed<bool>,
}

impl AppIndexMenuState {
    pub fn component(app: &App, app_index: &AppIndex) -> VDomComponent {
        let avaible_delete_current= create_avaible_delete_current(
            &app_index.data.driver,
            app_index.data.tab.current_content.clone()
        );
    
        let state = AppIndexMenuState {
            app_index: app_index.clone(),
            avaible_delete_button: avaible_delete_current,
        };

        let app = app.clone();

        VDomComponent::from_fn(move || -> VDomElement {
            render_menu(&app, &state)
        })
    }
}

fn render_menu(app: &App, state: &AppIndexMenuState) -> VDomElement {
    let on_click = {
        let app = app.clone();
        
        move || {
            app.current_edit();
        }
    };

    let on_rename = {
        let app = app.clone();

        move || {
            app.current_rename();
        }
    };

    let on_create = {
        let app = app.clone();
        
        move || {
            app.create_file();
        }
    };

    let on_mkdir = {
        let app = app.clone();

        move || {
            app.redirect_to_mkdir();
        }
    };

    let mut out = Vec::new();

    out.push(button("Utwórz plik", on_create));
    out.push(button("Zmień nazwę", on_rename));
    out.push(button("Edycja pliku", on_click));
    out.push(button("Utwórz katalog", on_mkdir));
    
    let avaible_delete_button = state.avaible_delete_button.get_value();

    if *avaible_delete_button {
        let alert = state.app_index.alert.clone();
        let on_delete = {
            move || {
                let path = alert.data.tab.full_path.get_value();
                alert.delete(path);
            }
        };

        out.push(button("Usuń", on_delete));
    }

    out.push(button("Wyszukaj", {
        let alert = state.app_index.alert.clone();
        move || {
            alert.redirect_to_search();
        }
    }));

    let current_path = state.app_index.data.tab.full_path.get_value();

    out.push(button("Przenieś", {
        let alert = state.app_index.alert.clone();
        move || {
            alert.move_current(current_path.clone());
        }
    }));

    html! {
        <div css={css_footer()}>
            { ..out }
        </div>
    }
}
