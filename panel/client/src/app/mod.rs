use vertigo::{
    DomDriver,
    VDomElement,
    computed::{
        Value,
        Computed,
        Dependencies,
    },
};
use std::rc::Rc;
use crate::state_data::CurrentContent;
use crate::state_data::StateData;

use self::index::ListItem;

mod index;
mod edit_content;
mod new_content;
mod rename_item;

#[derive(PartialEq)]
enum View {
    Index,
    EditContent {
        full_path: Vec<String>,
        file_hash: String,
        content: Rc<String>
    },
    RenameItem {
        base_path: Vec<String>,
        prev_name: String,
        prev_hash: String,
        prev_content: Option<String>
    },
    NewContent {
        parent: Vec<String>,
        list: Computed<Vec<ListItem>>,
    }
}

#[derive(PartialEq, Clone)]
pub struct StateView {
    root: Dependencies,
    driver: DomDriver,
    state_data: StateData,
    view: Value<View>,
}

impl StateView {
    fn new(root: &Dependencies, driver: &DomDriver) -> (Computed<View>, StateView) {
        let state_data = StateData::new(root, driver);

        let view = root.new_value(View::Index);

        (
            view.to_computed(),
            StateView {
                root: root.clone(),
                driver: driver.clone(),
                state_data: state_data.clone(),
                view,
            }
        )
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
                self.view.set_value(View::EditContent {
                    full_path,
                    file_hash,
                    content
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
        let content_hash = self.state_data.get_content_hash(&full_path);
        let get_content_string = self.state_data.get_content_string(&full_path);

        match content_hash {
            Some(content_hash) => {
                self.view.set_value(View::RenameItem {
                    base_path: base_path.clone(),
                    prev_name: select_item,
                    prev_hash: content_hash,
                    prev_content: get_content_string
                });
            },
            None => {
                log::error!("redirect_to_rename_item fail - {:?} {:?}", base_path, select_item);
            }
        }
    }

    pub fn redirect_to_index(&self) {
        self.view.set_value(View::Index);
    }

    pub fn redirect_to_index_with_path(&self, new_path: Vec<String>, new_item: Option<String>) {
        self.redirect_to_index();
        self.state_data.current_path_dir.set_value(new_path);
        self.state_data.current_path_item.set_value(new_item);
        self.state_data.state_root.refresh();
    }

    pub fn redirect_to_index_with_root_refresh(&self) {
        self.state_data.state_root.refresh();
        self.redirect_to_index();
    }

    pub fn redirect_to_new_content(&self, parent: &Vec<String>, list: Computed<Vec<ListItem>>) {
        self.view.set_value(View::NewContent {
            parent: parent.clone(),
            list
        });
    }
}


#[derive(PartialEq)]
pub struct State {
    view: Computed<View>,
    state_view: StateView,
}

impl State {
    pub fn new(root: &Dependencies, driver: &DomDriver) -> Computed<State> {
        let (view, state_view) = StateView::new(root, driver);

        root.new_computed_from(State {
            view,
            state_view,
        })
    }
}

pub fn render(state: &Computed<State>) -> VDomElement {

    let state_value = state.get_value();
    let view = state_value.view.get_value();

    match view.as_ref() {
        View::Index => {
            let state = index::State::new(
                &state_value.state_view.root,
                state_value.state_view.state_data.clone(),
                state_value.state_view.clone(),
            );

            index::render(&state)

            // html! {
            //     <div>
            //         "index ..."
            //         <component {index::render} data={state} />
            //     </div>
            // }
        },
        View::EditContent { full_path, file_hash, content } => {
            let state = edit_content::State::new(
                full_path.clone(),
                file_hash.clone(),
                content.as_ref().clone(),
                &state_value.state_view.root,
                &state_value.state_view.driver,
                state_value.state_view.clone(),
            );

            edit_content::render(&state)
        },
        View::NewContent { parent, list } => {
            let state = new_content::State::new(
                &state_value.state_view.root,
                parent.clone(),
                &state_value.state_view.driver,
                list.clone(),
                state_value.state_view.clone(),
            );

            new_content::render(&state)
        },
        View::RenameItem { base_path, prev_name, prev_hash, prev_content } => {
            let state = rename_item::State::new(
                base_path.clone(),
                prev_name.clone(),
                prev_hash.clone(),
                prev_content.clone(),
                &state_value.state_view.root,
                &state_value.state_view.driver,
                state_value.state_view.clone(),
            );

            rename_item::render(&state)
        }
    }
}
