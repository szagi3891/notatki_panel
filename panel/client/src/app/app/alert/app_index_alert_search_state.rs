use vertigo::{Css, Resource, Computed, Value, bind, Context, dom, DomNode, bind_rc};
use vertigo::{css};
use crate::data::{ListItem, ListItemType};
use crate::{components::AlertBox, data::{Data}};
use crate::components::icon;

use super::AppIndexAlert;

fn css_content() -> Css {
    css!("
        padding: 0 20px;
    ")
}

fn css_result() -> Css {
    css!("
        max-height: 80vh;
        overflow: scroll;
    ")
}

fn css_close() -> Css {
    css!("
        cursor: pointer;
    ")
}

fn css_result_row() -> Css {
    css!("
        display: flex;
        margin-right: 5px;
        margin-bottom: 5px;
        cursor: pointer;
    ")
}

fn css_result_icon() -> Css {
    css!("
        margin-right: 5px;
    ")
}

fn push_list<F: Fn(&String) -> bool>(
    context: &Context,
    data_state: &Data,
    result: &mut Vec<ListItem>,
    base: &Vec<String>,
    test_name: &F
) -> Resource<()> {
    let list = data_state.items.get_from_path(base.as_slice()).list.get(context)?;

    for item in list.iter() {
        if test_name(&item.name()) {
            result.push(item.clone());
        }
    }

    for item in list {
        if item.is_dir.get(context) == ListItemType::Dir {
            push_list(context, data_state, result, &item.full_path, test_name)?;
        }
    }

    Resource::Ready(())
}

fn new_results(data_state: &Data, phrase: Computed<String>) -> Computed<Vec<ListItem>> {
    let data_state = data_state.clone();

    Computed::from(move |context| {
        let mut result = Vec::<ListItem>::new();

        let phrase_value = phrase.get(context).to_lowercase();

        if phrase_value.len() < 2 {
            return result;
        }

        let test_name = move |name: &String| -> bool {
            name.to_lowercase().contains(phrase_value.as_str())
        };

        let result_push = push_list(context, &data_state, &mut result, &Vec::new(), &test_name);

        match result_push {
            Resource::Ready(()) => {},
            Resource::Loading => {},
            Resource::Error(err) => {
                log::error!("Error push list {:?}", err);
            }
        };

        result.sort();

        result
    })
}

#[derive(Clone, PartialEq)]
pub struct AppIndexAlertSearch {
    alert: AppIndexAlert,

    pub phrase: Value<String>,
    // pub 

    results: Computed<Vec<ListItem>>,
}

impl AppIndexAlertSearch {
    pub fn new(alert: &AppIndexAlert) -> AppIndexAlertSearch {
        let phrase = Value::new("".to_string());

        let results = new_results(
            &alert.data,
            phrase.to_computed(),
        );

        AppIndexAlertSearch {
            alert: alert.clone(),
            phrase,
            results,
        }
    }

    pub fn render(&self) -> DomNode {
        render(self)
    }
}

//spróbować z takim tworzeniem computed
// Computed::from_pars(
//     (aa, bb, cc),
//     |(aa, bb, cc), context| {

//     }
// )


fn render_results(search: &AppIndexAlertSearch) -> DomNode {
    let search = search.clone();

    let list = search.results.render_list(|item| item.to_string_path(), {
        let search = search.clone();
        move |item| {
            let redirect_to_item = search.alert.data.tab.build_redirect_to_item(item.clone());

            let on_click = Computed::from(bind!(search, redirect_to_item, |context| {
                let redirect_to_item = redirect_to_item.get(context);

                bind_rc!(search, || {
                    search.alert.close_modal();
                    redirect_to_item();
                })
            }));
            
            let icon_el = icon::icon_render(item);
            let path = item.to_string_path();

            dom! {
                <div css={css_result_row()} on_click={on_click}>
                    <div css={css_result_icon()}>
                        {icon_el}
                    </div>
                    {path}
                </div>
            }
        }
    });

    dom! {
        <div css={css_result()}>
            {list}
        </div>
    }
}

fn render_input(search: &AppIndexAlertSearch) -> DomNode {
    let current_value = search.phrase.to_computed();

    let on_input = bind!(search, |new_value: String| {
        search.phrase.set(new_value);
    });

    dom! {
        <input autofocus="" value={current_value} on_input={on_input} />
    }
}

fn render_close(search: &AppIndexAlertSearch) -> DomNode {
    let on_close = bind!(search, || {
        search.alert.close_modal();
    });

    dom! {
        <div css={css_close()} on_click={on_close}>
            "zamknij"
        </div>
    }
}
pub fn render(search: &AppIndexAlertSearch) -> DomNode {

    let results = render_results(search);

    let input_view = render_input(search);
    let close_view = render_close(search);

    let content = dom! {
        <div css={css_content()}>
            { input_view }
            <br/>
            
            { close_view }

            <br/>
            <br/>

            { results }
        </div>
    };

    AlertBox::render_popup(content)
}
