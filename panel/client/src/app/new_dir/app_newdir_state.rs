use common::{HandlerCreateDirBody};
use vertigo::{Driver, Computed, Value, VDomComponent};

use crate::app::App;
use crate::components::new_name;

use super::app_newdir_render::app_newdir_render;

#[derive(Clone)]
pub struct AppNewdir {
    pub driver: Driver,

    pub action_save: Value<bool>,

    pub parent: Vec<String>,
    pub new_name: Value<String>,

    pub save_enable: Computed<bool>,

    app_state: App,
}

impl AppNewdir {
    pub fn redirect_to_index(&self) {
        self.app_state.redirect_to_index();
    }

    pub fn component(app_state: &App) -> VDomComponent {
        log::info!("buduję stan dla new dir");
        let action_save = app_state.driver.new_value(false);
        let list = app_state.data.tab.list.clone();
        let parent = app_state.data.tab.dir_select.clone().get_value();

        let new_name = new_name::NewName::new(
            &app_state.driver,
            list,
        );

        let state = AppNewdir {
            driver: app_state.driver.clone(),

            action_save,

            parent: parent.as_ref().clone(),
            new_name: new_name.name.clone(),
            save_enable: new_name.is_valid.clone(),

            app_state: app_state.clone(),
        };

        let view_new_name = new_name.render(true);

        app_newdir_render(view_new_name, state)
    }

    pub async fn on_save(self) {
        let action_save = self.action_save.get_value();

        if *action_save {
            log::error!("Trwa obecnie zapis");
            return;
        }

        self.action_save.set_value(true);
    
        let new_dir_name = self.new_name.get_value().as_ref().clone();

        let body = HandlerCreateDirBody {
            path: self.parent.clone(),
            dir: new_dir_name.clone(),
        };

        let _ = self.driver
            .request("/create_dir")
            .body_json(body)
            .post().await;

        let parent_string = self.parent.join("/");
        log::info!("Tworzenie katalogu {:?} udane -> przekierowanie na -> {:?}", new_dir_name, parent_string);

        self.app_state.redirect_to_index_with_path(self.parent.clone(), Some(new_dir_name));
    }

    pub fn bind_on_save(&self) -> impl Fn() {
        let driver = self.driver.clone();
        let state = self.clone();
        move || {
            driver.spawn(state.clone().on_save());
        }
    }
}
