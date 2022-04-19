use common::{HandlerCreateFileBody};
use vertigo::{Driver, Computed, Value, VDomComponent};

use crate::app::App;
use crate::app::newcontent::app_newcontent_render::app_newcontent_render;
use crate::components::new_name::NewName;
use crate::data::Data;

#[derive(Clone)]
pub struct AppNewcontent {
    pub driver: Driver,

    pub action_save: Value<bool>,

    pub parent: Vec<String>,
    pub name: Value<String>,
    pub content: Value<String>,

    pub new_name: NewName,
    pub save_enable: Computed<bool>,
}

impl AppNewcontent {
    pub fn new(data: &Data) -> AppNewcontent {
        log::info!("buduję stan dla new content");
        let action_save = data.driver.new_value(false);

        let parent = data.tab.dir_select.clone().get_value();
        let list = data.tab.list.clone();

        let name = data.driver.new_value(String::from(""));
        let new_name = NewName::new(
            &data.driver,
            list,
            name.clone(),
            action_save.to_computed(),
        );

        let content = data.driver.new_value(String::from(""));


        let save_enable = {
            let content = content.to_computed();
            let is_valid = new_name.is_valid.clone();

            data.driver.from(move || -> bool {
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
            driver: data.driver.clone(),

            action_save,
            
            parent: parent.as_ref().clone(),
            name,
            content,

            new_name,
            save_enable,
        }
    }

    pub fn render(&self, app: &App) -> VDomComponent {
        app_newcontent_render(
            self.clone(),
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

    pub async fn on_save(self, app_state: App) {
        let action_save = self.action_save.get_value();

        if *action_save {
            log::error!("Trwa obecnie zapis");
            return;
        }

        self.action_save.set_value(true);

        let new_name_rc = self.name.get_value();
        let new_name = (*new_name_rc).clone();

        let body: HandlerCreateFileBody = HandlerCreateFileBody {
            path: self.parent.clone(),
            new_name: new_name.clone(),
            new_content: (*self.content.get_value()).clone(),
        };

        let _ = self.driver
            .request("/create_file")
            .body_json(body)
            .post()
            .await;

        let path_redirect = self.parent.clone(); 
        log::info!("Zapis udany -> przekierowanie na -> {:?} {:?}", path_redirect, new_name);
        app_state.redirect_to_index_with_path(path_redirect, Some(new_name));
    }

    pub fn bind_on_save(&self, app_state: App) -> impl Fn() {
        let driver = self.driver.clone();
        let state = self.clone();
        move || {            
            let app_state = app_state.clone();
            driver.spawn(state.clone().on_save(app_state));
        }
    }
}
