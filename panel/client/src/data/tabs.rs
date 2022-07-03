use vertigo::{Resource, Value, Computed, Context, transaction};
use super::{
    git::{Git, ListItem},
    open_links::OpenLinks,
    calculate_next_path::calculate_next_path, ViewDirList, ContentType, tabs_hash::Router
};


fn create_list_hash_map(git: &Git, router: &Router) -> Computed<Resource<ViewDirList>> {
    let git = git.clone();
    let router = router.clone();

    Computed::from(move |context| -> Resource<ViewDirList> {
        let current_path_rc = router.get_dir(context);
        let current_path = current_path_rc.as_ref();

        git.dir_list(context, current_path)
    })
}


fn create_list(list: &Computed<Resource<ViewDirList>>) -> Computed<Vec<ListItem>> {
    let list = list.clone();

    Computed::from(move |context| -> Vec<ListItem> {
        match list.get(context) {
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
    router: &Router,
    list: &Computed<Vec<ListItem>>
) -> Computed<Option<String>> {
    let router = router.clone();
    let list = list.clone();

    Computed::from(move |context| -> Option<String> {
        let current_item = router.get_item(context);

        if let Some(current_item) = current_item.as_ref() {
            return Some(current_item.clone());
        }

        let list = list.get(context);
        if let Some(first) = list.first() {
            return Some(first.name.clone());
        }

        None
    })
}

fn create_current_full_path(
    router: &Router,
    list_current_item: &Computed<Option<String>>,
    item_hover: &Value<Option<String>>,
) -> Computed<Vec<String>> {
    let router = router.clone();
    let list_current_item = list_current_item.clone();
    let item_hover = item_hover.clone();

    Computed::from(move |context| -> Vec<String> {
        let mut current_path_dir = router.get_dir(context);

        if let Some(item_hover) = item_hover.get(context).as_ref() {
            current_path_dir.push(item_hover.clone());
        } else if let Some(list_current_item) = list_current_item.get(context).as_ref() {
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

    Computed::from(move |context| -> Resource<ContentType> {
        let list_item = state_data_git.content_from_path(context, full_path.get(context).as_ref())?;

        list_item.get_content_type(context)
    })
}


#[derive(Clone, PartialEq)]
pub struct TabPath {

    /// Bazowy katalog który został wybrany
    // pub dir_select: Value<Vec<String>>,

    /// Wybrany element z listy
    /// Ta zmienna nie powinna być bezpośrednio modyfikowana z zewnątrz
    // item_select: Value<Option<String>>,

    pub router: Router,

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
        let router = Router::new();

        let item_hover = Value::new(None);

        let dir_hash_map = create_list_hash_map(git, &router);
        let list = create_list(&dir_hash_map);


        let current_item = create_current_item_view(&router, &list);

        let full_path = create_current_full_path(
            &router,
            &current_item,
            &item_hover,
        );

        let current_content = create_current_content(
            git,
            &full_path,
        );

        let open_links = OpenLinks::new();

        TabPath {
            router,
            // dir_select: dir_select.clone(),
            // item_select,
            item_hover,
            list,
            current_item,
            full_path,
            current_content,
            open_links,
        }
    }

    pub fn redirect_item_select_after_delete(&self, context: &Context) {
        let current_path_item = self.router.get_item(context);
        let list = self.list.get(context);

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
                    self.router.set_item(Some(prev.name.clone()), context);
                    return;
                }
            }

            if let Some(prev) = list.get(current_index + 1) {
                self.router.set_item(Some(prev.name.clone()), context);
                return;
            }
        };

        self.router.set_item(None, context);
    }

    pub fn redirect_to_item(&self, item: ListItem) {
        if item.is_dir {
            transaction(|context| {
                let mut path = item.get_base_dir();
                path.push(item.name.clone());

                self.router.set_dir(path, context);
                self.router.set_item(None, context);
            });
        } else {
            transaction(|context| {
                self.router.set_dir(item.get_base_dir(), context);
                self.router.set_item(Some(item.name.clone()), context);
            });
        }
    }

    pub fn redirect_to(&self, dir: Vec<String>, item: Option<String>) {
        transaction(move |context| {
            self.router.set_dir(dir, context);
            self.router.set_item(item, context);
        });
    }

    pub fn set_path(&self, context: &Context, path: Vec<String>) {
        let current_path = self.router.get_dir(context);

        if current_path == path.as_slice() {
            log::info!("path are equal");
            return;
        }
    
        let (new_current_path, new_current_item_value) = calculate_next_path(current_path.as_ref(), path);

        transaction(|context|{
            self.router.set_dir(new_current_path, context);
            self.router.set_item(new_current_item_value, context);
        });
    }

    fn find(&self, context: &Context, item_finding: &String) -> Option<isize> {
        let list = self.list.get(context);

        for (index, item) in list.iter().enumerate() {
            if item.name == *item_finding {
                return Some(index as isize);
            }
        }

        None
    }


    fn try_set_pointer_to(&self, context: &Context, index: isize) -> bool {
        if index < 0 {
            return false;
        }

        let index = index as usize;

        let list = self.list.get(context);

        if let Some(first) = list.get(index) {
            self.router.set_item(Some(first.name.clone()), context);
            return true;
        }

        false
    }

    fn try_set_pointer_to_end(&self, context: &Context) {
        let len = self.list.get(context).len() as isize;
        self.try_set_pointer_to(context, len - 1);
    }

    pub fn pointer_up(&self, context: &Context) {
        let list_pointer_rc = self.current_item.get(context);

        if let Some(list_pointer) = list_pointer_rc.as_ref() {
            if let Some(index) = self.find(context, list_pointer) {
                if !self.try_set_pointer_to(context, index - 1) {
                    self.try_set_pointer_to_end(context);
                }
            }
        } else {
            self.try_set_pointer_to(context, 0);
        }
    }

    pub fn pointer_down(&self, context: &Context) {
        let list_pointer_rc = self.current_item.get(context);

        if let Some(list_pointer) = list_pointer_rc.as_ref() {
            if let Some(index) = self.find(context, list_pointer) {
                if !self.try_set_pointer_to(context, index + 1) {
                    self.try_set_pointer_to(context, 0);
                }
            }
        } else {
            self.try_set_pointer_to(context, 0);
        }
    }

    pub fn pointer_escape(&self, context: &Context) {
        self.router.set_item(None, context);
    }

    pub fn pointer_enter(&self, context: &Context) {
        if let Some(current_item) = self.current_item.get(context).as_ref() {
            let content = self.current_content.get(context);

            if let Resource::Ready(ContentType::Dir { .. }) = content {
                let mut current = self.router.get_dir(context);
                current.push(current_item.clone());
                self.set_path(context, current);
            }
        }
    }

    pub fn backspace(&self, context: &Context) {
        let mut current_path = self.router.get_dir(context);
        current_path.pop();
        self.set_path(context, current_path);
    }

    pub fn hover_on(&self, name: &str) {
        self.item_hover.set(Some(name.to_string()));
    }

    pub fn hover_off(&self, context: &Context, name: &str) {
        let item_hover = self.item_hover.get(context);

        if let Some(item_hover) = item_hover.as_ref() {
            if item_hover == name {
                self.item_hover.set(None);
            }
        }
    }
}
