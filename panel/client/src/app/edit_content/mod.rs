mod render;

pub use render::render;


use std::rc::Rc;

use common::{HandlerSaveContentBody};
use vertigo::{Driver, Computed, Value, VDomComponent};

use crate::{app::StateApp};

#[derive(PartialEq)]
pub struct StateAppEditContent {
    driver: Driver,

    pub path: Vec<String>,          //edutowany element
    pub hash: String,               //hash poprzedniej zawartosci

    pub action_save: Value<bool>,
    pub edit_content: Value<String>,
    pub save_enable: Computed<bool>,

    app_state: Rc<StateApp>,
}

impl StateAppEditContent {
    pub fn redirect_to_index(&self) {
        self.app_state.redirect_to_index();
    }

    pub fn new(
        app_state: Rc<StateApp>,
        path: Vec<String>,
        hash: String,
        content: String,
    ) -> VDomComponent {
        let edit_content = app_state.driver.new_value(content.clone());

        let save_enable = {
            let edit_content = edit_content.to_computed();

            app_state.driver.from(move || -> bool {
                let edit_content = edit_content.get_value();
                let save_enabled = edit_content.as_ref() != &content;
                save_enabled
            })
        };

        let action_save = app_state.driver.new_value(false);

        let state = StateAppEditContent {
            driver: app_state.driver.clone(),

            path,
            hash,

            action_save,
            edit_content,
            save_enable,
            app_state: app_state.clone()
        };

        app_state.driver.bind_render(state, render::render)
    }

    pub fn on_input(&self, new_text: String) {
        let action_save = self.action_save.get_value();

        if *action_save {
            log::error!("Trwa obecnie zapis");
            return;
        }

        self.edit_content.set_value(new_text);
    }

    pub fn on_save(&self) {
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

        let request = self.driver
            .request("/save_content")
            .body_json(body)
            .post();

        let callback = self.app_state.clone();

        self.driver.spawn(async move {

            let _ = request.await;

            log::info!("Zapis udany");

            callback.redirect_to_index_with_root_refresh();
        });
    }
}

