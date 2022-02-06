use std::rc::Rc;
use vertigo::{
    VDomComponent,
    Resource,
};
use crate::app::App;
use crate::data::StateData;

use super::alert::AppIndexAlert;
use super::app_index_render::app_index_render;

#[derive(Clone)]
pub struct AppIndex {
    pub data: StateData,
    pub app_state: App,
    pub alert: AppIndexAlert,
}

impl AppIndex {
    pub fn component(app_state: &App) -> (VDomComponent, impl Fn(vertigo::KeyDownEvent) -> bool) {
        let state_data = app_state.data.clone();

        let (alert, alert_view) = AppIndexAlert::new(app_state.clone());

        let state = AppIndex {
            data: state_data,
            app_state: app_state.clone(),
            alert,
        };

        let keydown = {
            let state = state.clone();
            move |event: vertigo::KeyDownEvent| -> bool {
                state.keydown(event.code)
            }
        };

        let view = app_index_render(alert_view, state);
        (view, keydown)
    }


    pub fn set_path(&self, path: Vec<String>) {
        let current_path = self.data.tab.dir.get_value();

        if current_path.as_ref().as_slice() == path.as_slice() {
            log::info!("path are equal");
            return;
        }
    
        let (new_current_path, new_current_item_value) = calculate_next_path(current_path.as_ref(), path);

        self.app_state.driver.transaction(||{
            self.data.tab.dir.set_value(new_current_path);
            self.data.tab.file.set_value(new_current_item_value);
        });
    }

    pub fn click_list_item(&self, node: String) {
        let list_hash_map_rc = self.data.tab.list_hash_map.get_value();

        if let Resource::Ready(list) = list_hash_map_rc.as_ref() {
            if let Some(node_details) = list.get(&node) {
                if node_details.dir {
                    let mut current = self.data.tab.dir.get_value().as_ref().clone();
                    current.push(node.clone());
                    self.set_path(current);
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

        self.set_path(current_path);
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
        self.app_state.redirect_to_content(&path, &select_item);
    }

    pub fn create_file(&self) {
        let path = self.data.tab.dir.get_value();
        let list = self.data.tab.list.clone();

        self.app_state.redirect_to_new_content(path.as_ref(), list);
    }

    pub fn redirect_to_mkdir(&self) {
        self.app_state.redirect_to_mkdir(self.data.tab.list.clone());
    }

    pub fn current_rename(&self) {
        let path = self.data.tab.dir.get_value();
        let select_item = self.data.tab.current_item.get_value();

        if let Some(select_item) = select_item.as_ref() {
            self.app_state.redirect_to_rename_item(&path, select_item);
        } else {
            log::error!("current_rename fail");
        }
    }

    pub fn current_path_dir(&self) -> Rc<Vec<String>> {
        self.data.tab.dir.get_value()
    }
}


fn calculate_next_path(prev_path: &[String], new_path: Vec<String>) -> (Vec<String>, Option<String>) {
    if new_path.len() > prev_path.len() {
        return (new_path, None);
    }

    if prev_path[0..new_path.len()] == new_path[0..] {
        let last = prev_path.get(new_path.len());
        let last = last.cloned();
        return (new_path, last);
    }

    (new_path, None)
}

#[cfg(test)]
fn create_vector<const N: usize>(list: [&str; N]) -> Vec<String> {
    let mut out = Vec::new();

    for item in list.iter() {
        out.push(String::from(*item));
    }

    out
}

#[test]
fn test_set_path() {
    assert_eq!(
        calculate_next_path(&create_vector(["cc1"]), create_vector([])),
        (create_vector([]), Some(String::from("cc1")))
    );

    assert_eq!(
        calculate_next_path(&create_vector(["aa1", "aa2", "aa3"]), create_vector([])),
        (create_vector([]), Some("aa1".into()))
    );

    assert_eq!(
        calculate_next_path(&create_vector(["aa1", "aa2", "aa3"]), create_vector(["aa1"])),
        (create_vector(["aa1"]), Some("aa2".into()))
    );

    assert_eq!(
        calculate_next_path(&create_vector(["aa1", "aa2", "aa3"]), create_vector(["aa1", "aa2"])),
        (create_vector(["aa1", "aa2"]), Some("aa3".into()))
    );

    assert_eq!(
        calculate_next_path(&create_vector(["aa1", "aa2", "aa3"]), create_vector(["aa1", "aa2", "aa3"])),
        (create_vector(["aa1", "aa2", "aa3"]), None)
    );

    assert_eq!(
        calculate_next_path(&create_vector(["aa1", "aa2", "aa3"]), create_vector(["aa1", "aa2", "aa3", "aa4"])),
        (create_vector(["aa1", "aa2", "aa3", "aa4"]), None)
    );

    assert_eq!(
        calculate_next_path(&create_vector(["aa1", "aa2", "aa3"]), create_vector([])),
        (create_vector([]), Some("aa1".into()))
    );

    assert_eq!(
        calculate_next_path(&create_vector(["aa1", "aa2", "aa3"]), create_vector(["bb1"])),
        (create_vector(["bb1"]), None)
    );

    assert_eq!(
        calculate_next_path(&create_vector(["aa1", "aa2", "aa3"]), create_vector(["bb1", "bb2"])),
        (create_vector(["bb1", "bb2"]), None)
    );

    assert_eq!(
        calculate_next_path(&create_vector(["aa1", "aa2", "aa3"]), create_vector(["bb1", "bb2", "bb3"])),
        (create_vector(["bb1", "bb2", "bb3"]), None)
    );

    assert_eq!(
        calculate_next_path(&create_vector(["aa1", "aa2", "aa3"]), create_vector(["bb1", "bb2", "bb3", "bb4"])),
        (create_vector(["bb1", "bb2", "bb3", "bb4"]), None)
    );
}