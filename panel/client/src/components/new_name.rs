use std::rc::Rc;

use vertigo::{ 
    Computed, Value, VDomComponent, Driver,
};

use vertigo::{Css, VDomElement};
use vertigo::{css, html};
use crate::app::index::ListItem;

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
        driver: &Driver,
        list: Computed<Vec<ListItem>>,
        name: Value<String>,
        action_save: Computed<bool>,
    ) -> (Computed<bool>, Computed<bool>, VDomComponent) {
        let name_exists = {
            let name = name.clone();

            driver.from(move || -> bool {
                let list = list.get_value();

                let name = name.get_value();
                is_exist_in_list(&*name, list)
            })
        };

        let is_valid = {
            let name = name.clone();

            driver.from(move || -> bool {
                let name_exists = name_exists.get_value();

                if *name_exists {
                    return false;
                }

                if name.get_value().is_empty() {
                    return false;
                }

                true
            })
        };

        let save_enable = {
            let is_valid = is_valid.clone();

            driver.from(move || -> bool {
                let new_name_is_valid = is_valid.get_value();

                if !*new_name_is_valid  {
                    return false;
                }

                true
            })
        };

        let state = NewName {
            action_save,
            name,
            is_valid: is_valid.clone(),
        };

        (is_valid, save_enable, driver.bind_render(state, render))
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
