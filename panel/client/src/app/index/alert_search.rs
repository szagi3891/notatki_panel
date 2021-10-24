use std::{rc::Rc};

use vertigo::{Css, KeyDownEvent, VDomElement, computed::{Computed, Dependencies, Value}};
use vertigo_html::{css, html};
use crate::{components::AlertBox, request::ResourceError, state_data::{DataState}};

use super::alert::AlertState;

fn css_result() -> Css {
    css!("
        max-height: 80vh;
        overflow: scroll;
    ")
}

fn push_list<F: Fn(&String) -> bool>(
    data_state: &DataState,
    result: &mut Vec<Vec<String>>,
    base: &Vec<String>,
    test_name: &F
) -> Result<(), ResourceError> {
    let list = data_state.get_dir_content(base.as_slice())?;

    if let Some(last) = base.last() {
        if test_name(last) {
            result.push(base.clone());
        }
    }

    let files = {
        let mut files = Vec::<String>::new();

        for (name, item) in list.iter() {
            if !item.dir && test_name(name) {
                files.push(name.clone());
            }
        }

        files.sort();
        files
    };

    for file in files {
        let mut new_base = base.clone();
        new_base.push(file);

        result.push(new_base);
    }

    let dirs = {
        let mut dirs = Vec::<String>::new();

        for (name, item) in list.iter() {
            if item.dir {
                dirs.push(name.clone());
            }
        }

        dirs.sort();
        dirs
    };

    for dir in dirs {
        let mut new_base = base.clone();
        new_base.push(dir);

        push_list(data_state, result, &new_base, test_name)?;
    }

    Ok(())
}

fn new_results(dependencies: &Dependencies, data_state: &DataState, phrase: Computed<String>) -> Computed<Vec<Vec<String>>> {
    let data_state = data_state.clone();

    dependencies.from(move || {
        let mut result = Vec::<Vec<String>>::new();
        let phrase_value = phrase.get_value();

        if phrase_value.len() < 2 {
            return result;
        }

        let test_name = move |name: &String| -> bool {
            let result = name.contains(phrase_value.as_ref());
            result
        };

        let result_push = push_list(&data_state, &mut result, &Vec::new(), &test_name);

        match result_push {
            Ok(()) => {},
            Err(err) => {
                log::error!("Error push list {:?}", err);
            }
        };

        result
    })
}

#[derive(PartialEq)]
pub struct AlertSearch {
    alert_state: Rc<AlertState>,
    pub phrase: Value<String>,
    results: Computed<Vec<Vec<String>>>,
}

impl AlertSearch {
    pub fn new(alert_state: &Rc<AlertState>) -> Computed<AlertSearch> {
        let phrase = alert_state.app_state.root.new_value("".to_string());

        let results = new_results(
            &alert_state.app_state.root,
            &alert_state.app_state.data_state,
            phrase.to_computed(),
        );

        alert_state.app_state.root.new_computed_from(AlertSearch {
            alert_state: alert_state.clone(),
            phrase,
            results,
        })
    }

    fn render_results(state: &Computed<AlertSearch>) -> VDomElement {
        let alert_search_state = state.get_value();

        let results = alert_search_state.results.get_value();

        let mut list = Vec::<VDomElement>::new();

        for item in results.iter() {
            let path = item.as_slice().join("/");
            list.push(html! {
                <div>
                    { path }
                </div>
            });
        }

        html! {
            <div css={css_result()}>
                {..list}
            </div>
        }
    }

    pub fn render(state: &Computed<AlertSearch>) -> VDomElement {
        let alert_search_state = state.get_value();
        let phrase = alert_search_state.phrase.clone();
        let current_value = phrase.get_value();

        let on_input = {
            let phrase = phrase.clone();
            move |new_value: String| {
                phrase.set_value(new_value);
            }
        };

        let on_keydown = move |_event: KeyDownEvent| -> bool {
            false
        };
    
        let on_close = {
            let alert_state = alert_search_state.alert_state.clone();
            move || {
                alert_state.search_close();
            }
        };

        let content = html! {
            <div>
                <input value={current_value.as_ref()} on_input={on_input} on_key_down={on_keydown} />
                <br/>
                
                <div on_click={on_close}>
                    "zamknij"
                </div>

                <br/>
                <br/>

                <component {AlertSearch::render_results} data={state.clone()} />
            </div>
        };

        AlertBox::render_popup(content)
    }
}

