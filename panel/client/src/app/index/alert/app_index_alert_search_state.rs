use vertigo::{Css, Resource, VDomElement, Computed, Value, VDomComponent, bind};
use vertigo::{css, html};
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

#[derive(PartialEq, Clone)]
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
    data_state: &Data,
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

        for item in list.get_list() {
            if !item.is_dir && test_name(&item.name) {
                files.push(item.name.clone());
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

        for item in list.get_list() {
            if item.is_dir {
                dirs.push(item.name.clone());
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

fn new_results(data_state: &Data, phrase: Computed<String>) -> Computed<Vec<ResultItem>> {
    let data_state = data_state.clone();

    Computed::from(move || {
        let mut result = Vec::<ResultItem>::new();
        let phrase_value = phrase.get().to_lowercase();

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

#[derive(Clone)]
pub struct AppIndexAlertSearch {
    alert: AppIndexAlert,

    pub phrase: Value<String>,
    // pub 

    results: Computed<Vec<ResultItem>>,
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
        let icon_el = icon::icon_render(item.dir);
        let path = item.to_string();

        let on_click = bind(search)
            .and(item)
            .call(|search, item| {
                let path = item.path.clone();
                let dir = item.dir;

                search.alert.close_modal();
                if dir {
                    search.alert.data.tab.redirect_to_dir(&path);
                } else {
                    search.alert.data.tab.redirect_to_file(&path);
                }
            })
        ;

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

    VDomComponent::from_ref(search, move |search: &AppIndexAlertSearch| {
        let current_value = search.phrase.get();

        let on_input = bind(search).call_param(|search, new_value| {
            search.phrase.set(new_value);
        });

        let on_close = bind(search).call(|search| {
            search.alert.close_modal();
        });

        let content = html! {
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
        };

        AlertBox::render_popup(content)
    })
}
