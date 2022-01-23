mod git;
use std::{collections::HashMap, rc::Rc};

use vertigo::{Driver, Resource, Value, Computed};

use crate::app::index::ListItem;
use std::{cmp::Ordering};

use self::git::{StateDataGit, StateDataGitNodeDir, StateDataGitNodeContent, StateDataGitRoot, TreeItem};


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




fn create_list_hash_map(driver: &Driver, git: &StateDataGit, current_path: &Value<Vec<String>>) -> Computed<Resource<Rc<HashMap<String, TreeItem>>>> {
    let git = git.clone();
    let current_path = current_path.to_computed();

    driver.from(move || -> Resource<Rc<HashMap<String, TreeItem>>> {
        let current_path_rc = current_path.get_value();
        let current_path = current_path_rc.as_ref();

        git.dir_list(current_path)
    })
}

fn get_list_item_prirority(name: &String) -> u8 {
    if name.get(0..2) == Some("__") {
        return 0
    }

    if name.get(0..1) == Some("_") {
        return 2
    }

    1
}

fn create_list(driver: &Driver, list: &Computed<Resource<Rc<HashMap<String, TreeItem>>>>) -> Computed<Vec<ListItem>> {
    let list = list.clone();

    driver.from(move || -> Vec<ListItem> {
        let mut list_out: Vec<ListItem> = Vec::new();

        let result = list.get_value();

        match result.as_ref() {
            Resource::Ready(current_view) => {
                for (name, item) in current_view.as_ref() {
                    list_out.push(ListItem {
                        name: name.clone(),
                        dir: item.dir,
                        prirority: get_list_item_prirority(name),
                    });
                }

                list_out.sort_by(|a: &ListItem, b: &ListItem| -> Ordering {
                    let a_prirority = get_list_item_prirority(&a.name);
                    let b_prirority = get_list_item_prirority(&b.name);

                    if a_prirority == 2 && b_prirority == 2 {
                        if a.dir && !b.dir {
                            return Ordering::Less;
                        }

                        if !a.dir && b.dir {
                            return Ordering::Greater;
                        }
                    }

                    if a_prirority > b_prirority {
                        return Ordering::Less;
                    }

                    if a_prirority < b_prirority {
                        return Ordering::Greater;
                    }

                    a.name.to_lowercase().cmp(&b.name.to_lowercase())
                });

                list_out
            },
            Resource::Loading => {
                log::info!("Create list --> Loading");
                Vec::new()
            },
            Resource::Error(err) => {
                log::error!("Create list --> {:?}", err);
                Vec::new()
            }
        }
    })
}



#[derive(Clone, PartialEq)]
pub struct StateData {
    pub driver: Driver,
    pub git: StateDataGit,

    #[deprecated(note="please use `git.dir` instead")]
    pub dir: StateDataGitNodeDir,
    #[deprecated(note="please use `git.content` instead")]
    pub content: StateDataGitNodeContent,
    #[deprecated(note="please use `git.root` instead")]
    pub root: StateDataGitRoot,


    #[deprecated(note="Zrobić prywatne")]
    pub current_path_dir: Value<Vec<String>>,               //TODO - zrobić mozliwość 
    #[deprecated(note="Zrobić prywatne")]
    pub current_path_item: Value<Option<String>>,


    //Te dwie zmienne poniej trafią do stanu który będzie reprezentował taby ...

    pub list_hash_map: Computed<Resource<Rc<HashMap<String, TreeItem>>>>,
    //aktualnie wyliczona lista
    pub list: Computed<Vec<ListItem>>,
}

impl StateData {
    pub fn new(driver: &Driver) -> StateData {

        let git = StateDataGit::new(driver);


        let current_path_dir: Value<Vec<String>> = driver.new_value(Vec::new());
        let current_path_item: Value<Option<String>> = driver.new_value(None);

        let list_hash_map = create_list_hash_map(driver, &git, &current_path_dir);
        let list = create_list(driver, &list_hash_map);


        StateData {
            driver: driver.clone(),
            dir: git.dir.clone(),
            content: git.content.clone(),
            root: git.root.clone(),
            git,
            current_path_dir,
            current_path_item,
            list_hash_map,
            list
        }
    }

    pub fn redirect_to(&self, path: &Vec<String>) {
        let mut path = path.clone();
        let last = path.pop();

        self.current_path_dir.set_value(path);
        self.current_path_item.set_value(last);
    }

    pub fn current_path_dir(&self) -> Rc<Vec<String>> {
        self.current_path_dir.get_value()
    }

    pub fn redirect_after_delete(&self) {
        let current_path_item = self.current_path_item.get_value();
        let list = self.list.get_value();

        fn find_index(list: &Vec<ListItem>, value: &Option<String>) -> Option<usize> {
            if let Some(value) = value {
                for (index, item) in list.iter().enumerate() {
                    if item.name == *value {
                        return Some(index);
                    }
                }
            }
            None
        }

        if let Some(current_index) = find_index(list.as_ref(), current_path_item.as_ref()) {
            if current_index > 0 {
                if let Some(prev) = list.get(current_index - 1) {
                    self.current_path_item.set_value(Some(prev.name.clone()));
                    return;
                }
            }

            if let Some(prev) = list.get(current_index + 1) {
                self.current_path_item.set_value(Some(prev.name.clone()));
                return;
            }
        };
    }
}
