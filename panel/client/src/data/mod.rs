use vertigo::Driver;
use self::{git::StateDataGit, tabs::TabPath};

mod git;
mod tabs;

pub use git::CurrentContent;

#[derive(Clone, PartialEq)]
pub struct StateData {
    pub driver: Driver,
    pub git: StateDataGit,
    pub tab: TabPath,

    // #[deprecated(note="Zrobić prywatne")]
    // pub current_path_dir: Value<Vec<String>>,               //TODO - zrobić mozliwość 
    // #[deprecated(note="Zrobić prywatne")]
    // pub current_path_item: Value<Option<String>>,
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
