use std::{ops::Deref, rc::Rc};

use vertigo::{ 
    computed::{Computed, Dependencies, Value},
};

use vertigo::{Css, VDomElement};
use vertigo_html::{css, html};

use crate::{app::index::ListItem};


fn is_exist_in_list(name: &String, list: Rc<Vec<ListItem>>) -> bool {
    for item in list.as_ref() {
        if item.name == *name {
            return true;
        }
    }
    false
}


#[derive(PartialEq)]
pub struct NewName {
    pub action_save: Computed<bool>,
    pub relative_path: Value<Vec<String>>,
    pub name: Value<String>,
    pub is_valid: Computed<bool>,
    pub full_relative_new_path: Computed<Vec<String>>,
}

impl NewName {
    pub fn new(
        deep: &Dependencies,
        list: Computed<Vec<ListItem>>,
        action_save: Computed<bool>,
    ) -> Computed<NewName> {
        let new_dir = deep.new_value(Vec::new());
        let name = deep.new_value(String::from(""));

        let name_exists = {
            let new_dir = new_dir.clone();
            let name = name.clone();

            deep.from(move || -> bool {
                let list = list.get_value();

                let new_dir = new_dir.get_value();
                let first_dir_name = new_dir.get(0);

                if let Some(first_dir_name) = first_dir_name {
                    return is_exist_in_list(first_dir_name, list);
                }

                let name = name.get_value();
                is_exist_in_list(&*name, list)
            })
        };

        let is_valid = {
            let name = name.clone();

            deep.from(move || -> bool {
                let name_exists = name_exists.get_value();

                if *name_exists {
                    return false;
                }

                let name = name.get_value();
                if name.is_empty() {
                    return false;
                }

                true
            })
        };

        let new_path = {
            let new_dir = new_dir.clone();
            let name = name.clone();

            deep.from(move || -> Vec<String> {
                let mut new_dir = new_dir.get_value().deref().clone();
                let name = name.get_value().deref().clone();
                
                new_dir.push(name);
                new_dir
            })
        };
    
        deep.new_computed_from(NewName {
            action_save,
            relative_path: new_dir,
            name,
            is_valid,
            full_relative_new_path: new_path,
        })
    }


    pub fn on_input_name(&self, new_value: String) {
        let action_save = self.action_save.get_value();

        if *action_save {
            log::error!("Trwa obecnie zapis");
            return;
        }

        self.name.set_value(new_value);
    }

    pub fn on_add_dir(&self) {
        let name = self.name.get_value();

        if name.is_empty() {
            log::error!("Brak nazwy do dodania");
            return;
        }

        let mut relative_path = (*self.relative_path.get_value()).clone();
        relative_path.push((*name).clone());
        self.relative_path.set_value(relative_path);
        self.name.set_value("".into());
    }

    pub fn on_remove_last(&self) {
        let mut relative_path = (*self.relative_path.get_value()).clone();
        relative_path.pop();
        self.relative_path.set_value(relative_path);
    }
}

fn css_wrapper() -> Css {
    css!("
        padding: 5px;
        flex-shrink: 0;
        display: flex;
        flex-direction: column;
    ")
}

fn css_path() -> Css {
    css!("
        
    ")
}

fn css_input_wrapper() -> Css {
    css!("
        display: flex;
        margin: 5px 0;
    ")
}

fn css_input_name() -> Css {
    css!("
        flex-grow: 1;
        border: 0;
        border: 1px solid blue;
        :focus {
            border: 0;
        }

        height: 30px;
        line-height: 30px;
        padding: 0 5px;
    ")
}

fn css_button_dir() -> Css {
    css!("
        flex-shrink: 0;
        margin-left: 5px;
    ")
}

fn css_subdir() -> Css {
    css!("
        border: 1px solid black;
        margin: 0 5px;
        padding: 0 5px;
        border-radius: 5px;
        cursor: pointer;
    ")
}

fn render_path(state_computed: &Computed<NewName>) -> VDomElement {
    let state = state_computed.get_value();

    let mut subdir = Vec::new();

    let relative_path = state.relative_path.get_value();

    if relative_path.is_empty() {
        return html! {
            <div css={css_path()}>
                "..."
            </div>
        };
    }

    for dir_name in state.relative_path.get_value().iter() {
        subdir.push(html! {
            <span css={css_subdir()}>
                { dir_name }
            </span>
        });
    }

    let on_remove_last = {
        let state = state;

        move || {
            state.on_remove_last();
        }
    };

    html! {
        <div css={css_path()}>
            "Podkatalog:"
            { ..subdir }
            <button onClick={on_remove_last}>
                "usuń ostatni"
            </button>
        </div>
    }
}

pub fn render(state_computed: &Computed<NewName>) -> VDomElement {
    let state = state_computed.get_value();

    let content = &state.name.get_value();

    let on_input = {
        let state = state.clone();

        move |new_value: String| {
            state.on_input_name(new_value);
        }
    };

    let on_new_subdir = {
        let state = state;

        move || {
            state.on_add_dir();
        }
    };

    html! {
        <div css={css_wrapper()}>
            <component {render_path} data={state_computed.clone()} />
            <div css={css_input_wrapper()}>
                <input
                    css={css_input_name()}
                    onInput={on_input}
                    value={content.as_ref()}
                />
                <button onClick={on_new_subdir} css={css_button_dir()}>
                    "Dodaj jako podkatalog"
                </button>
            </div>
        </div>
    }
}
