use std::{collections::HashMap, rc::Rc};

use vertigo::{Driver, Resource, Value, Computed};

use crate::app::index::ListItem;
use std::{cmp::Ordering};

use super::git::{StateDataGit, TreeItem};



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
pub struct TabPath {
    pub dir: Value<Vec<String>>,               //TODO - zrobić mozliwość 
    pub file: Value<Option<String>>,

    pub list_hash_map: Computed<Resource<Rc<HashMap<String, TreeItem>>>>,
    //aktualnie wyliczona lista
    pub list: Computed<Vec<ListItem>>,
}

impl TabPath {
    pub fn new(driver: &Driver, git: &StateDataGit) -> TabPath {
        let current_path_dir: Value<Vec<String>> = driver.new_value(Vec::new());
        let current_path_item: Value<Option<String>> = driver.new_value(None);

        let list_hash_map = create_list_hash_map(driver, git, &current_path_dir);
        let list = create_list(driver, &list_hash_map);

        TabPath {
            dir: current_path_dir.clone(),
            file: current_path_item,
            list_hash_map,
            list
        }
    }

    pub fn redirect_after_delete(&self) {
        let current_path_item = self.file.get_value();
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
                    self.file.set_value(Some(prev.name.clone()));
                    return;
                }
            }

            if let Some(prev) = list.get(current_index + 1) {
                self.file.set_value(Some(prev.name.clone()));
                return;
            }
        };
    }
}
