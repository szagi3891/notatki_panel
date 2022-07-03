use common::{HandlerCreateDirBody};
use vertigo::{Computed, Value, bind, get_driver, Context, DomElement};

use crate::app::App;
use crate::app::response::check_request_response;
use crate::components::new_name::{self, NewName};

use super::app_newdir_render::app_newdir_render;

#[derive(Clone, PartialEq)]
pub struct AppNewdir {
    pub app: App,
    pub action_save: Value<bool>,

    pub parent: Vec<String>,
    pub new_name: NewName,

    pub save_enable: Computed<bool>,
}

impl AppNewdir {
    pub fn new(context: &Context, app: &App) -> AppNewdir {
        let action_save = Value::new(false);
        let list = app.data.tab.list.clone();
        let parent = app.data.tab.router.get_dir(context);

        let new_name = new_name::NewName::new(list);
        let is_valid = new_name.is_valid.clone();

        AppNewdir {
            app: app.clone(),
            action_save,

            parent,
            new_name,
            save_enable: is_valid,
        }
    }

    pub fn render(&self) -> DomElement {
        app_newdir_render(self.clone())
    }

    pub fn bind_on_save(&self, app: &App) -> impl Fn() {
        bind(self)
            .and(app)
            .spawn(|context, state, app| async move {
                let action_save = state.action_save.get(&context);

                if action_save {
                    log::error!("Trwa obecnie zapis");
                    return context;
                }

                state.action_save.set(true);
            
                let new_dir_name = state.new_name.name.get(&context);

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
                        app.show_message_error(&context, message, Some(10000));
                    }
                };

                context
            })
    }
}
