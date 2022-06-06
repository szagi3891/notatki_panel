use vertigo::{Resource, Value, Computed, get_driver};
use super::{
    git::{Git, ListItem},
    open_links::OpenLinks,
    calculate_next_path::calculate_next_path, ViewDirList, ContentType
};


fn create_list_hash_map(git: &Git, current_path: &Value<Vec<String>>) -> Computed<Resource<ViewDirList>> {
    let git = git.clone();
    let current_path = current_path.to_computed();

    Computed::from(move || -> Resource<ViewDirList> {
        let current_path_rc = current_path.get();
        let current_path = current_path_rc.as_ref();

        git.dir_list(current_path)
    })
}


fn create_list(list: &Computed<Resource<ViewDirList>>) -> Computed<Vec<ListItem>> {
    let list = list.clone();

    Computed::from(move || -> Vec<ListItem> {
        match list.get() {
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


fn create_current_item_view(
    current_item: &Value<Option<String>>,
    list: &Computed<Vec<ListItem>>
) -> Computed<Option<String>> {
    let current_item = current_item.clone();
    let list = list.clone();

    Computed::from(move || -> Option<String> {
        let current_item = current_item.get();

        if let Some(current_item) = current_item.as_ref() {
            return Some(current_item.clone());
        }

        let list = list.get();
        if let Some(first) = list.first() {
            return Some(first.name.clone());
        }

        None
    })
}

fn create_current_full_path(
    current_path_dir: &Value<Vec<String>>,
    list_current_item: &Computed<Option<String>>,
    item_hover: &Value<Option<String>>,
) -> Computed<Vec<String>> {
    let current_path_dir = current_path_dir.clone();
    let list_current_item = list_current_item.clone();
    let item_hover = item_hover.clone();

    Computed::from(move || -> Vec<String> {
        let mut current_path_dir = current_path_dir.get();

        if let Some(item_hover) = item_hover.get().as_ref() {
            current_path_dir.push(item_hover.clone());
        } else if let Some(list_current_item) = list_current_item.get().as_ref() {
            current_path_dir.push(list_current_item.clone());
        }

        current_path_dir
    })
}

fn create_current_content(
    state_data_git: &Git,
    full_path: &Computed<Vec<String>>,
) -> Computed<Resource<ContentType>> {

    let state_data_git = state_data_git.clone();
    let full_path = full_path.clone();

    Computed::from(move || -> Resource<ContentType> {
        let list_item = state_data_git.content_from_path(full_path.get().as_ref())?;

        list_item.get_content_type()
    })
}


#[derive(Clone)]
pub struct TabPath {
    /// Bazowy katalog który został wybrany
    pub dir_select: Value<Vec<String>>,

    /// Wybrany element z listy
    /// Ta zmienna nie powinna być bezpośrednio modyfikowana z zewnątrz
    item_select: Value<Option<String>>,

    ///Element nad którym znajduje się hover
    pub item_hover: Value<Option<String>>,

    /// Aktualnie wyliczona lista, która jest prezentowana w lewej kolumnie menu
    pub list: Computed<Vec<ListItem>>,


    /// Wybrany element z listy (dla widoku)
    /// Jeśli w zmiennej "item" znajduje się None, to brany jest pierwszy element z "list"
    pub current_item: Computed<Option<String>>,

    /// Suma "dir" + "current_item". Wskazuje na wybrany element do wyświetlenia w prawym panelu
    pub full_path: Computed<Vec<String>>,

    /// Aktualnie wyliczony wybrany content wskazywany przez full_path
    pub current_content: Computed<Resource<ContentType>>,

    //Otworzone zakładki z podględem do zewnętrznych linków
    pub open_links: OpenLinks,
}

impl TabPath {
    pub fn new(git: &Git) -> TabPath {
        let dir_select: Value<Vec<String>> = Value::new(Vec::new());
        let item_select: Value<Option<String>> = Value::new(None);
        let item_hover = Value::new(None);

        let dir_hash_map = create_list_hash_map(git, &dir_select);
        let list = create_list(&dir_hash_map);


        let current_item = create_current_item_view(&item_select, &list);

        let full_path = create_current_full_path(
            &dir_select,
            &current_item,
            &item_hover,
        );

        let current_content = create_current_content(
            git,
            &full_path,
        );

        let open_links = OpenLinks::new();

        TabPath {
            dir_select: dir_select.clone(),
            item_select,
            item_hover,
            list,
            current_item,
            full_path,
            current_content,
            open_links,
        }
    }

    pub fn redirect_after_delete(&self) {
        let current_path_item = self.item_select.get();
        let list = self.list.get();

        fn find_index(list: &Vec<ListItem>, value: Option<String>) -> Option<usize> {
            if let Some(value) = value {
                for (index, item) in list.iter().enumerate() {
                    if item.name == *value {
                        return Some(index);
                    }
                }
            }
            None
        }

        if let Some(current_index) = find_index(list.as_ref(), current_path_item) {
            if current_index > 0 {
                if let Some(prev) = list.get(current_index - 1) {
                    self.item_select.set(Some(prev.name.clone()));
                    return;
                }
            }

            if let Some(prev) = list.get(current_index + 1) {
                self.item_select.set(Some(prev.name.clone()));
                return;
            }
        };

        self.item_select.set(None);
    }

    pub fn redirect_to_item(&self, item: ListItem) {
        if item.is_dir {
            get_driver().transaction(|| {
                let mut path = item.get_base_dir();
                path.push(item.name.clone());

                self.dir_select.set(path);
                self.item_select.set(None);
            });
        } else {
            get_driver().transaction(|| {
                self.dir_select.set(item.get_base_dir());
                self.item_select.set(Some(item.name.clone()));
            });
        }
    }

    pub fn redirect_to(&self, dir: Vec<String>, item: Option<String>) {
        get_driver().transaction(move || {
            self.dir_select.set(dir);
            self.item_select.set(item);
        });
    }

    pub fn set_path(&self, path: Vec<String>) {
        let current_path = self.dir_select.get();

        if current_path == path.as_slice() {
            log::info!("path are equal");
            return;
        }
    
        let (new_current_path, new_current_item_value) = calculate_next_path(current_path.as_ref(), path);

        get_driver().transaction(||{
            self.dir_select.set(new_current_path);
            self.item_select.set(new_current_item_value);
        });
    }

    fn find(&self, item_finding: &String) -> Option<isize> {
        let list = self.list.get();

        for (index, item) in list.iter().enumerate() {
            if item.name == *item_finding {
                return Some(index as isize);
            }
        }

        None
    }


    fn try_set_pointer_to(&self, index: isize) -> bool {
        if index < 0 {
            return false;
        }

        let index = index as usize;

        let list = self.list.get();

        if let Some(first) = list.get(index) {
            self.item_select.set(Some(first.name.clone()));
            return true;
        }

        false
    }

    fn try_set_pointer_to_end(&self) {
        let len = self.list.get().len() as isize;
        self.try_set_pointer_to(len - 1);
    }

    pub fn pointer_up(&self) {
        let list_pointer_rc = self.current_item.get();

        if let Some(list_pointer) = list_pointer_rc.as_ref() {
            if let Some(index) = self.find(list_pointer) {
                if !self.try_set_pointer_to(index - 1) {
                    self.try_set_pointer_to_end();
                }
            }
        } else {
            self.try_set_pointer_to(0);
        }
    }

    pub fn pointer_down(&self) {
        let list_pointer_rc = self.current_item.get();

        if let Some(list_pointer) = list_pointer_rc.as_ref() {
            if let Some(index) = self.find(list_pointer) {
                if !self.try_set_pointer_to(index + 1) {
                    self.try_set_pointer_to(0);
                }
            }
        } else {
            self.try_set_pointer_to(0);
        }
    }

    pub fn pointer_escape(&self) {
        self.item_select.set(None);
    }

    pub fn pointer_enter(&self) {
        if let Some(current_item) = self.current_item.get().as_ref() {
            let content = self.current_content.get();

            if let Resource::Ready(content) = content {
                if let ContentType::Dir { .. } = content {
                    let mut current = self.dir_select.get();
                    current.push(current_item.clone());
                    self.set_path(current);
                } else {
                    // self.item_select.set(Some(current_item.clone()));
                }
            }
        }
    }

    pub fn backspace(&self) {
        let mut current_path = self.dir_select.get();

        current_path.pop();

        self.set_path(current_path);
    }

    pub fn hover_on(&self, name: &str) {
        self.item_hover.set(Some(name.to_string()));
    }

    pub fn hover_off(&self, name: &str) {
        let item_hover = self.item_hover.get();

        if let Some(item_hover) = item_hover.as_ref() {
            if item_hover == name {
                self.item_hover.set(None);
            }
        }
    }
}
