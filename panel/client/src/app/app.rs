use vertigo::{VDomComponent, VDomElement, html};
use vertigo::Value;
use crate::data::ContentView;
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
    pub fn new() -> App {
        let data = Data::new();

        let view = Value::new(View::Index {
            state: AppIndex::new(&data)
        });

        App {
            data,
            view,
        }
    }

    pub fn redirect_to_content(&self, full_path: &Vec<String>) {
        let full_path = full_path.clone();
        let content = self.data.git.get_content(&full_path);

        match content {
            Some(ContentView { id, content }) => {
                log::info!("redirect_to_content {full_path:?}");

                let state = AppEditcontent::new(
                    full_path.clone(),
                    id.clone(),
                    content.as_ref().clone(),
                );

                self.view.set_value(View::EditContent {
                    state
                });
            },
            None => {
                log::error!("Oczekiwano pliku, problem z pobraniem");
            },
        }
    }

    pub fn redirect_to_rename_item(&self, base_path: &Vec<String>, select_item: &String) {
        let select_item = select_item.clone();
        let full_path = self.data.tab.full_path.clone().get_value();
        let content = self.data.git.get_content(&full_path);

        match content {
            Some(ContentView { id, content }) => {
                log::info!("redirect_to_rename_item {base_path:?} {select_item:?}");

                let state = AppRenameitem::new(
                    base_path.clone(),
                    select_item,
                    id,
                    Some(content.as_ref().clone())
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
