use vertigo::utils::Action;

use super::state::StateAction;

#[derive(PartialEq)]
pub struct StateViewEditContent {
    pub path: Vec<String>,          //edutowany element
    pub hash: String,               //hash poprzedniej zawartosci
    pub content: String,            //edytowana tresc

    pub action: Action<StateAction>,
}

impl StateViewEditContent {
    pub fn redirect_to_index(&self) {
        self.action.trigger(StateAction::RedirectToIndex);
    }
}

