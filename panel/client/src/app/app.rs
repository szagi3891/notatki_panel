use vertigo::{VDomComponent, VDomElement, html};
use vertigo::{
    Driver,
    Value,
};
use crate::data::{CurrentContent};
use crate::data::Data;

use super::edit_content::AppEditcontent;
use super::index::AppIndex;
use super::new_dir::AppNewdir;
use super::newcontent::AppNewcontent;
use super::rename_item::AppRenameitem;

enum View {
    Index { state: AppIndex },
    EditContent { state: AppEditcontent },
    RenameItem { state: AppRenameitem },
    NewContent { state: AppNewcontent },
    Mkdir { state: AppNewdir },
}


#[derive(Clone)]
pub struct App {
    pub data: Data,
    view: Value<View>,
}

impl App {
    pub fn new(driver: &Driver) -> App {
        let data = Data::new(driver);

        let view = driver.new_value(View::Index {
            state: AppIndex::new(&data)
        });

        App {
            data,
            view,
        }
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

    pub fn redirect_to_rename_item(&self, base_path: &Vec<String>, select_item: &String) {
        let select_item = select_item.clone();
        let full_path = self.data.tab.full_path.clone().get_value();
        let content_hash = self.data.git.content_hash(&full_path);
        let get_content_string = self.data.git.get_content_string(&full_path);

        match content_hash {
            Some(content_hash) => {
                log::info!("redirect_to_rename_item {base_path:?} {select_item:?}");

                let state = AppRenameitem::new(
                    &self.data,
                    base_path.clone(),
                    select_item,
                    content_hash,
                    get_content_string
                );

                self.view.set_value(View::RenameItem {
                    state
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
        let state = AppNewdir::new(&self.data);

        self.view.set_value(View::Mkdir {
            state
        });
    }

    pub fn redirect_to_index_with_root_refresh(&self) {
        self.data.git.root.refresh();
        self.redirect_to_index();
    }

    pub fn redirect_to_new_content(&self) {
        let state = AppNewcontent::new(&self.data);
        self.view.set_value(View::NewContent { state });
    }

    pub fn current_edit(&self) {
        let full_path = self.data.tab.full_path.get_value();
        self.redirect_to_content(&full_path);
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

    pub fn render(&self) -> VDomComponent {
        let app = VDomComponent::new(self, app_render);
        self.data.tab.open_links.render(app)
    }
}

fn app_render(app: &App) -> VDomElement {
    let view = app.view.get_value();

    match view.as_ref() {
        View::Index { state }=> {
            let view = state.render(app);

            html! {
                <div id="root">
                    { view }
                </div>
            }
        },
        View::EditContent { state } => {
            let view = state.render(app);

            html! {
                <div id="root">
                    { view }
                </div>
            }
        },
        View::NewContent { state } => {
            let view = state.render(app);

            html! {
                <div id="root">
                    { view }
                </div>
            }
        },
        View::RenameItem {state } => {
            let view = state.render(app);

            html! {
                <div id="root">
                    {view}
                </div>
            }
        },
        View::Mkdir { state } => {
            let view = state.render(app.clone());

            html! {
                <div id="root">
                    { view }
                </div>
            }
        }
    }
}
