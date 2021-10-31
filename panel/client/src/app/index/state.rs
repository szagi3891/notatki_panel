use std::{cmp::Ordering, collections::HashMap};
use std::rc::Rc;
use vertigo::{
    computed::{
        Computed,
        Dependencies,
        Value
    },
};
use crate::app::AppState;
use crate::request::{ResourceError};
use crate::state_data::{CurrentContent, TreeItem};
use crate::state_data::DataState;

use super::alert::AlertState;

#[derive(PartialEq, Debug, Clone)]
pub struct ListItem {
    pub name: String,
    pub dir: bool,
    pub prirority: u8,
}


fn create_list_hash_map(root: &Dependencies, state_data: &DataState, current_path: &Value<Vec<String>>) -> Computed<Result<Rc<HashMap<String, TreeItem>>, ResourceError>> {
    let state_data = state_data.clone();
    let current_path = current_path.to_computed();

    root.from(move || -> Result<Rc<HashMap<String, TreeItem>>, ResourceError> {
        let current_path_rc = current_path.get_value();
        let current_path = current_path_rc.as_ref();

        state_data.get_dir_content(current_path)
    })
}

fn get_list_item_prirority(name: &String) -> u8 {
    if name.get(0..2) == Some("__") {
        return 0
    }

    if name.get(0..1) == Some("_") {
        return 2
    }

    1
}

fn create_list(root: &Dependencies, list: &Computed<Result<Rc<HashMap<String, TreeItem>>, ResourceError>>) -> Computed<Vec<ListItem>> {
    let list = list.clone();

    root.from(move || -> Vec<ListItem> {
        let mut list_out: Vec<ListItem> = Vec::new();

        let result = list.get_value();

        match result.as_ref() {
            Ok(current_view) => {
                for (name, item) in current_view.as_ref() {
                    list_out.push(ListItem {
                        name: name.clone(),
                        dir: item.dir,
                        prirority: get_list_item_prirority(name),
                    });
                }

                list_out.sort_by(|a: &ListItem, b: &ListItem| -> Ordering {
                    let a_prirority = get_list_item_prirority(&a.name);
                    let b_prirority = get_list_item_prirority(&b.name);

                    if a_prirority == 2 && b_prirority == 2 {
                        if a.dir && !b.dir {
                            return Ordering::Less;
                        }

                        if !a.dir && b.dir {
                            return Ordering::Greater;
                        }
                    }

                    if a_prirority > b_prirority {
                        return Ordering::Less;
                    }

                    if a_prirority < b_prirority {
                        return Ordering::Greater;
                    }

                    a.name.to_lowercase().cmp(&b.name.to_lowercase())
                });

                list_out
            },
            Err(ResourceError::Loading) => {
                log::info!("Create list --> Loading");
                Vec::new()
            },
            Err(err) => {
                log::error!("Create list --> {:?}", err);
                Vec::new()
            }
        }
    })
}

fn create_current_item_view(
    root: &Dependencies,
    current_item: &Value<Option<String>>,
    list: &Computed<Vec<ListItem>>
) -> Computed<Option<String>> {
    let current_item = current_item.clone();
    let list = list.clone();

    root.from(move || -> Option<String> {
        let current_item = current_item.get_value();

        if let Some(current_item) = current_item.as_ref() {
            return Some(current_item.clone());
        }

        let list = list.get_value();
        if let Some(first) = list.first() {
            return Some(first.name.clone());
        }

        None
    })
}

fn create_current_full_path(
    root: &Dependencies,
    current_path_dir: &Value<Vec<String>>,
    list_current_item: &Computed<Option<String>>,
) -> Computed<Vec<String>> {
    let current_path_dir = current_path_dir.clone();
    let list_current_item = list_current_item.clone();

    root.from(move || -> Vec<String> {
        let mut current_path_dir = current_path_dir.get_value().as_ref().clone();

        if let Some(list_current_item) = list_current_item.get_value().as_ref() {
            current_path_dir.push(list_current_item.clone());
        }

        current_path_dir
    })
}

