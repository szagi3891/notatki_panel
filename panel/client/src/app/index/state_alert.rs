use std::rc::Rc;

use vertigo::{Value, VDomElement, VDomComponent};
use vertigo::{html};
use crate::app::App;

use super::state_alert_search::StateAlertSearch;
use super::state_alert_delete::StateAlertDelete;

fn render_alert(alert_state: &AppIndexAlert) -> VDomElement {
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

