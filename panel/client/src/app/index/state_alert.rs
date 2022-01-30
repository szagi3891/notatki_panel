use std::rc::Rc;

use vertigo::{Driver, VDomElement, VDomComponent};
use vertigo::{
    Computed,
    Value
};
use vertigo::{html};
use crate::app::StateApp;

use super::ListItem;
use super::state_alert_search::StateAlertSearch;
use super::state_alert_delete::StateAlertDelete;

#[derive(PartialEq)]
pub enum AlertView {
    None,
    DeleteFile,
    SearchInPath,
    MoveItem,                       //TODO - zaimplementować
}

#[derive(PartialEq, Clone)]
pub struct StateAlert {
    driver: Driver,
    pub app_state: StateApp,
    progress: Value<bool>,
    list: Computed<Vec<ListItem>>,
    view: Value<AlertView>,
    current_full_path: Computed<Vec<String>>,
}

impl StateAlert {
    pub fn new(
        app_state: StateApp,
        current_full_path: Computed<Vec<String>>,
        list: Computed<Vec<ListItem>>,
        driver: Driver
    ) -> (StateAlert, VDomComponent) {
        let view = app_state.driver.new_value(AlertView::None);
        let progress = app_state.driver.new_value(false);

        let state = StateAlert {
            driver,
            app_state: app_state.clone(),
            progress,
            list,
            view,
            current_full_path,
        };

        let view = app_state.driver.bind_render(state.clone(), render_alert);
        (state, view)
    }

    pub fn is_visible(&self) -> bool {
        let view = self.view.get_value();
        *view != AlertView::None
    }

    fn is_precess(&self) -> bool {
        let value = self.progress.get_value();
        *value
    }

    pub fn delete(&self) {
        if self.is_precess() {
            return;
        }

        self.view.set_value(AlertView::DeleteFile);
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

fn render_alert(state: &Computed<StateAlert>) -> VDomElement {
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
            let progress = alert_state.progress.clone();
            let back = Rc::new({
                let view = alert_state.view.clone();
                move || {
                    view.set_value(AlertView::None);
                }
            });

            html! {
                <div>
                    { StateAlertDelete::render(alert_state, full_path, progress, back) }
                </div>
            }
        },
        AlertView::SearchInPath => {
            let view = StateAlertSearch::component(&alert_state);

            html! {
                <div>
                    { view }
                </div>
            }
        },
        AlertView::MoveItem => {
            let full_path = alert_state.current_full_path.get_value();

            html! {
                <div>
                    "przenoszenie elementu -> " {full_path.join("/")}
                </div>
            }
        }
    }
}

