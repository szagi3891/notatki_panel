use vertigo::{VDomComponent, Value, Resource, Computed, html, bind, css, Css};

use crate::{components::{AlertBox, item_default, ButtonComponent, ButtonState}, data::ListItem};

use super::AppIndexAlert;

#[derive(Clone)]
pub struct AppIndexAlertMoveitem {
    alert: AppIndexAlert,
    path: Vec<String>,
    target: Value<Vec<String>>,
    progress: Value<bool>,
}


impl AppIndexAlertMoveitem {
    pub fn new(alert: &AppIndexAlert, path: Vec<String>) -> AppIndexAlertMoveitem {
        let mut target = path.clone();
        target.pop();

        let target = Value::new(target);
        AppIndexAlertMoveitem {
            alert: alert.clone(),
            path,
            target,
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

fn render_list(state: &AppIndexAlertMoveitem) -> VDomComponent {
    fn css_list() -> Css {
        css!("
            max-height: 70vh;
            overflow-y: scroll;
        ")
    }

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

    let data = state.alert.data.clone();
    let alert = state.alert.clone();
    let target = state.target.clone();
    let list = Computed::from(move || list_calculate(&alert, &target));

    VDomComponent::from_fn(move || {
        let list = list.get();

        match list {
            Resource::Ready(list) => {
                let mut out = Vec::new();

                for item in list {
                    let on_click = bind(&item)
                        .call(|item| {
                            log::info!("kliknięto w element {name}", name = item.name);
                        });

                    out.push(item_default(&data, &item, on_click));
                }

                html! {
                    <div css={css_list()}>
                        { ..out }
                    </div>
                }
            },
            Resource::Error(error) => {
                let message = format!("error = {error}");

                html! {
                    <div>
                        { message }
                    </div>
                }
            },
            Resource::Loading => {
                html! {
                    <div>
                        "Loading ..."
                    </div>
                }
            }
        }
    })
}

fn render_button_yes(state: &AppIndexAlertMoveitem) -> VDomComponent {
    let state = state.clone();

    ButtonComponent::new(move || {
        ButtonState::active("tak", {
            let state = state.clone();
            move || {
                // state.delete_yes();
                log::info!("przenosimy ...");
            }
        })
    })
}

fn render_button_no(state: &AppIndexAlertMoveitem) -> VDomComponent {
    let state = state.clone();

    ButtonComponent::new(move || {
        ButtonState::active("Nie", {
            let state = state.clone();
            move || {
                state.delete_no();
            }
        })
    })
}

fn render_message(state: &AppIndexAlertMoveitem) -> VDomComponent {
    VDomComponent::from_ref(state, |state| {
        let message = format!("Przenoszenie -> {} ?", state.path.join("/"));

        html! {
            <div>
                { message }
            </div>
        }
    })
}

fn render(state: &AppIndexAlertMoveitem) -> VDomComponent {
    let content = render_list(state);
    let button_yes = render_button_yes(state);
    let button_no = render_button_no(state);

    let message = render_message(state);

    let progress = state.progress.to_computed();

    AlertBox::new(message, progress)
        .button(button_yes.clone())
        .button(button_no)
        .set_content(content)
        .render()
}

