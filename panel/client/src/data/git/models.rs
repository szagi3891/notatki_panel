use std::{rc::Rc};
use std::cmp::Ordering;

use vertigo::{Resource, Context, Computed, bind, AutoMap};

use crate::data::tabs_hash::RouterValue;

use super::{Git, ContentView};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct TreeItem {
    pub dir: bool,
    pub id: String,
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn get_ext(filename: &String) -> Option<String> {
    use std::path::Path;
    use std::ffi::OsStr;

    Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
        .map(|item| item.to_string())
}

#[test]
fn extract() {
    let name1 = String::from("aaaa.webp");
    let name2 = String::from("aaaa.txt");

    assert_eq!(get_ext(&name1), Some("webp".to_string()));
    assert_eq!(get_ext(&name2), Some("txt".to_string()));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq)]
pub enum ContentType {
    Dir {
        item: ListItem,
    },
    Text {
        content: Rc<String>,
    },
    Image {
        url: Rc<String>,
    }
}

#[derive(Clone, PartialEq)]
pub enum ListItemType {
    Dir,
    File,
    Unknown,
}


#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ListItemPath {
    path: Rc<Vec<String>>,
}

impl ListItemPath {
    pub fn new(path: impl Into<Vec<String>>) -> Self {
        Self {
            path: Rc::new(path.into())
        }
    }

    pub fn name(&self) -> String {
        let mut full_path = self.path.as_ref().clone();
        let name = full_path.pop();

        let Some(name) = name else {
            return "root".into();
        };

        name
    }

    pub fn as_slice(&self) -> &[String] {
        self.path.as_slice()
    }

    pub fn dir(&self) -> Self {
        let mut path = self.path.as_ref().clone();
        path.pop();
        Self::new(path)
    }


    pub fn push(&self, name: impl Into<String>) -> Self {
        let mut path = self.path.as_ref().clone();
        path.push(name.into());
        Self {
            path: Rc::new(path),
        }
    }

    pub fn to_string_path(&self) -> String {
        self.path.join("/")
    }

    pub fn to_vec_path(&self) -> Vec<String> {
        self.path.as_ref().clone()
    }

    pub fn is_root(&self) -> bool {
        self.path.is_empty()
    }
}


#[derive(Clone)]
pub struct ListItem {
    auto_map: AutoMap<ListItemPath, ListItem>,
    git: Git,

    full_path: ListItemPath,

    pub is_dir: Computed<ListItemType>,
    pub id: Computed<Resource<String>>,             //hash tego elementu
    pub list: Computed<Resource<Vec<ListItem>>>,
    pub todo_only: Computed<bool>,
    pub count_todo: Computed<u32>,                  //ilość elementów todo, które zawiera ten element
    pub redirect_view: Computed<RouterValue>,       //ten item, mona przekierować na ten stan za pomocą routera, aby go wyświetlić
}

/*
    TODO

    dorobic metodę, pobierz wszystie dzieci ...

    dorobic metodę, pobierz wszystkie dzieci które są TODOsami

    jesli to katalog, to transformacja do todosowego wpisu ma sie dziać z uwzględnieniem nazwy, w przypadku katalogu, dopisywana jest ilość dzieci todosowych


    aktualny wskaźnik, z wybranym drzewem do wyswietlenia, to moze być struktura ListItem
    a moze podswietlony element naliscie, to ten ListItem moze byc ... do zastanowienia


    To się moze udać, bo ListItem zawiera base_dir oraz name



    router zawiera dwie zmienne, dir oraz item (stanowia ścieke wybranego elementu)

    //wyliczać ListItem odpowiednie bazowe (katalog)
    //wyliczać ListItem aktualnie wyswietlane w podglądzie po prawej stronie

    mając strukturę ListItem, powinno się dać łatwo nawigować i rysować kolejne dane w głąb
*/

