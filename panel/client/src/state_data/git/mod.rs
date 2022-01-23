use std::{collections::HashMap, rc::Rc};

use vertigo::{Driver, Resource};

mod state_node_dir;
mod state_node_content;
mod state_root;

pub use state_node_dir::{StateNodeDir, TreeItem};
pub use state_node_content::StateNodeContent;
pub use state_root::StateRoot;

use super::CurrentContent;


fn get_item_from_map<'a>(current_wsk: &'a Rc<HashMap<String, TreeItem>>, path_item: &String) -> Resource<&'a TreeItem> {
    let wsk_child = current_wsk.get(path_item);

    let wsk_child = match wsk_child {
        Some(wsk_child) => wsk_child,
        None => {
            return Resource::Error(format!("missing tree_item {}", path_item));
        }
    };

    Resource::Ready(wsk_child)
}

fn move_pointer(state_data: &Git, list: Rc<HashMap<String, TreeItem>>, path_item: &String) -> Resource<Rc<HashMap<String, TreeItem>>> {

    let child = get_item_from_map(&list, path_item)?;

    if child.dir {
        let child_list = state_data.dir.get_list(&child.id)?;

        return Resource::Ready(child_list);
    }

    return Resource::Error(format!("dir expected {}", path_item));
}


#[derive(Clone, PartialEq)]
pub struct Git {
    pub driver: Driver,
    pub dir: StateNodeDir,
    pub content: StateNodeContent,
    pub root: StateRoot
}

impl Git {
    pub fn new(driver: &Driver) -> Git {
        let dir = StateNodeDir::new(driver);
        let content = StateNodeContent::new(driver);
        let root = StateRoot::new(driver, dir.clone());

        Git {
            driver: driver.clone(),
            dir,
            content,
            root,
        }
    }


    pub fn dir_list(&self, path: &[String]) -> Resource<Rc<HashMap<String, TreeItem>>> {
        let root_wsk = self.root.get_current_root()?;

        let mut result = self.dir.get_list(&root_wsk)?;

        for path_item in path {
            result = move_pointer(self, result, &path_item)?;
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
                let list = self.dir.get_list(&current_value.id)?;
                return Resource::Ready(CurrentContent::dir(current_item.clone(), current_value.id.clone(), list));
            } else {
                let content = self.content.get(&current_value.id)?;
                return Resource::Ready(CurrentContent::file(current_item.clone(), current_value.id.clone(), content.clone()));
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
