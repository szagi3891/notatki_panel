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
use crate::request::{Request, Resource, ResourceError};
use super::{StateNodeDir, StateRoot, TreeItem, state_node_content::StateNodeContent};

#[derive(PartialEq, Clone)]
pub enum CurrentContent {
    File {
        file: String,           //name
        file_hash: String,      //hash
    },
    Dir {
        dir: String,            //hash
    },
    None
}

#[derive(PartialEq)]
pub struct CurrentView {
    list: Rc<HashMap<String, TreeItem>>,
    content: CurrentContent,        //wskaźnik na content który jest plikiem jakimś
}

impl CurrentView {
    fn is_file_select(&self) -> bool {
        if let CurrentContent::File { .. } = self.content {
            return true;
        }

        false
    }

    fn get_select_file(&self) -> Option<String> {
        if let CurrentContent::File { file, .. } = &self.content {
            return Some(file.clone());
        }

        None
    }

    fn file(file: String, file_hash: String, list: Rc<HashMap<String, TreeItem>>) -> CurrentView {
        CurrentView {
            list,
            content: CurrentContent::File {
                file,
                file_hash,
            }
        }
    }

    fn dir(dir: String, list: Rc<HashMap<String, TreeItem>>) -> CurrentView {
        CurrentView {
            list,
            content: CurrentContent::Dir {
                dir,
            }
        }
    }

