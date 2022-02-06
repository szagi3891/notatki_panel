use std::rc::Rc;

use vertigo::{Value, VDomElement, VDomComponent};
use vertigo::{html};
use crate::app::App;

use super::state_alert_search::StateAlertSearch;
use super::state_alert_delete::StateAlertDelete;

#[derive(PartialEq)]
pub enum AlertView {
    None,
    DeleteFile { path: Rc<Vec<String>> },
    SearchInPath,
    MoveItem { path: Rc<Vec<String>> },                       //TODO - zaimplementować
}

#[derive(Clone)]
pub struct StateAlert {
    pub app_state: App,
    view: Value<AlertView>,
}

impl StateAlert {
    pub fn new(app_state: App) -> (StateAlert, VDomComponent) {
        let view = app_state.driver.new_value(AlertView::None);

        let state = StateAlert {
            app_state: app_state.clone(),
            view,
        };

        let view = VDomComponent::new(state.clone(), render_alert);
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

fn render_alert(alert_state: &StateAlert) -> VDomElement {
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

