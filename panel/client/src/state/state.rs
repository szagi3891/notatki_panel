use vertigo::{
    DomDriver,
    computed::{
        Computed,
        Dependencies,
    },
    utils::Action
};

use super::{StateData, StateViewEditContent, StateViewIndex, state_data::CurrentContent};

#[derive(PartialEq)]
pub enum View {
    Index,
    EditContent {
        state: Computed<StateViewEditContent>,
    }

    //zmiana nazwy
    //tworzenie pliku
    //tworzenie katalogu
}


//TODO - jakas forma event emmitera jest potrzebna

#[derive(PartialEq)]
pub enum StateAction {
    RedirectToIndex,
    RedirectToContent {
        path: Vec<String>,
    }
}



#[derive(PartialEq)]
pub struct State {
    pub state_view_index: Computed<StateViewIndex>,
    pub current_view: Computed<View>,
}

impl State {
    pub fn new(root: &Dependencies, driver: &DomDriver) -> Computed<State> {
        let state_data = StateData::new(root, driver);

        let (action, subscribe) = Action::<StateAction>::new();
        
        let state_view_index = StateViewIndex::new(
            root,
            state_data.clone(),
            action.clone(),
        );

        let current_view = root.new_value(View::Index);
        let current_view_computed = current_view.to_computed();

        {
            let root = root.clone();
            let action = action.clone();

            subscribe.subscribe(move |message| {
                match message {
                    StateAction::RedirectToIndex => {
                        current_view.set_value(View::Index);
                    },
                    StateAction::RedirectToContent { path } => {
                        let content = state_data.get_content_from_path(&path);

                        match content {
                            CurrentContent::File { file_hash, content, ..} => {

                                let state = StateViewEditContent {
                                    path,
                                    hash: file_hash,
                                    content: content.as_ref().clone(),
                                    action: action.clone(),
                                };

                                current_view.set_value(View::EditContent {
                                    state: root.new_computed_from(state)
                                });
                            },
                            CurrentContent::Dir { .. } => {
                                log::error!("Oczekiwano pliku, znaleziono katalog");
                            },
                            CurrentContent::None => {
                                log::error!("Oczekiwano pliku, nic nie znaleziono");
                            }
                        }
                    }
                }
            });
        }

        root.new_computed_from(State {
            state_view_index,
            current_view: current_view_computed,
        })
    }
}