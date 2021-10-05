use common::{HandlerCreateFileBody, HandlerCreateFileResponse};
use vertigo::{
    DomDriver, 
    computed::{Computed, Dependencies, Value},
};

use crate::{app::{StateView, index::ListItem}, request::Request};

#[derive(PartialEq)]
pub struct State {
    request: Request,

    pub action_save: Value<bool>,

    pub parent: Vec<String>,
    pub new_name: Computed<super::new_name::NewName>,
    pub content: Value<String>,

    pub save_enable: Computed<bool>,

    parent_state: StateView,
}

impl State {
    pub fn redirect_to_index(&self) {
        self.parent_state.redirect_to_index();
    }

    pub fn new(
        deep: &Dependencies,
        parent: Vec<String>,
        driver: &DomDriver,
        list: Computed<Vec<ListItem>>,
        parent_state: StateView,
    ) -> Computed<State> {
        let action_save = deep.new_value(false);
        let new_name = super::new_name::NewName::new(deep, list, action_save.to_computed());
        let content = deep.new_value(String::from(""));


        let save_enable = {
            let new_name = new_name.clone();
            let content = content.to_computed();

            deep.from(move || -> bool {
                let new_name_is_valid = new_name.get_value().is_valid.get_value();

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

        let request = Request::new(driver);

        deep.new_computed_from(State {
            request,

            action_save,
            
            parent,
            new_name,
            content,

            save_enable,

            parent_state,
        })
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

        let new_name_state = self.new_name.get_value();
        let full_relative_new_path = (*new_name_state.full_relative_new_path.get_value()).clone();
        let file_name = (*new_name_state.name.get_value()).clone();

        let body: HandlerCreateFileBody = HandlerCreateFileBody {
            path: self.parent.clone(),
            new_path: full_relative_new_path,
            new_content: (*self.content.get_value()).clone(),
        };

        let request = self.request
            .fetch("/create_file")
            .body(body)
            .post::<HandlerCreateFileResponse>();

        let callback = self.parent_state.clone();


        self.request.spawn_local({
            
            let mut path_redirect = self.parent.clone();
            let relative_new_path = (*new_name_state.relative_path.get_value()).clone();
            path_redirect.extend(relative_new_path.into_iter());
            
            let file_name = Some(file_name);
            
            async move {

                let response = request.await.unwrap();

                log::info!("Zapis udany {:?} -> przekierowanie na -> {:?} {:?}", response, path_redirect, file_name);

                callback.redirect_to_index_with_path(path_redirect, file_name);
            }
        });
    }
}
