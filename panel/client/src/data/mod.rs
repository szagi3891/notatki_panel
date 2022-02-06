use vertigo::Driver;
use self::{git::StateDataGit, tabs::TabPath};

mod git;
mod tabs;
mod open_links;

pub use git::CurrentContent;
pub use tabs::ListItem;

#[derive(Clone)]
pub struct StateData {
    pub driver: Driver,
    pub git: StateDataGit,
    pub tab: TabPath,
}

impl StateData {
    pub fn new(driver: &Driver) -> StateData {
        let git = StateDataGit::new(driver);
        let tab = TabPath::new(driver, &git);

        StateData {
            driver: driver.clone(),
            git,
            tab,
        }
    }
}
