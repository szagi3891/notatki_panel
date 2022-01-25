use common::{HandlerDeleteItemBody};
use vertigo::{Driver, VDomElement, VDomComponent};
use vertigo::{
    Computed,
    Value
};
use vertigo::{html};
use crate::app::StateApp;
use crate::components::AlertBox;

use super::ListItem;
use super::state_alert_search::StateAlertSearch;

#[derive(PartialEq)]
pub enum AlertView {
    None,
    DeleteFile,
    SearchInPath,
}

#[derive(PartialEq, Clone)]
pub struct StateAlert {
    driver: Driver,
    pub app_state: StateApp,
    list: Computed<Vec<ListItem>>,
    progress: Value<bool>,
    progress_computed: Computed<bool>,
    view: Value<AlertView>,
    current_full_path: Computed<Vec<String>>,
}

impl StateAlert {
    pub fn new(
        app_state: StateApp,
        current_full_path: Computed<Vec<String>>,
        list: Computed<Vec<ListItem>>,
        driver: Driver
    ) -> (StateAlert, VDomComponent) {
        let view = app_state.driver.new_value(AlertView::None);
        let progress = app_state.driver.new_value(false);
        let progress_computed = progress.to_computed();

        let state = StateAlert {
            driver,
            app_state: app_state.clone(),
            list,
            progress,
            progress_computed,
            view,
            current_full_path,
        };

        let view = app_state.driver.bind_render(state.clone(), render_alert);
        (state, view)
    }

    pub fn is_visible(&self) -> bool {
        let view = self.view.get_value();
        *view != AlertView::None
    }

    fn is_precess(&self) -> bool {
        let value = self.progress.get_value();
        *value
    }

    pub fn delete(&self) {
        if self.is_precess() {
            return;
        }

        self.view.set_value(AlertView::DeleteFile);
    }

    fn delete_yes(&self) {
        if self.is_precess() {
            return;
        }

        let current_path = self.current_full_path.get_value().as_ref().clone();
        let current_hash = self.app_state.data.git.content_hash(&current_path);
    
        let current_hash = match current_hash {
            Some(current_hash) => current_hash,
            None => {
                log::error!("Problem z usunięciem ...");
                return;
            }
        };

        log::info!("usuwamy ...");
        self.progress.set_value(true);

        let response = self.driver
            .request("/delete_item")
            .body_json(HandlerDeleteItemBody {
                path: current_path,
                hash: current_hash
                
            })
            .post();    //::<RootResponse>();

        let progress = self.progress.clone();
        let self_copy = self.clone();

        self.driver.spawn(async move {
            let _ = response.await;
            progress.set_value(false);
            self_copy.app_state.data.redirect_after_delete();
            self_copy.app_state.data.git.root.refresh();
            self_copy.view.set_value(AlertView::None);
        });
    }

    fn delete_no(&self) {
        if self.is_precess() {
            return;
        }

        self.view.set_value(AlertView::None);
    }

    pub fn redirect_to_search(&self) {
        if self.is_precess() {
            return;
        }

        self.view.set_value(AlertView::SearchInPath);
    }

    pub fn search_close(&self) {
        if self.is_precess() {
            return;
        }

        self.view.set_value(AlertView::None);
    }
}

fn render_alert(state: &Computed<StateAlert>) -> VDomElement {
    let alert_state = state.get_value();
    let alert = alert_state.view.get_value();

    match alert.as_ref() {
        AlertView::None => {
            html! {
                <div />
            }
        },
        AlertView::DeleteFile => {
            let full_path = alert_state.current_full_path.get_value();

            let message = format!("Czy usunąć -> {} ?", full_path.join("/"));
            let computed = alert_state.progress_computed.clone();

            let mut alert = AlertBox::new(message, computed);

            alert.button("Tak", {
                let alert_state = alert_state.clone();
                move || {
                    alert_state.delete_yes();
                }
            });

            alert.button("Nie", {
                move || {
                    alert_state.delete_no();
                }
            });

            alert.render()
        },
        AlertView::SearchInPath => {
            let view = StateAlertSearch::component(&alert_state);

            html! {
                <div>
                    { view }
                </div>
            }
        }
    }
}

