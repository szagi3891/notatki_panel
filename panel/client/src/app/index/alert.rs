use std::rc::Rc;
use common::{HandlerDeleteItemBody, RootResponse};
use vertigo::VDomElement;
use vertigo::{
    computed::{
        Computed,
        Value
    },
};
use vertigo_html::{html};
use crate::app::AppState;
use crate::components::AlertBox;
use crate::request::Request;

use super::ListItem;

#[derive(PartialEq)]
pub enum AlertView {
    None,
    DeleteFile {
        message: String,
    },
    //delete dir
}

#[derive(PartialEq, Clone)]
pub struct AlertState {
    request: Request,
    app_state: Rc<AppState>,
    list: Computed<Vec<ListItem>>,
    progress: Value<bool>,
    progress_computed: Computed<bool>,
    view: Value<AlertView>,
    // None,
    // DeleteFile {
    //     message: String,
    //     app_state: Rc<AppState>,
    //     progress: Value<bool>,
    // },
    //delete dir
}

impl AlertState {
    pub fn new(app_state: Rc<AppState>, list: Computed<Vec<ListItem>>, request: Request) -> Computed<AlertState> {
        let view = app_state.root.new_value(AlertView::None);
        let progress = app_state.root.new_value(false);
        let progress_computed = progress.to_computed();

        app_state.root.new_computed_from(AlertState {
            request,
            app_state: app_state.clone(),
            list,
            progress,
            progress_computed,
            view,
        })
    }

    fn is_precess(&self) -> bool {
        let value = self.progress.get_value();
        *value == true
    }

    pub fn delete(&self, message: String) {
        if self.is_precess() {
            return;
        }

        self.view.set_value(AlertView::DeleteFile {
            message,
        });
    }

    fn redirect_after_delete(&self) {
        let current_path_item = self.app_state.data_state.current_path_item.get_value();
        let list = self.list.get_value();

        fn find_index(list: &Vec<ListItem>, value: &Option<String>) -> Option<usize> {
            if let Some(value) = value {
                for (index, item) in list.iter().enumerate() {
                    if item.name == *value {
                        return Some(index);
                    }
                }
            }
            None
        }

        if let Some(current_index) = find_index(list.as_ref(), current_path_item.as_ref()) {
            if current_index > 0 {
                if let Some(prev) = list.get(current_index - 1) {
                    self.app_state.data_state.current_path_item.set_value(Some(prev.name.clone()));
                    return;
                }
            }

            if let Some(prev) = list.get(current_index + 1) {
                self.app_state.data_state.current_path_item.set_value(Some(prev.name.clone()));
                return;
            }
        };
    }

    fn delete_yes(&self) {
        if self.is_precess() {
            return;
        }

        let current_path = self.app_state.data_state.get_full_current_path();
        let current_hash = self.app_state.data_state.get_content_hash(&current_path);
    
        let current_hash = match current_hash {
            Some(current_hash) => current_hash,
            None => {
                log::error!("Problem z usunięciem ...");
                return;
            }
        };

        log::info!("usuwamy ...");
        self.progress.set_value(true);

        let response = self.request
            .fetch("/delete_item")
            .body(&HandlerDeleteItemBody {
                path: current_path,
                hash: current_hash
                
            })
            .post::<RootResponse>();

        let progress = self.progress.clone();
        let self_copy = self.clone();

        self.request.spawn_local(async move {
            let _ = response.await;
            progress.set_value(false);
            self_copy.redirect_after_delete();
            self_copy.app_state.data_state.state_root.refresh();
            self_copy.view.set_value(AlertView::None);
        });
    }

    fn delete_no(&self) {
        if self.is_precess() {
            return;
        }

        self.view.set_value(AlertView::None);
    }
}

pub fn render_alert(state: &Computed<AlertState>) -> VDomElement {
    let alert_state = state.get_value();
    let alert = alert_state.view.get_value();

    match alert.as_ref() {
        AlertView::None => {
            html! {
                <div />
            }
        },
        AlertView::DeleteFile {message} => {
            let message = format!("aler delete file ... {}", message);
            let computed = alert_state.progress_computed.clone();

            let mut alert = AlertBox::new(message, computed);

            alert.button("Nie", {
                let alert_state = alert_state.clone();
                move || {
                    alert_state.delete_no();
                }
            });

            alert.button("Tak", {
                let alert_state = alert_state.clone();
                move || {
                    alert_state.delete_yes();
                }
            });

            alert.render()
        }
    }
}