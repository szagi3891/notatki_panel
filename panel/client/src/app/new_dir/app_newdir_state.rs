use common::{HandlerCreateDirBody};
use vertigo::{Computed, Value, VDomComponent, bind, get_driver};

use crate::app::App;
use crate::app::response::check_request_response;
use crate::components::new_name::{self, NewName};
use crate::data::Data;

use super::app_newdir_render::app_newdir_render;

#[derive(Clone)]
pub struct AppNewdir {
    pub action_save: Value<bool>,

    pub parent: Vec<String>,
    pub new_name: NewName,

    pub save_enable: Computed<bool>,
}

impl AppNewdir {
    pub fn new(data: &Data) -> AppNewdir {
        log::info!("buduję stan dla new dir");
        let action_save = Value::new(false);
        let list = data.tab.list.clone();
        let parent = data.tab.dir_select.clone().get();

        let new_name = new_name::NewName::new(list);
        let is_valid = new_name.is_valid.clone();

        AppNewdir {
            action_save,

            parent,
            new_name,
            save_enable: is_valid,
        }
    }

    pub fn render(&self, app: App) -> VDomComponent {
        app_newdir_render(self.clone(), app)
    }

    pub fn bind_on_save(&self, app: &App) -> impl Fn() {
        bind(self)
            .and(app)
            .spawn(|state, app| async move {
                let action_save = state.action_save.get();

                if action_save {
                    log::error!("Trwa obecnie zapis");
                    return;
                }

                state.action_save.set(true);
            
                let new_dir_name = state.new_name.name.get();

                let body = HandlerCreateDirBody {
                    path: state.parent.clone(),
                    dir: new_dir_name.clone(),
                };

                let response = get_driver()
                    .request("/create_dir")
                    .body_json(body)
                    .post().await;

                state.action_save.set(false);

                match check_request_response(response) {
                    Ok(()) => {                
                        let parent_string = state.parent.join("/");
                        log::info!("Tworzenie katalogu {:?} udane -> przekierowanie na -> {:?}", new_dir_name, parent_string);

                        app.redirect_to_index_with_path(state.parent.clone(), Some(new_dir_name));
                    },
                    Err(message) => {
                        app.show_message_error(message, Some(10000));
                    }
                }
            })
    }
}