fn create_current_content(
    root: &Dependencies,
    state_data: &DataState,
    current_path_dir: &Value<Vec<String>>,
    list_current_item: &Computed<Option<String>>
) -> Computed<CurrentContent> {

    let state_data = state_data.clone();
    let current_path_dir = current_path_dir.to_computed();
    let list_current_item = list_current_item.clone();

    root.from(move || -> CurrentContent {
        let current_path_dir = current_path_dir.get_value();
        let list_current_item = list_current_item.get_value();

        state_data.get_content(current_path_dir.as_ref(), list_current_item.as_ref())
    })
}

fn create_avaible_delete_current(
    root: &Dependencies,
    current_content: Computed<CurrentContent>
) -> Computed<bool> {

    root.from(move || -> bool {
        let current = current_content.get_value();

        match current.as_ref() {
            CurrentContent::None => false,
            CurrentContent::File { .. } => true,
            CurrentContent::Dir { list, ..} => list.len() == 0
        }
    })
}

#[derive(PartialEq)]
pub struct AppIndexState {
    data_state: DataState,

    list_hash_map: Computed<Result<Rc<HashMap<String, TreeItem>>, ResourceError>>,

    //aktualnie wyliczona lista
    pub list: Computed<Vec<ListItem>>,

    //wybrany element z listy, dla widoku
    pub list_current_item: Computed<Option<String>>,

    // pub current_full_path: Computed<Vec<String>>,

    //aktualnie wyliczony wybrany content wskazywany przez current_path
    pub current_content: Computed<CurrentContent>,

    app_state: Rc<AppState>,

    pub alert: Computed<AlertState>,

    //true - jeśli aktualnie podświetlony element jest mozliwy do usuniecia
    pub avaible_delete_button: Computed<bool>,

    pub tabs_url: Value<Vec<String>>,
    pub tabs_active: Value<Option<String>>,
}

impl AppIndexState {
    pub fn new(
        app_state: Rc<AppState>,
    ) -> (Computed<AppIndexState>, impl Fn(vertigo::KeyDownEvent) -> bool) {
        let root = &app_state.root.clone();
        let state_data = app_state.data_state.clone();

        let list_hash_map = create_list_hash_map(root, &state_data, &state_data.current_path_dir);
        let list = create_list(root, &list_hash_map);

        let list_current_item = create_current_item_view(&root, &state_data.current_path_item, &list);
        let current_content = create_current_content(
            root,
            &state_data,
            &state_data.current_path_dir,
            &list_current_item,
        );

        let current_full_path = create_current_full_path(
            &root,
            &state_data.current_path_dir,
            &list_current_item,
        );

        let alert = AlertState::new(
            app_state.clone(),
            current_full_path,
            list.clone(),
            state_data.request.clone()
        );

        let avaible_delete_current= create_avaible_delete_current(&root, current_content.clone());
    
        let tabs_url = root.new_value(Vec::new());
        let tabs_active = root.new_value(None);

        let state = Rc::new(AppIndexState {
            data_state: state_data,
            list_hash_map,
            list,
            list_current_item,
            current_content,
            app_state,
            alert,
            avaible_delete_button: avaible_delete_current,
            tabs_url,
            tabs_active,
        });

        let keydown = {
            let state = state.clone();
            move |event: vertigo::KeyDownEvent| -> bool {
                state.keydown(event.code)
            }
        };

        (root.new_computed_from(state), keydown)
    }


    pub fn set_path(&self, path: Vec<String>) {
        let current_path = self.data_state.current_path_dir.get_value();

        if current_path.as_ref().as_slice() == path.as_slice() {
            log::info!("path are equal");
            return;
        }
    
        let (new_current_path, new_current_item_value) = calculate_next_path(current_path.as_ref(), path);

        self.app_state.root.transaction(||{
            self.data_state.current_path_dir.set_value(new_current_path);
            self.data_state.current_path_item.set_value(new_current_item_value);
        });
    }

    pub fn click_list_item(&self, node: String) {
        let list_hash_map_rc = self.list_hash_map.get_value();

        if let Ok(list) = list_hash_map_rc.as_ref() {
            if let Some(node_details) = list.get(&node) {
                if node_details.dir {
                    let mut current = self.data_state.current_path_dir.get_value().as_ref().clone();
                    current.push(node.clone());
                    self.set_path(current);
                } else {
                    self.data_state.current_path_item.set_value(Some(node.clone()));
                }
                return;
            }
        }

        log::error!("push_path - ignore: {}", node);
    }

