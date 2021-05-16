use std::{cmp::Ordering, collections::HashMap};
use std::rc::Rc;
use vertigo::{
    DomDriver,
    computed::{
        Computed,
        Dependencies,
        Value
    }
};
use crate::request::{Request, ResourceError};
use super::{StateNodeDir, StateRoot, TreeItem, state_node_content::StateNodeContent};

#[derive(PartialEq, Clone, Debug)]
pub enum CurrentContent {
    File {
        file: String,           //name
        file_hash: String,      //hash
        content: Rc<String>,    //content file
    },
    Dir {
        dir: String,            //hash
        list: Rc<HashMap<String, TreeItem>>,
    },
    None
}

impl CurrentContent {
    fn file(file: String, file_hash: String, content: Rc<String>) -> CurrentContent {
        CurrentContent::File {
            file,
            file_hash,
            content,
        }
    }

    fn dir(dir: String, list: Rc<HashMap<String, TreeItem>>) -> CurrentContent {
        CurrentContent::Dir {
            dir,
            list,
        }
    }

    pub fn to_string(&self) -> Option<Rc<String>> {
        if let CurrentContent::File { content, .. } = self {
            return Some(content.clone());
        }

        None
    }
}

fn get_item_from_map<'a>(current_wsk: &'a Rc<HashMap<String, TreeItem>>, path_item: &String) -> Result<&'a TreeItem, ResourceError> {
    let wsk_child = current_wsk.get(path_item);

    let wsk_child = match wsk_child {
        Some(wsk_child) => wsk_child,
        None => {
            return Err(ResourceError::Error(format!("missing tree_item {}", path_item)));
        }
    };

    Ok(wsk_child)
}

fn move_pointer(state_node_dir: &StateNodeDir, list: Rc<HashMap<String, TreeItem>>, path_item: &String) -> Result<Rc<HashMap<String, TreeItem>>, ResourceError> {

    let child = get_item_from_map(&list, path_item)?;

    if child.dir {
        let child_list = state_node_dir.get_list(&child.id)?;

        return Ok(child_list);
    }

    return Err(ResourceError::Error(format!("dir expected {}", path_item)));
}

fn get_dir_content(state_root: &StateRoot, state_node_dir: &StateNodeDir, current_path: &Vec<String>) -> Result<Rc<HashMap<String, TreeItem>>, ResourceError> {
    let root_wsk = state_root.get_current_root()?;

    let mut result = state_node_dir.get_list(&root_wsk)?;

    for path_item in current_path {
        result = move_pointer(&state_node_dir, result, &path_item)?;
    }

    Ok(result)
}

#[derive(PartialEq, Debug, Clone)]
pub struct ListItem {
    pub name: String,
    pub dir: bool,
}

fn create_list_hash_map(root: &Dependencies, state_root: &StateRoot, state_node_dir: &StateNodeDir, current_path: &Value<Vec<String>>) -> Computed<Result<Rc<HashMap<String, TreeItem>>, ResourceError>> {
    let state_root = state_root.clone();
    let state_node_dir = state_node_dir.clone();
    let current_path = current_path.to_computed();

    root.from(move || -> Result<Rc<HashMap<String, TreeItem>>, ResourceError> {
        let current_path_rc = current_path.get_value();
        let current_path = current_path_rc.as_ref();

        get_dir_content(&state_root, &state_node_dir, current_path)
    })
}

