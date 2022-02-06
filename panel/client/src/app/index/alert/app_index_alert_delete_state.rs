use std::rc::Rc;

use common::{HandlerDeleteItemBody};
use vertigo::{VDomElement};
use vertigo::{
    Value
};
use crate::components::AlertBox;

use super::AppIndexAlert;


pub struct AppIndexAlertDelete {
    // progress: Value<bool>,
    // progress_computed: Computed<bool>,
}

impl AppIndexAlertDelete {
    pub fn render(alert_state: &AppIndexAlert, full_path: &Rc<Vec<String>>) -> VDomElement {
        let full_path = full_path.clone();
        let progress: Value<bool> = alert_state.app_state.driver.new_value(false);
        let progress_computed = progress.to_computed();

        let message = format!("Czy usunąć -> {} ?", full_path.join("/"));
        let mut alert = AlertBox::new(message, progress_computed.clone());


        let delete_yes = {
            let progress = progress.clone();
            let alert_state = alert_state.clone();

            move || {
                if *progress.get_value() {
                    return;
                }

                let current_path = full_path.as_ref().clone();
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
        };

        let delete_no = {
            let process = progress.clone();
            let alert_state = alert_state.clone();
            move || {
                if *process.get_value() {
                    return;
                }

                alert_state.close_modal();
            }
        };

        alert.button("Tak", {
            // let alert_state = alert_state.clone();
            move || {
                delete_yes();
            }
        });

        alert.button("Nie", {
            move || {
                delete_no();
            }
        });

        alert.render()
    }
}

