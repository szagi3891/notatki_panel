use vertigo::{
    DomDriver,
    computed::{
        Computed,
        Dependencies,
        Value
    }
};

use super::{StateData, StateViewIndex};

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
        let state_data = StateData::new(root, driver);

        let state_view_index = StateViewIndex::new(
            root,
            state_data
        );

        let current_view = root.new_value(View::Index);

        root.new_computed_from(State {
            state_view_index,
            current_view,
        })
    }
}
