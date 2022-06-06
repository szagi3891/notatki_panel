use common::{HandlerDeleteItemBody};
use vertigo::{
    VDomComponent,
    Value,
    bind, get_driver, html, Resource,
};
use crate::{components::{AlertBox, ButtonComponent, ButtonState}, app::{response::check_request_response, App}};

use super::AppIndexAlert;

#[derive(Clone)]
pub struct AppIndexAlertDelete {
    app: App,
    pub alert: AppIndexAlert,
    full_path: Vec<String>,
    progress: Value<bool>,
}

impl AppIndexAlertDelete {
    pub fn new(app: App, alert: &AppIndexAlert, full_path: &Vec<String>) -> AppIndexAlertDelete {
        let progress: Value<bool> = Value::new(false);

        AppIndexAlertDelete {
            app,
            alert: alert.clone(),
            full_path: full_path.clone(),
            progress,
        }
    }

    async fn delete_yes(self, app: App, current_hash: String) {
        if self.progress.get() {
            return;
        }

        let current_path = self.full_path;

        log::info!("usuwamy ...");
        self.progress.set(true);

        let response = get_driver()
            .request("/delete_item")
            .body_json(HandlerDeleteItemBody {
                path: current_path,
                hash: current_hash
                
            })
            .post()
            .await;

        self.progress.set(false);

        match check_request_response(response) {
            Ok(()) => {
                self.alert.data.tab.redirect_item_select_after_delete();
                self.alert.data.git.root.refresh();
                self.alert.close_modal();
            },
            Err(message) => {
                app.show_message_error(message, Some(10000));
            }
        }
    }

    pub fn bind_delete_yes(&self) -> VDomComponent {
        let state = self.clone();
        let app = self.app.clone();

        ButtonComponent::new(move || {
            let full_path = state.full_path.clone();
            let item = state.alert.data.git.content_from_path(&full_path);

            if let Resource::Ready(item) = item {
                let action = bind(&state)
                    .and(&app)
                    .and(&item.id)
                    .spawn(|state, app, id| {
                        state.delete_yes(app, id)
                    });

                return ButtonState::active("Tak", action);
            }

            ButtonState::disabled("Tak")
        })
    }

    pub fn bind_delete_no(&self) -> VDomComponent {
        let state = self.clone();

        ButtonComponent::new(move || {
            let action = bind(&state).call(|state| {
                if state.progress.get() {
                    return;
                }
        
                state.alert.close_modal();
            });

            ButtonState::active("Nie", action)
        })
    }

    pub fn render(&self) -> VDomComponent {
        let message = render_message(self);
        let progress = self.progress.to_computed();
        AlertBox::new(message)
            .progress(progress)
            .button(self.bind_delete_no())
            .button(self.bind_delete_yes())
            .render()
    }
}

fn render_message(state: &AppIndexAlertDelete) -> VDomComponent {
    VDomComponent::from_ref(state, |state| {
        let full_path = state.full_path.clone();
        let message = format!("Czy usunąć -> {} ?", full_path.join("/"));
        html!{
            <div>
                { message }
            </div>
        }
    })
}