use std::rc::Rc;

use vertigo::{Resource, get_driver, transaction, dom, bind, DomNode, Computed};
use vertigo::Value;
use crate::components::{message_box, MessageBoxType, stict_to_top, ButtonState};
use crate::data::Data;

use crate::app::edit_content::AppEditcontent;
use super::alert::{AppIndexAlert};
use super::app_index_render;
use crate::app::new_dir::AppNewdir;
use crate::app::newcontent::AppNewcontent;
use crate::app::rename_item::AppRenameitem;
use vertigo::struct_mut::CounterMut;

#[derive(Clone, PartialEq, Eq)]
struct Error {
    id: u32,
    info: MessageBoxType,
    message: String,
}


#[derive(Clone, PartialEq)]
enum View {
    Index,
    EditContent { state: AppEditcontent },
    RenameItem { state: AppRenameitem },
    NewContent { state: AppNewcontent },
    Mkdir { state: AppNewdir },
}

#[derive(Clone)]
pub struct App {
    pub data: Data,
    pub alert: AppIndexAlert,
    view: Value<View>,

    next_id: Rc<CounterMut>,
    errors: Value<Vec<Error>>,
}

impl PartialEq for App {
    fn eq(&self, other: &Self) -> bool {
        self.view.id() == other.view.id()
    }
}

impl App {
    pub fn new() -> App {
        let data = Data::new();

        let view = Value::new(View::Index);

        let next_id = Rc::new(CounterMut::new(1));

        let alert = AppIndexAlert::new(data.clone());

        App {
            data,
            alert,
            view,
            next_id,
            errors: Value::new(Vec::new()),
        }
    }

    pub fn redirect_to_edit_content(&self, full_path: Vec<String>) {
        let state = AppEditcontent::new(
            self,
            full_path,
        );

        self.view.set(View::EditContent {
            state
        });
    }

    pub fn redirect_to_index(&self) {
        log::info!("redirect_to_index");
        self.view.set(View::Index);
    }

    pub fn redirect_to_index_with_path(&self, new_path: Vec<String>, new_item: Option<String>) {
        self.redirect_to_index();

        self.data.tab.redirect_to(new_path, new_item);
        self.data.git.root.refresh();
    }

    pub fn redirect_to_mkdir(&self) {
        let state = AppNewdir::new(self);

        self.view.set(View::Mkdir {
            state
        });
    }

    pub fn redirect_to_index_with_root_refresh(&self) {
        self.data.git.root.refresh();
        self.redirect_to_index();
    }

    pub fn redirect_to_new_content(&self) {
        let state = AppNewcontent::new(self);
        self.view.set(View::NewContent { state });
    }

    // pub fn current_edit(&self, context: &Context) {
    //     let current_list_item = self.data.tab.current_list_item.get(context);

    //     let Some(current_list_item) = current_list_item else {
    //         log::error!("current_edit - ignore");
    //         return;
    //     };

    //     self.redirect_to_edit_content(current_list_item.full_path.as_ref());
    // }

    pub fn render_current_rename(&self) -> DomNode {
        ButtonState::render({    
            Computed::from({                
                let app = self.clone();

                move |context| {
                    let path = app.data.tab.select_dir.get(context).full_path;

                    let Some(current_list_item) = app.data.tab.current_list_item.get(context) else {
                        return ButtonState::disabled("Zmień nazwę");
                    };
                    let select_item = current_list_item.name();

                    let Resource::Ready(id) = current_list_item.id.get(context) else {
                        return ButtonState::disabled("Zmień nazwę");
                    };
        
                    ButtonState::active("Zmień nazwę", bind!(app, path, select_item, || {
                        let base_path = path.as_ref().clone();
            
                        log::info!("redirect_to_rename_item {base_path:?} {select_item:?}");
            
                        let state = AppRenameitem::new(
                            &app,
                            base_path,
                            select_item.clone(),
                            id.clone(),
                        );
        
                        app.view.set(View::RenameItem {
                            state
                        });
                    }))
                }
            })
        })
    }

