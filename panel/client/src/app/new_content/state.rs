use common::{HandlerCreateFileBody, HandlerCreateFileResponse};
use vertigo::{
    DomDriver, 
    computed::{Computed, Dependencies, Value},
};

use crate::{app::{State as ParentState, index::ListItem}, request::Request};

#[derive(PartialEq)]
pub struct State {
    request: Request,

    pub action_save: Value<bool>,

    pub parent: Vec<String>,
    pub name: Value<String>,
    pub content: Value<String>,

    pub name_exists: Computed<bool>,
    pub save_enable: Computed<bool>,

    parent_state: Computed<ParentState>,
}

impl State {
    pub fn redirect_to_index(&self) {
        self.parent_state.get_value().redirect_to_index();
    }

    pub fn new(
        deep: &Dependencies,
        parent: Vec<String>,
        driver: &DomDriver,
        list: Computed<Vec<ListItem>>,
        parent_state: Computed<ParentState>,
    ) -> State {
        let action_save = deep.new_value(false);
        let name = deep.new_value(String::from(""));
        let content = deep.new_value(String::from(""));

        let name_exists = {
            let name = name.to_computed();

            deep.from(move || -> bool {
                let list = list.get_value();
                let name = name.get_value();

                for item in list.as_ref() {
                    if item.name == *name {
                        return true;
                    }
                }

                false
            })
        };

        let save_enable = {
            let name_exists = name_exists.clone();
            let name = name.to_computed();
            let content = content.to_computed();

            deep.from(move || -> bool {
                let name_exists = name_exists.get_value();
                if *name_exists == true {
                    return false;
                }

                let name = name.get_value();
                if *name == "" {
                    return false;
                }

                let content = content.get_value();
                if *content == "" {
                    return false;
                }

                true
            })
        };

        let request = Request::new(driver);

        State {
            request,

            action_save,
            
            parent,
            name,
            content,

            name_exists,
            save_enable,

            parent_state,
        }
    }

    pub fn on_input_name(&self, new_value: String) {
        let action_save = self.action_save.get_value();

        if *action_save == true {
            log::error!("Trwa obecnie zapis");
            return;
        }

        self.name.set_value(new_value);
    }

    pub fn on_input_content(&self, new_value: String) {
        let action_save = self.action_save.get_value();

        if *action_save == true {
            log::error!("Trwa obecnie zapis");
            return;
        }

        self.content.set_value(new_value);
    }

    pub fn on_save(&self) {
        let action_save = self.action_save.get_value();

        if *action_save == true {
            log::error!("Trwa obecnie zapis");
            return;
        }

        self.action_save.set_value(true);

        let name = (*self.name.get_value()).clone();

        let body: HandlerCreateFileBody = HandlerCreateFileBody {
            path: self.parent.clone(),
            new_path: vec!(name.clone()),
            new_content: (*self.content.get_value()).clone(),
        };

        let request = self.request
            .fetch("/create_file")
            .body(body)
            .post::<HandlerCreateFileResponse>();

        let callback = self.parent_state.get_value().clone();

        self.request.spawn_local({
            
            let parent = self.parent.clone();
            let name = Some(name);
            
            async move {

                let response = request.await.unwrap();

                log::info!("Zapis udany {:?}", response);

                callback.redirect_to_index_with_path(parent, name);
            }
        });
    }
}
