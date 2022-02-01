use common::{HandlerCreateDirBody};
use vertigo::{Driver, Computed, Value, VDomComponent};

use crate::{app::{StateApp}, data::ListItem};
use crate::components::new_name;

mod render;

#[derive(Clone)]
pub struct StateAppNewDir {
    driver: Driver,

    pub action_save: Value<bool>,

    pub parent: Vec<String>,
    pub new_name: Value<String>,

    pub save_enable: Computed<bool>,

    app_state: StateApp,
}

impl StateAppNewDir {
    pub fn redirect_to_index(&self) {
        self.app_state.redirect_to_index();
    }

    pub fn component(
        app_state: &StateApp,
        parent: Vec<String>,
        list: Computed<Vec<ListItem>>,
    ) -> VDomComponent {
        log::info!("buduję stan dla new dir");
        let action_save = app_state.driver.new_value(false);
        let new_name = app_state.driver.new_value(String::from(""));

        let new_name = new_name::NewName::new(
            &app_state.driver,
            list,
            new_name.clone(),
            action_save.to_computed(),
        );

        let state = StateAppNewDir {
            driver: app_state.driver.clone(),

            action_save,

            parent,
            new_name: new_name.name.clone(),
            save_enable: new_name.action_save.clone(),

            app_state: app_state.clone(),
        };

        let view_new_name = new_name.render();

        render::build_render(view_new_name, state)
    }

    pub fn on_save(&self) {
        let action_save = self.action_save.get_value();

        if *action_save {
            log::error!("Trwa obecnie zapis");
            return;
        }

        self.action_save.set_value(true);

        let new_dir_name = (*self.new_name.get_value()).clone();

        let body = HandlerCreateDirBody {
            path: self.parent.clone(),
            dir: new_dir_name.clone(),
        };

        let request = self.driver
            .request("/create_dir")
            .body_json(body)
            .post();

        let callback = self.app_state.clone();
        let parent = self.parent.clone();


        self.driver.spawn(async move {
            let _ = request.await;
            let parent_string = parent.join("/");
            log::info!("Tworzenie katalogu {:?} udane -> przekierowanie na -> {:?}", new_dir_name, parent_string);

            callback.redirect_to_index_with_path(parent, Some(new_dir_name));
        });
    }
}
