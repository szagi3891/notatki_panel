use std::{cmp::Ordering, collections::HashMap};
use std::rc::Rc;
use vertigo::{
    DomDriver,
    computed::{
        Computed,
        Dependencies,
        Value
    }
};
use crate::request::{Request, Resource, ResourceError};
use super::{StateNodeDir, StateRoot, TreeItem};

#[derive(PartialEq)]
pub struct CurrentView {
    list: Rc<HashMap<String, TreeItem>>,
    content: Option<String>,        //wskaźnik na content który jest plikiem jakimś
}

fn get_item_from_map<'a>(current_wsk: &'a Rc<HashMap<String, TreeItem>>, path_item: &String) -> Result<&'a TreeItem, ResourceError> {
    let wsk_child = current_wsk.get(path_item);

    let wsk_child = match wsk_child {
        Some(wsk_child) => wsk_child,
        None => {
            return Err(ResourceError::Error(format!("missing tree_item {}", path_item)));
        }
    };

    Ok(wsk_child)
}

fn move_pointer(state_node_dir: &StateNodeDir, current_wsk: &Rc<HashMap<String, TreeItem>>, path_item: &String) -> Result<Rc<HashMap<String, TreeItem>>, ResourceError> {

    let wsk_child = get_item_from_map(current_wsk, path_item)?;

    if !wsk_child.dir {
        return Err(ResourceError::Error(format!("Dir expected {}", path_item)));
    }

    let item = state_node_dir.get_list(&wsk_child.id)?;

    Ok(item)
}

fn create_current_view(
    root: &Dependencies,
    current_path: Computed<Vec<String>>,
    state_root: StateRoot,
    state_node_dir: StateNodeDir
) -> Computed<Resource<CurrentView>> {

    root.from(move || -> Resource<CurrentView> {
        let root_wsk = state_root.get_current_root()?;

        let mut current_wsk = state_node_dir.get_list(&root_wsk)?;

        let current_path = current_path.get_value();
        let mut current_path = (*current_path).clone();
        let content_name = current_path.pop();

        let content_name = match content_name {
            Some(content_name) => content_name,
            None => {
                return Ok(CurrentView {
                    list: current_wsk,
                    content: None,
                });
            }
        };

        for path_item in current_path {
            current_wsk = move_pointer(&state_node_dir, &current_wsk, &path_item)?;
        }

        let content_pointer = get_item_from_map(&current_wsk, &content_name)?;

        if content_pointer.dir {
            let current_wsk = move_pointer(&state_node_dir, &current_wsk, &content_pointer.id)?;

            return Ok(CurrentView {
                list: current_wsk,
                content: None,
            });
        }

        let content_id = content_pointer.id.clone();

        return Ok(CurrentView {
            list: current_wsk,
            content: Some(content_id),
        });
    })
}

#[derive(PartialEq, Debug)]
pub struct ListItem {
    pub name: String,
    pub dir: bool,
}

fn create_list(root: &Dependencies, current_view: &Computed<Resource<CurrentView>>) -> Computed<Vec<ListItem>> {
    let current_view = current_view.clone();

    root.from(move || -> Vec<ListItem> {
        let mut list_dir: Vec<ListItem> = Vec::new();
        let mut list_file: Vec<ListItem> = Vec::new();

        let current_view = current_view.get_value();

        match &*current_view {
            Ok(current_view) => {
                for (name, item) in &*current_view.list {
                    if item.dir {
                        list_dir.push(ListItem {
                            name: name.clone(),
                            dir: true,
                        });
                    } else {
                        list_file.push(ListItem {
                            name: name.clone(),
                            dir: false,
                        });
                    }
                }

                list_dir.sort_by(|a: &ListItem, b: &ListItem| -> Ordering {
                    a.name.cmp(&b.name)
                });

                list_file.sort_by(|a: &ListItem, b: &ListItem| -> Ordering {
                    a.name.cmp(&b.name)
                });

                list_dir.extend(list_file);

                return list_dir;
            },
            Err(_) => {
                return Vec::new();
            }
        };
    })
}

