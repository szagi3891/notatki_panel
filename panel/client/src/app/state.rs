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

#[derive(PartialEq)]
pub struct State {
    pub driver: DomDriver,
    pub current_path: Value<Vec<DataNodeIdType>>,
    data: AutoMap<DataNodeIdType, Resource<NodeState>>,
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
                                let inner_value = root.new_value(data_node);
                                let flag = root.new_value(false);
                                let node = NodeState::new(inner_value, node_fetch, flag);
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

        root.new_computed_from(State {
            driver: driver.clone(),
            current_path: root.new_value(Vec::new()),
            data: AutoMap::new(feth_node)
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
}

/*
dodać zakładki


    zakładka1: notatki
    zakładka2: parsowanie urla ...

    zakładka - zarządzanie gitem
        menu pozwalające usuwać niepotrzebne gałęzie gita ...
    
        pozwoli np. ta funkcja na uruchomienie polecenia rebejsującego
*/
