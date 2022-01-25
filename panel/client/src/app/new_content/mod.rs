mod render;

use std::rc::Rc;

use common::{HandlerCreateFileBody};
use vertigo::{Driver, Computed, Value, VDomComponent};

use crate::{app::{StateApp, index::ListItem}};
use crate::components::new_name;

#[derive(PartialEq)]
pub struct StateAppNewContent {
    driver: Driver,

    pub action_save: Value<bool>,

    pub parent: Vec<String>,
    pub name: Value<String>,
    pub new_name_view: VDomComponent,
    pub content: Value<String>,

    pub save_enable: Computed<bool>,

    app_state: Rc<StateApp>,
}

impl StateAppNewContent {
    pub fn redirect_to_index(&self) {
        self.app_state.redirect_to_index();
    }

    pub fn component(
        app_state: Rc<StateApp>,
        parent: Vec<String>,
        list: Computed<Vec<ListItem>>,
    ) -> VDomComponent {
        log::info!("buduję stan dla new content");
        let action_save = app_state.driver.new_value(false);
        // let name = new_name::NewName::new(&app_state, list, action_save.to_computed());

        let name = app_state.driver.new_value(String::from(""));
        let (is_valid, _new_name_save_enable, new_name_view) = new_name::NewName::component(
            &app_state.driver,
            list,
            name.clone(),
            action_save.to_computed(),
        );

        let content = app_state.driver.new_value(String::from(""));


        let save_enable = {
            let content = content.to_computed();

            app_state.driver.from(move || -> bool {
                let new_name_is_valid = is_valid.get_value();

                if !*new_name_is_valid  {
                    return false;
                }

                let content = content.get_value();
                if content.is_empty() {
                    return false;
                }

                true
            })
        };

        let state = StateAppNewContent {
            driver: app_state.driver.clone(),

            action_save,
            
            parent,
            name,
            new_name_view,
            content,

            save_enable,

            app_state: app_state.clone(),
        };

        app_state.driver.bind_render(state, render::render)
    }

    pub fn on_input_content(&self, new_value: String) {
        let action_save = self.action_save.get_value();

        if *action_save {
            log::error!("Trwa obecnie zapis");
            return;
        }

        self.content.set_value(new_value);
    }

    pub fn on_save(&self) {
        let action_save = self.action_save.get_value();

        if *action_save {
            log::error!("Trwa obecnie zapis");
            return;
        }

        self.action_save.set_value(true);

        let new_name_rc = self.name.get_value();
        let new_name = (*new_name_rc).clone();

        let body: HandlerCreateFileBody = HandlerCreateFileBody {
            path: self.parent.clone(),
            new_name: new_name.clone(),
            new_content: (*self.content.get_value()).clone(),
        };

        let request = self.driver
            .request("/create_file")
            .body_json(body)
            .post();

        let callback = self.app_state.clone();


        self.driver.spawn({
            let path_redirect = self.parent.clone();            
            
            async move {
                let _ = request.await;
                log::info!("Zapis udany -> przekierowanie na -> {:?} {:?}", path_redirect, new_name);
                callback.redirect_to_index_with_path(path_redirect, Some(new_name));
            }
        });
    }
}
