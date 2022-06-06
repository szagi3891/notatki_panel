use std::{rc::Rc};
use vertigo::Resource;
mod node_dir;
mod node_content;
mod root;
mod models;

pub use node_dir::Dir;
pub use node_content::Content;
pub use root::Root;

pub use models::{
    ContentType,
    GitDirList,
    TreeItem,
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

#[derive(Debug)]
pub struct ContentView {
    pub id: String,
    pub content: Rc<String>,
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
        Resource::Ready(ViewDirList::new(&self.dir, &self.content, base_dir, result))
    }

    fn node_content(&self, base_dir: &[String], current_item: &String) -> Resource<ListItem> {
        let list = self.dir_list(base_dir)?;
        let current_value = list.get(current_item);

        if let Some(current_value) = current_value {
            let base_dir = Rc::new(Vec::from(base_dir));

            if current_value.dir {
                let dir = ListItem {
                    dir: self.dir.clone(),
                    content: self.content.clone(),
                    base_dir: base_dir.clone(),
                    name: current_item.clone(),
                    is_dir: true,
                    id: current_value.id.clone()
                };
                Resource::Ready(dir)
            } else {
                let file = ListItem {
                    dir: self.dir.clone(),
                    content: self.content.clone(),
                    base_dir,
                    name: current_item.clone(),
                    is_dir: false,
                    id: current_value.id.clone()
                };

                Resource::Ready(file)
            }
        } else {
            let message = format!("Brakuje {current_item} w {base_dir:?}");
            Resource::Error(message)
        }
    }

    pub fn content_from_path(&self, path: &[String]) -> Resource<ListItem> {
        let mut path: Vec<String> = Vec::from(path);
        let last = path.pop();

        let last = match last {
            Some(last) => last,
            None => {

                let id = self.root.get_current_root()?;

                let dir = ListItem {
                    dir: self.dir.clone(),
                    content: self.content.clone(),
                    base_dir: Rc::new(Vec::new()),
                    name: "root".into(),
                    is_dir: true,
                    id: id
                };

                return Resource::Ready(dir);
            }
        };

        self.node_content(path.as_slice(), &last)
    }

    pub fn get_content(&self, path: &[String]) -> Option<ContentView> {
        let result = self.content_from_path(path);

        if let Resource::Ready(item) = result {
            let content_type = item.get_content_type();

            if let Resource::Ready(ContentType::Text { content }) = content_type {
                // return Some(content.as_ref().clone());
                return Some(ContentView {
                    id: item.id.clone(),
                    content: content.clone(),
                })
            }
        }

        None
    }
}

