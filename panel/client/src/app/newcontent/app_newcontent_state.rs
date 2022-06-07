use common::{HandlerCreateFileBody};
use vertigo::{Computed, Value, VDomComponent, bind, get_driver};

use crate::app::App;
use crate::app::newcontent::app_newcontent_render::app_newcontent_render;
use crate::app::response::check_request_response;
use crate::components::new_name::NewName;

#[derive(Clone)]
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

        let parent = app.data.tab.router.get_dir();
        let list = app.data.tab.list.clone();

        let new_name = NewName::new(list);

        let content = Value::new(String::from(""));


        let save_enable = {
            let content = content.to_computed();
            let is_valid = new_name.is_valid.clone();

            Computed::from(move || -> bool {
                if !is_valid.get()  {
                    return false;
                }

                let content = content.get();
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

    pub fn render(&self) -> VDomComponent {
        app_newcontent_render(
            self.app.clone(),
            &self,
        )
    }

    pub fn on_input_content(&self, new_value: String) {
        let action_save = self.action_save.get();

        if action_save {
            log::error!("Trwa obecnie zapis");
            return;
        }

        self.content.set(new_value);
    }

    pub fn on_save(&self) -> impl Fn() {
        bind(self)
            .and(&self.app)
            .spawn(|state, app| async move {
                let action_save = state.action_save.get();

                if action_save {
                    log::error!("Trwa obecnie zapis");
                    return;
                }

                state.action_save.set(true);

                let new_name = state.new_name.name.get();

                let body: HandlerCreateFileBody = HandlerCreateFileBody {
                    path: state.parent.clone(),
                    new_name: new_name.clone(),
                    new_content: state.content.get(),
                };

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
                        app.redirect_to_index_with_path(path_redirect, Some(new_name));
                    },
                    Err(message) => {
                        app.show_message_error(message, Some(10000));
                    }
                }
            })
    }
}
