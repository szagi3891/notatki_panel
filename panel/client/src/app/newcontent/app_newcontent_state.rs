use common::{HandlerCreateFileBody};
use vertigo::{Computed, Value, VDomComponent, bind, get_driver};

use crate::app::App;
use crate::app::newcontent::app_newcontent_render::app_newcontent_render;
use crate::components::new_name::NewName;
use crate::data::Data;

#[derive(Clone)]
pub struct AppNewcontent {
    pub action_save: Value<bool>,

    pub parent: Vec<String>,
    pub content: Value<String>,

    pub new_name: NewName,
    pub save_enable: Computed<bool>,
}

impl AppNewcontent {
    pub fn new(data: &Data) -> AppNewcontent {
        log::info!("buduję stan dla new content");
        let action_save = Value::new(false);

        let parent = data.tab.dir_select.clone().get_value();
        let list = data.tab.list.clone();

        let new_name = NewName::new(list);

        let content = Value::new(String::from(""));


        let save_enable = {
            let content = content.to_computed();
            let is_valid = new_name.is_valid.clone();

            Computed::from(move || -> bool {
                let new_name_is_valid = is_valid.get_value();

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

        AppNewcontent {
            action_save,
            
            parent: parent.as_ref().clone(),
            content,

            new_name,
            save_enable,
        }
    }

    pub fn render(&self, app: &App) -> VDomComponent {
        app_newcontent_render(
            &self,
            app.clone()
        )
    }

    pub fn on_input_content(&self, new_value: String) {
        let action_save = self.action_save.get_value();

        if *action_save {
            log::error!("Trwa obecnie zapis");
            return;
        }

        self.content.set_value(new_value);
    }

    pub fn on_save(&self, app: &App) -> impl Fn() {
        bind(self)
            .and(app)
            .spawn(|state, app| async move {
                let action_save = state.action_save.get_value();

                if *action_save {
                    log::error!("Trwa obecnie zapis");
                    return;
                }

                state.action_save.set_value(true);

                let new_name_rc = state.new_name.name.get_value();
                let new_name = (*new_name_rc).clone();

                let body: HandlerCreateFileBody = HandlerCreateFileBody {
                    path: state.parent.clone(),
                    new_name: new_name.clone(),
                    new_content: (*state.content.get_value()).clone(),
                };

                let _ = get_driver()
                    .request("/create_file")
                    .body_json(body)
                    .post()
                    .await;

                let path_redirect = state.parent.clone(); 
                log::info!("Zapis udany -> przekierowanie na -> {:?} {:?}", path_redirect, new_name);
                app.redirect_to_index_with_path(path_redirect, Some(new_name));
            })
    }
}
