
use common::{HandlerSaveContentBody};
use vertigo::{Driver, Computed, Value, VDomComponent};

use crate::{app::App, data::Data};
use super::app_editcontent_render::app_editcontent_render;

#[derive(Clone)]
pub struct AppEditcontent {
    pub driver: Driver,

    pub path: Vec<String>,          //edutowany element
    pub hash: String,               //hash poprzedniej zawartosci

    pub action_save: Value<bool>,
    pub edit_content: Value<String>,
    pub save_enable: Computed<bool>,
}

impl AppEditcontent {
    pub fn new(
        data: &Data,
        path: Vec<String>,
        hash: String,
        content: String,
    ) -> AppEditcontent {
        let edit_content = data.driver.new_value(content.clone());

        let save_enable = {
            let edit_content = edit_content.to_computed();

            data.driver.from(move || -> bool {
                let edit_content = edit_content.get_value();
                let save_enabled = edit_content.as_ref() != &content;
                save_enabled
            })
        };

        let action_save = data.driver.new_value(false);

        AppEditcontent {
            driver: data.driver.clone(),

            path,
            hash,

            action_save,
            edit_content,
            save_enable,
        }
    }

    pub fn render(&self, app: &App) -> VDomComponent {
        app_editcontent_render(app, self.clone())
    }

    pub fn on_input(&self, new_text: String) {
        let action_save = self.action_save.get_value();

        if *action_save {
            log::error!("Trwa obecnie zapis");
            return;
        }

        self.edit_content.set_value(new_text);
    }

    pub async fn on_save(self, app: App) {
        let action_save = self.action_save.get_value();

        if *action_save {
            log::error!("Trwa obecnie zapis");
            return;
        }

        self.action_save.set_value(true);

        let body: HandlerSaveContentBody = HandlerSaveContentBody {
            path: self.path.clone(),
            prev_hash: self.hash.clone(),
            new_content: (*self.edit_content.get_value()).clone(),
        };

        let _ = self.driver
            .request("/save_content")
            .body_json(body)
            .post().await;

        log::info!("Zapis udany");
    
        app.redirect_to_index_with_root_refresh();
    }

    pub fn bind_on_save(&self, app: &App) -> impl Fn() {
        let driver = self.driver.clone();
        let state = self.clone();
        let app = app.clone();
        move || {
            let app = app.clone();
            driver.spawn(state.clone().on_save(app));
        }
    }
}

