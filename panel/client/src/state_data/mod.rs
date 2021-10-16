mod state_root;
mod state_node_dir;
mod state_node_content;

use std::{collections::HashMap, rc::Rc};

use state_node_dir::StateNodeDir;
use state_node_content::StateNodeContent;
use state_root::StateRoot;

use vertigo::{DomDriver, computed::{Dependencies, Value}};
use crate::request::{ResourceError, Request};

pub use state_node_dir::{TreeItem};


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

fn move_pointer(state_data: &DataState, list: Rc<HashMap<String, TreeItem>>, path_item: &String) -> Result<Rc<HashMap<String, TreeItem>>, ResourceError> {

    let child = get_item_from_map(&list, path_item)?;

    if child.dir {
        let child_list = state_data.state_node_dir.get_list(&child.id)?;

        return Ok(child_list);
    }

    return Err(ResourceError::Error(format!("dir expected {}", path_item)));
}


#[derive(PartialEq, Clone, Debug)]
pub enum CurrentContent {
    File {
        file_name: String,      //name
        file_hash: String,      //hash
        content: Rc<String>,    //content file
    },
    Dir {
        dir: String,            //hash
        dir_hash: String,
        list: Rc<HashMap<String, TreeItem>>,
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

    fn dir(dir: String, dir_hash: String, list: Rc<HashMap<String, TreeItem>>) -> CurrentContent {
        CurrentContent::Dir {
            dir,
            dir_hash,
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


#[derive(Clone, PartialEq)]
pub struct DataState {
    pub request: Request,
    pub state_node_dir: StateNodeDir,
    pub state_node_content: StateNodeContent,
    pub state_root: StateRoot,
    pub current_path_dir: Value<Vec<String>>,
    pub current_path_item: Value<Option<String>>,
}

impl DataState {
    pub fn new(root: &Dependencies, driver: &DomDriver) -> DataState {

        let request = Request::new(driver);

        let state_node_dir = StateNodeDir::new(&request, root);
        let state_node_content = StateNodeContent::new(&request, root);
        let state_root = StateRoot::new(&request, root, state_node_dir.clone());

        let current_path_dir: Value<Vec<String>> = root.new_value(Vec::new());
        let current_path_item: Value<Option<String>> = root.new_value(None);

        DataState {
            request,
            state_node_dir,
            state_node_content,
            state_root,
            current_path_dir,
            current_path_item,
        }
    }

    pub fn get_dir_content(&self, path: &[String]) -> Result<Rc<HashMap<String, TreeItem>>, ResourceError> {
        let root_wsk = self.state_root.get_current_root()?;

        let mut result = self.state_node_dir.get_list(&root_wsk)?;

        for path_item in path {
            result = move_pointer(self, result, &path_item)?;
        }

        Ok(result)
    }

    fn get_content_inner(&self, base_dir: &[String], current_item: &Option<String>) -> Result<CurrentContent, ResourceError> {
        let list = self.get_dir_content(base_dir)?;

        let current_item = match current_item {
            Some(current_item) => current_item,
            None => {
                return Ok(CurrentContent::None);
            }
        };

        let current_value = list.get(current_item);

        if let Some(current_value) = current_value {
            if current_value.dir {
                let list = self.state_node_dir.get_list(&current_value.id);

                if let Ok(list) = list {
                    return Ok(CurrentContent::dir(current_item.clone(), current_value.id.clone(), list));
                }
                
                //return Ok(CurrentContent::None);
            } else {
                let content = self.state_node_content.get(&current_value.id);

                if let Ok(content) = content {
                    return Ok(CurrentContent::file(current_item.clone(), current_value.id.clone(), content.clone()));
                }

                //return Ok(CurrentContent::None);
            }
        }

        Ok(CurrentContent::None)
    }

    pub fn get_content(&self, base_dir: &[String], item: &Option<String>) -> CurrentContent {

        let result = self.get_content_inner(base_dir, item);

        match result {
            Ok(result) => result,
            Err(_) => CurrentContent::None,
        }
    }

    pub fn get_content_from_path(&self, path: &[String]) -> CurrentContent {
        let mut path: Vec<String> = Vec::from(path);

        let last = path.pop();

        self.get_content(path.as_slice(), &last)
    }

    pub fn get_content_hash(&self, path: &[String]) -> Option<String> {
        let result = self.get_content_from_path(path);

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
        let result = self.get_content_from_path(path);

        match result {
            CurrentContent::File { content, .. } => {
                Some(content.as_ref().clone())
            },
            CurrentContent::Dir { .. } => None,
            CurrentContent::None => None,
        }
    }

    pub fn get_full_current_path(&self) -> Vec<String> {
        let mut dir = self.current_path_dir.get_value().as_ref().clone();

        if let Some(current) = self.current_path_item.get_value().as_ref() {
            dir.push(current.clone());
        }

        dir
    }
}
