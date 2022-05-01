use common::{HandlerCreateDirBody};
use vertigo::{Driver, Computed, Value, VDomComponent, bind};

use crate::app::App;
use crate::components::new_name::{self, NewName};
use crate::data::Data;

use super::app_newdir_render::app_newdir_render;

#[derive(Clone)]
pub struct AppNewdir {
    pub driver: Driver,

    pub action_save: Value<bool>,

    pub parent: Vec<String>,
    pub new_name: NewName,

    pub save_enable: Computed<bool>,
}

impl AppNewdir {
    pub fn new(data: &Data) -> AppNewdir {
        log::info!("buduję stan dla new dir");
        let action_save = data.driver.new_value(false);
        let list = data.tab.list.clone();
        let parent = data.tab.dir_select.clone().get_value();

        let new_name = new_name::NewName::new(
            &data.driver,
            list,
        );

        AppNewdir {
            driver: data.driver.clone(),

            action_save,

            parent: parent.as_ref().clone(),
            new_name: new_name.clone(),
            save_enable: new_name.is_valid.clone(),
        }
    }

    pub fn render(&self, app: App) -> VDomComponent {
        app_newdir_render(self.clone(), app)
    }

    pub fn bind_on_save(&self, app: &App) -> impl Fn() {
        bind(self)
            .and(app)
            .spawn(self.driver.clone(), |state, app| async move {
                let action_save = state.action_save.get_value();

                if *action_save {
                    log::error!("Trwa obecnie zapis");
                    return;
                }

                state.action_save.set_value(true);
            
                let new_dir_name = state.new_name.name.get_value().as_ref().clone();

                let body = HandlerCreateDirBody {
                    path: state.parent.clone(),
                    dir: new_dir_name.clone(),
                };

                let _ = state.driver
                    .request("/create_dir")
                    .body_json(body)
                    .post().await;

                let parent_string = state.parent.join("/");
                log::info!("Tworzenie katalogu {:?} udane -> przekierowanie na -> {:?}", new_dir_name, parent_string);

                app.redirect_to_index_with_path(state.parent.clone(), Some(new_dir_name));
            })
    }
}
