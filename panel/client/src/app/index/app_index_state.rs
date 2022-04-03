use std::rc::Rc;
use vertigo::{
    VDomComponent,
    Resource,
};
use crate::app::App;
use crate::data::Data;

use super::alert::AppIndexAlert;
use super::app_index_render::app_index_render;

#[derive(Clone)]
pub struct AppIndex {
    pub data: Data,
    pub app: App,
    pub alert: AppIndexAlert,
}

impl AppIndex {
    pub fn component(app: &App) -> VDomComponent {
        let state_data = app.data.clone();

        let (alert, alert_view) = AppIndexAlert::new(app.clone());

        let state = AppIndex {
            data: state_data,
            app: app.clone(),
            alert,
        };

        app_index_render(alert_view, state)
    }

    pub fn click_list_item(&self, node: String) {
        let list_hash_map_rc = self.data.tab.list_hash_map.get_value();

        if let Resource::Ready(list) = list_hash_map_rc.as_ref() {
            if let Some(node_details) = list.get(&node) {
                if node_details.dir {
                    let mut current = self.data.tab.dir.get_value().as_ref().clone();
                    current.push(node.clone());
                    self.data.tab.set_path(current);
                } else {
                    self.data.tab.file.set_value(Some(node.clone()));
                }
                return;
            }
        }

        log::error!("push_path - ignore: {}", node);
    }

    fn find(&self, item_finding: &String) -> Option<isize> {
        let list = self.data.tab.list.get_value();

        for (index, item) in list.as_ref().iter().enumerate() {
            if item.name == *item_finding {
                return Some(index as isize);
            }
        }

        None
    }

    fn try_set_pointer_to(&self, index: isize) -> bool {
        if index < 0 {
            return false;
        }

        let index = index as usize;

        let list = self.data.tab.list.get_value();

        if let Some(first) = list.get(index) {
            self.data.tab.file.set_value(Some(first.name.clone()));
            return true;
        }

        false
    }

    fn try_set_pointer_to_end(&self) {
        let len = self.data.tab.list.get_value().len() as isize;
        self.try_set_pointer_to(len - 1);
    }

    fn pointer_up(&self) {
        let list_pointer_rc = self.data.tab.current_item.get_value();

        if let Some(list_pointer) = list_pointer_rc.as_ref() {
            if let Some(index) = self.find(list_pointer) {
                if !self.try_set_pointer_to(index - 1) {
                    self.try_set_pointer_to_end();
                }
            }
        } else {
            self.try_set_pointer_to(0);
        }
    }

    fn pointer_down(&self) {
        let list_pointer_rc = self.data.tab.current_item.get_value();

        if let Some(list_pointer) = list_pointer_rc.as_ref() {
            if let Some(index) = self.find(list_pointer) {
                if !self.try_set_pointer_to(index + 1) {
                    self.try_set_pointer_to(0);
                }
            }
        } else {
            self.try_set_pointer_to(0);
        }
    }

    fn pointer_enter(&self) {
        let list_pointer = self.data.tab.current_item.get_value();

        if let Some(list_pointer) = list_pointer.as_ref() {
            if self.find(list_pointer).is_some() {
                self.click_list_item(list_pointer.clone());
            }
        }
    }

    fn backspace(&self) {
        let current_path = self.data.tab.dir.get_value();
        let mut current_path = current_path.as_ref().clone();

        current_path.pop();

        self.data.tab.set_path(current_path);
    }

    pub fn keydown(&self, code: String) -> bool {
        if self.alert.is_visible() {
            if code == "Escape" {
                self.alert.close_modal();
                return true;
            }

            //TODO - dodać wskaźnik i nawigację klawiaturą po elemencie z listy wyników

            return false;
        }

        if code == "ArrowUp" {
            self.pointer_up();
            return true;
        } else if code == "ArrowDown" {
            self.pointer_down();
            return true;
        } else if code == "Escape" {
            self.data.tab.file.set_value(None);
            return true;
        } else if code == "ArrowRight" || code == "Enter" {
            self.pointer_enter();
            return true;
        } else if code == "ArrowLeft" || code == "Backspace" || code == "Escape" {
            self.backspace();
            return true;
        }

        log::info!("klawisz ... {:?} ", code);
        false
    }

    pub fn current_edit(&self) {
        let path = self.data.tab.dir.get_value();
        let select_item = self.data.tab.current_item.get_value();
        self.app.redirect_to_content(&path, &select_item);
    }

    pub fn create_file(&self) {
        let path = self.data.tab.dir.get_value();
        let list = self.data.tab.list.clone();

        self.app.redirect_to_new_content(path.as_ref(), list);
    }

    pub fn redirect_to_mkdir(&self) {
        self.app.redirect_to_mkdir(self.data.tab.list.clone());
    }

    pub fn current_rename(&self) {
        let path = self.data.tab.dir.get_value();
        let select_item = self.data.tab.current_item.get_value();

        if let Some(select_item) = select_item.as_ref() {
            self.app.redirect_to_rename_item(&path, select_item);
        } else {
            log::error!("current_rename fail");
        }
    }

    pub fn current_path_dir(&self) -> Rc<Vec<String>> {
        self.data.tab.dir.get_value()
    }
}
