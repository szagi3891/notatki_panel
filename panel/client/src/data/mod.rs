use std::{collections::HashMap, rc::Rc};

use vertigo::Driver;
use self::{git::{StateDataGit, TreeItem}, tabs::TabPath};

mod git;
mod tabs;

#[derive(PartialEq, Clone, Debug)]
pub enum CurrentContent {
    File {
        file_name: String,      //name
        file_hash: String,      //hash
        content: Rc<String>,    //content file
    },
    Dir {
        dir: String,            //hash
        dir_hash: String,
        list: Rc<HashMap<String, TreeItem>>,
    },
    None
}

impl CurrentContent {
    fn file(file_name: String, file_hash: String, content: Rc<String>) -> CurrentContent {
        CurrentContent::File {
            file_name,
            file_hash,
            content,
        }
    }

    fn dir(dir: String, dir_hash: String, list: Rc<HashMap<String, TreeItem>>) -> CurrentContent {
        CurrentContent::Dir {
            dir,
            dir_hash,
            list,
        }
    }

    pub fn to_string(&self) -> Option<Rc<String>> {
        if let CurrentContent::File { content, .. } = self {
            return Some(content.clone());
        }

        None
    }
}


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

    pub fn redirect_to(&self, path: &Vec<String>) {
        let mut path = path.clone();
        let last = path.pop();

        self.tab.dir.set_value(path);
        self.tab.file.set_value(last);
    }
}
