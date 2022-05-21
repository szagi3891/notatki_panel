
use common::{HandlerSaveContentBody};
use vertigo::{Computed, Value, VDomComponent, bind, get_driver};

use crate::{app::{App, response::check_request_response}, data::{Data, ContentView}};
use super::app_editcontent_render::app_editcontent_render;

#[derive(Clone)]
pub struct EditContent {
    pub content: String,
    pub hash: String,
}

#[derive(Clone)]
pub struct AppEditcontent {
    pub data: Data,
    pub path: Vec<String>,          //edutowany element
    // pub hash: String,               //hash poprzedniej zawartosci

    pub action_save: Value<bool>,
    pub content_edit: Value<Option<EditContent>>,
    pub save_enable: Computed<bool>,

    pub content_view: Computed<Option<EditContent>>,        //None - ładowanie
}

impl AppEditcontent {
    pub fn new(
        data: Data,
        path: Vec<String>,
    ) -> AppEditcontent {
        let content_edit = Value::<Option<EditContent>>::new(None);

        let save_enable = {
            let content_edit = content_edit.to_computed();

            Computed::from(move || -> bool {
                let content_edit = content_edit.get();
                content_edit.is_some()
                // let save_enabled = edit_content != content;
                // save_enabled

                //TODO - to miejsce mona ciut usprawnić
            })
        };

        let content_view = {
            let data = data.clone();
            let path = path.clone();
            let content_edit = content_edit.to_computed();

            Computed::from(move || {
                let content_edit = content_edit.get();

                if let Some(content_edit) = content_edit {
                    return Some(content_edit);
                }

                println!("ładowanie danych {path:?}");

                if let Some(ContentView { id, content }) = data.git.get_content(&path) {
                    let content = (*content).clone();

                    return Some(EditContent {
                        content,
                        hash: id,
                    });
                }

                None
            })
        };

        AppEditcontent {
            data,
            path,

            action_save: Value::new(false),
            content_edit,
            save_enable,
            content_view,
        }
    }

    pub fn render(&self, app: &App) -> VDomComponent {
        app_editcontent_render(app, self)
    }

    pub fn on_input(&self, new_text: String, new_hash: String) {
        let action_save = self.action_save.get();

        if action_save {
            log::error!("Trwa obecnie zapis");
            return;
        }

        self.content_edit.set(Some(EditContent {
            content: new_text,
            hash: new_hash,
        }));
    }

    pub fn on_save(&self, app: &App, and_back_to_view: bool) -> impl Fn() {
        bind(self)
            .and(app)
            .and(&and_back_to_view)
            .spawn(|state, app, and_back_to_view| async move {
                        
                let action_save = state.action_save.get();

                if action_save {
                    log::error!("Trwa obecnie zapis");
                    return;
                }

                let content_edit = match state.content_edit.get() {
                    Some(content_edit) => content_edit,
                    None => {
                        log::error!("Brak danych do zapisania");
                        return;
                    }
                };


                state.action_save.set(true);

                let body: HandlerSaveContentBody = HandlerSaveContentBody {
                    path: state.path.clone(),
                    prev_hash: content_edit.hash,
                    new_content: content_edit.content,
                };

                let response = get_driver()
                    .request("/save_content")
                    .body_json(body)
                    .post().await;

                state.action_save.set(false);

                match check_request_response(response) {
                    Ok(()) => {
                        log::info!("Zapis udany");

                        state.content_edit.set(None);

                        if and_back_to_view {
                            app.redirect_to_index_with_root_refresh();        
                        } else {
                            app.show_message_info("Zapis udany", Some(5000));
                            app.data.git.root.refresh();
                        }
                    },
                    Err(message) => {
                        app.show_message_error(message, Some(2000));
                    }
                }
            })
    }
}

