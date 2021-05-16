use vertigo::{
    DomDriver,
    computed::{
        Computed,
        Dependencies,
        Value
    }
};
use crate::request::{Request};
use super::{StateNodeDir, StateRoot, StateViewIndex, state_node_content::StateNodeContent};

#[derive(PartialEq)]
pub enum View {
    Index,
    // EditContent {
    //     path: Vec<String>,          //edutowany element
    //     hash: String,               //hash poprzedniej zawartosci
    //     content: String,            //edytowana tresc
    // }

    //zmiana nazwy
    //tworzenie pliku
    //tworzenie katalogu
}

#[derive(PartialEq)]
pub struct State {
    pub state_view_index: Computed<StateViewIndex>,
    pub current_view: Value<View>,
}

impl State {
    pub fn new(root: &Dependencies, driver: &DomDriver) -> Computed<State> {
        let request = Request::new(driver);

        let state_node_dir = StateNodeDir::new(&request, root);
        let state_node_content = StateNodeContent::new(&request, root);
        let state_root = StateRoot::new(&request, root, state_node_dir.clone());

        let state_view_index = StateViewIndex::new(
            root,
            &state_node_dir,
            &state_node_content,
            &state_root,
        );

        let current_view = root.new_value(View::Index);

        root.new_computed_from(State {
            state_view_index,
            current_view,
        })
    }
}