fn create_list(root: &Dependencies, list: &Computed<Result<Rc<HashMap<String, TreeItem>>, ResourceError>>) -> Computed<Vec<ListItem>> {
    let list = list.clone();

    root.from(move || -> Vec<ListItem> {
        let mut list_dir: Vec<ListItem> = Vec::new();
        let mut list_file: Vec<ListItem> = Vec::new();

        let result = list.get_value();

        match result.as_ref() {
            Ok(current_view) => {
                for (name, item) in current_view.as_ref() {
                    if item.dir {
                        list_dir.push(ListItem {
                            name: name.clone(),
                            dir: true,
                        });
                    } else {
                        list_file.push(ListItem {
                            name: name.clone(),
                            dir: false,
                        });
                    }
                }

                list_dir.sort_by(|a: &ListItem, b: &ListItem| -> Ordering {
                    a.name.cmp(&b.name)
                });

                list_file.sort_by(|a: &ListItem, b: &ListItem| -> Ordering {
                    a.name.cmp(&b.name)
                });

                list_dir.extend(list_file);

                return list_dir;
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
    state_node_dir: &StateNodeDir,
    state_node_content: &StateNodeContent,
    list: &Computed<Result<Rc<HashMap<String, TreeItem>>, ResourceError>>,
    current_item: &Computed<Option<String>>
) -> Computed<CurrentContent> {

    let state_node_dir = state_node_dir.clone();
    let state_node_content = state_node_content.clone();
    let list = list.clone();
    let current_item = current_item.clone();

    root.from(move || -> CurrentContent {
        let current_item_rc = current_item.get_value();
        let current_item = current_item_rc.as_ref();
    
        let current_item = match current_item {
            Some(current_item) => current_item,
            None => {
                return CurrentContent::None;
            }
        };

        let list = list.get_value();

        match list.as_ref() {
            Ok(list) => {
                let current_value = list.get(current_item);

                if let Some(current_value) = current_value {
                    if current_value.dir {
                        let list = state_node_dir.get_list(&current_value.id);

                        if let Ok(list) = list {
                            return CurrentContent::dir(current_item.clone(), list);
                        }
                        
                        return CurrentContent::None;
                    } else {
                        let content = state_node_content.get(&current_value.id);

                        if let Ok(content) = content {
                            return CurrentContent::file(current_item.clone(), current_value.id.clone(), content.clone());
                        }

                        return CurrentContent::None;
                    }
                }

                return CurrentContent::None;
            },
            Err(_) => {
                return CurrentContent::None;
            }
        }
    })
}

#[derive(PartialEq)]
pub enum CurrentAction {
    CurrentEdit {
        path: Vec<String>,          //edutowany element
        hash: String,               //hash poprzedniej zawartosci
        content: String,            //edytowana tresc
    }

    //zmiana nazwy
    //tworzenie pliku
    //tworzenie katalogu
}

#[derive(PartialEq)]
pub struct State {
    root: Dependencies,
    //aktualna ścieka powinna przechowywać tylko katalogi
    pub current_path: Value<Vec<String>>,                               //aktualna ściezka

    //aktualny wskaźnik na wybrany element z katalogu
    current_item_value: Value<Option<String>>,                            //aktualny wskaźnik (akcji)

    list_hash_map: Computed<Result<Rc<HashMap<String, TreeItem>>, ResourceError>>,
    //aktualnie wyliczona lista
    pub list: Computed<Vec<ListItem>>,
                                                                        //wybrany element z listy, dla widoku
    pub current_item: Computed<Option<String>>,


    //aktualnie wyliczony wybrany content wskazywany przez current_path
    pub current_content: Computed<CurrentContent>,
    
    //pub current_edit: Value<Option<CurrentAction>>,
}

impl State {
    pub fn new(root: &Dependencies, driver: &DomDriver) -> Computed<State> {
        let request = Request::new(driver);

        let state_node_dir = StateNodeDir::new(&request, root);
        let state_node_content = StateNodeContent::new(&request, root);
        let state_root = StateRoot::new(&request, root, state_node_dir.clone());

        let current_path = root.new_value(Vec::<String>::new());
        let current_item_value = root.new_value(None);

        let list_hash_map = create_list_hash_map(root, &state_root, &state_node_dir, &current_path);
        let list = create_list(root, &list_hash_map);

        let current_item = create_current_item_view(&root, &current_item_value, &list);

        let current_content = create_current_content(
            root,
            &state_node_dir,
            &state_node_content,
            &list_hash_map,
            &current_item,
        );

        root.new_computed_from(State {
            root: root.clone(),
            current_path,
            current_item_value,
            list_hash_map,
            list,
            current_item,
            current_content,
        })
    }

    pub fn set_path(&self, path: Vec<String>) {
        let current_path = self.current_path.get_value();

        let (new_current_path, new_current_item_value) = calculate_next_path(current_path.as_ref(), path);

        self.root.transaction(|| {
            self.current_path.set_value(new_current_path);
            self.current_item_value.set_value(new_current_item_value);
        });
    }

    pub fn click_list_item(&self, node: String) {
        let list_hash_map_rc = self.list_hash_map.get_value();

        if let Ok(list) = list_hash_map_rc.as_ref() {
            if let Some(node_details) = list.get(&node) {
                if node_details.dir {
                    let mut current = (*self.current_path.get_value()).clone();

                    current.push(node.clone());

                    self.root.transaction(|| {
                        self.current_path.set_value(current);
                        self.current_item_value.set_value(None);
                    });
                    return;
                } else {
                    self.current_item_value.set_value(Some(node.clone()));
                }
            }
        }

        log::error!("push_path - ignore: {}", node);
    }

    pub fn pop_path(&self) {
        let current_path = self.current_path.get_value();
        let mut current_path = current_path.as_ref().clone();
        let last = current_path.pop();

        if let Some(last) = last {
            self.root.transaction(|| {
                self.current_path.set_value(current_path);
                self.current_item_value.set_value(Some(last));
            });
        }
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
            self.current_item_value.set_value(Some(first.name.clone()));
            return true;
        }

        false
    }

    fn try_set_pointer_to_end(&self) {
        let len = self.list.get_value().len() as isize;
        self.try_set_pointer_to(len - 1);
    }

    fn pointer_up(&self) {
        let list_pointer_rc = self.current_item.get_value();
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
        let list_pointer_rc = self.current_item.get_value();
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
        let list_pointer = self.current_item.get_value();

        if let Some(list_pointer) = list_pointer.as_ref() {
            if let Some(_) = self.find(list_pointer) {
                self.click_list_item(list_pointer.clone());
            }
        }
    }

    pub fn keydown(&self, code: String) {
        if code == "ArrowUp" {
            self.pointer_up();
        } else if code == "ArrowDown" {
            self.pointer_down();
        } else if code == "Escape" {
            self.current_item_value.set_value(None);
        } else if code == "ArrowRight" || code == "Enter" {
            self.pointer_enter();
        } else if code == "ArrowLeft" {
            self.pop_path();
        }

        //po wejściu do nowego katalogu, zaznaczać pierwszy element na liście ????

        log::info!("klawisz ... {:?} ", code);
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