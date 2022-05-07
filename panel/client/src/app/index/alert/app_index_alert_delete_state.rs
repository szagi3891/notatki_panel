use std::rc::Rc;

use common::{HandlerDeleteItemBody};
use vertigo::{
    VDomComponent,
    Value,
    bind, get_driver,
};
use crate::{components::AlertBox, data::ContentView};

use super::AppIndexAlert;

#[derive(Clone)]
pub struct AppIndexAlertDelete {
    full_path: Rc<Vec<String>>,
    progress: Value<bool>,
    pub alert: AppIndexAlert,
}

impl AppIndexAlertDelete {
    pub fn new(alert: &AppIndexAlert, full_path: &Rc<Vec<String>>) -> AppIndexAlertDelete {
        let progress: Value<bool> = Value::new(false);

        AppIndexAlertDelete {
            full_path: full_path.clone(),
            progress,
            alert: alert.clone(),
        }
    }

    pub async fn delete_yes(self) {
        if *self.progress.get() {
            return;
        }

        let current_path = self.full_path.as_ref().clone();
        let current_hash = self.alert.data.git.get_content(&current_path);

        let current_hash = match current_hash {
            Some(ContentView { id, .. }) => id,
            None => {
                log::error!("Problem z usunięciem ...");
                return;
            }
        };

        log::info!("usuwamy ...");
        self.progress.set(true);

        let _ = get_driver()
            .request("/delete_item")
            .body_json(HandlerDeleteItemBody {
                path: current_path,
                hash: current_hash
                
            })
            .post()
            .await;    //::<RootResponse>();

        self.progress.set(false);
        self.alert.data.tab.redirect_after_delete();
        self.alert.data.git.root.refresh();
        self.alert.close_modal();
    }

    pub fn bind_delete_yes(&self) -> impl Fn() {
        bind(self).spawn(|state| {
            state.delete_yes()
        })
    }

    pub fn delete_no(&self) {
        if *self.progress.get() {
            return;
        }

        self.alert.close_modal();
    }

    pub fn bind_delete_no(&self) -> impl Fn() {
        bind(self).call(|state| {
            state.delete_no();
        })
    }

    pub fn render(&self) -> VDomComponent {
        VDomComponent::from_ref(self, |state: &AppIndexAlertDelete| {
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