#[derive(PartialEq)]
pub struct State {
    pub current_path: Value<Vec<String>>,
    pub current_view: Computed<Resource<CurrentView>>,                  //lista plików i katalogów w lewym panelu
    pub list: Computed<Vec<ListItem>>,
}


impl State {
    pub fn new(root: &Dependencies, driver: &DomDriver) -> Computed<State> {
        let request = Request::new(driver);

        let state_node_dir = StateNodeDir::new(&request, root);
        let state_root = StateRoot::new(&request, root, state_node_dir.clone());

        let current_path = root.new_value(Vec::<String>::new());

        let current_view = create_current_view(
            root,
            current_path.to_computed(),
            state_root,
            state_node_dir,
        );

        let list = create_list(root, &current_view);

        root.new_computed_from(State {
            current_path,
            current_view,
            list,
        })
    }

    // pub fn node_title(&self, node_id: &DataNodeIdType) -> Option<Rc<String>> {
    //     let item = self.data.get_value(node_id);
    //     let value = item.get_value();

    //     if let Resource::Ready(value_resource) = &*value {
    //         return Some(value_resource.title());
    //     }

    //     None
    // }

    // pub fn set_path(&self, node_id: DataNodeIdType) {
    //     let mut current = (*self.current_path.get_value()).clone();
    //     current.push(node_id);
    //     self.current_path.set_value(current);
    // }

    // pub fn create_dir(&self, name: String) {
    //     let node = self.current_node.get_value();
    //     let item = self.data.get_value(&*node).get_value();
        
    //     self.driver.spawn_local(async move {
    //         if let Resource::Ready(item) = &*item {
    //             item.create_dir(name).await;
    //         } else {
    //             log::error!("Błąd przy tworzeniu katalogu");
    //         }
    //     });
    // }
}

/*
dodać zakładki
    zakładka1: notatki
    zakładka2: parsowanie urla ...

    zakładka - zarządzanie gitem
        menu pozwalające usuwać niepotrzebne gałęzie gita ...
    
        pozwoli np. ta funkcja na uruchomienie polecenia rebejsującego
*/



        // let current_path = root.new_value(Vec::<DataNodeIdType>::new());

        // let current_node = {
        //     let current_path = current_path.to_computed();
        //     root.from::<DataNodeIdType, _>(move || -> DataNodeIdType {
        //         let current_path = current_path.get_value();
        //         let inner = &*current_path;
        //         let last = inner.last();

        //         if let Some(last) = last {
        //             return *last;
        //         }

        //         return ROOT_ID;
        //     })
        // };

        // let list = {
        //     let current_path = current_path.to_computed();
        //     let data = data.clone();

        //     let get_child = move |node: &u64| -> Resource<Option<Vec<DataNodeIdType>>> {
        //         let current_item = data.get_value(node).get_value();

        //         match current_item.as_ref() {
        //             Resource::Loading => {
        //                 return Resource::Loading;
        //             },
        //             Resource::Ready(data) => {
        //                 match data.child() {
        //                     Some(data) => Resource::Ready(Some(data)),
        //                     None => Resource::Ready(None),
        //                 }
        //             },
        //             Resource::Failed(mess) => {
        //                 return Resource::Failed(mess.clone());
        //             }
        //         }
        //     };

        //     root.from::<Resource<Vec<DataNodeIdType>>, _>(move || -> Resource<Vec<DataNodeIdType>> {
        //         let current_path = current_path.get_value();

        //         if current_path.len() == 0 {
        //             return get_child(&ROOT_ID).map(|item| {
        //                 match item {
        //                     Some(list) => list,
        //                     None => Vec::new()
        //                 }
        //             });
        //         }

        //         for item in current_path.iter().rev() {
        //             let child = get_child(item);

        //             match child {
        //                 Resource::Loading => {
        //                     return Resource::Loading;
        //                 }
        //                 Resource::Ready(data) => {
        //                     match data {
        //                         Some(data) => {
        //                             return Resource::Ready(data);
        //                         },
        //                         None => {
        //                             continue;
        //                         }
        //                     }
        //                 },
        //                 Resource::Failed(mess) => {
        //                     return Resource::Failed(mess);
        //                 }
        //             }
        //         }

        //         return Resource::Failed("Empty path (2)".into());
        //     })
        // };
