use vertigo::{Driver, Resource, Value, Computed};
use super::{git::{Git}, CurrentContent, open_links::OpenLinks, DirList};


fn create_list_hash_map(driver: &Driver, git: &Git, current_path: &Value<Vec<String>>) -> Computed<Resource<DirList>> {
    let git = git.clone();
    let current_path = current_path.to_computed();

    driver.from(move || -> Resource<DirList> {
        let current_path_rc = current_path.get_value();
        let current_path = current_path_rc.as_ref();

        git.dir_list(current_path)
    })
}


fn create_list(driver: &Driver, list: &Computed<Resource<DirList>>) -> Computed<Vec<ListItem>> {
    let list = list.clone();

    driver.from(move || -> Vec<ListItem> {
        match list.get_value().as_ref() {
            Resource::Ready(current_view) => {
                current_view.get_list()
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
    state_data_git: &Git,
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

    pub list_hash_map: Computed<Resource<DirList>>,
    //aktualnie wyliczona lista
    pub list: Computed<Vec<ListItem>>,



    //wybrany element z listy, dla widoku
    pub current_item: Computed<Option<String>>,

    pub full_path: Computed<Vec<String>>,

    //aktualnie wyliczony wybrany content wskazywany przez current_path
    pub current_content: Computed<CurrentContent>,

    //Otworzone zakładki z podględem do zewnętrznych linków
    pub open_links: OpenLinks,
}

impl TabPath {
    pub fn new(driver: &Driver, git: &Git) -> TabPath {
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
