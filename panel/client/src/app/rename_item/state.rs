use common::{HandlerRenameItemBody, HandlerRenameItemResponse};
use vertigo::{
    DomDriver, 
    computed::{Computed, Dependencies, Value},
};

use crate::{app::State as ParentState, request::Request};

#[derive(PartialEq)]
pub struct State {
    request: Request,

    pub path: Vec<String>,          //edutowany element
    pub prev_name: String,
    pub prev_hash: String,               //hash poprzedniej zawartosci

    pub new_name: Value<String>,
    pub action_save: Value<bool>,

    pub save_enable: Computed<bool>,

    parent_state: Computed<ParentState>,
}

impl State {
    pub fn redirect_to_index(&self) {
        self.parent_state.get_value().redirect_to_index();
    }

    pub fn new(
        path: Vec<String>,
        prev_name: String,
        prev_hash: String,
        deep: &Dependencies,
        driver: &DomDriver,
        parent_state: Computed<ParentState>,
    ) -> State {
        let new_name = deep.new_value(prev_name.clone());

        let save_enable = {
            let prev_name = prev_name.clone();
            let new_name = new_name.to_computed();

            deep.from(move || -> bool {
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

        let action_save = deep.new_value(false);

        let request = Request::new(driver);

        State {
            request,

            path,
            prev_name,
            prev_hash,

            new_name,

            action_save,
            save_enable,
            parent_state
        }
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

        let request = self.request
            .fetch("/rename_item")
            .body(body)
            .post::<HandlerRenameItemResponse>();

        let parent_state = self.parent_state.get_value();
        let redirect_path = self.path.clone();
        let redirect_new_name = self.new_name.get_value().as_ref().clone();

        self.request.spawn_local(async move {

            let response = request.await.unwrap();

            log::info!("Zapis udany {:?}", response);

            parent_state.redirect_to_index_with_path(redirect_path, Some(redirect_new_name));
        });
    }
}

