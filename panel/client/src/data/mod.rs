use std::rc::Rc;

use self::{git::Git, tabs::TabPath};

mod git;
mod tabs_hash;
mod tabs;
mod open_links;
mod calculate_next_path;

pub use git::{GitDirList, ViewDirList, ContentType, ContentView};
pub use git::{ListItem, ListItemType};
pub use open_links::OpenLinks;
pub use tabs_hash::Router;
use vertigo::AutoMap;

#[derive(Clone, PartialEq)]
pub struct Data {
    pub git: Git,
    pub tab: TabPath,
    items: AutoMap<Rc<Vec<String>>, ListItem>,

}

impl Data {
    pub fn new() -> Data {
        let git = Git::new();
        let tab = TabPath::new(&git);

        let items = AutoMap::new({
            let git = git.clone();

            move |
                auto_map: &AutoMap<Rc<Vec<String>>, ListItem>,
                full_path: &Rc<Vec<String>>,
            | -> ListItem {

                ListItem::new_full(git.clone(), full_path.clone())
            }
        });

        Data {
            git,
            tab,
            items,
        }
    }
}

impl Default for Data {
    fn default() -> Self {
        Self::new()
    }
}
