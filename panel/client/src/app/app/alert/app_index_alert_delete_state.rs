use common::{HandlerDeleteItemBody};
use vertigo::{
    Value,
    bind, bind_spawn, Resource, Computed, dom, transaction, RequestBuilder, DomNode,
};
use crate::{components::{AlertBox, ButtonState}, app::{response::check_request_response, App}};

use super::AppIndexAlert;

#[derive(Clone, PartialEq)]
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
        let progress = transaction(|context| {
            self.progress.get(context)
        });

        if progress {
            return;
        }

        let current_path = self.full_path;

        log::info!("usuwamy ...");
        self.progress.set(true);

        let response = RequestBuilder::post("/delete_item")
            .body_json(HandlerDeleteItemBody {
                path: current_path,
                hash: current_hash
                
            })
            .call()
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
        };
    }

    pub fn bind_delete_yes(&self) -> DomNode {
        let state = self.clone();
        let app = self.app.clone();

        ButtonState::render(Computed::from(move |context| {
            let full_path = state.full_path.clone();
            let item = state.alert.data.items.get_from_path(&full_path);

            let id = &item.id.get(context);

            if let Resource::Ready(id) = id {
                let action = bind_spawn!(state, app, id, async move {
                    state.delete_yes(app, id).await;
                });

                return ButtonState::active("Tak", action);
            } else {
                ButtonState::disabled("Tak")
            }
        }))
    }

    pub fn bind_delete_no(&self) -> DomNode {
        let state = self.clone();

        ButtonState::render(Computed::from(move |_| {
            let action = bind!(state, || {
                let progress = transaction(|context| {
                    state.progress.get(context)
                });

                if progress {
                    return;
                }
        
                state.alert.close_modal();
            });

            ButtonState::active("Nie", action)
        }))
    }

    pub fn render(&self) -> DomNode {
        let message = render_message(self);
        let progress = self.progress.to_computed();
        AlertBox::new(message)
            .progress(progress)
            .button(self.bind_delete_no())
            .button(self.bind_delete_yes())
            .render()
    }
}

fn render_message(state: &AppIndexAlertDelete) -> DomNode {
    let full_path = state.full_path.clone();
    let message = format!("Czy usunąć -> {} ?", full_path.join("/"));
    dom!{
        <div>
            { message }
        </div>
    }
}