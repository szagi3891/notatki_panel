use common::{HandlerCreateFileBody};
use vertigo::{Computed, Value, get_driver, transaction, DomElement, bind, bind_spawn};

use crate::app::App;
use crate::app::newcontent::app_newcontent_render::app_newcontent_render;
use crate::app::response::check_request_response;
use crate::components::new_name::NewName;

#[derive(Clone, PartialEq)]
pub struct AppNewcontent {
    app: App,
    pub action_save: Value<bool>,

    pub parent: Vec<String>,
    pub content: Value<String>,

    pub new_name: NewName,
    pub save_enable: Computed<bool>,
}

impl AppNewcontent {
    pub fn new(app: &App) -> AppNewcontent {
        log::info!("buduję stan dla new content");
        let action_save = Value::new(false);

        let parent = transaction(|context| app.data.tab.router.get_dir(context));
        let list = app.data.tab.list.clone();

        let new_name = NewName::new(list);

        let content = Value::new(String::from(""));


        let save_enable = {
            let content = content.to_computed();
            let is_valid = new_name.is_valid.clone();

            Computed::from(move |context| -> bool {
                if !is_valid.get(context)  {
                    return false;
                }

                let content = content.get(context);
                if content.is_empty() {
                    return false;
                }

                true
            })
        };

        AppNewcontent {
            app: app.clone(),
            action_save,
            
            parent,
            content,

            new_name,
            save_enable,
        }
    }

    pub fn render(&self) -> DomElement {
        app_newcontent_render(
            self.app.clone(),
            self,
        )
    }

    pub fn on_input_content(&self, new_value: String) {
        transaction(|context| {
            let action_save = self.action_save.get(context);

            if action_save {
                log::error!("Trwa obecnie zapis");
                return;
            }

            self.content.set(new_value);
        });
    }

    pub fn on_save(&self) -> impl Fn() {
        let state = self;

        bind_spawn!(state, async move {
            let action_save = transaction(|context| {
                state.action_save.get(context)
            });

            if action_save {
                log::error!("Trwa obecnie zapis");
                return;
            }

            state.action_save.set(true);

            let (new_name, body) = transaction(|context| {
                let new_name = state.new_name.name.get(context);

                (
                    new_name.clone(),
                    HandlerCreateFileBody {
                        path: state.parent.clone(),
                        new_name,
                        new_content: state.content.get(context),
                    }
                )
            });
            
            let response = get_driver()
                .request("/create_file")
                .body_json(body)
                .post()
                .await;

            state.action_save.set(false);
            
            match check_request_response(response) {
                Ok(()) => {       
                    let path_redirect = state.parent.clone(); 
                    log::info!("Zapis udany -> przekierowanie na -> {:?} {:?}", path_redirect, new_name);
                    state.app.redirect_to_index_with_path(path_redirect, Some(new_name));
                },
                Err(message) => {
                    state.app.show_message_error(message, Some(10000));
                }
            };
        })
    }
}
