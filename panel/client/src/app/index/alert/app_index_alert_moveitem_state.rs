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
}

fn render_list(state: &AppIndexAlertMoveitem) -> VDomComponent {
    fn css_list() -> Css {
        css!("
            max-height: 70vh;
            overflow-y: scroll;
        ")
    }

    fn list_calculate(alert: &AppIndexAlert, target: &Value<Vec<String>>) -> Resource<Vec<ListItem>> {
        let target = target.get();
        let list = alert.data.git.dir_list(target.as_slice())?;
        let list = list.get_list();

        let mut out = Vec::new();

        for item in list {
            if item.is_dir {
                out.push(item);
            }
        }

        Resource::Ready(out)
    }

    let data = state.alert.data.clone();
    let alert = state.alert.clone();
    let target = state.target.clone();
    let list = Computed::from({
        let target = target.clone();
        move || list_calculate(&alert, &target)
    });

    VDomComponent::from_fn(move || {
        let target_value = target.get().join("/");
        let target_message = format!("Target ==> {target_value}");
        let list = list.get();

        match list {
            Resource::Ready(list) => {
                let mut out = Vec::new();

                out.push(html!{
                    <div>
                        { target_message }
                    </div>
                });

                for item in list {
                    let on_click = bind(&item)
                        .and(&target)
                        .call(|item, target| {
                            log::info!("kliknięto w element {name}", name = item.name);

                            let mut target_value = target.get();
                            target_value.push(item.name.clone());
                            target.set(target_value);
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
        
        //Trzeba sprawdzić, jeśli da się przenieść element, to pkazuj przycisk tak, ze jest aktywny ...

        ButtonState::active("Tak", {
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
                if state.progress.get() {
                    return;
                }

                state.alert.close_modal();
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

    AlertBox::new(message)
        .progress(progress)
        .button(button_no)
        .button(button_yes)
        .set_content(content)
        .render()
}

