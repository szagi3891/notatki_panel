use vertigo::{Css, Resource, VDomElement, Computed, Value, VDomComponent, bind};
use vertigo::{css, html};
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
    data_state: &Data,
    result: &mut Vec<ListItem>,
    base: &Vec<String>,
    test_name: &F
) -> Resource<()> {
    let list = data_state.git.dir_list(base.as_slice())?;

    for item in list.get_list() {
        if test_name(&item.name) {
            result.push(item);
        }
    }

    for item in list.get_list() {
        if item.is_dir {
            push_list(data_state, result, &item.full_path(), test_name)?;
        }
    }

    Resource::Ready(())
}

fn new_results(data_state: &Data, phrase: Computed<String>) -> Computed<Vec<ListItem>> {
    let data_state = data_state.clone();

    Computed::from(move || {
        let mut result = Vec::<ListItem>::new();

        let phrase_value = phrase.get().to_lowercase();

        if phrase_value.len() < 2 {
            return result;
        }

        let test_name = move |name: &String| -> bool {
            name.to_lowercase().contains(phrase_value.as_str())
        };

        let result_push = push_list(&data_state, &mut result, &Vec::new(), &test_name);

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

    pub fn render(&self) -> VDomComponent {
        render(self)
    }
}


fn render_results(search: &AppIndexAlertSearch) -> VDomElement {
    let results = search.results.get();

    let mut list = Vec::<VDomElement>::new();

    for item in results.iter() {
        let on_click = bind(search)
            .and(item)
            .call(|search, item| {
                search.alert.close_modal();
                search.alert.data.tab.redirect_to_item(item.clone());
            })
        ;

        let icon_el = icon::icon_render(item.is_dir);
        let path = item.to_string();

        list.push(html! {
            <div css={css_result_row()} on_click={on_click}>
                <div css={css_result_icon()}>
                    {icon_el}
                </div>
                {path}
            </div>
        });
    }

    html! {
        <div css={css_result()}>
            {..list}
        </div>
    }
}

pub fn render(search: &AppIndexAlertSearch) -> VDomComponent {

    let results = VDomComponent::from_ref(search, render_results);

    let content = VDomComponent::from_ref(search, move |search: &AppIndexAlertSearch| {
        let current_value = search.phrase.get();

        let on_input = bind(search).call_param(|search, new_value| {
            search.phrase.set(new_value);
        });

        let on_close = bind(search).call(|search| {
            search.alert.close_modal();
        });

        html! {
            <div css={css_content()}>
                <input autofocus="" value={current_value} on_input={on_input} />
                <br/>
                
                <div css={css_close()} on_click={on_close}>
                    "zamknij"
                </div>

                <br/>
                <br/>

                { results.clone() }
            </div>
        }
    });

    AlertBox::render_popup(content)
}
