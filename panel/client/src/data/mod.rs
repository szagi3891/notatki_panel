use self::{git::Git, tabs::TabPath};

mod git;
mod tabs;
mod open_links;
mod calculate_next_path;

pub use git::{GitDirList, ViewDirList, ContentType, ContentView};
pub use git::ListItem;
pub use open_links::OpenLinks;

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
