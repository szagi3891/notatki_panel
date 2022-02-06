use std::{collections::HashMap, rc::Rc};

use vertigo::{Driver, Resource, Value, Computed};
use std::cmp::Ordering;

use super::{git::{StateDataGit, TreeItem}, CurrentContent, open_links::OpenLinks};


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



#[derive(PartialEq, Debug, Clone)]
pub struct ListItem {
    pub name: String,
    pub dir: bool,
    pub prirority: u8,
}

fn create_current_item_view(
    driver: &Driver,
    current_item: &Value<Option<String>>,
    list: &Computed<Vec<ListItem>>
) -> Computed<Option<String>> {
    let current_item = current_item.clone();
    let list = list.clone();

    driver.from(move || -> Option<String> {
        let current_item = current_item.get_value();

        if let Some(current_item) = current_item.as_ref() {
            return Some(current_item.clone());
        }

        let list = list.get_value();
        if let Some(first) = list.first() {
            return Some(first.name.clone());
        }

        None
    })
}

fn create_current_full_path(
    driver: &Driver,
    current_path_dir: &Value<Vec<String>>,
    list_current_item: &Computed<Option<String>>,
) -> Computed<Vec<String>> {
    let current_path_dir = current_path_dir.clone();
    let list_current_item = list_current_item.clone();

    driver.from(move || -> Vec<String> {
        let mut current_path_dir = current_path_dir.get_value().as_ref().clone();

        if let Some(list_current_item) = list_current_item.get_value().as_ref() {
            current_path_dir.push(list_current_item.clone());
        }

        current_path_dir
    })
}

fn create_current_content(
    driver: &Driver,
    state_data_git: &StateDataGit,
    current_path_dir: &Value<Vec<String>>,
    list_current_item: &Computed<Option<String>>
) -> Computed<CurrentContent> {

    let state_data_git = state_data_git.clone();
    let current_path_dir = current_path_dir.to_computed();
    let list_current_item = list_current_item.clone();

    driver.from(move || -> CurrentContent {
        let current_path_dir = current_path_dir.get_value();
        let list_current_item = list_current_item.get_value();

        state_data_git.get_content(current_path_dir.as_ref(), list_current_item.as_ref())
    })
}



#[derive(Clone)]
pub struct TabPath {
    pub dir: Value<Vec<String>>,               //TODO - zrobić mozliwość 
    pub file: Value<Option<String>>,           //TODO - to mozna docelowo ukryć przed bezpośrednimi modyfikacjami zewnętrznymi

    pub list_hash_map: Computed<Resource<Rc<HashMap<String, TreeItem>>>>,
    //aktualnie wyliczona lista
    pub list: Computed<Vec<ListItem>>,



    //wybrany element z listy, dla widoku
    pub current_item: Computed<Option<String>>,

    pub full_path: Computed<Vec<String>>,

    //aktualnie wyliczony wybrany content wskazywany przez current_path
    pub current_content: Computed<CurrentContent>,


    // //Otworzone zakładki z podględem do zewnętrznych linków
    // pub tabs_url: Value<Vec<String>>,
    // pub tabs_active: Value<Option<String>>,

    pub open_links: OpenLinks,
}

impl TabPath {
    pub fn new(driver: &Driver, git: &StateDataGit) -> TabPath {
        let dir: Value<Vec<String>> = driver.new_value(Vec::new());
        let file: Value<Option<String>> = driver.new_value(None);

        let list_hash_map = create_list_hash_map(driver, git, &dir);
        let list = create_list(driver, &list_hash_map);


        let current_item = create_current_item_view(driver, &file, &list);

        let full_path = create_current_full_path(
            driver,
            &dir,
            &current_item,
        );
        let current_content = create_current_content(
            driver,
            git,
            &dir,
            &current_item,
        );

        // let tabs_url = driver.new_value(Vec::new());
        // let tabs_active = driver.new_value(None);
        let open_links = OpenLinks::new(driver);

        TabPath {
            dir: dir.clone(),
            file,
            list_hash_map,
            list,
            current_item,
            full_path,
            current_content,

            // tabs_url,
            // tabs_active,
            open_links,
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

    pub fn redirect_to_dir(&self, path: &Vec<String>) {
        self.dir.set_value(path.clone());
        self.file.set_value(None);
    }

    pub fn redirect_to_file(&self, path: &Vec<String>) {
        let mut path = path.clone();
        let last = path.pop();

        self.dir.set_value(path);
        self.file.set_value(last);
    }
}
