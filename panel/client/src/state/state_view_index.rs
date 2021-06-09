use std::{cmp::Ordering, collections::HashMap};
use std::rc::Rc;
use vertigo::{computed::{
        Computed,
        Dependencies,
        Value
    }, utils::Action};
use crate::request::{ResourceError};
use super::{StateData, state::StateAction, state_data::{CurrentContent, TreeItem}};


#[derive(PartialEq, Debug, Clone)]
pub struct ListItem {
    pub name: String,
    pub dir: bool,
}


fn create_list_hash_map(root: &Dependencies, state_data: &StateData, current_path: &Value<Vec<String>>) -> Computed<Result<Rc<HashMap<String, TreeItem>>, ResourceError>> {
    let state_data = state_data.clone();
    let current_path = current_path.to_computed();

    root.from(move || -> Result<Rc<HashMap<String, TreeItem>>, ResourceError> {
        let current_path_rc = current_path.get_value();
        let current_path = current_path_rc.as_ref();

        state_data.get_dir_content(current_path)
    })
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
                    });
                }

                list_out.sort_by(|a: &ListItem, b: &ListItem| -> Ordering {
                    let a_prefix = a.name.get(0..1) == Some("_");
                    let b_prefix = b.name.get(0..1) == Some("_");

                    if a_prefix == true && b_prefix == false {
                        return Ordering::Less;
                    }

                    if a_prefix == false && b_prefix == true {
                        return Ordering::Greater;
                    }

                    a.name.to_lowercase().cmp(&b.name.to_lowercase())
                });

                return list_out;
            },
            Err(err) => {
                log::error!("Create list --> {:?}", err);
                return Vec::new();
            }
        };
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

fn create_current_content(
    root: &Dependencies,
    state_data: &StateData,
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


#[derive(PartialEq)]
pub struct StateViewIndex {
    root: Dependencies,

    //aktualna ścieka powinna przechowywać tylko katalogi
    pub current_path_dir: Value<Vec<String>>,                               //aktualna ściezka

    //aktualny wskaźnik na wybrany element z katalogu
    current_path_item: Value<Option<String>>,                            //aktualny wskaźnik (akcji)

    list_hash_map: Computed<Result<Rc<HashMap<String, TreeItem>>, ResourceError>>,

    //aktualnie wyliczona lista
    pub list: Computed<Vec<ListItem>>,
                                                                        //wybrany element z listy, dla widoku
    pub list_current_item: Computed<Option<String>>,


    //aktualnie wyliczony wybrany content wskazywany przez current_path
    pub current_content: Computed<CurrentContent>,

    action: Action<StateAction>,
}

impl StateViewIndex {
    pub fn new(
        root: &Dependencies,
        state_data: StateData,
        action: Action<StateAction>,
    ) -> Computed<StateViewIndex> {

        let current_path_dir = root.new_value(Vec::<String>::new());
        let current_path_item = root.new_value(None);

        let list_hash_map = create_list_hash_map(root, &state_data, &current_path_dir);
        let list = create_list(root, &list_hash_map);

        let list_current_item = create_current_item_view(&root, &current_path_item, &list);

        let current_content = create_current_content(
            root,
            &state_data,
            &current_path_dir,
            &list_current_item,
        );

        root.new_computed_from(StateViewIndex {
            root: root.clone(),
            current_path_dir,
            current_path_item,
            list_hash_map,
            list,
            list_current_item,
            current_content,
            action,
        })
    }


    pub fn set_path(&self, path: Vec<String>) {
        let current_path = self.current_path_dir.get_value();

        if current_path.as_ref().as_slice() == path.as_slice() {
            log::info!("path are equal");
            return;
        }
    
        let (new_current_path, new_current_item_value) = calculate_next_path(current_path.as_ref(), path);

        self.root.transaction(|| {
            self.current_path_dir.set_value(new_current_path);
            self.current_path_item.set_value(new_current_item_value);
        });
    }

    pub fn click_list_item(&self, node: String) {
        let list_hash_map_rc = self.list_hash_map.get_value();

        if let Ok(list) = list_hash_map_rc.as_ref() {
            if let Some(node_details) = list.get(&node) {
                if node_details.dir {
                    let mut current = self.current_path_dir.get_value().as_ref().clone();
                    current.push(node.clone());
                    self.set_path(current);
                } else {
                    self.current_path_item.set_value(Some(node.clone()));
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
            self.current_path_item.set_value(Some(first.name.clone()));
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
        let list_pointer = list_pointer_rc.as_ref();

        if let Some(list_pointer) = list_pointer {
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
        let list_pointer = list_pointer_rc.as_ref();

        if let Some(list_pointer) = list_pointer {
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
        let current_path = self.current_path_dir.get_value();
        let mut current_path = current_path.as_ref().clone();

        current_path.pop();

        self.set_path(current_path);
    }

    pub fn keydown(&self, code: String) {
        if code == "ArrowUp" {
            self.pointer_up();
        } else if code == "ArrowDown" {
            self.pointer_down();
        } else if code == "Escape" {
            self.current_path_item.set_value(None);
        } else if code == "ArrowRight" || code == "Enter" {
            self.pointer_enter();
        } else if code == "ArrowLeft" || code == "Backspace" || code == "Escape" {
            self.backspace();
        }

        log::info!("klawisz ... {:?} ", code);
    }

    pub fn current_edit(&self) {
        let mut path = self.current_path_dir.get_value().as_ref().clone();
        
        if let Some(current_item) = self.list_current_item.get_value().as_ref() {
            path.push(current_item.clone());
        }

        self.action.trigger(StateAction::RedirectToContent { path });
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