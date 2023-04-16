use std::{rc::Rc};
use std::cmp::Ordering;

use vertigo::{Resource, Context, Computed, bind, AutoMap};

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

#[derive(Clone)]
pub struct ListItem {
    auto_map: AutoMap<Rc<Vec<String>>, ListItem>,
    git: Git,

    //TODO - tylko prywatne ? 
    #[deprecated]
    pub full_path: Rc<Vec<String>>,

    pub is_dir: Computed<ListItemType>,
    pub id: Computed<Resource<String>>,     //hash tego elementu
    pub list: Computed<Resource<Vec<ListItem>>>,
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
    pub fn new_full(auto_map: &AutoMap<Rc<Vec<String>>, ListItem>, git: Git, full_path: Rc<Vec<String>>) -> Self {
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
                return Resource::Error(format!("nie znaleziono id={}", full_path.join("/")));
            };

            Resource::Ready(content.id)
        }));

        let list = Computed::from({
            let auto_map = auto_map.clone();
            let git = git.clone();
            let dir_path = full_path.clone();

            move |context| -> Resource<Vec<ListItem>> {
                let list = git.dir_list(context, dir_path.as_ref())?;

                let mut list_out: Vec<ListItem> = Vec::new();

                for name in list.keys() {
                    let mut dir_path = dir_path.as_ref().clone();
                    dir_path.push(name.clone());

                    let item = auto_map.get(&Rc::new(dir_path));

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

                    a.name().to_lowercase().cmp(&b.name().to_lowercase())
                });

                Resource::Ready(list_out)
            }
        });

        Self {
            auto_map: auto_map.clone(),
            git,

            full_path,

            is_dir,
            id,
            list,
        }
    }

    //TODO - dodać jakiesz keszowanie na nazwę pliku ?

    pub fn dir(&self) -> Vec<String> {
        let mut full_path = self.full_path.as_ref().clone();
        full_path.pop();
        full_path
    }

    pub fn name(&self) -> String {
        // &self.name
        let mut full_path = self.full_path.as_ref().clone();
        let name = full_path.pop();

        let Some(name) = name else {
            return "root".into();
        };

        name
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

    pub fn prirority(&self) -> u8 {
        get_list_item_prirority(&self.name())
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
        self.full_path.join("/")
    }

    pub fn to_vec_path(&self) -> Vec<String> {
        self.full_path.as_ref().clone()
    }

    pub fn is_root(&self) -> bool {
        self.full_path.is_empty()
    }

    fn prirority_for_sort(&self, context: &Context) -> u8 {
        let mut prirority = 2 * get_list_item_prirority(&self.name());
        if self.is_dir.get(context) == ListItemType::Dir {
            prirority += 1;
        }

        prirority
    }

    fn get_from_path(&self, path: &[String]) -> ListItem {
        let path = Rc::new(Vec::from(path));

        self.auto_map.get(&path)
    }

    pub fn back(&self) -> ListItem {
        let mut full_path = self.full_path.as_ref().clone();
        full_path.pop();

        self.get_from_path(&full_path)
    }

    pub fn push(&self, name: impl Into<String>) -> ListItem {
        let mut full_path = self.full_path.as_ref().clone();
        full_path.push(name.into());

        self.get_from_path(&full_path)
    }
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


fn get_list_item_prirority(name: &String) -> u8 {
    if name.get(0..2) == Some("__") {
        return 4
    }

    if name.get(0..1) == Some("_") {
        return 2
    }

    1
}

