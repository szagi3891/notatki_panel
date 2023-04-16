use common::HandlerMoveItemBody;
use vertigo::{Value, Resource, Computed, bind, css, Css, dom, transaction, Context, bind_spawn, RequestBuilder, DomNode, dom_element, bind_rc};

use crate::{components::{AlertBox, item_default, item_dot_html, ButtonState, render_path}, data::{ListItem, ListItemType}, app::{response::check_request_response, App}};

use super::AppIndexAlert;

#[derive(Clone, PartialEq)]
pub struct AppIndexAlertMoveitem {
    pub app: App,
    alert: AppIndexAlert,
    item: ListItem,                                     //pełna ściezka do przenoszonego elementu
    hash: String,                                       //hash przenoszonego elementu
    target_dir: Value<ListItem>,                        //nowy katalog do którego będziemy przenosić ten element
    progress: Value<bool>,

    // new_path: Computed<ListItem>,                       //docelowa lokalizacja do której zostanie przeniesiony item.
}


impl AppIndexAlertMoveitem {
    pub fn new(app: &App, alert: &AppIndexAlert, item: ListItem, hash: String) -> AppIndexAlertMoveitem {
        let target = item.back();

        AppIndexAlertMoveitem {
            app: app.clone(),
            alert: alert.clone(),
            item,
            hash,
            target_dir: Value::new(target),
            progress: Value::new(false),
        }
    }

    fn prepare_new_path(&self) -> Option<Vec<String>> {
        let self_clone = self.clone();

        transaction(|context| {
            let mut path = self_clone.item.to_vec_path();
            let mut target = self_clone.target_dir.clone().get(context).full_path.as_ref().clone();
    
            let last = match path.pop() {
                Some(last) => last,
                None => {
                    return None;
                }
            };
    
            target.push(last);
    
            Some(target)
        })

    }

    async fn on_save(&self) -> Result<(), String> {
        let new_path = match self.prepare_new_path() {
            Some(new_path) => new_path,
            None => {
                return Err("Problem with computing a new path".into());
            }
        };

        let body: HandlerMoveItemBody = HandlerMoveItemBody {
            path: self.item.to_vec_path(),
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
        let new_target = state.app.data.items.get_from_path(&node_id);

        state.target_dir.set(new_target);
    });
    
    let target_path = render_path(&state.target_dir.to_computed(), on_click_path);

    dom! {
        <div css={css_wrapper()}>
            { target_path }
        </div>
    }
}

fn render_back(state: &AppIndexAlertMoveitem) -> DomNode {
    let state = state.clone();

    let target_is_root = state.target_dir.map(|item| item.is_root());

    target_is_root.render_value_option(move |is_empty| {
        match is_empty {
            true => None,
            false => {
                let target = &state.target_dir;

                let on_click = bind!(target, || {
                    transaction(|context| {                        
                        let dir = target.get(context).back();
                        target.set(dir);
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

    fn list_calculate(context: &Context, target: &Value<ListItem>) -> Resource<Vec<ListItem>> {
        let target = target.get(context);
        let list = target.list.get(context)?;

        let mut out = Vec::new();

        for item in list {
            if item.is_dir.get(context) == ListItemType::Dir {
                out.push(item);
            }
        }

        Resource::Ready(out)
    }

    let data = state.alert.data.clone();
    let target = state.target_dir.clone();
    let list = Computed::from({
        let target = target.clone();
        move |context| list_calculate(context, &target)
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
                                log::info!("kliknięto w element {name}", name = item.name());
    
                                target.set(item.clone());
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

//TODO - dodać sprawdzenie zeby sie uprwnić, ze ListItem wskazywany new_path na pewno nie istnieje

fn render_button_yes(state: &AppIndexAlertMoveitem) -> DomNode {
    let state = state.clone();

    ButtonState::render(Computed::from(move |context| {
        let path = state.item.back();

        let target = state.target_dir.get(context);

        if path == target {
            return ButtonState::disabled("Tak");
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
    let path = state.item.to_string_path();

    dom! {
        <div>
            "Przenoszenie -> "
            { path }
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