    fn find(&self, item_finding: &String) -> Option<isize> {
        let list = self.list.get_value();

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

        let list = self.list.get_value();

        if let Some(first) = list.get(index) {
            self.data_state.current_path_item.set_value(Some(first.name.clone()));
            return true;
        }

        false
    }

    fn try_set_pointer_to_end(&self) {
        let len = self.list.get_value().len() as isize;
        self.try_set_pointer_to(len - 1);
    }

    fn pointer_up(&self) {
        let list_pointer_rc = self.list_current_item.get_value();

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
        let list_pointer_rc = self.list_current_item.get_value();

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
        let list_pointer = self.list_current_item.get_value();

        if let Some(list_pointer) = list_pointer.as_ref() {
            if let Some(_) = self.find(list_pointer) {
                self.click_list_item(list_pointer.clone());
            }
        }
    }

    fn backspace(&self) {
        let current_path = self.data_state.current_path_dir.get_value();
        let mut current_path = current_path.as_ref().clone();

        current_path.pop();

        self.set_path(current_path);
    }

    pub fn keydown(&self, code: String) -> bool {
        if code == "ArrowUp" {
            self.pointer_up();
            return true;
        } else if code == "ArrowDown" {
            self.pointer_down();
            return true;
        } else if code == "Escape" {
            self.data_state.current_path_item.set_value(None);
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
        let path = self.data_state.current_path_dir.get_value();
        let select_item = self.list_current_item.get_value();
        self.app_state.redirect_to_content(&path, &select_item);
    }

    pub fn create_file(&self) {
        let path = self.data_state.current_path_dir.get_value();
        let list = self.list.clone();

        self.app_state.redirect_to_new_content(path.as_ref(), list);
    }

    pub fn redirect_to_mkdir(&self) {
        self.app_state.redirect_to_mkdir(self.list.clone());
    }

    pub fn current_rename(&self) {
        let path = self.data_state.current_path_dir.get_value();
        let select_item = self.list_current_item.get_value();

        if let Some(select_item) = select_item.as_ref() {
            self.app_state.redirect_to_rename_item(&path, &select_item);
        } else {
            log::error!("current_rename fail");
        }
    }

    pub fn current_path_dir(&self) -> Rc<Vec<String>> {
        self.data_state.current_path_dir.get_value()
    }

    pub fn tabs_has(&self, url: &String) -> bool {
        let tabs_url = self.tabs_url.get_value();
        tabs_url.contains(url)
    }

    pub fn tabs_add(&self, url: String) {
        log::info!("add ... {}", &url);
        let tabs_url = self.tabs_url.get_value();

        if tabs_url.contains(&url) {
            log::error!("is contain {}", url);
            return;
        }

        let mut tabs_url = tabs_url.as_ref().clone();
        tabs_url.push(url);
        self.tabs_url.set_value(tabs_url);
    }

    pub fn tabs_remove(&self, url: String) {
        let tabs_url = self.tabs_url.get_value();

        if !tabs_url.contains(&url) {
            log::error!("not contain {}", url);
            return;
        }
        
        let tabs_url = tabs_url.as_ref().clone();
        let mut new_tabs = Vec::<String>::with_capacity(tabs_url.len());

        for tab_url in tabs_url.into_iter() {
            if tab_url != url {
                new_tabs.push(tab_url);
            }
        }

        self.tabs_url.set_value(new_tabs);
    }

    pub fn tabs_set(&self, url: String) {
        let tabs_url = self.tabs_url.get_value();

        if !tabs_url.contains(&url) {
            log::error!("not contain {}", url);
            return;
        }

        self.tabs_active.set_value(Some(url));
    }

    pub fn tabs_default(&self) {
        self.tabs_active.set_value(None);
    }
}


fn calculate_next_path(prev_path: &[String], new_path: Vec<String>) -> (Vec<String>, Option<String>) {
    if new_path.len() > prev_path.len() {
        return (new_path, None);
    }

    if prev_path[0..new_path.len()] == new_path[0..] {
        let last = prev_path.get(new_path.len());
        let last = last.map(|item| item.clone());
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