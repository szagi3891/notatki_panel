use common::{HandlerCreateDirBody};
use vertigo::{Computed, Value, get_driver, transaction, bind_spawn, DomNode};

use crate::app::App;
use crate::app::response::check_request_response;
use crate::components::new_name::{self, NewName};
use crate::data::ListItem;

use super::app_newdir_render::app_newdir_render;

#[derive(Clone, PartialEq)]
pub struct AppNewdir {
    pub app: App,
    pub action_save: Value<bool>,

    pub select_dir: ListItem,
    pub new_name: NewName,

    pub save_enable: Computed<bool>,
}

impl AppNewdir {
    pub fn new(app: &App, select_dir: ListItem) -> AppNewdir {
        let action_save = Value::new(false);

        let new_name = new_name::NewName::new(select_dir.clone());
        let is_valid = new_name.is_valid.clone();

        AppNewdir {
            app: app.clone(),
            action_save,

            select_dir,
            new_name,
            save_enable: is_valid,
        }
    }

    pub fn render(&self) -> DomNode {
        app_newdir_render(self.clone())
    }

    pub fn bind_on_save(&self, app: &App) -> impl Fn() {
        let state = self.clone();

        bind_spawn!(state, app, async move {
            let action_save = transaction(|context| state.action_save.get(context));

            if action_save {
                log::error!("Trwa obecnie zapis");
                return;
            }

            state.action_save.set(true);
        
            let new_dir_name = transaction(|context| state.new_name.name.get(context));

            let body = HandlerCreateDirBody {
                path: state.select_dir.full_path.as_ref().clone(),
                dir: new_dir_name.clone(),
            };

            let response = get_driver()
                .request_post("/create_dir")
                .body_json(body)
                .call().await;

            state.action_save.set(false);

            match check_request_response(response) {
                Ok(()) => {
                    //TODO - zamienić join("/") na to_string_path 
                    let parent_string = state.select_dir.to_string_path();
                    log::info!("Tworzenie katalogu {:?} udane -> przekierowanie na -> {:?}", new_dir_name, parent_string);

                    app.redirect_to_index_with_path(state.select_dir.full_path.as_ref().clone(), Some(new_dir_name));
                },
                Err(message) => {
                    app.show_message_error(message, Some(10000));
                }
            };
        })
    }
}