impl ListItem {
    pub fn new_full(auto_map: &AutoMap<ListItemPath, ListItem>, git: Git, full_path: ListItemPath, todo_only: Computed<bool>) -> Self {
        let is_dir = Computed::from(bind!(git, full_path, |context| -> ListItemType {
            let content = git.get_item_from_path(context, &full_path);
            
            let Resource::Ready(Some(content)) = content else {
                return ListItemType::Unknown;   
            };

            match content.dir {
                true => ListItemType::Dir,
                false => ListItemType::File
            }
        }));

        let id = Computed::from(bind!(git, full_path, |context| -> Resource<String> {
            let content = git.get_item_from_path(context, &full_path)?;
            
            let Some(content) = content else {
                return Resource::Error(format!("nie znaleziono id={}", full_path.to_string_path()));
            };

            Resource::Ready(content.id)
        }));

        let list = Computed::from({
            let todo_only = todo_only.clone();
            let auto_map = auto_map.clone();
            let git = git.clone();
            let dir_path = full_path.clone();

            move |context| -> Resource<Vec<ListItem>> {
                let list = git.dir_list(context, dir_path.as_slice())?;

                let mut list_out: Vec<ListItem> = Vec::new();

                for name in list.keys() {
                    let item = auto_map.get(&dir_path.push(name));
                    list_out.push(item);
                }

                list_out.sort_by(|a: &ListItem, b: &ListItem| -> Ordering {
                    let a_prirority = a.prirority_for_sort(context);
                    let b_prirority = b.prirority_for_sort(context);

                    if a_prirority > b_prirority {
                        return Ordering::Less;
                    }

                    if a_prirority < b_prirority {
                        return Ordering::Greater;
                    }

                    a.name_without_prefix().to_lowercase().cmp(&b.name_without_prefix().to_lowercase())
                });


                let todo_only = todo_only.get(context);

                if todo_only {
                    let mut result = Vec::new();

                    for item in list_out {
                        if item.count_todo.get(context) > 0 {
                            result.push(item);
                        }
                    }

                    return Resource::Ready(result);
                }

                Resource::Ready(list_out)
            }
        });


        let count_todo = Computed::from({
            let full_path = full_path.clone();
            let is_dir = is_dir.clone();
            let list = list.clone();

            move |context: &Context| -> u32 {
                let is_dir = is_dir.get(context);

                match is_dir {
                    ListItemType::File => {
                        if is_todo_name(&full_path.name()) {
                            1
                        } else {
                            0
                        }
                    },
                    ListItemType::Dir => {
                        let mut count = 0;

                        if let Resource::Ready(list) = list.get(context) {
                            for item in list {
                                let item_count = item.count_todo.get(context);
                                count += item_count;
                            }
                        }

                        count
                    },
                    ListItemType::Unknown => {
                        0
                    }
                }
            }
        });

        let redirect_view = Computed::from({
            let full_path = full_path.clone();
            let is_dir = is_dir.clone();

            move |context| {
                match is_dir.get(context) {
                    ListItemType::File => {
                        RouterValue::new(full_path.dir().to_vec_path(), Some(full_path.name()))
                    },
                    ListItemType::Dir => {
                        RouterValue::new(full_path.to_vec_path(), None)
                    },
                    ListItemType::Unknown => {
                        RouterValue::new(full_path.to_vec_path(), None)
                    }
                }
            }
        });

        Self {
            auto_map: auto_map.clone(),
            git,

            full_path,

            is_dir,
            id,
            list,
            count_todo,
            todo_only,
            redirect_view,
        }
    }

    //TODO - dodać jakiesz keszowanie na nazwę pliku ?

    pub fn dir(&self) -> ListItem {
        self.get_from_path(&self.full_path.dir())
    }

    pub fn name(&self) -> String {
        self.full_path.name()
    }

    pub fn get_ext(&self) -> Option<String> {
        get_ext(&self.name())
    }

    pub fn get_picture_ext(&self) -> Option<String> {
        let ext = self.get_ext();

        if let Some(ext) = ext {
            let ext_str = ext.as_str();

            if ext_str == "webp" || ext_str == "jpg" || ext_str == "jpeg" || ext_str == "png" {
                return Some(ext);
            }
        }

        None
    }

    pub fn prirority(&self) -> bool {
        let name = self.name();
        name.get(0..1) == Some("_")
    }

    //TODO - zrobić z tego computed

