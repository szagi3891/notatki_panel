use vertigo::{Value, Context, render_value, DomComment, dom};
use crate::app::App;
use crate::app::app::alert::app_index_alert_delete_state::AppIndexAlertDelete;
use crate::app::app::alert::app_index_alert_search_state::AppIndexAlertSearch;
use crate::data::Data;

use super::app_index_alert_moveitem_state::AppIndexAlertMoveitem;

#[derive(Clone, PartialEq)]
enum AlertView {
    None,
    DeleteFile { state: AppIndexAlertDelete },
    SearchInPath { state: AppIndexAlertSearch },
    MoveItem { state: AppIndexAlertMoveitem },                       //TODO - zaimplementować
}

#[derive(Clone, PartialEq)]
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

    pub fn render(&self) -> DomComment {
        app_index_alert_render(self)
    }

    pub fn is_visible(&self, context: &Context) -> bool {
        let view = self.view.get(context);
        match view {
            AlertView::None => false,
            _ => true
        }
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


fn app_index_alert_render(alert: &AppIndexAlert) -> DomComment {
    render_value(alert.view.to_computed(), |view| {
        match view {
            AlertView::None => {
                None
            },
            AlertView::DeleteFile { state } => {
                Some(dom! {
                    <div>
                        { state.render() }
                    </div>
                })
            },
            AlertView::SearchInPath { state } => {
                Some(dom! {
                    <div>
                        { state.render() }
                    </div>
                })
            },
            AlertView::MoveItem { state } => {
                Some(dom! {
                    <div>
                        { state.render() }
                    </div>
                })
            }
        }
    })
}

