use common::{HandlerSaveContentBody, HandlerSaveContentResponse};
use vertigo::{
    DomDriver, 
    computed::{Computed, Dependencies, Value},
};

use crate::{app::State as ParentState, request::Request};

#[derive(PartialEq)]
pub struct State {
    request: Request,

    pub path: Vec<String>,          //edutowany element
    pub hash: String,               //hash poprzedniej zawartosci

    pub action_save: Value<bool>,
    pub edit_content: Value<String>,
    pub save_enable: Computed<bool>,

    parent_state: Computed<ParentState>,
}

impl State {
    pub fn redirect_to_index(&self) {
        self.parent_state.get_value().redirect_to_index();
    }

    pub fn new(
        path: Vec<String>,
        hash: String,
        content: String,
        deep: &Dependencies,
        driver: &DomDriver,
        parent_state: Computed<ParentState>,
    ) -> State {
        let edit_content = deep.new_value(content.clone());

        let save_enable = {
            let edit_content = edit_content.to_computed();

            deep.from(move || -> bool {
                let edit_content = edit_content.get_value();
                let save_enabled = edit_content.as_ref() != &content;
                save_enabled
            })
        };

        let action_save = deep.new_value(false);

        let request = Request::new(driver);

        State {
            request,

            path,
            hash,

            action_save,
            edit_content,
            save_enable,
            parent_state
        }
    }

    pub fn on_input(&self, new_text: String) {
        let action_save = self.action_save.get_value();

        if *action_save == true {
            log::error!("Trwa obecnie zapis");
            return;
        }

        self.edit_content.set_value(new_text);
    }

    pub fn on_save(&self) {
        let action_save = self.action_save.get_value();

        if *action_save == true {
            log::error!("Trwa obecnie zapis");
            return;
        }

        self.action_save.set_value(true);

        let body: HandlerSaveContentBody = HandlerSaveContentBody {
            path: self.path.clone(),
            prev_hash: self.hash.clone(),
            new_content: (*self.edit_content.get_value()).clone(),
        };

        let request = self.request
            .fetch("/save_content")
            .body(body)
            .post::<HandlerSaveContentResponse>();

        let callback = self.parent_state.get_value().clone();

        self.request.spawn_local(async move {

            let response = request.await.unwrap();

            log::info!("Zapis udany {:?}", response);

            callback.redirect_to_index_with_root_refresh();
        });
    }
}

