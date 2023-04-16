use common::{HandlerRenameItemBody};
use vertigo::{Computed, Value, get_driver, bind, transaction, bind_spawn, DomNode};

use crate::{app::{App, response::check_request_response}, components::ButtonState, data::ListItem};

use super::app_renameitem_render::app_renameitem_render;

#[derive(Clone, PartialEq)]
pub struct AppRenameitem {
    pub app: App,
    pub item: ListItem,                 //edutowany element
    pub prev_hash: String,              //hash poprzedniej zawartosci

    pub new_name: Value<String>,
    pub action_save: Value<bool>,

    save_enable: Computed<bool>,
}

impl AppRenameitem {
    pub fn new(
        app: &App,
        item: ListItem,
        prev_hash: String,
    ) -> AppRenameitem {
        let new_name = Value::new(item.name());

        let save_enable = {
            let prev_name = item.name();
            let new_name = new_name.to_computed();

            Computed::from(move |context| -> bool {
                let new_name = new_name.get(context);
                
                if new_name.trim() == "" {
                    return false;
                }

                if new_name != prev_name {
                    return true;
                }

                //TODO - dodać sprawdzenie, czy ta nowa nazwa występuje w katalogu w którym jesteśmy

                false
            })
        };

        let action_save = Value::new(false);

        AppRenameitem {
            app: app.clone(),
            item,
            prev_hash,

            new_name,

            action_save,
            save_enable,
        }
    }

    pub fn render(&self) -> DomNode {
        app_renameitem_render(self)
    }

    #[deprecated]
    pub fn get_full_path(&self) -> String {
        self.item.to_string_path()
    }

    pub fn on_input(&self, new_text: String) {
        transaction(|context| {
            let action_save = self.action_save.get(context);

            if action_save {
                log::error!("Trwa obecnie zapis");
                return;
            }

            self.new_name.set(new_text);
        });
    }

    async fn on_save(&self) -> Result<(), String> {
        let body: HandlerRenameItemBody = transaction(|context| {
            HandlerRenameItemBody {
                path: self.item.dir(),
                prev_name: self.item.name(),
                prev_hash: self.prev_hash.clone(),
                new_name: self.new_name.get(context),
            }
        });

        let response = get_driver()
            .request_post("/rename_item")
            .body_json(body)
            .call()
            .await;

        check_request_response(response)
    }

    pub fn button_on_back(&self) -> DomNode {
        ButtonState::render({
            let app = self.app.clone();

            Computed::from(move |_| ButtonState::active("Wróć", bind!(app, || {
                app.redirect_to_index();
            })))
        })
    }

    pub fn button_on_save(&self) -> DomNode {
        ButtonState::render({
            let state = self.clone();
            let app = self.app.clone();

            Computed::from(move |context| {
                if state.action_save.get(context) {
                    return ButtonState::process("Zapisywanie ...");
                }

                match state.save_enable.get(context) {
                    true => {
                        let state = state.clone();

                        let action = bind_spawn!(state, app, async move {
                            let action_save = transaction(|context| {
                                state.action_save.get(context)
                            });

                            if action_save {
                                log::error!("Trwa obecnie zapis");
                                return;
                            }

                            state.action_save.set(true);
                            let response = state.on_save().await;
                            state.action_save.set(false);

                            match response {
                                Ok(()) => {
                                    let redirect_path = state.item.dir();
                                    let redirect_new_name = transaction(|context| {
                                        state.new_name.get(context)
                                    });

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
                        ButtonState::disabled("Zapisz zmianę nazwy")
                    }
                }
            })
        })
    }

}
