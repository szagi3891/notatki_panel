use common::{HandlerRenameItemBody};
use vertigo::{Computed, Value, VDomComponent, get_driver, bind};

use crate::{app::{App, response::check_request_response}, data::Data, components::ButtonState};

use super::app_renameitem_render::app_renameitem_render;

#[derive(Clone)]
pub struct AppRenameitem {
    pub data: Data,
    pub path: Vec<String>,          //edutowany element
    pub prev_name: String,
    pub prev_hash: String,               //hash poprzedniej zawartosci

    pub new_name: Value<String>,
    pub action_save: Value<bool>,

    save_enable: Computed<bool>,
}

impl AppRenameitem {
    pub fn new(
        data: Data,
        path: Vec<String>,
        prev_name: String,
        prev_hash: String,
    ) -> AppRenameitem {
        let new_name = Value::new(prev_name.clone());

        let save_enable = {
            let prev_name = prev_name.clone();
            let new_name = new_name.to_computed();

            Computed::from(move || -> bool {
                let new_name = new_name.get();
                
                if new_name.trim() == "" {
                    return false;
                }

                if new_name != prev_name {
                    return true;
                }

                false
            })
        };

        let action_save = Value::new(false);

        AppRenameitem {
            data,
            path,
            prev_name,
            prev_hash,

            new_name,

            action_save,
            save_enable,
        }
    }

    pub fn render(&self, app: &App) -> VDomComponent {
        app_renameitem_render(self, app.clone())
    }

    pub fn get_full_path(&self) -> String {
        let mut path = self.path.clone();
        let prev_name = self.prev_name.clone();

        path.push(prev_name);

        path.as_slice().join("/")
    }

    pub fn on_input(&self, new_text: String) {
        let action_save = self.action_save.get();

        if action_save {
            log::error!("Trwa obecnie zapis");
            return;
        }

        self.new_name.set(new_text);
    }

    async fn on_save(&self) -> Result<(), String> {
        let body: HandlerRenameItemBody = HandlerRenameItemBody {
            path: self.path.clone(),
            prev_name: self.prev_name.clone(),
            prev_hash: self.prev_hash.clone(),
            new_name: self.new_name.get(),
        };

        let response = get_driver()
            .request("/rename_item")
            .body_json(body)
            .post()
            .await;

        check_request_response(response)
    }


    pub fn button_on_save(&self, app: &App) -> Computed<ButtonState> {
        Computed::from({
            let state = self.clone();
            let app = app.clone();

            move || {
                if state.action_save.get() {
                    return ButtonState::Process { label: "Zapisywanie ...".into() };
                }

                match state.save_enable.get() {
                    true => {
                        let state = state.clone();

                        let action = bind(&state)
                            .and(&app)
                            .spawn(|state, app| async move {
                                let action_save = state.action_save.get();

                                if action_save {
                                    log::error!("Trwa obecnie zapis");
                                    return;
                                }

                                state.action_save.set(true);

                                let response = state.on_save().await;

                                match response {
                                    Ok(()) => {  
                                        let redirect_path = state.path.clone();
                                        let redirect_new_name = state.new_name.get();

                                        log::info!("Zapis udany");

                                        app.redirect_to_index_with_path(redirect_path, Some(redirect_new_name));
                                    },
                                    Err(message) => {
                                        app.show_message_error(message, Some(10000));
                                    }
                                };
                            });

                        ButtonState::active("Zapisz zmianę nazwy", action)
                    },
                    false => {
                        ButtonState::Disabled { label: "Zapisz zmianę nazwy".into() }
                    }
                }
            }
        })
    }

}
