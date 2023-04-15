use vertigo::{Computed, Value, dom, DomNode, dom_element, Resource};

use vertigo::{Css, css};

use crate::data::ListItem;

fn is_exist_in_list(name: String, list: Vec<ListItem>) -> bool {
    for item in list.into_iter() {
        if *item.name() == name {
            return true;
        }
    }
    false
}

#[derive(Clone, PartialEq)]
pub struct NewName {
    pub name: Value<String>,
    pub is_valid: Computed<bool>,
}

impl NewName {
    pub fn new(select_dir: Computed<ListItem>) -> NewName {
        let name: Value<String> = Value::new("".to_string());

        let name_exists = {
            let name = name.clone();

            Computed::from(move |context| -> bool {
                let list = select_dir.get(context).list.get(context);

                if let Resource::Ready(list) = list {
                    let name = name.get(context);
                    return is_exist_in_list(name, list);
                }

                false
            })
        };

        let is_valid = {
            let name = name.clone();

            Computed::from(move |context| -> bool {
                let name_exists = name_exists.get(context);

                if name_exists {
                    return false;
                }

                if name.get(context).is_empty() {
                    return false;
                }

                true
            })
        };

        NewName {
            name,
            is_valid,
        }
    }

    pub fn render(&self, autofocus: bool) -> DomNode {
        render(self, autofocus)
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

pub fn render(state: &NewName, autofocus: bool) -> DomNode {
    let content = state.name.to_computed();

    let on_input = {
        let state = state.clone();

        move |new_value: String| {
            state.on_input_name(new_value);
        }
    };

    let input = dom_element! {
        <input
            css={css_input_name()}
            on_input={on_input}
            value={content}
            autofocus=""
        />
    };

    let input = if autofocus {
        input.attr("autofocus", "")
    } else {
        input
    };

    dom! {
        <div css={css_wrapper()}>
            <div css={css_input_wrapper()}>
                { input }
            </div>
        </div>
    }
}
