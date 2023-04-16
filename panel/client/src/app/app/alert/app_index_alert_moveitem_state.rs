use common::HandlerMoveItemBody;
use vertigo::{Value, Resource, Computed, bind, css, Css, dom, transaction, Context, bind_spawn, RequestBuilder, DomNode, dom_element, bind_rc};

use crate::{components::{AlertBox, item_default, item_dot_html, ButtonState, render_path}, data::{ListItem, ListItemType}, app::{response::check_request_response, App}};

use super::AppIndexAlert;

//TODO - wypozycjonować to okno wyskakujace, zeby górna krawędź nie skakała w sytuacji gdy zmienia się wysokość zawartości

#[derive(Clone, PartialEq)]
pub struct AppIndexAlertMoveitem {
    pub app: App,
    alert: AppIndexAlert,
    item: ListItem,                                     //pełna ściezka do przenoszonego elementu
    hash: String,                                       //hash przenoszonego elementu
    target_dir: Value<ListItem>,                        //nowy katalog do którego będziemy przenosić ten element
    progress: Value<bool>,

    new_path: Computed<ListItem>,                       //docelowa lokalizacja do której zostanie przeniesiony item.
}

impl AppIndexAlertMoveitem {
    pub fn new(app: &App, alert: &AppIndexAlert, item: ListItem, hash: String) -> AppIndexAlertMoveitem {
        let target_dir = Value::new(item.dir());

        let new_path = Computed::from({
            let item = item.clone();
            let target_dir = target_dir.clone();

            move |context| {
                let name = item.name();
                let target = target_dir.get(context);
                target.push(name)
            }
        });

        AppIndexAlertMoveitem {
            app: app.clone(),
            alert: alert.clone(),
            item,
            hash,
            target_dir,
            progress: Value::new(false),
            new_path,
        }
    }

    async fn on_save(&self, new_path: ListItem) -> Result<(), String> {
        let body: HandlerMoveItemBody = HandlerMoveItemBody {
            path: self.item.to_vec_path(),
            hash: self.hash.clone(),
            new_path: new_path.to_vec_path(),
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
                        let dir = target.get(context).dir();
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

fn render_button_yes(state: &AppIndexAlertMoveitem) -> DomNode {
    let state = state.clone();

    ButtonState::render(Computed::from(move |context| {
        let path = state.item.dir();

        let target = state.target_dir.get(context);

        if path == target {
            return ButtonState::disabled("Tak");
        }

        let new_path = state.new_path.get(context);
        let id = state.new_path.get(context).id.get(context);

        match id {
            Resource::Ready(_) => {
                return ButtonState::disabled("Tak");
            },
            Resource::Loading => {
                return ButtonState::disabled("Tak");
            },
            Resource::Error(_) => {}
        };

        let action = bind_spawn!(state, new_path, async move {
            let state = state.clone();

            let progress = transaction(|context| {
                state.progress.get(context)
            });

            if progress {
                log::error!("Trwa obecnie przenoszenie elementu");
                return;
            }

            state.progress.set(true);
            let response = state.on_save(new_path).await;
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
    let new_path = state.new_path.map(|item| item.to_string_path());

    let message = Computed::from({
        let item = state.item.clone();
        let new_path = state.new_path.clone();
    
        move |context| {
            let new_path = new_path.get(context);

            if item == new_path {
                return Some("Wybierz docelowy katalog".into());
            }

            let id = new_path.id.get(context);

            match id {
                Resource::Ready(_) => {
                    Some(format!("W docelowym katalogu już istnieje plik o takiej nazwie \"{}\"", new_path.name()))
                },
                Resource::Loading => {
                    None
                },
                Resource::Error(_) => {
                    None
                }
            }
        }
    });

    let message = message.render_value(|message| {
        match message {
            Some(message) => {
                let css = css!("
                    color: red;
                    margin-top: 20px;
                ");

                dom! {
                    <div css={css}>
                        { message }
                    </div>
                }
            },
            None => {
                dom! {
                    <span />
                }
            }
        }
    });

    fn wrapper_table() -> Css {
        css!("
            border: 1px solid #777;
            border-collapse: collapse;
        ")
    }

    fn wrapper_td() -> Css {
        css!("
            border: 1px solid #777;
            border-collapse: collapse;
            padding: 5px;
        ")
    }

    let main_wrapper = css!("
        align-items: center;
        display: flex;
        flex-direction: column;
    ");

    dom! {
        <div css={main_wrapper}>
            <table css={wrapper_table()}>
                <tr>
                    <td css={wrapper_td()}>
                        "Przenoszenie"
                    </td>
                    <td css={wrapper_td()}>
                        { path }
                    </td>
                </tr>
                <tr>
                    <td css={wrapper_td()}>
                        "Do"
                    </td>
                    <td css={wrapper_td()}>
                        { new_path }
                    </td>
                </tr>
            </table>

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

