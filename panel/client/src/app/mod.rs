use vertigo::{
    Driver,
    VDomElement,
    computed::{
        Value,
        Computed,
        Dependencies,
    },
};
use vertigo_html::html;
use std::rc::Rc;
use crate::state_data::CurrentContent;
use crate::state_data::DataState;

use self::index::ListItem;

pub mod index;
mod edit_content;
mod new_content;
mod new_dir;
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
    },
    Mkdir {
        parent: Rc<Vec<String>>,
        list: Computed<Vec<ListItem>>,
    }
}

#[derive(PartialEq, Clone)]
pub struct AppState {
    pub root: Dependencies,
    driver: Driver,
    pub data_state: DataState,
    view: Value<View>,
}

impl AppState {
    pub fn new(root: &Dependencies, driver: &Driver) -> Computed<AppState> {
        let state_data = DataState::new(root, driver);

        let view = root.new_value(View::Index);

        root.new_computed_from(AppState {
            root: root.clone(),
            driver: driver.clone(),
            data_state: state_data.clone(),
            view,
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
        let content = self.data_state.get_content_from_path(&full_path);

        match content {
            CurrentContent::File { file_hash, content, ..} => {
                log::info!("redirect_to_content {:?}", full_path);
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
        let content_hash = self.data_state.get_content_hash(&full_path);
        let get_content_string = self.data_state.get_content_string(&full_path);

        match content_hash {
            Some(content_hash) => {
                log::info!("redirect_to_rename_item {:?} {:?}", base_path, select_item);
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
        log::info!("redirect_to_index");
        self.view.set_value(View::Index);
    }

    pub fn redirect_to_index_with_path(&self, new_path: Vec<String>, new_item: Option<String>) {
        self.redirect_to_index();
        self.data_state.current_path_dir.set_value(new_path);
        self.data_state.current_path_item.set_value(new_item);
        self.data_state.state_root.refresh();
    }

    pub fn redirect_to_mkdir(&self, list: Computed<Vec<ListItem>>) {
        let parent = self.data_state.current_path_dir.clone().get_value();
        self.view.set_value(View::Mkdir { parent, list });
    }

    pub fn redirect_to_index_with_root_refresh(&self) {
        self.data_state.state_root.refresh();
        self.redirect_to_index();
    }

    pub fn redirect_to_new_content(&self, parent: &Vec<String>, list: Computed<Vec<ListItem>>) {
        log::info!("redirect_to_new_content {:?}", parent);
        self.view.set_value(View::NewContent {
            parent: parent.clone(),
            list
        });
    }
}

pub fn render(state_computed: &Computed<AppState>) -> VDomElement {

    let app_state = state_computed.get_value();
    let view = app_state.view.get_value();

    match view.as_ref() {
        View::Index => {
            let (state, on_keydown) = index::AppIndexState::new(
                app_state.clone(),
            );

            html! {
                <div id="root" on_key_down={on_keydown}>
                    <component {index::render} data={state} />
                </div>
            }
        },
        View::EditContent { full_path, file_hash, content } => {
            let state = edit_content::State::new(
                app_state.clone(),
                full_path.clone(),
                file_hash.clone(),
                content.as_ref().clone(),
            );

            html! {
                <div id="root">
                    <component {edit_content::render} data={state} />
                </div>
            }
        },
        View::NewContent { parent, list } => {
            let state = new_content::State::new(
                app_state.clone(),
                parent.clone(),
                list.clone(),
            );

            html! {
                <div id="root">
                    <component {new_content::render} data={state} />
                </div>
            }
        },
        View::RenameItem { base_path, prev_name, prev_hash, prev_content } => {
            let state = rename_item::State::new(
                app_state.clone(),
                base_path.clone(),
                prev_name.clone(),
                prev_hash.clone(),
                prev_content.clone(),
            );

            html! {
                <div id="root">
                    <component {rename_item::render} data={state} />
                </div>
            }
        },
        View::Mkdir { parent, list } => {
            let state = new_dir::State::new(app_state.clone(), (*parent).to_vec(), list.clone());

            html! {
                <div id="root">
                    <component {new_dir::render} data={state} />
                </div>
            }
        }
    }
}
