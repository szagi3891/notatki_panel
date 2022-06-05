use vertigo::{VDomComponent, Value, Resource, Computed, html, bind, css, Css};

use crate::{components::{AlertBox, item_default, item_dot_html, ButtonComponent, ButtonState, render_path}, data::ListItem};

use super::AppIndexAlert;

#[derive(Clone)]
pub struct AppIndexAlertMoveitem {
    alert: AppIndexAlert,
    path: Vec<String>,                  //pełna ściezka do przenoszonego elementu
    target: Value<Vec<String>>,         //nowy katalog do którego będziemy przenosić ten element
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

fn render_target(state: &AppIndexAlertMoveitem) -> VDomComponent {
    fn css_wrapper() -> Css {
        css!("
            margin-top: 5px;
            border-top: 1px solid black;
            margin-bottom: 5px;
        ")
    }
    let on_click_path = bind(state).call_param(|data, node_id: Vec<String>| {
        data.target.set(node_id);
    });
    
    let target_path = render_path(&state.target, on_click_path);

    VDomComponent::from_html(
        html! {
            <div css={css_wrapper()}>
                { target_path }
            </div>
        }
    )
}

fn render_back(state: &AppIndexAlertMoveitem) -> VDomComponent {
    VDomComponent::from_ref(state, |state| {
        let target = state.target.clone().get();

        if target.is_empty() {
            html! {
                <div/>
            }
        } else {
            let on_click = bind(&state.target)
                .call(|target| {
                    let mut value = target.get();
                    value.pop();
                    target.set(value);
                });
    
            html! {
                <div>
                    { item_dot_html(on_click) }
                </div>
            }
        }
    })
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

    let target_view = render_target(state);
    let back_view = render_back(state);

    VDomComponent::from_fn(move || {
        let list = list.get();

        match list {
            Resource::Ready(list) => {
                let mut out = Vec::new();

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
                        { target_view.clone() }
                        { back_view.clone() }
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
        let mut path = state.path.clone();
        path.pop();

        let target = state.target.get();

        if path == target {
            return ButtonState::Disabled { label: "Tak".into() };
        }
    
        let target = target.join("/");

        ButtonState::active("Tak", {
            let state = state.clone();
            move || {
                // state.delete_yes();
                log::info!("przenosimy do ... {target}");
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

