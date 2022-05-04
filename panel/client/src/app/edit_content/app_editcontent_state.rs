
use common::{HandlerSaveContentBody};
use vertigo::{Computed, Value, VDomComponent, bind, get_driver};

use crate::{app::App};
use super::app_editcontent_render::app_editcontent_render;

#[derive(Clone)]
pub struct AppEditcontent {
    pub path: Vec<String>,          //edutowany element
    pub hash: String,               //hash poprzedniej zawartosci

    pub action_save: Value<bool>,
    pub edit_content: Value<String>,
    pub save_enable: Computed<bool>,
}

impl AppEditcontent {
    pub fn new(
        path: Vec<String>,
        hash: String,
        content: String,
    ) -> AppEditcontent {
        let edit_content = Value::new(content.clone());

        let save_enable = {
            let edit_content = edit_content.to_computed();

            Computed::from(move || -> bool {
                let edit_content = edit_content.get();
                let save_enabled = edit_content.as_ref() != &content;
                save_enabled
            })
        };

        let action_save = Value::new(false);

        AppEditcontent {
            path,
            hash,

            action_save,
            edit_content,
            save_enable,
        }
    }

    pub fn render(&self, app: &App) -> VDomComponent {
        app_editcontent_render(app, self)
    }

    pub fn on_input(&self, new_text: String) {
        let action_save = self.action_save.get();

        if *action_save {
            log::error!("Trwa obecnie zapis");
            return;
        }

        self.edit_content.set(new_text);
    }

    pub fn on_save(&self, app: &App) -> impl Fn() {
        bind(self)
            .and(app)
            .spawn(|state, app| async move {
                        
                let action_save = state.action_save.get();

                if *action_save {
                    log::error!("Trwa obecnie zapis");
                    return;
                }

                state.action_save.set(true);

                let body: HandlerSaveContentBody = HandlerSaveContentBody {
                    path: state.path.clone(),
                    prev_hash: state.hash.clone(),
                    new_content: (*state.edit_content.get()).clone(),
                };

                let _ = get_driver()
                    .request("/save_content")
                    .body_json(body)
                    .post().await;

                log::info!("Zapis udany");
            
                app.redirect_to_index_with_root_refresh();
            })
    }
}

