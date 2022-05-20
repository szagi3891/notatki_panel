use std::rc::Rc;

use vertigo::{VDomComponent, VDomElement, html, Resource, get_driver};
use vertigo::Value;
use crate::components::{message_box, MessageBoxType, stict_to_top};
use crate::data::ContentView;
use crate::data::Data;

use super::edit_content::AppEditcontent;
use super::index::AppIndex;
use super::new_dir::AppNewdir;
use super::newcontent::AppNewcontent;
use super::rename_item::AppRenameitem;
use vertigo::struct_mut::CounterMut;

#[derive(Clone)]
struct Error {
    id: u32,
    info: MessageBoxType,
    message: String,
}


#[derive(Clone)]
enum View {
    Index { state: AppIndex },
    EditContent { state: AppEditcontent },
    RenameItem { state: AppRenameitem },
    NewContent { state: AppNewcontent },
    Mkdir { state: AppNewdir },
}

#[derive(Clone)]
pub struct App {
    pub data: Data,
    view: Value<View>,

    next_id: Rc<CounterMut>,
    errors: Value<Vec<Error>>,
}

impl App {
    pub fn new() -> App {
        let data = Data::new();

        let view = Value::new(View::Index {
            state: AppIndex::new(&data)
        });

        let next_id = Rc::new(CounterMut::new(1));

        App {
            data,
            view,
            next_id,
            errors: Value::new(Vec::new()),
        }
    }

    pub fn redirect_to_edit_content(&self, full_path: &Vec<String>) {
        let full_path = full_path.clone();
        let content = self.data.git.get_content(&full_path);

        match content {
            Some(ContentView { id, content }) => {
                log::info!("redirect_to_content {full_path:?}");

                let state = AppEditcontent::new(
                    full_path.clone(),
                    id.clone(),
                    content.as_ref().clone(),
                );

                self.view.set(View::EditContent {
                    state
                });
            },
            None => {
                log::error!("Oczekiwano pliku, problem z pobraniem");
            },
        }
    }

    pub fn redirect_to_rename_item(&self, base_path: &Vec<String>, select_item: &String) {
        let select_item = select_item.clone();
        let full_path = self.data.tab.full_path.clone().get();
        let content = self.data.git.content_from_path(&full_path);

        match content {
            Resource::Ready(list_item) => {
                log::info!("redirect_to_rename_item {base_path:?} {select_item:?}");

                let state = AppRenameitem::new(
                    self.data.clone(),
                    base_path.clone(),
                    select_item,
                    list_item.id,
                );

                self.view.set(View::RenameItem {
                    state
                });
            },
            _ => {
                log::error!("redirect_to_rename_item fail - {base_path:?} {select_item:?}");
            }
        }
    }

    pub fn redirect_to_index(&self) {
        log::info!("redirect_to_index");
        self.view.set(View::Index {
            state: AppIndex::new(&self.data)
        });
    }

    pub fn redirect_to_index_with_path(&self, new_path: Vec<String>, new_item: Option<String>) {
        self.redirect_to_index();

        self.data.tab.redirect_to(new_path, new_item);
        self.data.git.root.refresh();
    }

    pub fn redirect_to_mkdir(&self) {
        let state = AppNewdir::new(&self.data);

        self.view.set(View::Mkdir {
            state
        });
    }

    pub fn redirect_to_index_with_root_refresh(&self) {
        self.data.git.root.refresh();
        self.redirect_to_index();
    }

    pub fn redirect_to_new_content(&self) {
        let state = AppNewcontent::new(&self.data);
        self.view.set(View::NewContent { state });
    }

    pub fn current_edit(&self) {
        let full_path = self.data.tab.full_path.get();
        self.redirect_to_edit_content(&full_path);
    }

    pub fn current_rename(&self) {
        let path = self.data.tab.dir_select.get();
        let select_item = self.data.tab.current_item.get();

        if let Some(select_item) = select_item.as_ref() {
            self.redirect_to_rename_item(&path, select_item);
        } else {
            log::error!("current_rename fail");
        }
    }

    fn message_add(&self, info: MessageBoxType, message: String) -> u32 {
        let message_id = self.next_id.get_next();

        let error = Error {
            id: message_id,
            info,
            message,
        };

        let mut messages = self.errors.get();
        messages.push(error);
        self.errors.set(messages);

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
        let mut messages = self.errors.get();
        messages.retain(|item| item.id != message_id);
        self.errors.set(messages);
    }

    fn render_view(&self) -> VDomComponent {
        let app = VDomComponent::from_ref(self, app_render);
        let view = self.data.tab.open_links.render(app);
        view
    }

    fn render_errors(&self) -> VDomComponent {
        VDomComponent::from_ref(self, |state| {
            let errors = state.errors.get();

            let mut list = Vec::new();

            for error in errors {
                let Error { id, info, message } = error;
                let state = state.clone();

                list.push(message_box(info, message, move || {
                    state.remove_message(id);
                }));
            }

            stict_to_top(html! {
                <div>
                    {..list}
                </div>
            })
        })
    }

    pub fn render(&self) -> VDomComponent {
        let view = self.render_view();
        let errors = self.render_errors();

        VDomComponent::from_fn(move || {
            let view = view.clone();
            let errors = errors.clone();

            html! {
                <div>
                    { view }
                    { errors }
                </div>
            }
        })
    }
}

fn app_render(app: &App) -> VDomElement {
    let view = app.view.get();

    match view {
        View::Index { state }=> {
            let view = state.render(app);

            html! {
                <div id="root">
                    { view }
                </div>
            }
        },
        View::EditContent { state } => {
            let view = state.render(app);

            html! {
                <div id="root">
                    { view }
                </div>
            }
        },
        View::NewContent { state } => {
            let view = state.render(app);

            html! {
                <div id="root">
                    { view }
                </div>
            }
        },
        View::RenameItem {state } => {
            let view = state.render(app);

            html! {
                <div id="root">
                    {view}
                </div>
            }
        },
        View::Mkdir { state } => {
            let view = state.render(app.clone());

            html! {
                <div id="root">
                    { view }
                </div>
            }
        }
    }
}
