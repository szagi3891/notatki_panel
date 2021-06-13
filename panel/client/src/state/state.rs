use vertigo::{
    DomDriver,
    computed::{
        Computed,
        Dependencies,
    },
    utils::Action
};

use crate::state_data::CurrentContent;
use super::{StateViewEditContent, StateViewIndex, StateViewNewContent};
use crate::state_data::StateData;

#[derive(PartialEq)]
pub enum View {
    Index,
    EditContent {
        state: Computed<StateViewEditContent>,
    },
    NewContent {
        state: Computed<StateViewNewContent>,
    }
    //TODO - zmiana nazwy
}


//TODO - jakas forma event emmitera jest potrzebna

#[derive(PartialEq)]
pub enum StateAction {
    RedirectToIndex,
    RedirectToIndexWithRootRefresh,
    RedirectToContent {
        path: Vec<String>,
    },
    RedirectToNewContent {
        parent: Vec<String>,
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
            let driver = driver.clone();
            let state_data = state_data.clone();

            subscribe.subscribe(move |message| {
                match message {
                    StateAction::RedirectToIndex => {
                        current_view.set_value(View::Index);
                    },
                    StateAction::RedirectToIndexWithRootRefresh => {
                        state_data.state_root.refresh();
                        current_view.set_value(View::Index);
                    },
                    StateAction::RedirectToContent { path } => {
                        let content = state_data.get_content_from_path(&path);

                        match content {
                            CurrentContent::File { file_hash, content, ..} => {

                                let state = StateViewEditContent::new(
                                    path,
                                    file_hash,
                                    content.as_ref().clone(),
                                    &action,
                                    &root,
                                    &driver
                                );

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
                    },
                    StateAction::RedirectToNewContent { parent } => {
                        let new_content = StateViewNewContent::new(
                            &root,
                            parent,
                            &action,
                        );
                        current_view.set_value(View::NewContent{
                            state: root.new_computed_from(new_content)
                        });
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
