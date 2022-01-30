use std::rc::Rc;

use vertigo::{Driver, VDomElement, VDomComponent};
use vertigo::{
    Computed,
    Value
};
use vertigo::{html};
use crate::app::StateApp;

use super::state_alert_search::StateAlertSearch;
use super::state_alert_delete::StateAlertDelete;

#[derive(PartialEq)]
pub enum AlertView {
    None,
    DeleteFile { path: Rc<Vec<String>> },
    SearchInPath,
    MoveItem { path: Rc<Vec<String>> },                       //TODO - zaimplementować
}

#[derive(PartialEq, Clone)]
pub struct StateAlert {
    driver: Driver,
    pub app_state: StateApp,
    view: Value<AlertView>,
    current_full_path: Computed<Vec<String>>,
}

impl StateAlert {
    pub fn new(
        app_state: StateApp,
        current_full_path: Computed<Vec<String>>,
        driver: Driver
    ) -> (StateAlert, VDomComponent) {
        let view = app_state.driver.new_value(AlertView::None);

        let state = StateAlert {
            driver,
            app_state: app_state.clone(),
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

    pub fn delete(&self, path: Rc<Vec<String>>) {
        if self.is_visible() {
            return;
        }

        // let full_path = self.current_full_path.get_value();
        self.view.set_value(AlertView::DeleteFile { path });
    }

    pub fn redirect_to_search(&self) {
        if self.is_visible() {
            return;
        }

        self.view.set_value(AlertView::SearchInPath);
    }

    pub fn move_current(&self,  path: Rc<Vec<String>>) {
        if self.is_visible() {
            return;
        }

        self.view.set_value(AlertView::MoveItem { path });
    }

    pub fn close_modal(&self) {
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
        AlertView::DeleteFile { path } => {
            html! {
                <div>
                    { StateAlertDelete::render(alert_state, path) }
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
        AlertView::MoveItem { path } => {
            html! {
                <div>
                    "przenoszenie elementu -> " {path.join("/")}
                </div>
            }
        }
    }
}

