use std::rc::Rc;

use common::{HandlerDeleteItemBody};
use vertigo::{
    VDomComponent,
    Value,
};
use crate::components::AlertBox;

use super::AppIndexAlert;

#[derive(Clone)]
pub struct AppIndexAlertDelete {
    full_path: Rc<Vec<String>>,
    progress: Value<bool>,
    pub alert: AppIndexAlert,
}

impl AppIndexAlertDelete {
    pub fn new(alert: &AppIndexAlert, full_path: &Rc<Vec<String>>) -> AppIndexAlertDelete {
        let progress: Value<bool> = alert.data.driver.new_value(false);

        AppIndexAlertDelete {
            full_path: full_path.clone(),
            progress,
            alert: alert.clone(),
        }
    }

    pub async fn delete_yes(self) {
        if *self.progress.get_value() {
            return;
        }

        let current_path = self.full_path.as_ref().clone();
        let current_hash = self.alert.data.git.content_hash(&current_path);

        let current_hash = match current_hash {
            Some(current_hash) => current_hash,
            None => {
                log::error!("Problem z usunięciem ...");
                return;
            }
        };

        log::info!("usuwamy ...");
        self.progress.set_value(true);

        let _ = self.alert.data.driver
            .request("/delete_item")
            .body_json(HandlerDeleteItemBody {
                path: current_path,
                hash: current_hash
                
            })
            .post()
            .await;    //::<RootResponse>();

        self.progress.set_value(false);
        self.alert.data.tab.redirect_after_delete();
        self.alert.data.git.root.refresh();
        self.alert.close_modal();
    }

    pub fn bind_delete_yes(&self) -> impl Fn() {
        let driver = self.alert.data.driver.clone();
        let state = self.clone();

        move || {
            driver.spawn(state.clone().delete_yes());
        }
    }

    pub fn delete_no(&self) {
        if *self.progress.get_value() {
            return;
        }

        self.alert.close_modal();
    }

    pub fn bind_delete_no(&self) -> impl Fn() {
        let state = self.clone();

        move || {
            state.delete_no();
        }
    }

    pub fn render(&self) -> VDomComponent {
        VDomComponent::new(self, |state: &AppIndexAlertDelete| {
            let full_path = state.full_path.clone();
            let progress_computed = state.progress.to_computed();

            let message = format!("Czy usunąć -> {} ?", full_path.join("/"));
            let mut alert = AlertBox::new(message, progress_computed.clone());

            alert.button("Tak", state.bind_delete_yes());
            alert.button("Nie", state.bind_delete_no());

            alert.render()
        })
    }
}

