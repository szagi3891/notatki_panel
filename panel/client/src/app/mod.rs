use vertigo::{
    DomDriver,
    VDomElement,
    computed::{
        Value,
        Computed,
        Dependencies,
    },
    Callback,
};

use crate::state_data::CurrentContent;
use crate::state_data::StateData;

use self::index::ListItem;

mod index;
mod edit_content;
mod new_content;

#[derive(PartialEq)]
pub enum View {
    Index,
    EditContent {
        state: Computed<edit_content::State>,
    },
    NewContent {
        state: Computed<new_content::State>,
    }
    //TODO - zmiana nazwy
}

#[derive(Clone, PartialEq)]
pub struct CallbackBuilder {
    root: Dependencies,
    driver: DomDriver,
    state_data: StateData,
    current_path_dir: Value<Vec<String>>,
    current_path_item: Value<Option<String>>,
    current_view: Value<View>,
}

impl CallbackBuilder {
    pub fn new(
        root: &Dependencies,
        driver: &DomDriver,
        state_data: &StateData,
        current_path_dir: &Value<Vec<String>>,
        current_path_item: &Value<Option<String>>,
        current_view: &Value<View>
    ) -> CallbackBuilder {
        CallbackBuilder {
            root: root.clone(),
            driver: driver.clone(),
            state_data: state_data.clone(),
            current_path_dir: current_path_dir.clone(),
            current_path_item: current_path_item.clone(),
            current_view: current_view.clone(),
        }
    }

    pub fn redirect_to_content(&self, path: Vec<String>) {
        let content = self.state_data.get_content_from_path(&path);

        match content {
            CurrentContent::File { file_hash, content, ..} => {

                let root = self.root.clone();

                let state = edit_content::State::new(
                    path,
                    file_hash,
                    content.as_ref().clone(),
                    &root,
                    &self.driver,
                    self.clone(),
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

    pub fn redirect_to_index(&self) {
        self.current_view.set_value(View::Index);
    }

    pub fn redirect_to_index_with_path(&self, new_path: Vec<String>, new_item: Option<String>) {
        self.current_view.set_value(View::Index);
        self.current_path_dir.set_value(new_path);
        self.current_path_item.set_value(new_item);
        self.state_data.state_root.refresh();
    }

    pub fn redirect_to_index_with_root_refresh(&self) -> Callback<()> {
        let current_view = self.current_view.clone();
        let state_data = self.state_data.clone();

        Callback::new(move |_| {
            state_data.state_root.refresh();
            current_view.set_value(View::Index);
        })
    }

    pub fn redirect_to_new_content(&self, parent: Vec<String>, list: Computed<Vec<ListItem>>) {
        let state = new_content::State::new(
            &self.root,
            parent,
            &self.driver,
            list,
            self.clone(),
        );

        let state = self.root.new_computed_from(state);

        self.current_view.set_value(View::NewContent { state });
    }
}

#[derive(PartialEq)]
pub struct State {
    state_view_index: Computed<index::State>,
    current_view: Computed<View>,
}

impl State {
    pub fn new(root: &Dependencies, driver: &DomDriver) -> Computed<State> {
        let state_data = StateData::new(root, driver);

        let current_view = root.new_value(View::Index);
        let current_view_computed = current_view.to_computed();

        let current_path_dir: Value<Vec<String>> = root.new_value(Vec::new());
        let current_path_item: Value<Option<String>> = root.new_value(None);

        let callback = CallbackBuilder::new(root, driver, &state_data, &current_path_dir, &current_path_item, &current_view);

        root.new_computed_from(State {
            state_view_index: index::State::new(
                root,
                state_data.clone(),
                current_path_dir,
                current_path_item,
                callback.clone(),
            ),
            current_view: current_view_computed,
        })
    }
}


pub fn render(state: &Computed<State>) -> VDomElement {

    let state_value = state.get_value();
    let view = state_value.current_view.get_value();

    match view.as_ref() {
        View::Index => {
            index::render(&state_value.state_view_index)
        },
        View::EditContent { state } => {
            edit_content::render(state)
        },
        View::NewContent { state } => {
            new_content::render(state)
        }
    }
}
