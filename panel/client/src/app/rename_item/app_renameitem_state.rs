use common::{HandlerRenameItemBody};
use vertigo::{Driver, Computed, Value, VDomComponent};

use crate::{app::App, data::Data};

use super::app_renameitem_render::app_renameitem_render;

#[derive(Clone)]
pub struct AppRenameitem {
    driver: Driver,

    pub path: Vec<String>,          //edutowany element
    pub prev_name: String,
    pub prev_hash: String,               //hash poprzedniej zawartosci
    pub prev_content: Option<String>,

    pub new_name: Value<String>,
    pub action_save: Value<bool>,

    pub save_enable: Computed<bool>,
}

impl AppRenameitem {
    pub fn new(
        data: &Data,
        path: Vec<String>,
        prev_name: String,
        prev_hash: String,
        prev_content: Option<String>,
    ) -> AppRenameitem {
        let new_name = data.driver.new_value(prev_name.clone());

        let save_enable = {
            let prev_name = prev_name.clone();
            let new_name = new_name.to_computed();

            data.driver.from(move || -> bool {
                let new_name = new_name.get_value();
                
                if new_name.as_ref().trim() == "" {
                    return false;
                }

                if new_name.as_ref() != &prev_name {
                    return true;
                }

                false
            })
        };

        let action_save = data.driver.new_value(false);

        AppRenameitem {
            driver: data.driver.clone(),

            path,
            prev_name,
            prev_hash,
            prev_content,

            new_name,

            action_save,
            save_enable,
        }
    }

    pub fn render(&self, app: &App) -> VDomComponent {
        app_renameitem_render(self.clone(), app.clone())
    }

    pub fn get_full_path(&self) -> String {
        let mut path = self.path.clone();
        let prev_name = self.prev_name.clone();

        path.push(prev_name);

        path.as_slice().join("/")
    }

    pub fn on_input(&self, new_text: String) {
        let action_save = self.action_save.get_value();

        if *action_save {
            log::error!("Trwa obecnie zapis");
            return;
        }

        self.new_name.set_value(new_text);
    }

    pub fn on_save(&self, app: &App) -> impl Fn() {
        let driver = self.driver.clone();
        let state = self.clone();
        let app = app.clone();

        move || {
            let state = state.clone();
            let app = app.clone();

            driver.spawn(async move {
                                
                let action_save = state.action_save.get_value();

                if *action_save {
                    log::error!("Trwa obecnie zapis");
                    return;
                }

                state.action_save.set_value(true);

                let body: HandlerRenameItemBody = HandlerRenameItemBody {
                    path: state.path.clone(),
                    prev_name: state.prev_name.clone(),
                    prev_hash: state.prev_hash.clone(),
                    new_name: (*state.new_name.get_value()).clone(),
                };

                let _ = state.driver
                    .request("/rename_item")
                    .body_json(body)
                    .post()
                    .await;

                let redirect_path = state.path.clone();
                let redirect_new_name = state.new_name.get_value().as_ref().clone();

                log::info!("Zapis udany");

                app.redirect_to_index_with_path(redirect_path, Some(redirect_new_name));
            });
        }
    }
}

