use std::rc::Rc;

use vertigo::Resource;

mod node_dir;
mod node_content;
mod root;
mod models;

pub use node_dir::Dir;
pub use node_content::Content;
pub use root::Root;

pub use models::{
    GitDirList,
    TreeItem,
    CurrentContent,
    ViewDirList,
    ListItem
};



fn get_item_from_map<'a>(current_wsk: &'a GitDirList, path_item: &String) -> Resource<&'a TreeItem> {
    let wsk_child = current_wsk.get(path_item);

    let wsk_child = match wsk_child {
        Some(wsk_child) => wsk_child,
        None => {
            return Resource::Error(format!("missing tree_item {}", path_item));
        }
    };

    Resource::Ready(wsk_child)
}

fn move_pointer(state_data: &Git, list: GitDirList, path_item: &String) -> Resource<GitDirList> {

    let child = get_item_from_map(&list, path_item)?;

    if child.dir {
        let child_list = state_data.dir.get_list(&child.id)?;

        return Resource::Ready(child_list);
    }

    return Resource::Error(format!("dir expected {}", path_item));
}


#[derive(Clone)]
pub struct Git {
    pub dir: Dir,
    pub content: Content,
    pub root: Root
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

    pub fn dir_list(&self, path: &[String]) -> Resource<ViewDirList> {
        let root_wsk = self.root.get_current_root()?;

        let mut result = self.dir.get_list(&root_wsk)?;

        for path_item in path {
            result = move_pointer(self, result, path_item)?;
        }

        let base_dir = Rc::new(Vec::from(path));
        Resource::Ready(ViewDirList::new(base_dir, result))
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
            let base_dir = Rc::new(Vec::from(base_dir));

            if current_value.dir {
                let dir = ListItem {
                    base_dir: base_dir.clone(),
                    name: current_item.clone(),
                    dir: true,
                    id: current_value.id.clone()
                };
                let list = self.dir.get_list(&current_value.id)?;
                let dir_list_view = ViewDirList::new(base_dir, list);

                return Resource::Ready(CurrentContent::dir(dir, dir_list_view));
            } else {
                let file = ListItem {
                    base_dir,
                    name: current_item.clone(),
                    dir: false,
                    id: current_value.id.clone()
                };
                let content = self.content.get(&current_value.id)?;
                return Resource::Ready(CurrentContent::file(file, content));
            }
        }

        Resource::Ready(CurrentContent::None)
    }

    fn get_content(&self, base_dir: &[String], item: &Option<String>) -> CurrentContent {

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
            CurrentContent::File { file, .. } => {
                Some(file.id)
            },
            CurrentContent::Dir { dir, .. } => {
                Some(dir.id)
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

