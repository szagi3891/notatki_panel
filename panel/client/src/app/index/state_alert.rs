use std::rc::Rc;
use common::{HandlerDeleteItemBody};
use vertigo::{Driver, VDomElement};
use vertigo::{
    Computed,
    Value
};
use vertigo::{html};
use crate::app::AppState;
use crate::components::AlertBox;

use super::ListItem;
use super::state_alert_search::StateAlertSearch;

#[derive(PartialEq)]
pub enum AlertView {
    None,
    DeleteFile,
    SearchInPath,
}

#[derive(PartialEq, Clone)]
pub struct StateAlert {
    driver: Driver,
    pub app_state: Rc<AppState>,
    list: Computed<Vec<ListItem>>,
    progress: Value<bool>,
    progress_computed: Computed<bool>,
    view: Value<AlertView>,
    current_full_path: Computed<Vec<String>>,
}

impl StateAlert {
    pub fn new(
        app_state: Rc<AppState>,
        current_full_path: Computed<Vec<String>>,
        list: Computed<Vec<ListItem>>,
        driver: Driver
    ) -> Computed<StateAlert> {
        let view = app_state.driver.new_value(AlertView::None);
        let progress = app_state.driver.new_value(false);
        let progress_computed = progress.to_computed();

        app_state.driver.new_computed_from(StateAlert {
            driver,
            app_state: app_state.clone(),
            list,
            progress,
            progress_computed,
            view,
            current_full_path,
        })
    }

    pub fn is_visible(&self) -> bool {
        let view = self.view.get_value();
        *view != AlertView::None
    }

    fn is_precess(&self) -> bool {
        let value = self.progress.get_value();
        *value == true
    }

    pub fn delete(&self) {
        if self.is_precess() {
            return;
        }

        self.view.set_value(AlertView::DeleteFile);
    }

    fn redirect_after_delete(&self) {
        let current_path_item = self.app_state.data.current_path_item.get_value();
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
                    self.app_state.data.current_path_item.set_value(Some(prev.name.clone()));
                    return;
                }
            }

            if let Some(prev) = list.get(current_index + 1) {
                self.app_state.data.current_path_item.set_value(Some(prev.name.clone()));
                return;
            }
        };
    }

    fn delete_yes(&self) {
        if self.is_precess() {
            return;
        }

        let current_path = self.current_full_path.get_value().as_ref().clone();
        let current_hash = self.app_state.data.git.content_hash(&current_path);
    
        let current_hash = match current_hash {
            Some(current_hash) => current_hash,
            None => {
                log::error!("Problem z usunięciem ...");
                return;
            }
        };

        log::info!("usuwamy ...");
        self.progress.set_value(true);

        let response = self.driver
            .request("/delete_item")
            .body_json(HandlerDeleteItemBody {
                path: current_path,
                hash: current_hash
                
            })
            .post();    //::<RootResponse>();

        let progress = self.progress.clone();
        let self_copy = self.clone();

        self.driver.spawn(async move {
            let _ = response.await;
            progress.set_value(false);
            self_copy.redirect_after_delete();
            self_copy.app_state.data.root.refresh();
            self_copy.view.set_value(AlertView::None);
        });
    }

    fn delete_no(&self) {
        if self.is_precess() {
            return;
        }

        self.view.set_value(AlertView::None);
    }

    pub fn redirect_to_search(&self) {
        if self.is_precess() {
            return;
        }

        self.view.set_value(AlertView::SearchInPath);
    }

    pub fn search_close(&self) {
        if self.is_precess() {
            return;
        }

        self.view.set_value(AlertView::None);
    }
}

pub fn render_alert(state: &Computed<StateAlert>) -> VDomElement {
    let alert_state = state.get_value();
    let alert = alert_state.view.get_value();

    match alert.as_ref() {
        AlertView::None => {
            html! {
                <div />
            }
        },
        AlertView::DeleteFile => {
            let full_path = alert_state.current_full_path.get_value();

            let message = format!("Czy usunąć -> {} ?", full_path.join("/"));
            let computed = alert_state.progress_computed.clone();

            let mut alert = AlertBox::new(message, computed);

            alert.button("Tak", {
                let alert_state = alert_state.clone();
                move || {
                    alert_state.delete_yes();
                }
            });

            alert.button("Nie", {
                let alert_state = alert_state.clone();
                move || {
                    alert_state.delete_no();
                }
            });

            alert.render()
        },
        AlertView::SearchInPath => {
            let state = StateAlertSearch::new(&alert_state);

            html! {
                <div>
                    <component {StateAlertSearch::render} data={state} />
                </div>
            }
        }
    }
}
