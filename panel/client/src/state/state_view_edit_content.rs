use vertigo::{computed::{Computed, Dependencies, Value}, utils::Action};

use super::state::StateAction;

#[derive(PartialEq)]
pub struct StateViewEditContent {
    pub path: Vec<String>,          //edutowany element
    pub hash: String,               //hash poprzedniej zawartosci
    pub action: Action<StateAction>,

    pub edit_content: Value<String>,
    pub save_enable: Computed<bool>,
}

impl StateViewEditContent {
    pub fn redirect_to_index(&self) {
        self.action.trigger(StateAction::RedirectToIndex);
    }

    pub fn new(path: Vec<String>, hash: String, content: String, action: Action<StateAction>, deep: &Dependencies) -> StateViewEditContent {
        let edit_content = deep.new_value(content.clone());

        let save_enable = {
            let edit_content = edit_content.to_computed();

            deep.from(move || -> bool {
                let edit_content = edit_content.get_value();
                let save_enabled = edit_content.as_ref() != &content;
                save_enabled
            })
        };

        StateViewEditContent {
            path,
            hash,
            action,

            edit_content,
            save_enable,
        }
    }

    pub fn on_input(&self, new_text: String) {
        self.edit_content.set_value(new_text);
    }
}

