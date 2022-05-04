use std::rc::Rc;

use vertigo::{Computed, Value, VDomComponent};

use vertigo::{Css, VDomElement};
use vertigo::{css, html};

use crate::data::ListItem;

fn is_exist_in_list(name: &String, list: Rc<Vec<ListItem>>) -> bool {
    for item in list.as_ref() {
        if item.name == *name {
            return true;
        }
    }
    false
}

#[derive(Clone)]
pub struct NewName {
    pub name: Value<String>,
    pub is_valid: Computed<bool>,
}

impl NewName {
    pub fn new(
        list: Computed<Vec<ListItem>>,
    ) -> NewName {
        let name: Value<String> = Value::new("".to_string());

        let name_exists = {
            let name = name.clone();

            Computed::from(move || -> bool {
                let list = list.get();

                let name = name.get();
                is_exist_in_list(name.as_ref(), list)
            })
        };

        let is_valid = {
            let name = name.clone();

            Computed::from(move || -> bool {
                let name_exists = name_exists.get();

                if *name_exists {
                    return false;
                }

                if name.get().is_empty() {
                    return false;
                }

                true
            })
        };

        NewName {
            name,
            is_valid: is_valid.clone(),
        }
    }

    pub fn render(&self, autofocus: bool) -> VDomComponent {
        VDomComponent::new(self, move |state: &NewName| {
            render(state, autofocus)
        })
    }

    pub fn on_input_name(&self, new_value: String) {
        self.name.set(new_value);
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

pub fn render(state: &NewName, autofocus: bool) -> VDomElement {
    let content = &state.name.get();

    let on_input = {
        let state = state.clone();

        move |new_value: String| {
            state.on_input_name(new_value);
        }
    };

    let input = if autofocus {
        html! {
            <input
                css={css_input_name()}
                on_input={on_input}
                value={content.as_ref()}
                autofocus=""
            />
        }
    } else {
        html! {
            <input
                css={css_input_name()}
                on_input={on_input}
                value={content.as_ref()}
            />
        }
    };

    html! {
        <div css={css_wrapper()}>
            <div css={css_input_wrapper()}>
                { input }
            </div>
        </div>
    }
}
