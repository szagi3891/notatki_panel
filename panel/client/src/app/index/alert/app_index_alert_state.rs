use std::rc::Rc;

use vertigo::{Value, VDomElement, VDomComponent};
use vertigo::{html};
use crate::app::App;
use crate::app::index::alert::app_index_alert_delete_state::AppIndexAlertDelete;
use crate::app::index::alert::app_index_alert_search_state::AppIndexAlertSearch;

use super::app_index_alert_moveitem_state::AppIndexAlertMoveitem;


#[derive(PartialEq)]
pub enum AlertView {
    None,
    DeleteFile { path: Rc<Vec<String>> },
    SearchInPath,
    MoveItem { path: Rc<Vec<String>> },                       //TODO - zaimplementować
}

#[derive(Clone)]
pub struct AppIndexAlert {
    pub app_state: App,
    view: Value<AlertView>,
}

impl AppIndexAlert {
    pub fn new(app_state: App) -> (AppIndexAlert, VDomComponent) {
        let view = app_state.driver.new_value(AlertView::None);

        let state = AppIndexAlert {
            app_state: app_state.clone(),
            view,
        };

        let view = VDomComponent::new(state.clone(), app_index_alert_render);
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


fn app_index_alert_render(alert_state: &AppIndexAlert) -> VDomElement {
    let alert = alert_state.view.get_value();

    match alert.as_ref() {
        AlertView::None => {
            html! {
                <div />
            }
        },
        AlertView::DeleteFile { path } => {
            let view = AppIndexAlertDelete::new(alert_state, path).render();

            html! {
                <div>
                    { view }
                </div>
            }
        },
        AlertView::SearchInPath => {
            let view = AppIndexAlertSearch::component(&alert_state);

            html! {
                <div>
                    { view }
                </div>
            }
        },
        AlertView::MoveItem { path } => {
            let view = AppIndexAlertMoveitem::component(&alert_state, path);

            html! {
                <div>
                    { view }
                </div>
            }
        }
    }
}

