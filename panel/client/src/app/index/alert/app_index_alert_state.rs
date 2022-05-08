use vertigo::{Value, VDomElement, VDomComponent};
use vertigo::{html};
use crate::app::index::alert::app_index_alert_delete_state::AppIndexAlertDelete;
use crate::app::index::alert::app_index_alert_search_state::AppIndexAlertSearch;
use crate::data::Data;

use super::app_index_alert_moveitem_state::AppIndexAlertMoveitem;

#[derive(Clone)]
enum AlertView {
    None,
    DeleteFile { state: AppIndexAlertDelete },
    SearchInPath { state: AppIndexAlertSearch },
    MoveItem { state: AppIndexAlertMoveitem },                       //TODO - zaimplementować
}

#[derive(Clone)]
pub struct AppIndexAlert {
    pub data: Data,
    view: Value<AlertView>,
}

impl AppIndexAlert {
    pub fn new(data: Data) -> AppIndexAlert {
        let view = Value::new(AlertView::None);

        AppIndexAlert {
            data: data.clone(),
            view,
        }
    }

    pub fn render(&self) -> VDomComponent {
        VDomComponent::from_ref(self, app_index_alert_render)
    }

    pub fn is_visible(&self) -> bool {
        let view = self.view.get();
        match view {
            AlertView::None => false,
            _ => true
        }
        // *view != AlertView::None
    }

    pub fn delete(&self, path: Vec<String>) {
        if self.is_visible() {
            return;
        }

        let state = AppIndexAlertDelete::new(self, &path);

        self.view.set(AlertView::DeleteFile { state });
    }

    pub fn redirect_to_search(&self) {
        if self.is_visible() {
            return;
        }

        let state = AppIndexAlertSearch::new(&self);
        self.view.set(AlertView::SearchInPath { state });
    }

    pub fn move_current(&self,  path: Vec<String>) {
        if self.is_visible() {
            return;
        }

        let state = AppIndexAlertMoveitem::new(&self, path);
        self.view.set(AlertView::MoveItem { state });
    }

    pub fn close_modal(&self) {
        self.view.set(AlertView::None);
    }
}


fn app_index_alert_render(alert: &AppIndexAlert) -> VDomElement {
    match alert.view.get() {
        AlertView::None => {
            html! {
                <div />
            }
        },
        AlertView::DeleteFile { state } => {
            let view = state.render();

            html! {
                <div>
                    { view }
                </div>
            }
        },
        AlertView::SearchInPath { state } => {
            let view = state.render();

            html! {
                <div>
                    { view }
                </div>
            }
        },
        AlertView::MoveItem { state } => {
            let view = state.render();

            html! {
                <div>
                    { view }
                </div>
            }
        }
    }
}

