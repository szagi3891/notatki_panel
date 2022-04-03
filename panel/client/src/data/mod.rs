use vertigo::Driver;
use self::{git::Git, tabs::TabPath};

mod git;
mod tabs;
mod open_links;
mod calculate_next_path;

pub use git::{CurrentContent, DirList};
pub use tabs::ListItem;
pub use open_links::OpenLinks;

#[derive(Clone)]
pub struct Data {
    pub driver: Driver,
    pub git: Git,
    pub tab: TabPath,
}

impl Data {
    pub fn new(driver: &Driver) -> Data {
        let git = Git::new(driver);
        let tab = TabPath::new(driver, &git);

        Data {
            driver: driver.clone(),
            git,
            tab,
        }
    }
}
