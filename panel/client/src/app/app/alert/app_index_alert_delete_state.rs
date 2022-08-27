use common::{HandlerDeleteItemBody};
use vertigo::{
    Value,
    bind, get_driver, Resource, Computed, Context, DomElement, dom, bind3,
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

    async fn delete_yes(self, context: Context, app: App, current_hash: String) -> Context {
        if self.progress.get(&context) {
            return context;
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
                self.alert.data.tab.redirect_item_select_after_delete(&context);
                self.alert.data.git.root.refresh();
                self.alert.close_modal();
            },
            Err(message) => {
                app.show_message_error(&context, message, Some(10000));
            }
        };

        context
    }

    pub fn bind_delete_yes(&self) -> DomElement {
        let state = self.clone();
        let app = self.app.clone();

        ButtonState::render(Computed::from(move |context| {
            let full_path = state.full_path.clone();
            let item = state.alert.data.git.content_from_path(context, &full_path);

            if let Resource::Ready(item) = item {
                let action = bind3(&state, &app, &item.id).spawn(|context, state, app, id| async move {
                    let context = state.delete_yes(context, app, id).await;
                    context
                });

                return ButtonState::active("Tak", action);
            }

            ButtonState::disabled("Tak")
        }))
    }

    pub fn bind_delete_no(&self) -> DomElement {
        let state = self.clone();

        ButtonState::render(Computed::from(move |_| {
            let action = bind(&state).call(|context, state| {
                if state.progress.get(context) {
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