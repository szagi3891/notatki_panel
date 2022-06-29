use common::HandlerMoveItemBody;
use vertigo::{VDomComponent, Value, Resource, Computed, html, bind, css, Css, get_driver, render_value, dom};

use crate::{components::{AlertBox, item_default, item_dot_html, ButtonState, render_path}, data::ListItem, app::{response::check_request_response, App}};

use super::AppIndexAlert;

#[derive(Clone)]
pub struct AppIndexAlertMoveitem {
    pub app: App,
    alert: AppIndexAlert,
    path: Vec<String>,                  //pełna ściezka do przenoszonego elementu
    hash: String,                       //hash przenoszonego elementu
    target: Value<Vec<String>>,         //nowy katalog do którego będziemy przenosić ten element
    progress: Value<bool>,
}


impl AppIndexAlertMoveitem {
    pub fn new(app: &App, alert: &AppIndexAlert, path: Vec<String>, hash: String) -> AppIndexAlertMoveitem {
        let mut target = path.clone();
        target.pop();

        let target = Value::new(target);
        AppIndexAlertMoveitem {
            app: app.clone(),
            alert: alert.clone(),
            path,
            hash,
            target,
            progress: Value::new(false),
        }
    }

    fn prepare_new_path(&self) -> Option<Vec<String>> {
        let mut path = self.path.clone();
        let mut target = self.target.clone().get();

        let last = match path.pop() {
            Some(last) => last,
            None => {
                return None;
            }
        };

        target.push(last);

        Some(target)
    }

    async fn on_save(&self) -> Result<(), String> {
        let new_path = match self.prepare_new_path() {
            Some(new_path) => new_path,
            None => {
                return Err("Problem with computing a new path".into());
            }
        };

        let body: HandlerMoveItemBody = HandlerMoveItemBody {
            path: self.path.clone(),
            hash: self.hash.clone(),
            new_path: new_path,
        };

        let response = get_driver()
            .request("/move_item")
            .body_json(body)
            .post()
            .await;

        check_request_response(response)
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
    
    let target_path = render_path(&state.target.to_computed(), on_click_path);

    VDomComponent::from_fn(move || {
        html! {
            <div css={css_wrapper()}>
                { target_path.clone() }
            </div>
        }
    })
}

fn render_back(state: &AppIndexAlertMoveitem) -> VDomComponent {
    let state = state.clone();
    let target_is_empty = state.target.to_computed().map(|target| {
        target.is_empty()
    });

    let component = render_value(target_is_empty, move |is_empty| {
        match is_empty {
            true => None,
            false => {
                let on_click = bind(&state.target)
                    .call(|target| {
                        let mut value = target.get();
                        value.pop();
                        target.set(value);
                    });
                
                Some(item_dot_html(on_click))
            }
        }  
    });

    let dom = dom! {
        <div>
            {component}
        </div>
    };

    
    VDomComponent::dom(dom)
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

    VDomComponent::dom(ButtonState::render(Computed::from(move || {
        let mut path = state.path.clone();
        path.pop();

        let target = state.target.get();

        if path == target {
            return ButtonState::Disabled { label: "Tak".into() };
        }

        let action = bind(&state)
            .spawn(|state| async move {
                let progress = state.progress.get();

                if progress {
                    log::error!("Trwa obecnie przenoszenie elementu");
                    return;
                }

                state.progress.set(true);
                let response = state.on_save().await;
                state.progress.set(false);

                match response {
                    Ok(()) => {  
                        log::info!("Przenoszenie udane");
                        state.alert.data.git.root.refresh();
                        state.alert.close_modal();
                        state.app.show_message_info("Udane przenoszenie", Some(1000));
                        state.app.data.tab.redirect_item_select_after_delete();
                    },
                    Err(message) => {
                        let message = format!("nie udane przenoszenie {message}");
                        state.app.show_message_error(message.clone(), Some(10000));
                        log::error!("{message}");
                    }
                };
            });
        
        ButtonState::active("Tak", action)
    })))
}

fn render_button_no(state: &AppIndexAlertMoveitem) -> VDomComponent {
    let state = state.clone();

    VDomComponent::dom(ButtonState::render(Computed::from(move || {
        ButtonState::active("Nie", {
            let state = state.clone();
            move || {
                if state.progress.get() {
                    return;
                }

                state.alert.close_modal();
            }
        })
    })))
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

