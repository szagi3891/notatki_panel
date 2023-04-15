use std::rc::Rc;

use vertigo::{Resource, Computed, Context, transaction, Value, bind, bind_rc};
use super::{
    git::ListItem,
    open_links::OpenLinks,
    calculate_next_path::calculate_next_path, ViewDirList, ContentType, tabs_hash::Router, ListItemType, AutoMapListItem
};

fn create_list_hash_map(items: &AutoMapListItem, router: &Router) -> Computed<Resource<ViewDirList>> {
    Computed::from(bind!(items, router, |context| -> Resource<ViewDirList> {
        let current_path_rc = router.get_dir(context);
        let current_path = current_path_rc.as_ref();

        items.dir_list(context, current_path)
    }))
}

fn create_list(todo_only: Value<bool>, list: &Computed<Resource<ViewDirList>>) -> Computed<Vec<ListItem>> {
    Computed::from(bind!(list, |context| -> Vec<ListItem> {
        match list.get(context) {
            Resource::Ready(current_view) => {
                let todo_only = todo_only.get(context);

                let list = current_view.get_list(context);

                list
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
    }))
}

fn create_current_item_view(
    router: &Router,
    list: &Computed<Vec<ListItem>>
) -> Computed<Option<String>> {
    Computed::from(bind!(router, list, |context| -> Option<String> {
        let current_item = router.get_item(context);

        if let Some(current_item) = current_item.as_ref() {
            return Some(current_item.clone());
        }

        let list = list.get(context);
        if let Some(first) = list.first() {
            return Some(first.name());
        }

        None
    }))
}

fn create_current_full_path(
    router: &Router,
    list_current_item: &Computed<Option<String>>,
) -> Computed<Vec<String>> {
    Computed::from(bind!(router, list_current_item, |context| -> Vec<String> {
        let mut current_path_dir = router.get_dir(context);

        if let Some(item_hover) = router.get_hover(context).as_ref() {
            current_path_dir.push(item_hover.clone());
        } else if let Some(list_current_item) = list_current_item.get(context).as_ref() {
            current_path_dir.push(list_current_item.clone());
        }

        current_path_dir
    }))
}


#[derive(Clone, PartialEq)]
pub struct TabPath {

    pub router: Router,

    pub todo_only: Value<bool>,

    /// Wybrany katalog
    pub select_dir: Computed<ListItem>,

    /// Wybrany element z listy (dla widoku)
    /// Jeśli w zmiennej "item" znajduje się None, to brany jest pierwszy element z "list"
    #[deprecated]
    pub current_item: Computed<Option<String>>,

    /// Suma "dir" + "current_item". Wskazuje na wybrany element do wyświetlenia w prawym panelu
    #[deprecated]
    pub full_path: Computed<Vec<String>>,

    /// Aktualnie wyliczony wybrany ListItem wskazywany przez full_path
    pub current_list_item: Computed<Option<ListItem>>,

    //Otworzone zakładki z podględem do zewnętrznych linków
    pub open_links: OpenLinks,
}

impl TabPath {
    pub fn new(items: &AutoMapListItem) -> TabPath {
        let router = Router::new();

        let todo_only = Value::new(false);

        let select_dir = Computed::from({
            let router = router.clone();
            let items = items.clone();

            move |context| {
                let dir = router.get_dir(context);
                items.get_from_path(&dir)
            }
        });

        let dir_hash_map = create_list_hash_map(items, &router);
        let list = create_list(todo_only.clone(), &dir_hash_map);


        let current_item = create_current_item_view(&router, &list);

        //TODO - full_path bezpośrednio wyliczać z routingu, nie jest do tego potrzebna zmienne current_item w poniszej regule

        let full_path = create_current_full_path(
            &router,
            &current_item,
        );

        let current_list_item = Computed::from({
            let select_dir = select_dir.clone();
            let router = router.clone();
            let items = items.clone();

            move |context| -> Option<ListItem> {

                let mut path = router.get_dir(context);

                let current_item = router.get_item(context);

                if let Some(current_item) = current_item.as_ref() {
                    path.push(current_item.clone());
                    return Some(items.get_from_path(&path));
                }
        
                let list = select_dir.get(context).list.get(context);

                if let Resource::Ready(list) = list {
                    if let Some(first) = list.first() {
                        let name = first.name();
                        path.push(name);
                        return Some(items.get_from_path(&path));
                    }
                }

                None
            }
        });

        let open_links = OpenLinks::new();

        //TODO - dodać opcję todo
        //list filtrowane w zalenosci od todo_only
        //kazdy z katalogow dociagal bedzie dodatkowa informacje o ilosci elementów w środku które posiadają todosu
        //przycisk w menu, będzie reagował na flagę todo_only

        //TODO - transaction - zminimalizować ilość tranzakcji, na rzecz renderowania przycisków

        TabPath {
            todo_only,
            router,
            select_dir,
            current_item,
            full_path,
            current_list_item,
            open_links,
        }
    }

    pub fn redirect_item_select_after_delete(&self) {
        transaction(|context| {
            let current_path_item = self.router.get_item(context);
            if let Resource::Ready(list) = self.select_dir.get(context).list.get(context) {
                fn find_index(list: &Vec<ListItem>, value: Option<String>) -> Option<usize> {
                    if let Some(value) = value {
                        for (index, item) in list.iter().enumerate() {
                            if item.name() == value {
                                return Some(index);
                            }
                        }
                    }
                    None
                }

                if let Some(current_index) = find_index(list.as_ref(), current_path_item) {
                    if current_index > 0 {
                        if let Some(prev) = list.get(current_index - 1) {
                            self.router.set_only_item(Some(prev.name()));
                            return;
                        }
                    }

                    if let Some(prev) = list.get(current_index + 1) {
                        self.router.set_only_item(Some(prev.name()));
                        return;
                    }
                };

                self.router.set_only_item(None);
            } else {
                log::error!("redirect_item_select_after_delete - ignore");
            }
        });
    }

    pub fn build_redirect_to_item(&self, item: ListItem) -> Computed<Rc<dyn Fn() + 'static>> {
        let self_clone = self;

        Computed::from(bind!(item, self_clone, |context| {
            match item.is_dir.get(context) {
                ListItemType::Dir => {
                    bind_rc!(item, self_clone, || {
                        let path = item.full_path.as_ref().clone();
                        self_clone.router.set(path, None);
                    })
                },
                ListItemType::File => {
                    bind_rc!(self_clone, item, || {
                        let mut path = item.full_path.as_ref().clone();
                        path.pop();

                        self_clone.router.set(path, Some(item.name()));
                    })
                },
                ListItemType::Unknown => {
                    bind_rc!(|| {
                        log::error!("redirect to the item, not ready");
                    })
                }
            }
        }))
    }

    pub fn redirect_to(&self, dir: Vec<String>, item: Option<String>) {
        self.router.set(dir, item);
    }

    pub fn set_path(&self, path: Vec<String>) {
        let current_path = transaction(|context| {
            self.router.get_dir(context)
        });

        if current_path == path.as_slice() {
            log::info!("path are equal");
            return;
        }
    
        let (new_current_path, new_current_item_value) = calculate_next_path(current_path.as_ref(), path);

        self.router.set(new_current_path, new_current_item_value);
    }

    fn find(&self, context: &Context, item_finding: &String) -> Option<isize> {
        if let Resource::Ready(list) = self.select_dir.get(context).list.get(context) {
            for (index, item) in list.iter().enumerate() {
                if item.name() == *item_finding {
                    return Some(index as isize);
                }
            }
        }

        None
    }


    fn try_set_pointer_to(&self, context: &Context, index: isize) -> bool {
        if index < 0 {
            return false;
        }

        let index = index as usize;

        if let Resource::Ready(list) = self.select_dir.get(context).list.get(context) {
            if let Some(first) = list.get(index) {
                self.router.set_only_item(Some(first.name()));
                return true;
            }
        }

        false
    }

    fn try_set_pointer_to_end(&self, context: &Context) {
        if let Resource::Ready(list) = self.select_dir.get(context).list.get(context) {
            let len = list.len() as isize;
            self.try_set_pointer_to(context, len - 1);
        } else {
            log::error!("try_set_pointer_to_end - ignore");
        }
    }

    pub fn pointer_up(&self) {
        transaction(|context| {
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
        });
    }

    pub fn pointer_down(&self) {
        transaction(|context| {
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
        });
    }

    pub fn pointer_escape(&self) {
        self.router.set_only_item(None);
    }

    pub fn pointer_enter(&self) {
        transaction(|context| {
            if let Some(current_item) = self.current_item.get(context).as_ref() {

                if let Some(current_item) = self.current_list_item.get(context) {
                    let content = current_item.get_content_type(context);

                    if let Resource::Ready(ContentType::Dir { .. }) = content {
                        let mut current = self.router.get_dir(context);
                        current.push(current_item.name());
                        self.set_path(current);
                    }    
                }
            }
        });
    }

    pub fn backspace(&self) {
        transaction(|context| {
            let mut current_path = self.router.get_dir(context);
            current_path.pop();
            self.set_path(current_path);
        });
    }
}
