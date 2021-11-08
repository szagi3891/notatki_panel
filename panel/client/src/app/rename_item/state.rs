use std::rc::Rc;

use common::{HandlerRenameItemBody};
use vertigo::{DomDriver, computed::{Computed, Value}};

use crate::{app::AppState};

#[derive(PartialEq)]
pub struct State {
    driver: DomDriver,

    pub path: Vec<String>,          //edutowany element
    pub prev_name: String,
    pub prev_hash: String,               //hash poprzedniej zawartosci
    pub prev_content: Option<String>,

    pub new_name: Value<String>,
    pub action_save: Value<bool>,

    pub save_enable: Computed<bool>,

    app_state: Rc<AppState>,
}

impl State {
    pub fn redirect_to_index(&self) {
        self.app_state.redirect_to_index();
    }

    pub fn new(
        app_state: Rc<AppState>,
        path: Vec<String>,
        prev_name: String,
        prev_hash: String,
        prev_content: Option<String>,
    ) -> Computed<State> {
        let new_name = app_state.root.new_value(prev_name.clone());

        let save_enable = {
            let prev_name = prev_name.clone();
            let new_name = new_name.to_computed();

            app_state.root.from(move || -> bool {
                let new_name = new_name.get_value();
                
                if new_name.as_ref().trim() == "" {
                    return false;
                }

                if new_name.as_ref() != &prev_name {
                    return true;
                }

                false
            })
        };

        let action_save = app_state.root.new_value(false);

        app_state.root.new_computed_from(State {
            driver: app_state.driver.clone(),

            path,
            prev_name,
            prev_hash,
            prev_content,

            new_name,

            action_save,
            save_enable,
            app_state: app_state.clone(),
        })
    }

    pub fn get_full_path(&self) -> String {
        let mut path = self.path.clone();
        let prev_name = self.prev_name.clone();

        path.push(prev_name);

        path.as_slice().join("/")
    }

    pub fn on_input(&self, new_text: String) {
        let action_save = self.action_save.get_value();

        if *action_save {
            log::error!("Trwa obecnie zapis");
            return;
        }

        self.new_name.set_value(new_text);
    }

    pub fn on_save(&self) {
        let action_save = self.action_save.get_value();

        if *action_save {
            log::error!("Trwa obecnie zapis");
            return;
        }

        self.action_save.set_value(true);

        let body: HandlerRenameItemBody = HandlerRenameItemBody {
            path: self.path.clone(),
            prev_name: self.prev_name.clone(),
            prev_hash: self.prev_hash.clone(),
            new_name: (*self.new_name.get_value()).clone(),
        };

        let request = self.driver
            .request("/rename_item")
            .body_json(body)
            .post();

        let parent_state = self.app_state.clone();
        let redirect_path = self.path.clone();
        let redirect_new_name = self.new_name.get_value().as_ref().clone();

        self.driver.spawn(async move {

            let _ = request.await;

            log::info!("Zapis udany");

            parent_state.redirect_to_index_with_path(redirect_path, Some(redirect_new_name));
        });
    }
}

