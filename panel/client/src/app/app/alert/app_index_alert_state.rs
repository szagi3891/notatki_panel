use vertigo::{Value, VDomElement, VDomComponent, Context};
use vertigo::{html};
use crate::app::App;
use crate::app::app::alert::app_index_alert_delete_state::AppIndexAlertDelete;
use crate::app::app::alert::app_index_alert_search_state::AppIndexAlertSearch;
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
        VDomComponent::from(self.clone(), app_index_alert_render)
    }

    pub fn is_visible(&self, context: &Context) -> bool {
        let view = self.view.get(context);
        match view {
            AlertView::None => false,
            _ => true
        }
        // *view != AlertView::None
    }

    pub fn delete(&self, context: &Context, app: App, path: Vec<String>) {
        if self.is_visible(context) {
            return;
        }

        let state = AppIndexAlertDelete::new(app, self, &path);

        self.view.set(AlertView::DeleteFile { state });
    }

    pub fn redirect_to_search(&self, context: &Context) {
        if self.is_visible(context) {
            return;
        }

        let state = AppIndexAlertSearch::new(&self);
        self.view.set(AlertView::SearchInPath { state });
    }

    pub fn move_current(&self, context: &Context, app: &App, path: &Vec<String>, hash: &String) {
        if self.is_visible(context) {
            return;
        }

        let state = AppIndexAlertMoveitem::new(app, &self, path.clone(), hash.clone());
        self.view.set(AlertView::MoveItem { state });
    }

    pub fn close_modal(&self) {
        self.view.set(AlertView::None);
    }
}


fn app_index_alert_render(context: &Context, alert: &AppIndexAlert) -> VDomElement {
    match alert.view.get(context) {
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

