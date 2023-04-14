use common::HandlerMoveItemBody;
use vertigo::{Value, Resource, Computed, bind, css, Css, dom, transaction, Context, bind_spawn, RequestBuilder, DomNode, dom_element, bind_rc};

use crate::{components::{AlertBox, item_default, item_dot_html, ButtonState, render_path}, data::{ListItem, ListItemType}, app::{response::check_request_response, App}};

use super::AppIndexAlert;

#[derive(Clone, PartialEq)]
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
        let mut target = transaction(|context| {
            self.target.clone().get(context)
        });

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
            new_path,
        };

        let response = RequestBuilder::post("/move_item")
            .body_json(body)
            .call()
            .await;

        check_request_response(response)
    }

    pub fn render(&self) -> DomNode {
        render(self)
    }
}

fn render_target(state: &AppIndexAlertMoveitem) -> DomNode {
    fn css_wrapper() -> Css {
        css!("
            margin-top: 5px;
            border-top: 1px solid black;
            margin-bottom: 5px;
        ")
    }
    let on_click_path = bind!(state, |node_id: Vec<String>| {
        state.target.set(node_id);
    });
    
    let target_path = render_path(&state.target.to_computed(), on_click_path);

    dom! {
        <div css={css_wrapper()}>
            { target_path }
        </div>
    }
}

fn render_back(state: &AppIndexAlertMoveitem) -> DomNode {
    let state = state.clone();
    let target_is_empty = state.target.to_computed().map(|target| {
        target.is_empty()
    });

    target_is_empty.render_value_option(move |is_empty| {
        match is_empty {
            true => None,
            false => {
                let target = &state.target;

                let on_click = bind!(target, || {
                    target.change(|inner| {
                        inner.pop();
                    });
                });
                
                Some(item_dot_html(on_click))
            }
        }  
    })
}

fn render_list(state: &AppIndexAlertMoveitem) -> DomNode {
    fn css_list() -> Css {
        css!("
            max-height: 70vh;
            overflow-y: scroll;
        ")
    }

    fn list_calculate(context: &Context, alert: &AppIndexAlert, target: &Value<Vec<String>>) -> Resource<Vec<ListItem>> {
        let target = target.get(context);
        let list = alert.data.git.dir_list(context, target.as_slice())?;
        let list = list.get_list(context);

        let mut out = Vec::new();

        for item in list {
            if item.is_dir.get(context) == ListItemType::Dir {
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
        move |context| list_calculate(context, &alert, &target)
    });


    list.render_value({
        let state = state.clone();

        move |list| {
            match list {
                Resource::Ready(list) => {
                    let target_view = render_target(&state);
                    let back_view = render_back(&state);
                
                    let out = dom_element! {
                        <div css={css_list()}>
                            { target_view }
                            { back_view }
                        </div>
                    };

                    for item in list {
                        let on_click = Computed::from(bind!(target, item, |_| {
                            bind_rc!(target, item, || {
                                log::info!("kliknięto w element {name}", name = item.name);
    
                                target.change(|inner| {
                                    inner.push(item.name.clone());    
                                });
                            })
                        }));

                        out.add_child(item_default(&data, &item, on_click));
                    }

                    out.into()
                },
                Resource::Error(error) => {
                    let message = format!("error = {error}");

                    dom! {
                        <div>
                            { message }
                        </div>
                    }
                },
                Resource::Loading => {
                    dom! {
                        <div>
                            "Loading ..."
                        </div>
                    }
                }
            }
        }
    })
}

fn render_button_yes(state: &AppIndexAlertMoveitem) -> DomNode {
    let state = state.clone();

    ButtonState::render(Computed::from(move |context| {
        let mut path = state.path.clone();
        path.pop();

        let target = state.target.get(context);

        if path == target {
            return ButtonState::Disabled { label: "Tak".into() };
        }

        let action = bind_spawn!(state, async move {
            let state = state.clone();

            let progress = transaction(|context| {
                state.progress.get(context)
            });

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
    }))
}

fn render_button_no(state: &AppIndexAlertMoveitem) -> DomNode {
    let state = state.clone();

    ButtonState::render(Computed::from(move |_| {
        ButtonState::active("Nie", {
            let state = state.clone();
            move || {
                transaction(|context| {
                    if !state.progress.get(context) {
                        state.alert.close_modal();
                    }
                });
            }
        })
    }))
}

fn render_message(state: &AppIndexAlertMoveitem) -> DomNode {
    let message = format!("Przenoszenie -> {} ?", state.path.join("/"));

    dom! {
        <div>
            { message }
        </div>
    }
}

fn render(state: &AppIndexAlertMoveitem) -> DomNode {
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