    fn none(list: Rc<HashMap<String, TreeItem>>) -> CurrentView {
        CurrentView {
            list,
            content: CurrentContent::None
        }
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

fn move_pointer(state_node_dir: &StateNodeDir, current_wsk: CurrentView, path_item: &String) -> Result<CurrentView, ResourceError> {

    if current_wsk.is_file_select() {
        return Err(ResourceError::Error(format!("Dir expected {}", path_item)));
    }

    let child = get_item_from_map(&current_wsk.list, path_item)?;

    if child.dir {
        log::info!("return2");

        let child_list = state_node_dir.get_list(&child.id)?;

        return Ok(CurrentView::dir(path_item.clone(), child_list));
    }

    Ok(CurrentView::file(path_item.clone(), child.id.clone(), current_wsk.list))
}

fn create_current_view(
    root: &Dependencies,
    current_path: Computed<Vec<String>>,
    state_root: StateRoot,
    state_node_dir: StateNodeDir
) -> Computed<Resource<CurrentView>> {

    root.from(move || -> Resource<CurrentView> {
        let root_wsk = state_root.get_current_root()?;

        let current_wsk = state_node_dir.get_list(&root_wsk)?;

        let mut result = CurrentView::none(current_wsk);

        for path_item in current_path.get_value().as_ref() {
            result = move_pointer(&state_node_dir, result, &path_item)?;
        }

        Ok(result)
    })
}

#[derive(PartialEq, Debug, Clone)]
pub struct ListItem {
    pub name: String,
    pub dir: bool,
}

fn create_list(root: &Dependencies, current_view: &Computed<Resource<CurrentView>>) -> Computed<Vec<ListItem>> {
    let current_view = current_view.clone();

    root.from(move || -> Vec<ListItem> {
        let mut list_dir: Vec<ListItem> = Vec::new();
        let mut list_file: Vec<ListItem> = Vec::new();

        let current_view = current_view.get_value();

        match current_view.as_ref() {
            Ok(current_view) => {
                for (name, item) in current_view.list.as_ref() {
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

fn create_list_select_item(root: &Dependencies, current_view: &Computed<Resource<CurrentView>>) -> Computed<Option<String>> {
    let current_view = current_view.clone();

    root.from(move || -> Option<String> {
        let current_view = current_view.get_value();

        match current_view.as_ref() {
            Ok(current_view) => {
                current_view.get_select_file()
            },
            Err(_) => None
        }
    })
}

#[derive(PartialEq)]
pub enum CurrentContentFullDetails {
    File {
        content: Rc<String>,
    },
    None,
}

impl CurrentContentFullDetails {
    pub fn to_string(&self) -> Option<Rc<String>> {
        if let CurrentContentFullDetails::File { content } = self {
            return Some(content.clone());
        }

        None
    }
}

fn create_current_content(root: &Dependencies, current_view: &Computed<Resource<CurrentView>>, state_node_content: StateNodeContent) -> Computed<CurrentContentFullDetails> {
    let current_view = current_view.clone();

    root.from(move || -> CurrentContentFullDetails {
        let current_view = current_view.get_value();

        match current_view.as_ref() {
            Ok(current_view) => {
                match &current_view.content {
                    CurrentContent::File { file_hash, .. } => {
                        let result = state_node_content.get(file_hash);

                        if let Ok(result) = result {
                            return CurrentContentFullDetails::File {
                                content: result.clone(),
                            };
                        }

                        CurrentContentFullDetails::None
                    },
                    _ => CurrentContentFullDetails::None,
                }
            },
            Err(_) => CurrentContentFullDetails::None,
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
    pub current_path: Value<Vec<String>>,                               //aktualna ściezka
    pub list_pointer: Value<Option<String>>,                            //aktualny wskaźnik (akcji)
    pub current_view: Computed<Resource<CurrentView>>,                  //lista plików i katalogów w lewym panelu
    pub current_content: Computed<CurrentContentFullDetails>,
    pub list: Computed<Vec<ListItem>>,
    pub list_current_show_item: Computed<Option<String>>,                     //podświetlenie elementu aktualnie wskazanego przez ściezkę
    pub current_edit: Value<Option<CurrentAction>>,
}

impl State {
    pub fn new(root: &Dependencies, driver: &DomDriver) -> Computed<State> {
        let request = Request::new(driver);

        let state_node_dir = StateNodeDir::new(&request, root);
        let state_node_content = StateNodeContent::new(&request, root);
        let state_root = StateRoot::new(&request, root, state_node_dir.clone());

        let current_path = root.new_value(Vec::<String>::new());
        let list_pointer = root.new_value(None);

        let current_view = create_current_view(
            root,
            current_path.to_computed(),
            state_root,
            state_node_dir,
        );

        let list = create_list(root, &current_view);

        let list_current_show_item = create_list_select_item(root, &current_view);

        let current_content= create_current_content(root, &current_view, state_node_content);

        let current_edit = root.new_value(None);

        root.new_computed_from(State {
            root: root.clone(),
            current_path,
            list_pointer,
            current_view,
            current_content,
            list,
            list_current_show_item,
            current_edit,
        })
    }

    pub fn set_path(&self, path: Vec<String>) {
        self.root.transaction(|| {
            self.current_path.set_value(path);
            self.list_pointer.set_value(None);
        });
    }

    pub fn push_path(&self, node: String) {
        let current_view = &*self.current_view.get_value();

        let is_file_select = match current_view {
            Ok(current) => current.is_file_select(),
            Err(err) => {
                log::error!("Ignore action push_path, reason -> {:?}", err);
                return;
            }
        };


        let mut current = (*self.current_path.get_value()).clone();

        if is_file_select {
            let last = current.pop();

            if last.is_none() {
                log::error!("Ignore action push_path, reason -> missing last element");
                return;
            }
        }

        current.push(node.clone());

        self.root.transaction(|| {
            let reset_pointer = {
                let list = self.list.get_value();

                let mut result = false;
    
                for item in list.as_ref() {
                    if item.name == node {
                        result = item.dir;
                        break;
                    }
                }

                result
            };

            self.current_path.set_value(current);

            if reset_pointer {
                self.list_pointer.set_value(None);
            }
        });
    }

    pub fn pop_path(&self) {
        let current_path = self.current_path.get_value();
        let mut current_path = current_path.as_ref().clone();
        let last = current_path.pop();

        if let Some(last) = last {
            self.root.transaction(|| {
                self.current_path.set_value(current_path);
                self.list_pointer.set_value(Some(last));
            });
        }
    }

    pub fn set_pointer(&self, name: &String) {
        self.list_pointer.set_value(Some(name.clone()));
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
            self.list_pointer.set_value(Some(first.name.clone()));
            return true;
        }

        false
    }

    fn try_set_pointer_to_end(&self) {
        let len = self.list.get_value().len() as isize;
        self.try_set_pointer_to(len - 1);
    }

    fn pointer_up(&self) {
        let list_pointer_rc = self.list_pointer.get_value();
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
        let list_pointer_rc = self.list_pointer.get_value();
        let list_pointer = list_pointer_rc.as_ref();

        if let Some(list_pointer) = list_pointer {
            if let Some(index) = self.find(list_pointer) {
                if !self.try_set_pointer_to(index + 1) {
                    self.try_set_pointer_to(0);
                }
            }
        } else {
            self.try_set_pointer_to_end();
        }
    }

    fn pointer_enter(&self) {
        let list_pointer = self.list_pointer.get_value();

        if let Some(list_pointer) = list_pointer.as_ref() {
            if let Some(_) = self.find(list_pointer) {
                self.push_path(list_pointer.clone());
            }
        }
    }

    pub fn keydown(&self, code: String) {
        if code == "ArrowUp" {
            self.pointer_up();
        } else if code == "ArrowDown" {
            self.pointer_down();
        } else if code == "Escape" {
            self.list_pointer.set_value(None);
        } else if code == "ArrowRight" || code == "Enter" {
            self.pointer_enter();
        } else if code == "ArrowLeft" {
            self.pop_path();
        }

        //po wejściu do nowego katalogu, zaznaczać pierwszy element na liście ????
        

        log::info!("klawisz ... {:?} ", code);
    }
}

