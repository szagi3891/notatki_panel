use std::{collections::HashMap, rc::Rc};
use std::cmp::Ordering;

use vertigo::{Resource, Context, Computed, Value};

use super::{Content, Dir};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct TreeItem {
    pub dir: bool,
    pub id: String,
}


#[derive(PartialEq, Eq, Clone, Debug)]
pub struct GitDirList {
    list: Rc<HashMap<String, TreeItem>>,
}

impl GitDirList {
    pub fn new(list: Rc<HashMap<String, TreeItem>>) -> GitDirList {
        GitDirList {
            list
        }
    }

    pub fn get(&self, current_item: &String) -> Option<&TreeItem> {
        self.list.get(current_item)
    }
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


#[derive(Clone)]
pub struct ViewDirList {
    dir: Dir,
    content: Content,
    dir_path: Rc<Vec<String>>,
    list: Rc<HashMap<String, TreeItem>>,
}

impl PartialEq for ViewDirList {
    fn eq(&self, other: &Self) -> bool {
        self.dir_path == other.dir_path && self.list == other.list
    }
}

impl ViewDirList {
    pub fn new(dir: &Dir, content: &Content, base_dir: Rc<Vec<String>>, list: GitDirList) -> ViewDirList {
        ViewDirList {
            dir: dir.clone(),
            content: content.clone(),
            dir_path: base_dir,
            list: list.list,
        }
    }

    pub fn get_list(&self) -> Vec<ListItem> {
        let mut list_out: Vec<ListItem> = Vec::new();

        for (name, item) in self.list.as_ref() {
            list_out.push(ListItem::new(
                self.content.clone(),
                self.dir.clone(),
                self.dir_path.clone(),
                name.clone(),
                item.dir,
                item.id.clone(),
            ));
        }

        list_out.sort_by(|a: &ListItem, b: &ListItem| -> Ordering {
            let a_prirority = a.prirority_for_sort();
            let b_prirority = b.prirority_for_sort();

            if a_prirority > b_prirority {
                return Ordering::Less;
            }

            if a_prirority < b_prirority {
                return Ordering::Greater;
            }

            a.name.to_lowercase().cmp(&b.name.to_lowercase())
        });

        list_out
    }

    pub fn get(&self, current_item: &String) -> Option<&TreeItem> {
        self.list.get(current_item)
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn dir_path(&self) -> Rc<Vec<String>> {
        self.dir_path.clone()
    }
}


#[derive(Clone, PartialEq)]
pub enum ContentType {
    Dir {
        list: ViewDirList
    },
    Text {
        content: Rc<String>,
    },
    Image {
        url: Rc<String>,
    }
}

fn ordering_result(ordering: Ordering) -> Option<Ordering> {
    if ordering == Ordering::Equal {
        None
    } else {
        Some(ordering)
    }
}

#[derive(Clone)]
pub struct ListItem {
    content: Content,                   //TODO - scalić te dwa store
    dir: Dir,                           //TODO - scalić te dwa store

    //zmienne adresujące treść, one są stałę
    pub base_dir: Rc<Vec<String>>,
    pub name: String,
    pub is_dir: bool,
    
    //TODO - zamienić na computed, ta wartość moze sie zmienic
    pub id: Computed<String>,     //hash tego elementu
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
    pub fn new(content: Content, dir: Dir, base_dir: Rc<Vec<String>>, name: String, is_dir: bool, id: String) -> Self {
        Self {
            content,
            dir,

            base_dir,
            name,
            is_dir,

            //TODO - zorbić aby ten computed się wyliczał
            id: Value::new(id).to_computed(),
        }
    }

    
}

impl PartialEq for ListItem {
    fn eq(&self, other: &Self) -> bool {
        self.base_dir == other.base_dir && self.name == other.name && self.is_dir == other.is_dir
    }
}

impl Eq for ListItem {}

impl PartialOrd for ListItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for ListItem {
    fn cmp(&self, other: &Self) -> Ordering {

        if let Some(result) = ordering_result(self.base_dir.cmp(&other.base_dir)) {
            return result;
        }

        if let Some(result) = ordering_result(self.name.cmp(&other.name)) {
            return result;
        }

        self.is_dir.cmp(&other.is_dir)
    }
}

impl ListItem {
    pub fn get_ext(&self) -> Option<String> {
        get_ext(&self.name)
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

    pub fn prirority(&self) -> u8 {
        get_list_item_prirority(&self.name)
    }

    pub fn full_path(&self) -> Vec<String> {
        let mut result = self.base_dir.as_ref().clone();
        result.push(self.name.clone());
        result
    }

    pub fn get_content_type(&self, context: &Context) -> Resource<ContentType> {
        if self.is_dir {
            let id = self.id.get(context);

            let list = self.dir.get_list(context, &id)?;

            let mut full_path = self.base_dir.as_ref().clone();
            full_path.push(self.name.clone());

            let dir_list = ViewDirList::new(
                &self.dir,
                &self.content,
                Rc::new(full_path),
                list,
            );

            return Resource::Ready(ContentType::Dir {
                list: dir_list
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

        let id = self.id.get(context);

        let content = match file_type {
            FileType::Txt => {
                let content = self.content.get(context, &id)?;
                ContentType::Text { content }
            },
            FileType::Image { ext } => {
                let id = &id;
                let url = format!("/image/{id}/{ext}");
                ContentType::Image { url: Rc::new(url) }
            }
            FileType::Unknown => {
                let content = self.content.get(context, &id)?;
                ContentType::Text { content }
            }
        };

        Resource::Ready(content)
    }

    pub fn to_string(&self) -> String {
        self.full_path().join("/")
    }

    pub fn get_base_dir(&self) -> Vec<String> {
        (*(self.base_dir)).clone()
    }

    fn prirority_for_sort(&self) -> u8 {
        let mut prirority = 2 * get_list_item_prirority(&self.name);
        if self.is_dir {
            prirority += 1;
        }

        prirority
    }

    pub fn get_id(&self) -> (String, bool) {
        (self.name.clone(), self.is_dir)
    }
}


fn get_list_item_prirority(name: &String) -> u8 {
    if name.get(0..2) == Some("__") {
        return 4
    }

    if name.get(0..1) == Some("_") {
        return 2
    }

    1
}

