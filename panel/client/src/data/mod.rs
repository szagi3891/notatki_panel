use self::{git::Git, tabs::TabPath};

mod git;
mod tabs_hash;
mod tabs;
mod open_links;

pub use git::{ContentType, ContentView};
pub use git::{ListItem, ListItemPath, ListItemType};
pub use open_links::OpenLinks;
pub use tabs_hash::Router;
use vertigo::{AutoMap, Value};

#[derive(Clone, PartialEq)]
pub struct AutoMapListItem {
    git: Git,
    pub items: AutoMap<ListItemPath, ListItem>,
    pub todo_only: Value<bool>,
}

impl AutoMapListItem {
    fn new(git: &Git) -> Self {
        let todo_only = Value::new(false);

        let items = AutoMap::new({
            let git = git.clone();
            let todo_only = todo_only.clone();

            move |
                auto_map: &AutoMap<ListItemPath, ListItem>,
                full_path: &ListItemPath,
            | -> ListItem {
                ListItem::new_full(auto_map, git.clone(), full_path.clone(), todo_only.to_computed())
            }
        });

        AutoMapListItem {
            git: git.clone(),
            items,
            todo_only
        }
    }

    pub fn get_from_path(&self, path: &[String]) -> ListItem {
        let path = ListItemPath::new(path);

        self.items.get(&path)
    }

    pub fn root(&self) -> ListItem {
        self.get_from_path(&Vec::new())
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
}

impl Default for Data {
    fn default() -> Self {
        Self::new()
    }
}
