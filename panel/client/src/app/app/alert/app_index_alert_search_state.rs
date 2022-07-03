use vertigo::{Css, Resource, Computed, Value, bind, Context, DomElement, dom, render_list};
use vertigo::{css};
use crate::data::ListItem;
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
    let list = data_state.git.dir_list(context, base.as_slice())?;

    for item in list.get_list() {
        if test_name(&item.name) {
            result.push(item);
        }
    }

    for item in list.get_list() {
        if item.is_dir {
            push_list(context, data_state, result, &item.full_path(), test_name)?;
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

#[derive(Clone)]
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

    pub fn render(&self) -> DomElement {
        render(self)
    }
}


fn render_results(search: &AppIndexAlertSearch) -> DomElement {
    let search = search.clone();

    let list = render_list(
        search.results.clone(),
        |item| item.to_string(),
        move |item| {
            let on_click = bind(&search)
                .and(item)
                .call(|_, search, item| {
                    search.alert.close_modal();
                    search.alert.data.tab.redirect_to_item(item.clone());
                })
            ;

            let icon_el = icon::icon_render(item.is_dir);
            let path = item.to_string();

            dom! {
                <div css={css_result_row()} on_click={on_click}>
                    <div css={css_result_icon()}>
                        {icon_el}
                    </div>
                    {path}
                </div>
            }
        }
    );

    dom! {
        <div css={css_result()}>
            {list}
        </div>
    }
}

fn render_input(search: &AppIndexAlertSearch) -> DomElement {
    let current_value = search.phrase.to_computed();

    let on_input = bind(search).call_param(|_, search, new_value| {
        search.phrase.set(new_value);
    });

    dom! {
        <input autofocus="" value={current_value} on_input={on_input} />
    }
}

fn render_close(search: &AppIndexAlertSearch) -> DomElement {
    let on_close = bind(search).call(|_, search| {
        search.alert.close_modal();
    });

    dom! {
        <div css={css_close()} on_click={on_close}>
            "zamknij"
        </div>
    }
}
pub fn render(search: &AppIndexAlertSearch) -> DomElement {

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
