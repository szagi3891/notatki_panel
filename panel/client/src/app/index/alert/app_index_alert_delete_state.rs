use common::{HandlerDeleteItemBody};
use vertigo::{
    VDomComponent,
    Value,
    bind, get_driver,
};
use crate::{components::AlertBox, data::ContentView, app::{response::check_request_response, App}};

use super::AppIndexAlert;

#[derive(Clone)]
pub struct AppIndexAlertDelete {
    full_path: Vec<String>,
    progress: Value<bool>,
    pub alert: AppIndexAlert,
}

impl AppIndexAlertDelete {
    pub fn new(alert: &AppIndexAlert, full_path: &Vec<String>) -> AppIndexAlertDelete {
        let progress: Value<bool> = Value::new(false);

        AppIndexAlertDelete {
            full_path: full_path.clone(),
            progress,
            alert: alert.clone(),
        }
    }

    pub async fn delete_yes(self, app: App) {
        if self.progress.get() {
            return;
        }

        let current_path = self.full_path;
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

        let response = get_driver()
            .request("/delete_item")
            .body_json(HandlerDeleteItemBody {
                path: current_path,
                hash: current_hash
                
            })
            .post()
            .await;    //::<RootResponse>();

        self.progress.set(false);

        match check_request_response(response) {
            Ok(()) => {
                self.alert.data.tab.redirect_after_delete();
                self.alert.data.git.root.refresh();
                self.alert.close_modal();
            },
            Err(message) => {
                app.show_message_error(message, Some(10000));
            }
        }
    }

    pub fn bind_delete_yes(&self, app: &App) -> impl Fn() {
        bind(self)
            .and(app)
            .spawn(|state, app| {
                state.delete_yes(app)
            })
    }

    pub fn delete_no(&self) {
        if self.progress.get() {
            return;
        }

        self.alert.close_modal();
    }

    pub fn bind_delete_no(&self) -> impl Fn() {
        bind(self).call(|state| {
            state.delete_no();
        })
    }

    pub fn render(&self, app: &App) -> VDomComponent {
        VDomComponent::from((self.clone(), app.clone()), |(state, app)| {
            let full_path = state.full_path.clone();
            let progress_computed = state.progress.to_computed();

            let message = format!("Czy usunąć -> {} ?", full_path.join("/"));
            let mut alert = AlertBox::new(message, progress_computed.clone());

            alert.button("Tak", state.bind_delete_yes(app));
            alert.button("Nie", state.bind_delete_no());

            alert.render()
        })
    }
}

