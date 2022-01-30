use vertigo::{Css, Driver, Resource, VDomElement, Computed, Value, VDomComponent};
use vertigo::{css, html};
use crate::{components::AlertBox, data::{StateData}};
use crate::components::icon;

use super::state_alert::StateAlert;

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

#[derive(PartialEq)]
struct ResultItem {
    path: Vec<String>,
    dir: bool,
}

impl ResultItem {
    fn new_dir(path: Vec<String>) -> ResultItem {
        ResultItem {
            path,
            dir: true
        }
    }
    fn new_file(path: Vec<String>) -> ResultItem {
        ResultItem {
            path,
            dir: false
        }
    }
    fn to_string(&self) -> String {
        self.path.as_slice().join("/")
    }
}

fn push_list<F: Fn(&String) -> bool>(
    data_state: &StateData,
    result: &mut Vec<ResultItem>,
    base: &Vec<String>,
    test_name: &F
) -> Resource<()> {
    let list = data_state.git.dir_list(base.as_slice())?;

    if let Some(last) = base.last() {
        if test_name(last) {
            result.push(ResultItem::new_dir(base.clone()));
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

        result.push(ResultItem::new_file(new_base));
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

    Resource::Ready(())
}

fn new_results(driver: &Driver, data_state: &StateData, phrase: Computed<String>) -> Computed<Vec<ResultItem>> {
    let data_state = data_state.clone();

    driver.from(move || {
        let mut result = Vec::<ResultItem>::new();
        let phrase_value = phrase.get_value().to_lowercase();

        if phrase_value.len() < 2 {
            return result;
        }

        let test_name = move |name: &String| -> bool {
            let result = name.to_lowercase().contains(phrase_value.as_str());
            result
        };

        let result_push = push_list(&data_state, &mut result, &Vec::new(), &test_name);

        match result_push {
            Resource::Ready(()) => {},
            Resource::Loading => {},
            Resource::Error(err) => {
                log::error!("Error push list {:?}", err);
            }
        };

        result
    })
}

#[derive(PartialEq)]
pub struct StateAlertSearch {
    alert_state: StateAlert,
    pub phrase: Value<String>,
    results: Computed<Vec<ResultItem>>,
}

impl StateAlertSearch {
    pub fn component(alert_state: &StateAlert) -> VDomComponent {
        let phrase = alert_state.app_state.driver.new_value("".to_string());

        let results = new_results(
            &alert_state.app_state.driver,
            &alert_state.app_state.data,
            phrase.to_computed(),
        );

        let state = StateAlertSearch {
            alert_state: alert_state.clone(),
            phrase,
            results,
        };

        alert_state.app_state.driver.bind_render(state, render)
    }
}


fn render_results(state: &Computed<StateAlertSearch>) -> VDomElement {
    let alert_search_state = state.get_value();

    let results = alert_search_state.results.get_value();

    let mut list = Vec::<VDomElement>::new();

    for item in results.iter() {
        let icon_el = icon::icon_render(item.dir);
        let path = item.to_string();

        let on_click = {
            let alert_search_state = alert_search_state.clone();
            let path = item.path.clone();
            let dir = item.dir;

            move || {
                alert_search_state.alert_state.close_modal();
                if dir {
                    alert_search_state.alert_state.app_state.data.tab.redirect_to_dir(&path);
                } else {
                    alert_search_state.alert_state.app_state.data.tab.redirect_to_file(&path);
                }
            }
        };

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

pub fn render(state: &Computed<StateAlertSearch>) -> VDomElement {
    let alert_search_state = state.get_value();
    let phrase = alert_search_state.phrase.clone();
    let current_value = phrase.get_value();

    let on_input = {
        move |new_value: String| {
            phrase.set_value(new_value);
        }
    };

    let alert_state = alert_search_state.alert_state.clone();

    let on_close = {
        move || {
            alert_state.close_modal();
        }
    };

    let content = html! {
        <div css={css_content()}>
            <input autofocus="" value={current_value.as_ref()} on_input={on_input} />
            <br/>
            
            <div css={css_close()} on_click={on_close}>
                "zamknij"
            </div>

            <br/>
            <br/>

            <component {render_results} data={state.clone()} />
        </div>
    };

    AlertBox::render_popup(content)
}
