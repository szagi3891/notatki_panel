use std::rc::Rc;

use vertigo::{VDomComponent, VDomElement, html, Resource, get_driver, Context, bind, transaction};
use vertigo::Value;
use crate::components::{message_box, MessageBoxType, stict_to_top};
use crate::data::Data;

use crate::app::edit_content::AppEditcontent;
use super::alert::{AppIndexAlert};
use super::app_index_render;
use crate::app::new_dir::AppNewdir;
use crate::app::newcontent::AppNewcontent;
use crate::app::rename_item::AppRenameitem;
use vertigo::struct_mut::CounterMut;

#[derive(Clone)]
struct Error {
    id: u32,
    info: MessageBoxType,
    message: String,
}


#[derive(Clone)]
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

    pub fn redirect_to_edit_content(&self, full_path: &Vec<String>) {
        let full_path = full_path.clone();

        let state = AppEditcontent::new(
            self,
            full_path.clone(),
        );

        self.view.set(View::EditContent {
            state
        });
    }

    pub fn redirect_to_rename_item(&self, context: &Context, base_path: &Vec<String>, select_item: &String) {
        let select_item = select_item.clone();
        let full_path = self.data.tab.full_path.clone().get(context);
        let content = self.data.git.content_from_path(context, &full_path);

        match content {
            Resource::Ready(list_item) => {
                log::info!("redirect_to_rename_item {base_path:?} {select_item:?}");

                let state = AppRenameitem::new(
                    self,
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
        self.view.set(View::Index);
    }

    pub fn redirect_to_index_with_path(&self, new_path: Vec<String>, new_item: Option<String>) {
        self.redirect_to_index();

        self.data.tab.redirect_to(new_path, new_item);
        self.data.git.root.refresh();
    }

    pub fn redirect_to_mkdir(&self, context: &Context) {
        let state = AppNewdir::new(context, self);

        self.view.set(View::Mkdir {
            state
        });
    }

    pub fn redirect_to_index_with_root_refresh(&self) {
        self.data.git.root.refresh();
        self.redirect_to_index();
    }

    pub fn redirect_to_new_content(&self, context: &Context) {
        let state = AppNewcontent::new(self, context);
        self.view.set(View::NewContent { state });
    }

    pub fn current_edit(&self, context: &Context) {
        let full_path = self.data.tab.full_path.get(context);
        self.redirect_to_edit_content(&full_path);
    }

    pub fn current_rename(&self, context: &Context) {
        let path = self.data.tab.router.get_dir(context);
        let select_item = self.data.tab.current_item.get(context);

        if let Some(select_item) = select_item.as_ref() {
            self.redirect_to_rename_item(context, &path, select_item);
        } else {
            log::error!("current_rename fail");
        }
    }

    fn message_add(&self, context: &Context, info: MessageBoxType, message: String) -> u32 {
        let message_id = self.next_id.get_next();

        let error = Error {
            id: message_id,
            info,
            message,
        };

        let mut messages = self.errors.get(context);
        messages.push(error);
        self.errors.set(messages);

        message_id
    }

    fn message_off_with_timeout(&self, message_id: u32, timeout: u32) {
        let self_copy = self.clone();

        get_driver().spawn(async move {
            get_driver().sleep(timeout).await;
            transaction(|context| {
                self_copy.remove_message(context, message_id);
            });
        });
    }

    pub fn show_message_error(&self, context: &Context, message: impl Into<String>, timeout: Option<u32>) {
        let message_id = self.message_add(context, MessageBoxType::Error, message.into());
        if let Some(timeout) = timeout {
            self.message_off_with_timeout(message_id, timeout);
        }
    }

    pub fn show_message_info(&self, context: &Context, message: impl Into<String>, timeout: Option<u32>) {
        let message_id = self.message_add(context, MessageBoxType::Info, message.into());
        if let Some(timeout) = timeout {
            self.message_off_with_timeout(message_id, timeout);
        }
    }

    pub fn remove_message(&self, context: &Context, message_id: u32) {
        let mut messages = self.errors.get(context);
        messages.retain(|item| item.id != message_id);
        self.errors.set(messages);
    }

    pub fn keydown(&self, context: &Context, code: String) -> bool {
        if self.alert.is_visible(context) {
            if code == "Escape" {
                self.alert.close_modal();
                return true;
            }

            //TODO - dodać wskaźnik i nawigację klawiaturą po elemencie z listy wyników

            return false;
        }

        if code == "ArrowUp" {
            self.data.tab.pointer_up(context);
            return true;
        } else if code == "ArrowDown" {
            self.data.tab.pointer_down(context);
            return true;
        } else if code == "Escape" {
            self.data.tab.pointer_escape(context);
            return true;
        } else if code == "ArrowRight" || code == "Enter" {
            self.data.tab.pointer_enter(context);
            return true;
        } else if code == "ArrowLeft" || code == "Backspace" || code == "Escape" {
            self.data.tab.backspace(context);
            return true;
        }

        log::info!("klawisz ... {:?} ", code);
        false
    }

    fn render_view(&self) -> VDomComponent {
        let app = VDomComponent::from_ref(self, app_render);
        let view = self.data.tab.open_links.render(app);
        view
    }

    fn render_errors(&self) -> VDomComponent {
        VDomComponent::from_ref(self, |context, state| {
            let errors = state.errors.get(context);

            let mut list = Vec::new();

            for error in errors {
                let Error { id, info, message } = error;
                let state = state.clone();

                let on_remove = bind(&state)
                    .and(&id)
                    .call(|context, state, id| {
                        state.remove_message(context, *id);
                    });

                list.push(message_box(info, message, on_remove));
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

        VDomComponent::from_fn(move |_| {
            html! {
                <div>
                    { view.clone() }
                    { errors.clone() }
                </div>
            }
        })
    }
}

fn app_render(context: &Context, app: &App) -> VDomElement {
    let view = app.view.get(context);

    match view {
        View::Index => {
            let view = app_index_render(app);

            html! {
                <div id="root">
                    { view }
                </div>
            }
        },
        View::EditContent { state } => {
            let view = state.render();

            html! {
                <div id="root">
                    { view }
                </div>
            }
        },
        View::NewContent { state } => {
            let view = state.render();

            html! {
                <div id="root">
                    { view }
                </div>
            }
        },
        View::RenameItem {state } => {
            let view = state.render();

            html! {
                <div id="root">
                    {view}
                </div>
            }
        },
        View::Mkdir { state } => {
            let view = state.render();

            html! {
                <div id="root">
                    { view }
                </div>
            }
        },
    }
}
