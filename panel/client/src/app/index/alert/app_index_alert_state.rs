use std::rc::Rc;

use vertigo::{Value, VDomElement, VDomComponent};
use vertigo::{html};
use crate::app::App;
use crate::app::index::alert::app_index_alert_delete_state::AppIndexAlertDelete;
use crate::app::index::alert::app_index_alert_search_state::AppIndexAlertSearch;

use super::app_index_alert_moveitem_state::AppIndexAlertMoveitem;


pub enum AlertView {
    None,
    DeleteFile { state: AppIndexAlertDelete }, //path: Rc<Vec<String>> },
    SearchInPath,
    MoveItem { path: Rc<Vec<String>> },                       //TODO - zaimplementować
}

#[derive(Clone)]
pub struct AppIndexAlert {
    pub app: App,
    view: Value<AlertView>,
}

impl AppIndexAlert {
    pub fn new(app_state: App) -> (AppIndexAlert, VDomComponent) {
        let view = app_state.driver.new_value(AlertView::None);

        let state = AppIndexAlert {
            app: app_state.clone(),
            view,
        };

        let view = VDomComponent::new(state.clone(), app_index_alert_render);
        (state, view)
    }

    pub fn is_visible(&self) -> bool {
        let view = self.view.get_value();
        match view.as_ref() {
            AlertView::None => false,
            _ => true
        }
        // *view != AlertView::None
    }

    pub fn delete(&self, path: Rc<Vec<String>>) {
        if self.is_visible() {
            return;
        }

        let state = AppIndexAlertDelete::new(self, &path);

        self.view.set_value(AlertView::DeleteFile { state });
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


fn app_index_alert_render(alert: &AppIndexAlert) -> VDomElement {
    // let alert = alert_state.view.get_value();

    match alert.view.get_value().as_ref() {
        AlertView::None => {
            html! {
                <div />
            }
        },
        AlertView::DeleteFile { state } => {
            let view = state.clone().render();

            html! {
                <div>
                    { view }
                </div>
            }
        },
        AlertView::SearchInPath => {
            let view = AppIndexAlertSearch::component(&alert);

            html! {
                <div>
                    { view }
                </div>
            }
        },
        AlertView::MoveItem { path } => {
            let view = AppIndexAlertMoveitem::component(&alert, path);

            html! {
                <div>
                    { view }
                </div>
            }
        }
    }
}