    fn message_add(&self, info: MessageBoxType, message: String) -> u32 {
        let message_id = self.next_id.get_next();

        let error = Error {
            id: message_id,
            info,
            message,
        };

        self.errors.change(|messages| {
            messages.push(error);
        });

        message_id
    }

    fn message_off_with_timeout(&self, message_id: u32, timeout: u32) {
        let self_copy = self.clone();

        get_driver().spawn(async move {
            get_driver().sleep(timeout).await;
            self_copy.remove_message(message_id);
        });
    }

    pub fn show_message_error(&self, message: impl Into<String>, timeout: Option<u32>) {
        let message_id = self.message_add(MessageBoxType::Error, message.into());
        if let Some(timeout) = timeout {
            self.message_off_with_timeout(message_id, timeout);
        }
    }

    pub fn show_message_info(&self, message: impl Into<String>, timeout: Option<u32>) {
        let message_id = self.message_add(MessageBoxType::Info, message.into());
        if let Some(timeout) = timeout {
            self.message_off_with_timeout(message_id, timeout);
        }
    }

    pub fn remove_message(&self, message_id: u32) {
        transaction(|context| {
            let mut messages = self.errors.get(context);
            messages.retain(|item| item.id != message_id);
            self.errors.set(messages);
        })
    }

    pub fn keydown(&self, code: String) -> bool {
        if self.alert.is_visible() {
            if code == "Escape" {
                self.alert.close_modal();
                return true;
            }

            //TODO - dodać wskaźnik i nawigację klawiaturą po elemencie z listy wyników

            return false;
        }

        if code == "ArrowUp" {
            self.data.tab.pointer_up();
            return true;
        } else if code == "ArrowDown" {
            self.data.tab.pointer_down();
            return true;
        } else if code == "Escape" {
            self.data.tab.pointer_escape();
            return true;
        } else if code == "ArrowRight" || code == "Enter" {
            self.data.tab.pointer_enter();
            return true;
        } else if code == "ArrowLeft" || code == "Backspace" || code == "Escape" {
            self.data.tab.backspace();
            return true;
        }

        log::info!("klawisz ... {:?}", code);
        false
    }

    pub fn render(&self) -> DomNode {
        let view = render_view(self);
        let errors = render_errors(self);

        dom! {
            <html>
                <head>
                    <title>"Notatki"</title>
                    <meta charset="utf-8"/>
                    <style type="text/css">"
                        * {
                            box-sizing: border-box;
                        }
                        html, body {
                            width: 100%;
                            height: 100%;
                            margin: 0;
                            padding: 0;
                            border: 0;
                        }
                    "</style>
                </head>
                <body>
                    <div>
                        { view }
                        { errors }
                    </div>
                </body>
            </html>
        }
    }
}


fn render_view(state: &App) -> DomNode {
    let app = app_render(state);
    state.data.tab.open_links.render(app)
}

fn render_error_one(state: &App, error: Error) -> DomNode {
    let Error { id, info, message } = error;
    let state = state.clone();

    let on_remove = bind!(state, id, || {
        state.remove_message(id);
    });

    message_box(info, message, on_remove)
}

fn render_errors(state: &App) -> DomNode {
    let errors_view = state.errors.render_list(|error| error.id, {
        let state = state.clone();
        move |error| {
            render_error_one(&state, error.clone())
        }
    });

    stict_to_top(dom! {
        <div>
            { errors_view }
        </div>
    })
}


fn app_render(app: &App) -> DomNode {
    app.view.render_value({
        let app = app.clone();
        move |view| {
            match view {
                View::Index => {
                    dom! {
                        <div id="root">
                            { app_index_render(&app) }
                        </div>
                    }
                },
                View::EditContent { state } => {
                    dom! {
                        <div id="root">
                            { state.render() }
                        </div>
                    }
                },
                View::NewContent { state } => {
                    dom! {
                        <div id="root">
                            { state.render() }
                        </div>
                    }
                },
                View::RenameItem {state } => {
                    dom! {
                        <div id="root">
                            { state.render() }
                        </div>
                    }
                },
                View::Mkdir { state } => {
                    dom! {
                        <div id="root">
                            { state.render() }
                        </div>
                    }
                },
            }
        }
    })
}
