use common::{HandlerDeleteItemBody};
use vertigo::{
    Value,
    bind, get_driver, Resource, Computed, DomElement, dom, transaction,
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
        };
    }

    pub fn bind_delete_yes(&self) -> DomElement {
        let state = self.clone();
        let app = self.app.clone();

        ButtonState::render(Computed::from(move |context| {
            let full_path = state.full_path.clone();
            let item = state.alert.data.git.content_from_path(context, &full_path);

            if let Resource::Ready(item) = item {
                let id = &item.id;

                let action = bind!(state, app, id, || {
                    get_driver().spawn(bind!(state, app, id, async move {
                        state.delete_yes(app, id).await;
                    }));
                });

                return ButtonState::active("Tak", action);
            }

            ButtonState::disabled("Tak")
        }))
    }

    pub fn bind_delete_no(&self) -> DomElement {
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

    pub fn render(&self) -> DomElement {
        let message = render_message(self);
        let progress = self.progress.to_computed();
        AlertBox::new(message)
            .progress(progress)
            .button(self.bind_delete_no())
            .button(self.bind_delete_yes())
            .render()
    }
}

fn render_message(state: &AppIndexAlertDelete) -> DomElement {
    let full_path = state.full_path.clone();
    let message = format!("Czy usunąć -> {} ?", full_path.join("/"));
    dom!{
        <div>
            { message }
        </div>
    }
}