use std::rc::Rc;

use vertigo::{VDomComponent, Value};

use crate::components::AlertBox;

use super::AppIndexAlert;

#[derive(Clone)]
pub struct AppIndexAlertMoveitem {
    alert: AppIndexAlert,
    path: Rc<Vec<String>>,
    progress: Value<bool>,
}


impl AppIndexAlertMoveitem {
    pub fn new(alert: &AppIndexAlert, path: &Rc<Vec<String>>) -> AppIndexAlertMoveitem {
        AppIndexAlertMoveitem {
            alert: alert.clone(),
            path: path.clone(),
            progress: alert.data.driver.new_value(false),
        }
    }

    pub fn render(&self) -> VDomComponent {
        render(self)
    }

    pub fn delete_no(&self) {
        if *self.progress.get_value() {
            return;
        }

        self.alert.close_modal();
    }
}

fn render(state: &AppIndexAlertMoveitem) -> VDomComponent {
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

