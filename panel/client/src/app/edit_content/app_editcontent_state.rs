
use common::{HandlerSaveContentBody};
use vertigo::{Computed, Value, bind, get_driver, Context, transaction, bind_spawn, DomNode};

use crate::{app::{App, response::check_request_response}, data::ContentView};
use super::app_editcontent_render::app_editcontent_render;

#[derive(Clone)]
pub struct EditContent {
    pub content: String,
    pub hash: Option<String>,
}


#[derive(Clone, PartialEq)]
pub struct AppEditcontent {
    app: App,
    pub path: Vec<String>,          //edutowany element

    pub action_save: Value<bool>,

    pub edit_content: Value<Option<String>>,
    pub edit_hash: Value<Option<String>>,

    pub save_enable: Computed<bool>,

    pub content_view: Computed<Option<EditContent>>,        //None - ładowanie
}

impl AppEditcontent {
    pub fn new(
        app: &App,
        path: Vec<String>,
    ) -> AppEditcontent {
        let edit_content = Value::<Option<String>>::new(None);
        let edit_hash = Value::<Option<String>>::new(None);

        let save_enable = {
            let data = app.data.clone();
            let path = path.clone();

            let edit_content = edit_content.to_computed();

            Computed::from(move |context| -> bool {
                let edit_content = edit_content.get(context);
                if let Some(edit_content) = edit_content {
                    if let Some(ContentView { id: _, content }) = data.get_content(context, &path) {
                        return edit_content != *content;
                    }
                }

                false
            })
        };

        let content_view = {
            let data = app.data.clone();
            let path = path.clone();
            let edit_content = edit_content.to_computed();
            let edit_hash = edit_hash.to_computed();

            Computed::from(move |context| -> Option<EditContent> {
                let edit_content = edit_content.get(context);
                let edit_hash = edit_hash.get(context);

                if let (Some(content_edit), Some(edit_hash)) = (&edit_content, edit_hash) {
                    return Some(EditContent {
                        content: content_edit.clone(),
                        hash: Some(edit_hash)
                    });
                }

                println!("ładowanie danych {path:?}");

                if let Some(ContentView { id, content }) = data.get_content(context, &path) {
                    let content = (*content).clone();

                    return Some(EditContent {
                        content,
                        hash: Some(id),
                    });
                }

                if let Some(content_edit) = edit_content {
                    return Some(EditContent {
                        content: content_edit,
                        hash: None,
                    });
                }

                None
            })
        };

        AppEditcontent {
            app: app.clone(),
            path,

            action_save: Value::new(false),

            edit_content,
            edit_hash,

            save_enable,
            content_view,
        }
    }

    pub fn render(&self) -> DomNode {
        app_editcontent_render(&self.app, self)
    }

    pub fn on_input(&self, context: &Context, new_text: String, new_hash: String) {
        let action_save = self.action_save.get(context);

        if action_save {
            log::error!("Trwa obecnie zapis");
            return;
        }

        self.edit_content.set(Some(new_text));
        self.edit_hash.set(Some(new_hash));
    }

    pub fn on_reset(&self) -> impl Fn() {
        let state = self;
        bind!(state, || {
            state.edit_content.set(None);
            state.edit_hash.set(None);
        })
    }

    pub fn on_save(&self, app: &App, and_back_to_view: bool) -> impl Fn() {
        let state = self.clone();

        bind_spawn!(state, app, and_back_to_view, async move {
                    
            let action_save = transaction(|context| state.action_save.get(context));

            if action_save {
                log::error!("Trwa obecnie zapis");
                return;
            }

            let content_edit = match transaction(|context| state.edit_content.get(context)) {
                Some(content_edit) => content_edit,
                None => {
                    log::error!("Brak danych do zapisania");
                    return;
                }
            };

            let content_edit_hash = match transaction(|context| state.edit_hash.get(context)) {
                Some(content_edit_hash) => content_edit_hash,
                None => {
                    log::error!("Brak hasha danych");
                    return;
                }
            };


            state.action_save.set(true);

            let body: HandlerSaveContentBody = HandlerSaveContentBody {
                path: state.path.clone(),
                prev_hash: content_edit_hash,
                new_content: content_edit,
            };

            let response = get_driver()
                .request_post("/save_content")
                .body_json(body)
                .call().await;

            state.action_save.set(false);

            match check_request_response(response) {
                Ok(()) => {
                    log::info!("Zapis udany");

                    state.edit_hash.set(None);

                    if and_back_to_view {
                        app.redirect_to_index_with_root_refresh();        
                    } else {
                        app.show_message_info("Zapis udany", Some(5000));
                        app.data.git.root.refresh();
                    }
                },
                Err(message) => {
                    // app.data.git.root.refresh();
                    app.show_message_error(message, Some(2000));
                }
            };
        })
    }
}

