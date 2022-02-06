use std::rc::Rc;

use common::{HandlerDeleteItemBody};
use vertigo::{
    VDomComponent,
    Value
};
use crate::components::AlertBox;

use super::AppIndexAlert;

#[derive(Clone)]
pub struct AppIndexAlertDelete {
    full_path: Rc<Vec<String>>,
    progress: Value<bool>,
    alert_state: AppIndexAlert,
    // progress_computed: Computed<bool>,
}

impl AppIndexAlertDelete {
    pub fn new(alert_state: &AppIndexAlert, full_path: &Rc<Vec<String>>) -> AppIndexAlertDelete {
        let progress: Value<bool> = alert_state.app_state.driver.new_value(false);

        AppIndexAlertDelete {
            full_path: full_path.clone(),
            progress,
            alert_state: alert_state.clone(),
        }
    }

    pub fn delete_yes(&self) {
        if *self.progress.get_value() {
            return;
        }

        let progress = self.progress.clone();
        let alert_state = self.alert_state.clone();

        let current_path = self.full_path.as_ref().clone();
        let current_hash = alert_state.app_state.data.git.content_hash(&current_path);
    
        let current_hash = match current_hash {
            Some(current_hash) => current_hash,
            None => {
                log::error!("Problem z usunięciem ...");
                return;
            }
        };

        log::info!("usuwamy ...");
        progress.set_value(true);

        let response = alert_state.app_state.driver
            .request("/delete_item")
            .body_json(HandlerDeleteItemBody {
                path: current_path,
                hash: current_hash
                
            })
            .post();    //::<RootResponse>();


        alert_state.app_state.driver.spawn({
            let alert_state = alert_state.clone();
            let progress = progress.clone();
            let self_copy = alert_state.clone();

            async move {
                let _ = response.await;
                progress.set_value(false);
                self_copy.app_state.data.tab.redirect_after_delete();
                self_copy.app_state.data.git.root.refresh();
                // self_copy.view.set_value(AlertView::None);
                alert_state.close_modal();
            }
        });
    }

    pub fn delete_no(&self) {
        if *self.progress.get_value() {
            return;
        }

        self.alert_state.close_modal();
    }

    pub fn render(self) -> VDomComponent {
        VDomComponent::new(self, |state: &AppIndexAlertDelete| {
            let full_path = state.full_path.clone();
            let progress_computed = state.progress.to_computed();

            let message = format!("Czy usunąć -> {} ?", full_path.join("/"));
            let mut alert = AlertBox::new(message, progress_computed.clone());

            alert.button("Tak", {
                let state = state.clone();
                move || {
                    state.delete_yes();
                }
            });

            alert.button("Nie", {
                let state = state.clone();
                move || {
                    state.delete_no();
                }
            });

            alert.render()
        })
    }
}

