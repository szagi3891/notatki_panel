use vertigo::VDomComponent;
use vertigo::{
    Driver,
    Value,
};
use crate::data::{CurrentContent};
use crate::data::Data;

use super::app_render::app_render;
use super::edit_content::AppEditcontent;
use super::index::AppIndex;

pub enum View {
    Index { state: AppIndex },
    EditContent { state: AppEditcontent },
    RenameItem {
        base_path: Vec<String>,
        prev_name: String,
        prev_hash: String,
        prev_content: Option<String>
    },
    NewContent,
    Mkdir,
}


#[derive(Clone)]
pub struct App {
    pub driver: Driver,
    pub data: Data,
    pub view: Value<View>,
}

impl App {
    pub fn component(driver: &Driver) -> VDomComponent {
        let state_data = Data::new(driver);

        let view = driver.new_value(View::Index {
            state: AppIndex::new(&state_data)
        });

        let state = App {
            driver: driver.clone(),
            data: state_data,
            view,
        };

        let open_links = state.data.tab.open_links.clone();
        
        let app = VDomComponent::new(state, app_render);

        open_links.render(app)
    }

    pub fn redirect_to_content(&self, full_path: &Vec<String>) {
        let full_path = full_path.clone();
        let content = self.data.git.content_from_path(&full_path);

        match content {
            CurrentContent::File { file_hash, content, ..} => {
                log::info!("redirect_to_content {full_path:?}");

                let state = AppEditcontent::new(
                    &self.data,
                    full_path.clone(),
                    file_hash.clone(),
                    content.as_ref().clone(),
                );

                self.view.set_value(View::EditContent {
                    state
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

    //TODO --- do usunięcia ?????

    fn create_full_path(&self, path: &Vec<String>, select_item: &String) -> Vec<String> {
        let mut path = path.clone();

        path.push(select_item.clone());

        path
    }

    pub fn redirect_to_rename_item(&self, base_path: &Vec<String>, select_item: &String) {
        let select_item = select_item.clone();
        let full_path = self.create_full_path(base_path, &select_item);
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
        self.view.set_value(View::Index {
            state: AppIndex::new(&self.data)
        });
    }

    pub fn redirect_to_index_with_path(&self, new_path: Vec<String>, new_item: Option<String>) {
        self.redirect_to_index();

        self.data.tab.redirect_to(new_path, new_item);
        self.data.git.root.refresh();
    }

    pub fn redirect_to_mkdir(&self) {
        self.view.set_value(View::Mkdir);
    }

    pub fn redirect_to_index_with_root_refresh(&self) {
        self.data.git.root.refresh();
        self.redirect_to_index();
    }

    pub fn redirect_to_new_content(&self) {
        self.view.set_value(View::NewContent);
    }

    pub fn current_edit(&self) {
        let full_path = self.data.tab.full_path.get_value();
        self.redirect_to_content(&full_path);
    }

    pub fn create_file(&self) {
        self.redirect_to_new_content();
    }

    pub fn current_rename(&self) {
        let path = self.data.tab.dir_select.get_value();
        let select_item = self.data.tab.current_item.get_value();

        if let Some(select_item) = select_item.as_ref() {
            self.redirect_to_rename_item(&path, select_item);
        } else {
            log::error!("current_rename fail");
        }
    }
}
