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
use vertigo::{AutoMap};

#[derive(Clone, PartialEq)]
pub struct AutoMapListItem {
    items: AutoMap<Rc<Vec<String>>, ListItem>,
}

impl AutoMapListItem {
    fn new(git: &Git) -> Self {
        let items = AutoMap::new({
            let git = git.clone();

            move |
                _auto_map: &AutoMap<Rc<Vec<String>>, ListItem>,
                full_path: &Rc<Vec<String>>,
            | -> ListItem {

                ListItem::new_full(git.clone(), full_path.clone())
            }
        });

        AutoMapListItem {
            items
        }
    }

    //TODO - Tylko w ten sposób mozna tworzyc nowe struktury ListItem

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

        let tab = TabPath::new(&git, &items);


        Data {
            git,
            tab,
            items
        }
    }
}

impl Default for Data {
    fn default() -> Self {
        Self::new()
    }
}
