use std::rc::Rc;

use common::{HandlerCreateDirBody};
use vertigo::{Driver, Computed, Value, VDomElement};

use crate::{app::{StateApp, index::ListItem}};
use crate::components::new_name;

mod render;

#[derive(PartialEq)]
pub struct StateAppNewDir {
    driver: Driver,

    pub action_save: Value<bool>,

    pub parent: Vec<String>,
    pub new_name: Computed<new_name::NewName>,

    pub save_enable: Computed<bool>,

    app_state: Rc<StateApp>,
}

impl StateAppNewDir {
    pub fn redirect_to_index(&self) {
        self.app_state.redirect_to_index();
    }

    pub fn new(
        app_state: Rc<StateApp>,
        parent: Vec<String>,
        list: Computed<Vec<ListItem>>,
    ) -> StateAppNewDir {
        log::info!("buduję stan dla new dir");
        let action_save = app_state.driver.new_value(false);
        let new_name = new_name::NewName::new(&app_state, list, action_save.to_computed());

        let save_enable = {
            let new_name = new_name.clone();

            app_state.driver.from(move || -> bool {
                let new_name_is_valid = new_name.get_value().is_valid.get_value();

                if !*new_name_is_valid  {
                    return false;
                }

                true
            })
        };

        StateAppNewDir {
            driver: app_state.driver.clone(),

            action_save,

            parent,
            new_name,

            save_enable,

            app_state: app_state.clone(),
        }
    }

    pub fn on_save(&self) {
        let action_save = self.action_save.get_value();

        if *action_save {
            log::error!("Trwa obecnie zapis");
            return;
        }

        self.action_save.set_value(true);

        let new_name_state = self.new_name.get_value();
        let new_dir_name = (*new_name_state.name.get_value()).clone();

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

    pub fn render(self) -> VDomElement {
        let self_computed = self.driver.clone().new_computed_from(self);
        render::render(&self_computed)
    }
}
