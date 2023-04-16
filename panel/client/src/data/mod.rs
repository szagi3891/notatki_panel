use std::rc::Rc;

use self::{git::Git, tabs::TabPath};

mod git;
mod tabs_hash;
mod tabs;
mod open_links;
mod calculate_next_path;

pub use git::{ContentType, ContentView};
pub use git::{ListItem, ListItemType};
pub use open_links::OpenLinks;
pub use tabs_hash::Router;
use vertigo::{AutoMap, Resource, Context};

#[derive(Clone, PartialEq)]
pub struct AutoMapListItem {
    git: Git,
    items: AutoMap<Rc<Vec<String>>, ListItem>,
}

impl AutoMapListItem {
    fn new(git: &Git) -> Self {
        let items = AutoMap::new({
            let git = git.clone();

            move |
                auto_map: &AutoMap<Rc<Vec<String>>, ListItem>,
                full_path: &Rc<Vec<String>>,
            | -> ListItem {

                ListItem::new_full(auto_map, git.clone(), full_path.clone())
            }
        });

        AutoMapListItem {
            git: git.clone(),
            items
        }
    }

    pub fn get_from_path(&self, path: &[String]) -> ListItem {
        let path = Rc::new(Vec::from(path));

        self.items.get(&path)
    }
}

#[derive(Clone, PartialEq)]
pub struct Data {
    pub git: Git,
    pub tab: TabPath,
    pub items: AutoMapListItem,

}

//TODO - zastanowić sie nad zamianą Rc<Vec<String>> na lzejszą strukturę

impl Data {
    pub fn new() -> Data {
        let git = Git::new();

        let items = AutoMapListItem::new(&git);

        let tab = TabPath::new(&items);


        Data {
            git,
            tab,
            items
        }
    }

    //TODO - docelowo scalić z ListItem

    pub fn get_content(&self, context: &Context, path: &[String]) -> Option<ContentView> {
        let item = self.items.get_from_path(path);

        let content_type = item.get_content_type(context);

        if let Resource::Ready(ContentType::Text { content }) = content_type {
            let Resource::Ready(id) = item.id.get(context) else {
                return None;
            };

            return Some(ContentView {
                id,
                content,
            })
        }

        None
    }
}

impl Default for Data {
    fn default() -> Self {
        Self::new()
    }
}