    pub fn get_content_type(&self, context: &Context) -> Resource<ContentType> {
        let is_dir = self.is_dir.get(context);

        if is_dir == ListItemType::Dir {
            return Resource::Ready(ContentType::Dir {
                item: self.clone(),
            });
        }

        let ext = self.get_ext();

        enum FileType {
            Txt,
            Image {
                ext: String,
            },
            Unknown,
        }

        let file_type = match ext {
            Some(ext) => {
                match ext.as_str() {
                    "txt" => FileType::Txt,
                    "todo" => FileType::Txt,
                    "webp" => FileType::Image { ext: "webp".into() },
                    "jpg" => FileType::Image { ext: "jpg".into() },
                    "jpeg" => FileType::Image { ext: "jpeg".into() },
                    "png" => FileType::Image { ext: "png".into() },
                    "key" => FileType::Txt,
                    _ => {
                        log::warn!("Nierozpoznany typ pliku: {ext}");
                        FileType::Unknown
                    }
                }
            },
            None => FileType::Txt,
        };

        let id = self.id.get(context)?;

        let content = match file_type {
            FileType::Txt => {
                let content = self.git.get_content_string(context, &id)?;
                ContentType::Text { content }
            },
            FileType::Image { ext } => {
                let id = &id;
                let url = format!("/image/{id}/{ext}");
                ContentType::Image { url: Rc::new(url) }
            }
            FileType::Unknown => {
                let content = self.git.get_content_string(context, &id)?;
                ContentType::Text { content }
            }
        };

        Resource::Ready(content)
    }


    pub fn get_content(&self, context: &Context) -> Option<ContentView> {
        let content_type = self.get_content_type(context);

        if let Resource::Ready(ContentType::Text { content }) = content_type {
            let Resource::Ready(id) = self.id.get(context) else {
                return None;
            };

            return Some(ContentView {
                id,
                content,
            })
        }

        None
    }

    pub fn to_string_path(&self) -> String {
        self.full_path.to_string_path()
    }

    pub fn to_vec_path(&self) -> Vec<String> {
        self.full_path.to_vec_path()
    }

    pub fn is_root(&self) -> bool {
        self.full_path.is_root()
    }

    pub fn is_todo(&self) -> bool {
        is_todo_name(&self.name())
    }

    fn prirority_for_sort(&self, context: &Context) -> u8 {
        let mut prirority = 0;

        if self.is_todo() {
            prirority += 4;
        }

        if self.prirority() {
            prirority += 2;
        }

        if self.is_dir.get(context) == ListItemType::Dir {
            prirority += 1;
        }

        prirority
    }

    fn get_from_path(&self, path: &ListItemPath) -> ListItem {
        self.auto_map.get(path)
    }

    pub fn push(&self, name: impl Into<String>) -> ListItem {
        self.get_from_path(&self.full_path.push(name))
    }

    pub fn name_without_prefix(&self) -> String {
        remove_prefix(&self.name())
    }

    pub fn get_all_items(&self) -> Vec<ListItem> {
        let mut path_items_all = Vec::new();

        let mut wsk = self.clone();

        loop {
            path_items_all.push(wsk.clone());

            let prev = wsk.dir();

            if prev == wsk {
                break;
            }

            wsk = prev;
        }

        path_items_all.reverse();
        path_items_all
    }

    //RouterValue
}


impl PartialEq for ListItem {
    fn eq(&self, other: &Self) -> bool {
        self.full_path == other.full_path
    }
}

impl Eq for ListItem {}

impl PartialOrd for ListItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ListItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.full_path.cmp(&other.full_path)
    }
}


fn remove_first(chars: &[char]) -> &[char] {
    if let Some((name_item, rest_path)) = chars.split_first() {
        if *name_item == '_' {
            return rest_path;
        }
    }

    chars
}


fn remove_prefix(name: &String) -> String {
    let chars = name.chars().collect::<Vec<char>>();

    let chars = remove_first(&chars);
    let chars = remove_first(chars);

    let mut out: String = String::new();

    for char in chars {
        out.push(*char);
    }

    out
}

fn is_todo_name(name: &str) -> bool {
    let aa = name.chars().rev().take(5).collect::<String>();
    aa == "odot."
}

#[test]
fn test_is_todo_name() {
    assert_eq!(is_todo_name("dsada.todo"), true);
    assert_eq!(is_todo_name("dsadasd.todo"), true);
    assert_eq!(is_todo_name("dsadatodo"), false);
    assert_eq!(is_todo_name("as"), false);
    assert_eq!(is_todo_name("todo"), false);
    assert_eq!(is_todo_name("o"), false);
    assert_eq!(is_todo_name(""), false);
}