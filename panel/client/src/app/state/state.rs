use common::GitTreeItem;
use vertigo::{
    DomDriver,
    computed::{
        Computed,
        Dependencies,
        Value
    }
};
use crate::request::{Request, Resource};
use super::{StateNodeDir, StateRoot};


fn create_list(
    root: &Dependencies,
    current_path: Computed<Vec<String>>,
    state_root: StateRoot,
    state_node_dir: StateNodeDir
) -> Computed<Resource<Vec<GitTreeItem>>> {

    root.from(move || -> Resource<Vec<GitTreeItem>> {

        let current = state_root.current.get_value();
        let handler_root = &*current.value.get_value();

        let handler_root = match handler_root {
            Resource::Ready(value) => value,
            Resource::Loading => return Resource::Loading,
            Resource::Error(err) => return Resource::Error(err.clone()),
            //a @ _ => return *a,
        };

        let current_path = &*current_path.get_value();

        let mut current_wsk = handler_root.root.clone();

        for path_item in current_path {
            let node = &*state_node_dir.get(path_item).get();

            let node = match node {
                Resource::Ready(node) => node,
                Resource::Error(err) => return Resource::Error(err.clone()),
                Resource::Loading => return Resource::Loading,
            };

            let tree_item = node.get(path_item);

            let tree_item = match tree_item {
                Some(tree_item) => tree_item,
                None => {
                    return Resource::Error(format!("missing tree_item {}", path_item));
                }
            };

            
        }


        todo!()
    })

    // let root = state_root.get_hash();

    // todo!()
}

#[derive(PartialEq)]
pub struct State {
    pub driver: DomDriver,
    state_node_dir: StateNodeDir,
    state_root: StateRoot,
    pub current_path: Value<Vec<String>>,
    pub list: Computed<Resource<Vec<GitTreeItem>>>,                  //lista plików i katalogów w lewym panelu
    //current_node: Computed<DataNodeIdType>,                         //id aktualnie wybranego węzła
}

/*
    dir1/dir2/dir3
    
    list => dir3
    current => dir3

    dir1/dir2/file

    list => dir2
    current => file

    list => zawsze wskazuje na dir
    current => na dir | hash pliku + jego nazwa | loading
*/

impl State {
    pub fn new(root: &Dependencies, driver: &DomDriver) -> Computed<State> {
        let request = Request::new(driver);

        let state_node_dir = StateNodeDir::new(&request, root);
        let state_root = StateRoot::new(&request, root, state_node_dir.clone());

        let current_path = root.new_value(Vec::<String>::new());

        let list = create_list(
            root,
            current_path.to_computed(),
            state_root.clone(),
            state_node_dir.clone(),
        );

        root.new_computed_from(State {
            driver: driver.clone(),
            state_node_dir,
            state_root,
            current_path,
            list,
            // current_node,
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
