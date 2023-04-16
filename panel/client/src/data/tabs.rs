use std::rc::Rc;

use vertigo::{Resource, Computed, Context, transaction, bind, bind_rc};
use super::{
    git::ListItem,
    open_links::OpenLinks,
    ContentType, tabs_hash::Router, ListItemType, AutoMapListItem, ListItemPath
};

#[derive(Clone, PartialEq)]
pub struct TabPath {

    router: Router,
    pub items: AutoMapListItem,

    /// Wybrany katalog
    pub select_dir: Computed<ListItem>,

    /// Częściowe stany składające się na select_content
    pub select_content_hover: Computed<Option<ListItem>>,
    pub select_content_current: Computed<Option<ListItem>>,

    /// Aktualnie wyliczony wybrany ListItem wskazywany przez full_path
    pub select_content: Computed<Option<ListItem>>,

    //Otworzone zakładki z podględem do zewnętrznych linków
    pub open_links: OpenLinks,
}

impl TabPath {
    pub fn new(items: &AutoMapListItem) -> TabPath {
        let router = Router::new();

        let select_dir = Computed::from({
            let router = router.clone();
            let items = items.clone();

            move |context| {
                let dir = router.get_dir(context);
                items.get_from_path(&dir)
            }
        });

        let select_content_hover = Computed::from({
            let items = items.clone();
            let router = router.clone();

            move |context| {
                let path = ListItemPath::new(router.get_dir(context));

                if let Some(hover) = router.get_hover(context) {
                    let curret_path = path.push(hover);
                    return Some(items.items.get(&curret_path));
                }

                None
            }
        });

        let select_content_current = Computed::from({
            let select_dir = select_dir.clone();
            let items = items.clone();
            let router = router.clone();

            move |context| {

                let current_item = router.get_item(context);

                if let Some(current_item) = current_item {
                    let path = ListItemPath::new(router.get_dir(context));
                    let curret_path = path.push(current_item);
                    return Some(items.items.get(&curret_path));
                }

                let list = select_dir.get(context).list.get(context);
                if let Resource::Ready(list) = list {
                    if let Some(first) = list.first() {
                        return Some(first.clone());
                    }
                }

                None
            }
        });

        let select_content = Computed::from({
            let select_content_hover = select_content_hover.clone();
            let select_content_current = select_content_current.clone();

            move |context| {
                let select_content_hover = select_content_hover.get(context);
                if select_content_hover.is_some() {
                    return select_content_hover;
                }

                select_content_current.get(context)
            }
        });

        let open_links = OpenLinks::new();

        //TODO - dodać opcję todo
        //list filtrowane w zalenosci od todo_only
        //kazdy z katalogow dociagal bedzie dodatkowa informacje o ilosci elementów w środku które posiadają todosu
        //przycisk w menu, będzie reagował na flagę todo_only

        //TODO - transaction - zminimalizować ilość tranzakcji, na rzecz renderowania przycisków

        //TODO = kolor dla wciśnietego przycisku todo background-color: #00ff00b0;
    
        TabPath {
            router,
            items: items.clone(),
            select_dir,

            select_content_hover,
            select_content_current,

            select_content,
            open_links,
        }
    }

    pub fn current_item(&self, context: &Context) -> Option<String> {
        self.router.get_item(context)
    }

    pub fn hover_on(&self, name: &str) {
        self.router.hover_on(name);
    }

    pub fn hover_off(&self, name: &str) {
        self.router.hover_off(name);
    }

    pub fn get_hover(&self, context: &Context) -> Option<String> {
        self.router.item_hover.get(context)
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
                        self_clone.router.set(item.clone(), None);
                    })
                },
                ListItemType::File => {
                    bind_rc!(self_clone, item, || {
                        self_clone.router.set(item.dir(), Some(item.name()));
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

    pub fn redirect_to(&self, dir: ListItem, item: Option<String>) {
        self.router.set(dir, item);
    }

    pub fn set_path(&self, path: ListItem) {
        self.router.set(path, None);
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
            if let Some(current_item) = self.select_content.get(context) {
                if let Some(index) = self.find(context, &current_item.name()) {
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
            if let Some(current_item) = self.select_content.get(context) {
                if let Some(index) = self.find(context, &current_item.name()) {
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
            if let Some(current_item) = self.select_content.get(context) {
                let content = current_item.get_content_type(context);

                if let Resource::Ready(ContentType::Dir { item }) = content {
                    self.router.set(item, None);
                }    
            }
        });
    }

    pub fn backspace(&self) {
        transaction(|context| {
            self.router.set(self.select_dir.get(context).dir(), None);
        });
    }
}
