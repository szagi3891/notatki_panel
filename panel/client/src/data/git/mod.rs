use std::{rc::Rc, collections::HashMap};
use vertigo::{Resource, Context};
mod node_dir;
mod node_content;
mod root;
mod models;

use node_dir::Dir;
use node_content::Content;
pub use root::Root;

pub use models::{
    ContentType,
    TreeItem,
    ListItem,
    ListItemPath,
    ListItemType,
};

fn get_item_from_map<'a>(current_wsk: &'a Rc<HashMap<String, TreeItem>>, path_item: &String) -> Resource<&'a TreeItem> {
    let wsk_child = current_wsk.get(path_item);

    let wsk_child = match wsk_child {
        Some(wsk_child) => wsk_child,
        None => {
            return Resource::Error(format!("missing tree_item {path_item}"));
        }
    };

    Resource::Ready(wsk_child)
}

fn move_pointer(context: &Context, state_data: &Git, list: Rc<HashMap<String, TreeItem>>, path_item: &String) -> Resource<Rc<HashMap<String, TreeItem>>> {

    let child = get_item_from_map(&list, path_item)?;

    if child.dir {
        let child_list = state_data.dir.get_list(context, &child.id)?;

        return Resource::Ready(child_list);
    }

    return Resource::Error(format!("dir expected {path_item}"));
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContentView {
    pub id: String,
    pub content: Rc<String>,
}

#[derive(Clone, PartialEq)]
pub struct Git {
    dir: Dir,
    content: Content,
    pub root: Root,
}

impl Default for Git {
    fn default() -> Self {
        Self::new()
    }
}

impl Git {
    pub fn new() -> Git {
        let dir = Dir::new();
        let content = Content::new();
        let root = Root::new();

        Git {
            dir,
            content,
            root,
        }
    }

    pub fn dir_list(&self, context: &Context, path: &[String]) -> Resource<Rc<HashMap<String, TreeItem>>> {
        let root_wsk = self.root.get_current_root(context)?;

        let mut result = self.dir.get_list(context, &root_wsk)?;

        for path_item in path {
            result = move_pointer(context, self, result, path_item)?;
        }

        Resource::Ready(result)
    }

    pub fn get_item(&self, context: &Context, base_dir: &[String], current_item: &String) -> Resource<Option<TreeItem>> {
        let list = self.dir_list(context, base_dir)?;
        let current_value = list.get(current_item).cloned();
        Resource::Ready(current_value)
    }

    pub fn get_item_from_path(&self, context: &Context, path: &ListItemPath) -> Resource<Option<TreeItem>> {
        let mut path: Vec<String> = Vec::from(path.as_slice());
        let last = path.pop();

        let last = match last {
            Some(last) => last,
            None => {
                let id = self.root.get_current_root(context)?;

                return Resource::Ready(Some(TreeItem {
                    dir: true,
                    id
                }));
            }
        };

        self.get_item(context, path.as_slice(), &last)
    }

    pub fn get_list(&self, context: &Context, id: &String) -> Resource<Rc<HashMap<String, TreeItem>>> {
        self.dir.get_list(context, id)
    }

    pub fn get_content_string(&self, context: &Context, id: &String) -> Resource<Rc<String>> {
        self.content.get(context, id)
    }
}

