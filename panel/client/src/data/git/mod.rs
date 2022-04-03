use std::{collections::HashMap, rc::Rc};
use vertigo::{Driver, Resource};
use std::cmp::Ordering;

mod node_dir;
mod node_content;
mod root;

pub use node_dir::{Dir, TreeItem};
pub use node_content::Content;
pub use root::Root;

use super::ListItem;

#[derive(PartialEq, Clone, Debug)]
pub struct DirList {
    list: Rc<HashMap<String, TreeItem>>,
}

impl DirList {
    pub fn new(list: Rc<HashMap<String, TreeItem>>) -> DirList {
        DirList {
            list
        }
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn get(&self, current_item: &String) -> Option<&TreeItem> {
        self.list.get(current_item)
    }

    pub fn get_list(&self) -> Vec<ListItem> {
        let mut list_out: Vec<ListItem> = Vec::new();

        for (name, item) in self.list.as_ref() {
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
    }
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


#[derive(PartialEq, Clone, Debug)]
pub enum CurrentContent {
    File {
        file_name: String,      //name
        file_hash: String,      //hash
        content: Rc<String>,    //content file
    },
    Dir {
        dir_full: Vec<String>,  //Pełna ścieka prowadząca do tego katalogu
        dir: String,            //hash
        dir_hash: String,
        list: DirList,
    },
    None
}

impl CurrentContent {
    fn file(file_name: String, file_hash: String, content: Rc<String>) -> CurrentContent {
        CurrentContent::File {
            file_name,
            file_hash,
            content,
        }
    }

    fn dir(dir_full: Vec<String>, dir: String, dir_hash: String, list: DirList) -> CurrentContent {
        CurrentContent::Dir {
            dir_full,
            dir,
            dir_hash,
            list,
        }
    }

    // pub fn is_file(&self) -> bool {
    //     if let Self::File{..} = self {
    //         true
    //     } else {
    //         false
    //     }
    // }

    // pub fn is_dir(&self) -> bool {
    //     if let Self::Dir{..} = self {
    //         true
    //     } else {
    //         false
    //     }
    // }

    // pub fn is_none(&self) -> bool {
    //     if let Self::None = self {
    //         true
    //     } else {
    //         false
    //     }
    // }
}



fn get_item_from_map<'a>(current_wsk: &'a DirList, path_item: &String) -> Resource<&'a TreeItem> {
    let wsk_child = current_wsk.get(path_item);

    let wsk_child = match wsk_child {
        Some(wsk_child) => wsk_child,
        None => {
            return Resource::Error(format!("missing tree_item {}", path_item));
        }
    };

    Resource::Ready(wsk_child)
}

fn move_pointer(state_data: &Git, list: DirList, path_item: &String) -> Resource<DirList> {

    let child = get_item_from_map(&list, path_item)?;

    if child.dir {
        let child_list = state_data.dir.get_list(&child.id)?;

        return Resource::Ready(child_list);
    }

    return Resource::Error(format!("dir expected {}", path_item));
}


#[derive(Clone)]
pub struct Git {
    pub driver: Driver,
    pub dir: Dir,
    pub content: Content,
    pub root: Root
}

impl Git {
    pub fn new(driver: &Driver) -> Git {
        let dir = Dir::new(driver);
        let content = Content::new(driver);
        let root = Root::new(driver);

        Git {
            driver: driver.clone(),
            dir,
            content,
            root,
        }
    }


    pub fn dir_list(&self, path: &[String]) -> Resource<DirList> {
        let root_wsk = self.root.get_current_root()?;

        let mut result = self.dir.get_list(&root_wsk)?;

        for path_item in path {
            result = move_pointer(self, result, path_item)?;
        }

        Resource::Ready(result)
    }

    fn node_content(&self, base_dir: &[String], current_item: &Option<String>) -> Resource<CurrentContent> {
        let list = self.dir_list(base_dir)?;

        let current_item = match current_item {
            Some(current_item) => current_item,
            None => {
                return Resource::Ready(CurrentContent::None);
            }
        };

        let current_value = list.get(current_item);

        if let Some(current_value) = current_value {
            if current_value.dir {
                let mut dir_full = Vec::from(base_dir);
                dir_full.push(current_item.clone());

                let list = self.dir.get_list(&current_value.id)?;
                return Resource::Ready(CurrentContent::dir(dir_full, current_item.clone(), current_value.id.clone(), list));
            } else {
                let content = self.content.get(&current_value.id)?;
                return Resource::Ready(CurrentContent::file(current_item.clone(), current_value.id.clone(), content));
            }
        }

        Resource::Ready(CurrentContent::None)
    }

    pub fn get_content(&self, base_dir: &[String], item: &Option<String>) -> CurrentContent {

        let result = self.node_content(base_dir, item);

        if let Resource::Ready(result) = result {
            return result;
        }

        CurrentContent::None
    }

    pub fn content_from_path(&self, path: &[String]) -> CurrentContent {
        let mut path: Vec<String> = Vec::from(path);

        let last = path.pop();

        self.get_content(path.as_slice(), &last)
    }

    pub fn content_hash(&self, path: &[String]) -> Option<String> {
        let result = self.content_from_path(path);

        match result {
            CurrentContent::File { file_hash, .. } => {
                Some(file_hash)
            },
            CurrentContent::Dir { dir_hash, .. } => {
                Some(dir_hash)
            },
            CurrentContent::None => None,
        }
    }

    pub fn get_content_string(&self, path: &[String]) -> Option<String> {
        let result = self.content_from_path(path);

        match result {
            CurrentContent::File { content, .. } => {
                Some(content.as_ref().clone())
            },
            CurrentContent::Dir { .. } => None,
            CurrentContent::None => None,
        }
    }

}

