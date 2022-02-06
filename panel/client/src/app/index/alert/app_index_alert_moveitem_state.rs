use std::rc::Rc;

use vertigo::{VDomComponent, html, VDomElement, css, Css};

use crate::components::AlertBox;

use super::AppIndexAlert;

fn css_close() -> Css {
    css!("
        cursor: pointer;
    ")
}

pub struct AppIndexAlertMoveitem {
    alert_state: AppIndexAlert,
    path: Rc<Vec<String>>,
}


impl AppIndexAlertMoveitem {
    pub fn component(alert_state: &AppIndexAlert, path: &Rc<Vec<String>>) -> VDomComponent {
        let state = AppIndexAlertMoveitem {
            alert_state: alert_state.clone(),
            path: path.clone(),
        };

        VDomComponent::new(state, render)
    }
}

fn render(state: &AppIndexAlertMoveitem) -> VDomElement {
    let on_close = {
        let alert_state = state.alert_state.clone();

        move || {
            //state.close_modal();
            alert_state.close_modal();
        }
    };

    let content = html! {
        <div>
            <div css={css_close()} on_click={on_close}>
                "zamknij"
            </div>
            
            "przenoszenie elementu -> " {state.path.join("/")}


        </div>
    };

    AlertBox::render_popup(content)
}

