use vertigo::VDomComponent;
use vertigo::{
    Driver,
    Value,
    Computed,
};
use std::rc::Rc;
use crate::data::{CurrentContent, ListItem};
use crate::data::Data;

use super::app_render::app_render;

#[derive(PartialEq)]
pub enum View {
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


#[derive(Clone)]
pub struct App {
    pub driver: Driver,
    pub data: Data,
    pub view: Value<View>,

    //TODO - kontekst renderowania, idgrafu
}

impl App {
    pub fn component(driver: &Driver) -> VDomComponent {
        let state_data = Data::new(driver);

        let view = driver.new_value(View::Index);

        let state = App {
            driver: driver.clone(),
            data: state_data,
            view,
        };

        let open_links = state.data.tab.open_links.clone();
        
        let app = VDomComponent::new(state, app_render);

        open_links.render(app)
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

        self.data.tab.redirect_to(new_path, new_item);
        self.data.git.root.refresh();
    }

    pub fn redirect_to_mkdir(&self, list: Computed<Vec<ListItem>>) {
        let parent = self.data.tab.dir.clone().get_value();
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
