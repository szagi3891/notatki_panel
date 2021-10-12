use std::{rc::Rc};

use vertigo::{ 
    computed::{Computed, Value},
};

use vertigo::{Css, VDomElement};
use vertigo_html::{css, html};

use crate::{app::{AppState, index::ListItem}};


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
    pub name: Value<String>,
    pub is_valid: Computed<bool>,
}

impl NewName {
    pub fn new(
        app_state: &Rc<AppState>,
        list: Computed<Vec<ListItem>>,
        action_save: Computed<bool>,
    ) -> Computed<NewName> {
        let name = app_state.root.new_value(String::from(""));

        let name_exists = {
            let name = name.clone();

            app_state.root.from(move || -> bool {
                let list = list.get_value();

                let name = name.get_value();
                is_exist_in_list(&*name, list)
            })
        };

        let is_valid = {
            let name = name.clone();

            app_state.root.from(move || -> bool {
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

        app_state.root.new_computed_from(NewName {
            action_save,
            name,
            is_valid,
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
}

fn css_wrapper() -> Css {
    css!("
        padding: 5px;
        flex-shrink: 0;
        display: flex;
        flex-direction: column;
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

pub fn render(state_computed: &Computed<NewName>) -> VDomElement {
    let state = state_computed.get_value();

    let content = &state.name.get_value();

    let on_input = {
        let state = state.clone();

        move |new_value: String| {
            state.on_input_name(new_value);
        }
    };

    html! {
        <div css={css_wrapper()}>
            <div css={css_input_wrapper()}>
                <input
                    css={css_input_name()}
                    on_input={on_input}
                    value={content.as_ref()}
                />
            </div>
        </div>
    }
}