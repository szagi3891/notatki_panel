use self::{git::Git, tabs::TabPath};

mod git;
mod tabs_hash;
mod tabs;
mod open_links;
mod calculate_next_path;

pub use git::{GitDirList, ViewDirList, ContentType, ContentView};
pub use git::ListItem;
pub use open_links::OpenLinks;
pub use tabs_hash::Router;

#[derive(Clone)]
pub struct Data {
    pub git: Git,
    pub tab: TabPath,
}

impl Data {
    pub fn new() -> Data {
        let git = Git::new();
        let tab = TabPath::new(&git);

        Data {
            git,
            tab,
        }
    }
}
