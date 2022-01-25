use vertigo::VDomComponent;
use vertigo::{
    Driver,
    VDomElement,
    Value,
    Computed,
};
use vertigo::html;
use std::rc::Rc;
use crate::data::CurrentContent;
use crate::data::StateData;

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
pub struct StateApp {
    pub driver: Driver,
    pub data: StateData,
    view: Value<View>,

    //TODO - kontekst renderowania, idgrafu
}

impl StateApp {
    pub fn component(driver: &Driver) -> VDomComponent {
        let state_data = StateData::new(driver);

        let view = driver.new_value(View::Index);

        let state = StateApp {
            driver: driver.clone(),
            data: state_data,
            view,
        };

        driver.bind_render(state, render)
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
        let content = self.data.git.content_from_path(&full_path);

        match content {
            CurrentContent::File { file_hash, content, ..} => {
                log::info!("redirect_to_content {full_path:?}");
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
        let content_hash = self.data.git.content_hash(&full_path);
        let get_content_string = self.data.git.get_content_string(&full_path);

        match content_hash {
            Some(content_hash) => {
                log::info!("redirect_to_rename_item {base_path:?} {select_item:?}");
                self.view.set_value(View::RenameItem {
                    base_path: base_path.clone(),
                    prev_name: select_item,
                    prev_hash: content_hash,
                    prev_content: get_content_string
                });
            },
            None => {
                log::error!("redirect_to_rename_item fail - {base_path:?} {select_item:?}");
            }
        }
    }

    pub fn redirect_to_index(&self) {
        log::info!("redirect_to_index");
        self.view.set_value(View::Index);
    }

    pub fn redirect_to_index_with_path(&self, new_path: Vec<String>, new_item: Option<String>) {
        self.redirect_to_index();
        self.data.current_path_dir.set_value(new_path);
        self.data.current_path_item.set_value(new_item);
        self.data.git.root.refresh();
    }

    pub fn redirect_to_mkdir(&self, list: Computed<Vec<ListItem>>) {
        let parent = self.data.current_path_dir.clone().get_value();
        self.view.set_value(View::Mkdir { parent, list });
    }

    pub fn redirect_to_index_with_root_refresh(&self) {
        self.data.git.root.refresh();
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

fn render(state_computed: &Computed<StateApp>) -> VDomElement {

    let app_state = state_computed.get_value();
    let view = app_state.view.get_value();

    match view.as_ref() {
        View::Index => {
            let (view, on_keydown) = index::AppIndexState::component(&app_state);

            html! {
                <div id="root" on_key_down={on_keydown}>
                    { view }
                </div>
            }
        },
        View::EditContent { full_path, file_hash, content } => {
            let view = edit_content::StateAppEditContent::component(
                app_state,
                full_path.clone(),
                file_hash.clone(),
                content.as_ref().clone(),
            );

            html! {
                <div id="root">
                    { view }
                </div>
            }
        },
        View::NewContent { parent, list } => {
            let view = new_content::StateAppNewContent::component(
                app_state,
                parent.clone(),
                list.clone(),
            );

            html! {
                <div id="root">
                    { view }
                </div>
            }
        },
        View::RenameItem { base_path, prev_name, prev_hash, prev_content } => {
            let view = rename_item::StateAppRenameItem::component(
                app_state,
                base_path.clone(),
                prev_name.clone(),
                prev_hash.clone(),
                prev_content.clone(),
            );

            html! {
                <div id="root">
                    {view}
                </div>
            }
        },
        View::Mkdir { parent, list } => {
            let view = new_dir::StateAppNewDir::component(app_state, (*parent).to_vec(), list.clone());

            html! {
                <div id="root">
                    { view }
                </div>
            }
        }
    }
}
