use std::sync::Arc;
use common::DataNodeIdType;
use vertigo::{
    DomDriver,
    computed::{
        Value,
        Computed,
        Dependencies,
        AutoMap
    },
};
use std::rc::Rc;
use super::node_state::{
    Resource,
    NodeState,
    NodeFetch
};

static ROOT_ID: u64 = 1;

#[derive(PartialEq)]
pub struct State {
    pub driver: DomDriver,
    pub current_path: Value<Vec<Arc<String>>>,
    data: Arc<AutoMap<DataNodeIdType, Resource<NodeState>>>,
    current_node: Computed<DataNodeIdType>,                         //id aktualnie wybranego węzła
    pub list: Computed<Resource<Vec<DataNodeIdType>>>,                  //lista plików i katalogów w lewym panelu
}

impl State {
    pub fn new(root: &Dependencies, driver: &DomDriver) -> Computed<State> {
        let feth_node = {
            let root = root.clone();
            let driver = driver.clone();

            move |path: &DataNodeIdType| -> Computed<Resource<NodeState>> {
                let value = root.new_value(Resource::Loading);
                let result = value.to_computed();
    
                let node_fetch = NodeFetch::new(path.clone(), driver.clone());

                driver.spawn_local({
                    let root = root.clone();

                    async move {
                        let result = node_fetch.get().await;

                        match result {
                            Ok(data_node) => {
                                let node = NodeState::new(root, node_fetch, data_node);
                                value.set_value(Resource::Ready(node));
                            },
                            Err(err) => {
                                value.set_value(Resource::Failed(err));
                            }
                        }
                    }
                });
    
                result
            }
        };

        let data = Arc::new(AutoMap::new(feth_node));

        let current_path = root.new_value(Vec::<DataNodeIdType>::new());

        let current_node = {
            let current_path = current_path.to_computed();
            root.from::<DataNodeIdType, _>(move || -> DataNodeIdType {
                let current_path = current_path.get_value();
                let inner = &*current_path;
                let last = inner.last();

                if let Some(last) = last {
                    return *last;
                }

                return ROOT_ID;
            })
        };

        let list = {
            let current_path = current_path.to_computed();
            let data = data.clone();

            let get_child = move |node: &u64| -> Resource<Option<Vec<DataNodeIdType>>> {
                let current_item = data.get_value(node).get_value();

                match current_item.as_ref() {
                    Resource::Loading => {
                        return Resource::Loading;
                    },
                    Resource::Ready(data) => {
                        match data.child() {
                            Some(data) => Resource::Ready(Some(data)),
                            None => Resource::Ready(None),
                        }
                    },
                    Resource::Failed(mess) => {
                        return Resource::Failed(mess.clone());
                    }
                }
            };

            root.from::<Resource<Vec<DataNodeIdType>>, _>(move || -> Resource<Vec<DataNodeIdType>> {
                let current_path = current_path.get_value();

                if current_path.len() == 0 {
                    return get_child(&ROOT_ID).map(|item| {
                        match item {
                            Some(list) => list,
                            None => Vec::new()
                        }
                    });
                }

                for item in current_path.iter().rev() {
                    let child = get_child(item);

                    match child {
                        Resource::Loading => {
                            return Resource::Loading;
                        }
                        Resource::Ready(data) => {
                            match data {
                                Some(data) => {
                                    return Resource::Ready(data);
                                },
                                None => {
                                    continue;
                                }
                            }
                        },
                        Resource::Failed(mess) => {
                            return Resource::Failed(mess);
                        }
                    }
                }

                return Resource::Failed("Empty path (2)".into());
            })
        };

        root.new_computed_from(State {
            driver: driver.clone(),
            current_path,
            data,
            current_node,
            list
        })
    }

    pub fn node_title(&self, node_id: &DataNodeIdType) -> Option<Rc<String>> {
        let item = self.data.get_value(node_id);
        let value = item.get_value();

        if let Resource::Ready(value_resource) = &*value {
            return Some(value_resource.title());
        }

        None
    }

    pub fn set_path(&self, node_id: DataNodeIdType) {
        let mut current = (*self.current_path.get_value()).clone();
        current.push(node_id);
        self.current_path.set_value(current);
    }

    pub fn create_dir(&self, name: String) {
        let node = self.current_node.get_value();
        let item = self.data.get_value(&*node).get_value();
        
        self.driver.spawn_local(async move {
            if let Resource::Ready(item) = &*item {
                item.create_dir(name).await;
            } else {
                log::error!("Błąd przy tworzeniu katalogu");
            }
        });
    }
}

/*
dodać zakładki
    zakładka1: notatki
    zakładka2: parsowanie urla ...

    zakładka - zarządzanie gitem
        menu pozwalające usuwać niepotrzebne gałęzie gita ...
    
        pozwoli np. ta funkcja na uruchomienie polecenia rebejsującego
*/
