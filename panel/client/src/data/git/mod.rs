use std::{rc::Rc, sync::Arc};
use vertigo::{Resource, Context, AutoMap};
mod node_dir;
mod node_content;
mod root;
mod models;

use node_dir::Dir;
use node_content::Content;
pub use root::Root;

pub use models::{
    ContentType,
    GitDirList,
    TreeItem,
    ViewDirList,
    ListItem,
    ListItemType,
};

fn get_item_from_map<'a>(current_wsk: &'a GitDirList, path_item: &String) -> Resource<&'a TreeItem> {
    let wsk_child = current_wsk.get(path_item);

    let wsk_child = match wsk_child {
        Some(wsk_child) => wsk_child,
        None => {
            return Resource::Error(format!("missing tree_item {path_item}"));
        }
    };

    Resource::Ready(wsk_child)
}

fn move_pointer(context: &Context, state_data: &Git, list: GitDirList, path_item: &String) -> Resource<GitDirList> {

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

    pub fn dir_list(&self, context: &Context, path: &[String]) -> Resource<ViewDirList> {
        let root_wsk = self.root.get_current_root(context)?;

        let mut result = self.dir.get_list(context, &root_wsk)?;

        for path_item in path {
            result = move_pointer(context, self, result, path_item)?;
        }

        let base_dir = Rc::new(Vec::from(path));
        Resource::Ready(ViewDirList::new(self, base_dir, result))
    }

    pub fn get_item(&self, context: &Context, base_dir: &[String], current_item: &String) -> Resource<Option<TreeItem>> {
        let list = self.dir_list(context, base_dir)?;
        let current_value = list.get(current_item).cloned();
        Resource::Ready(current_value)
    }

    pub fn get_item_from_path(&self, context: &Context, path: &[String]) -> Resource<Option<TreeItem>> {
        let mut path: Vec<String> = Vec::from(path);
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

    // pub fn content_from_path2(&self, context: &Context, path: &[String]) -> Resource<ListItem> {
    //     let mut path: Vec<String> = Vec::from(path);
    //     let last = path.pop();

    //     let last = match last {
    //         Some(last) => last,
    //         None => {

    //             let id = self.root.get_current_root(context)?;

    //             let dir = ListItem::new(
    //                 self.clone(),
    //                 Rc::new(Vec::new()),
    //                 "root".into(),
    //                 true,
    //                 id
    //             );

    //             return Resource::Ready(dir);
    //         }
    //     };

    //     self.node_content2(context, path.as_slice(), &last)
    // }

    #[deprecated]
    fn node_content(&self, context: &Context, base_dir: &[String], current_item: &String) -> Resource<ListItem> {
        let list = self.dir_list(context, base_dir)?;
        let current_value = list.get(current_item);

        if let Some(current_value) = current_value {
            let base_dir = Rc::new(Vec::from(base_dir));

            Resource::Ready(ListItem::new(
                self.clone(),
                base_dir,
                current_item.clone(),
            ))
        } else {
            let message = format!("Brakuje {current_item} w {base_dir:?}");
            Resource::Error(message)
        }
    }

    #[deprecated]
    pub fn content_from_path(&self, context: &Context, path: &[String]) -> Resource<ListItem> {
        let mut path: Vec<String> = Vec::from(path);
        let last = path.pop();

        let last = match last {
            Some(last) => last,
            None => {
                let dir = ListItem::new(
                    self.clone(),
                    Rc::new(Vec::new()),
                    "root".into(),
                );

                return Resource::Ready(dir);
            }
        };

        self.node_content(context, path.as_slice(), &last)
    }

    pub fn get_content(&self, context: &Context, path: &[String]) -> Option<ContentView> {
        let result = self.content_from_path(context, path);

        if let Resource::Ready(item) = result {
            let content_type = item.get_content_type(context);

            if let Resource::Ready(ContentType::Text { content }) = content_type {
                // return Some(content.as_ref().clone());
                let Resource::Ready(id) = item.id.get(context) else {
                    return None;
                };

                return Some(ContentView {
                    id,
                    content,
                })
            }
        }

        None
    }

    pub fn get_list(&self, context: &Context, id: &String) -> Resource<GitDirList> {
        self.dir.get_list(context, id)
    }

    pub fn get_content_string(&self, context: &Context, id: &String) -> Resource<Rc<String>> {
        self.content.get(context, id)
    }
}

