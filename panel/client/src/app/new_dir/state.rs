use std::rc::Rc;

use common::{HandlerCreateDirBody, RootResponse};
use vertigo::{
    computed::{Computed, Value},
};

use crate::{app::{AppState, index::ListItem}, request::Request};

#[derive(PartialEq)]
pub struct State {
    request: Request,

    pub action_save: Value<bool>,

    pub parent: Vec<String>,
    pub new_name: Computed<super::new_name::NewName>,

    pub save_enable: Computed<bool>,

    app_state: Rc<AppState>,
}

impl State {
    pub fn redirect_to_index(&self) {
        self.app_state.redirect_to_index();
    }

    pub fn new(
        app_state: Rc<AppState>,
        parent: Vec<String>,
        list: Computed<Vec<ListItem>>,
    ) -> Computed<State> {
        log::info!("buduję stan dla new dir");
        let action_save = app_state.root.new_value(false);
        let new_name = super::new_name::NewName::new(&app_state, list, action_save.to_computed());

        let save_enable = {
            let new_name = new_name.clone();

            app_state.root.from(move || -> bool {
                let new_name_is_valid = new_name.get_value().is_valid.get_value();

                if !*new_name_is_valid  {
                    return false;
                }

                true
            })
        };

        let request = Request::new(&app_state.driver);

        app_state.root.new_computed_from(State {
            request,

            action_save,

            parent,
            new_name,

            save_enable,

            app_state: app_state.clone(),
        })
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

        let request = self.request
            .fetch("/create_dir")
            .body(body)
            .post::<RootResponse>();

        let callback = self.app_state.clone();
        let parent = self.parent.clone();


        self.request.spawn_local(async move {
            let response = request.await.unwrap();
            let parent_string = parent.join("/");
            log::info!("Tworzenie katalogu {:?} udane {:?} -> przekierowanie na -> {:?}", new_dir_name, response, parent_string);

            callback.redirect_to_index_with_path(parent, Some(new_dir_name));
        });
    }
}
