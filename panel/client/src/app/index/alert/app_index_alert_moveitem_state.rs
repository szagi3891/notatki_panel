use vertigo::{VDomComponent, Value, Resource, Computed, VDomElement, html};

use crate::{components::AlertBox, data::ListItem};

use super::AppIndexAlert;

fn list_calculate(alert: &AppIndexAlert,  target: &Value<Vec<String>>) -> Resource<Vec<ListItem>> {
    let target = target.get();
    let list = alert.data.git.dir_list(target.as_slice())?;

    let mut out = Vec::new();

    for item in list.get_list() {
        if item.is_dir {
            out.push(item);
        }
    }

    Resource::Ready(out)
}

fn list_computed(alert: &AppIndexAlert,  target: &Value<Vec<String>>) -> Computed<Resource<Vec<ListItem>>> {
    let alert = alert.clone();
    let target = target.clone();
    Computed::from(move || list_calculate(&alert, &target))
}

#[derive(Clone)]
pub struct AppIndexAlertMoveitem {
    alert: AppIndexAlert,
    path: Vec<String>,
    target: Value<Vec<String>>,
    list: Computed<Resource<Vec<ListItem>>>,
    progress: Value<bool>,
}


impl AppIndexAlertMoveitem {
    pub fn new(alert: &AppIndexAlert, path: Vec<String>) -> AppIndexAlertMoveitem {
        let mut target = path.clone();
        target.pop();

        let target = Value::new(target);

        let list = list_computed(alert, &target);

        AppIndexAlertMoveitem {
            alert: alert.clone(),
            path,
            target,
            list,
            progress: Value::new(false),
        }
    }

    pub fn render(&self) -> VDomComponent {
        render(self)
    }

    pub fn delete_no(&self) {
        if self.progress.get() {
            return;
        }

        self.alert.close_modal();
    }
}

fn render_list(state: &AppIndexAlertMoveitem) -> VDomElement {
    
    html! {
        <div>
            "lista rozwijanych elementów"
        </div>
    }
}

fn render(state: &AppIndexAlertMoveitem) -> VDomComponent {
    let content = VDomComponent::from_ref(state, render_list);

    VDomComponent::from_ref(state, move |state: &AppIndexAlertMoveitem| {
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

        alert.set_content(content.clone());

        alert.render()
    })
}

