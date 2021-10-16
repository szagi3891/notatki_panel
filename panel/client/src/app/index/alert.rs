use std::rc::Rc;
use vertigo::VDomElement;
use vertigo::{
    computed::{
        Computed,
        Value
    },
};
use vertigo_html::{html};
use crate::app::AppState;
use crate::components::AlertBox;

#[derive(PartialEq)]
pub enum AlertView {
    None,
    DeleteFile {
        message: String,
    },
    //delete dir
}

#[derive(PartialEq)]
pub struct AlertState {
    app_state: Rc<AppState>,
    progress: Value<bool>,
    progress_computed: Computed<bool>,
    view: Value<AlertView>,
    // None,
    // DeleteFile {
    //     message: String,
    //     app_state: Rc<AppState>,
    //     progress: Value<bool>,
    // },
    //delete dir
}

impl AlertState {
    pub fn new(app_state: Rc<AppState>) -> Computed<AlertState> {
        let view = app_state.root.new_value(AlertView::None);
        let progress = app_state.root.new_value(false);
        let progress_computed = progress.to_computed();

        app_state.root.new_computed_from(AlertState {
            app_state: app_state.clone(),
            progress,
            progress_computed,
            view,
        })
    }

    fn is_precess(&self) -> bool {
        let value = self.progress.get_value();
        *value == true
    }

    pub fn delete(&self, message: String) {
        if self.is_precess() {
            return;
        }

        log::info!("delete akcja ...");

        self.view.set_value(AlertView::DeleteFile {
            message,
        });

        //trzeba przełączyć się na alert który pyta czy skasować
    }

    //delete_yes ... przetwarzanie ...
    fn delete_yes(&self) {
        if self.is_precess() {
            return;
        }

        log::info!("usuwamy ...");

        //pyknac request
        //przestawic process na true

        
    }

    fn delete_no(&self) {
        if self.is_precess() {
            return;
        }

        self.view.set_value(AlertView::None);
    }
}

pub fn render_alert(state: &Computed<AlertState>) -> VDomElement {
    let alert_state = state.get_value();
    let alert = alert_state.view.get_value();

    match alert.as_ref() {
        AlertView::None => {
            html! {
                <div />
            }
        },
        AlertView::DeleteFile {message} => {
            let message = format!("aler delete file ... {}", message);
            let computed = alert_state.progress_computed.clone();

            let mut alert = AlertBox::new(message, computed);

            alert.button("Nie", {
                let alert_state = alert_state.clone();
                move || {
                    alert_state.delete_no();
                }
            });

            alert.button("Tak", {
                let alert_state = alert_state.clone();
                move || {
                    alert_state.delete_yes();
                }
            });

            alert.render()
        }
    }
}