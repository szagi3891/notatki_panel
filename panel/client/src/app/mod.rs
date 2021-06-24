use vertigo::{
    DomDriver,
    VDomElement,
    computed::{
        Value,
        Computed,
        Dependencies,
    },
};

use crate::state_data::CurrentContent;
use crate::state_data::StateData;

use self::index::ListItem;

mod index;
mod edit_content;
mod new_content;
mod rename_item;

#[derive(PartialEq)]
pub enum View {
    Index {
        state: Computed<index::State>,
    },
    EditContent {
        state: Computed<edit_content::State>,
    },
    NewContent {
        state: Computed<new_content::State>,
    },
    RenameItem {
        state: Computed<rename_item::State>,
    }
}

#[derive(PartialEq)]
pub struct State {
    root: Dependencies,
    driver: DomDriver,
    state_data: StateData,
    current_view: Value<View>,
    self_state: Computed<State>,
}

impl State {
    pub fn new(root: &Dependencies, driver: &DomDriver) -> Computed<State> {
        let state_data = StateData::new(root, driver);

        root.new_state(|self_state: &Computed<State>| -> State {
            let current_view = root.new_value(View::Index {
                state: index::State::new(
                    root,
                    state_data.clone(),
                    self_state.clone(),                 //TODO - jak się odwołamy w tej funkcji do self_state, to wybuchnie
                ),
            });

            State {
                root: root.clone(),
                driver: driver.clone(),
                state_data: state_data.clone(),
                current_view,
                self_state: self_state.clone(),
            }
        })
    }

    fn create_full_path(&self, path: &Vec<String>, select_item: &Option<String>) -> Vec<String> {
        let mut path = path.clone();

        if let Some(select_item) = select_item {
            path.push(select_item.clone());
        }

        path
    }
    
    pub fn redirect_to_content(&self, base_path: &Vec<String>, select_item: &Option<String>) {
        let full_path = self.create_full_path(base_path, select_item);
        let content = self.state_data.get_content_from_path(&full_path);

        match content {
            CurrentContent::File { file_hash, content, ..} => {

                let root = self.root.clone();

                let state = edit_content::State::new(
                    full_path,
                    file_hash,
                    content.as_ref().clone(),
                    &root,
                    &self.driver,
                    self.self_state.clone(),
                );

                self.current_view.set_value(View::EditContent {
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

    pub fn redirect_to_rename_item(&self, base_path: &Vec<String>, select_item: &String) {
        let select_item = select_item.clone();
        let full_path = self.create_full_path(base_path, &Some(select_item.clone()));
        let content = self.state_data.get_content_hash(&full_path);

        match content {
            Some(content_hash) => {
                let state = rename_item::State::new(
                    base_path.clone(),
                    select_item,
                    content_hash,
                    &self.root,
                    &self.driver,
                    self.self_state.clone(),
                );

                self.current_view.set_value(View::RenameItem {
                    state: self.root.new_computed_from(state),
                });
            },
            None => {
                log::error!("redirect_to_rename_item fail - {:?} {:?}", base_path, select_item);
            }
        }
    }

    pub fn redirect_to_index(&self) {
        self.current_view.set_value(View::Index {
            state: index::State::new(
                &self.root,
                self.state_data.clone(),
                self.self_state.clone(),
            ),
        });
    }

    pub fn redirect_to_index_with_path(&self, new_path: Vec<String>, new_item: Option<String>) {
        // self.current_view.set_value(View::Index);
        self.redirect_to_index();
        self.state_data.current_path_dir.set_value(new_path);
        self.state_data.current_path_item.set_value(new_item);
        self.state_data.state_root.refresh();
    }

    pub fn redirect_to_index_with_root_refresh(&self) {
        self.state_data.state_root.refresh();
        // self.current_view.set_value(View::Index);
        self.redirect_to_index();
    }

    pub fn redirect_to_new_content(&self, parent: &Vec<String>, list: Computed<Vec<ListItem>>) {
        let state = new_content::State::new(
            &self.root,
            parent.clone(),
            &self.driver,
            list,
            self.self_state.clone(),
        );

        let state = self.root.new_computed_from(state);

        self.current_view.set_value(View::NewContent { state });
    }
}


pub fn render(state: &Computed<State>) -> VDomElement {

    let state_value = state.get_value();
    let view = state_value.current_view.get_value();

    match view.as_ref() {
        View::Index { state } => {
            index::render(state)
        },
        View::EditContent { state } => {
            edit_content::render(state)
        },
        View::NewContent { state } => {
            new_content::render(state)
        },
        View::RenameItem { state } => {
            rename_item::render(state)
        }
    }
}
