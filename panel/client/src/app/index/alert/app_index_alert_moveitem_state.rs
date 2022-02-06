use std::rc::Rc;

use vertigo::{VDomComponent, css, Css, Value};

use crate::components::AlertBox;

use super::AppIndexAlert;

#[derive(Clone)]
pub struct AppIndexAlertMoveitem {
    alert_state: AppIndexAlert,
    path: Rc<Vec<String>>,
    progress: Value<bool>,
}


impl AppIndexAlertMoveitem {
    pub fn component(alert_state: &AppIndexAlert, path: &Rc<Vec<String>>) -> VDomComponent {
        let state = AppIndexAlertMoveitem {
            alert_state: alert_state.clone(),
            path: path.clone(),
            progress: alert_state.app_state.driver.new_value(false),
        };

        render(state)
    }

    pub fn delete_no(&self) {
        if *self.progress.get_value() {
            return;
        }

        self.alert_state.close_modal();
    }
}

fn render(state: AppIndexAlertMoveitem) -> VDomComponent {
    VDomComponent::new(state, move |state: &AppIndexAlertMoveitem| {
        let progress = state.progress.to_computed();

        let message = format!("Przenoszenie -> {} ?", state.path.join("/"));
        let mut alert = AlertBox::new(message, progress);

        alert.button("Tak", {
            // let state = state.clone();
            move || {
                // state.delete_yes();
            }
        });

        alert.button("Nie", {
            let state = state.clone();
            move || {
                state.delete_no();
            }
        });

        // alert.set_content()
        //VDomComponent - ustawić content dla tego popupa z listą do nawigowania po docelowym katalogu

        alert.render()
    })
}

