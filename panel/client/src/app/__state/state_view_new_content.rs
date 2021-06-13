use vertigo::{
    // DomDriver,
    computed::{
        Computed,
        Dependencies,
        Value,
    },
    utils::Action
};

use super::state::StateAction;


#[derive(PartialEq)]
pub struct StateViewNewContent {
    parent: Vec<String>,
    name: Value<String>,
    content: Value<String>,

    pub action: Action<StateAction>,
}


impl StateViewNewContent {
    pub fn new(root: &Dependencies, parent: Vec<String>, action: &Action<StateAction>) -> StateViewNewContent {
        let name = root.new_value("".into());
        let content = root.new_value("".into());
        StateViewNewContent {
            parent,
            name,
            content,
            action: action.clone(),
        }
    }
}

/*

POST http://0.0.0.0:4000/create_file
Content-Type: application/json

{
    "path": ["spiski", "nazisci argentyna"],
    "new_path": ["Nowa szwabia"],
    "new_content": "a"
}

*/