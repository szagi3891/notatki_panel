use common::{DataNode, DataPost, DataNodeIdType, ServerFetchNodePost};
use vertigo::{
    DomDriver,
    FetchMethod,
    computed::{
        Value,
        Computed,
        Dependencies,
        AutoMap
    },
    // utils::BoxRefCell,
};
use std::rc::Rc;

#[derive(PartialEq)]
struct NodeFetch {
    node_id: DataNodeIdType,
    driver: DomDriver
}

impl NodeFetch {
    fn new(node_id: DataNodeIdType, driver: DomDriver) -> NodeFetch {
        NodeFetch {
            node_id,
            driver
        }
    }

    async fn get(&self) -> Result<DataPost, String> {
        let url = format!("/fetch_node");
        let body = ServerFetchNodePost {
            node_id: self.node_id,
        };
        
        let body_str = serde_json::to_string(&body).unwrap();

        let response = self.driver.fetch(
            FetchMethod::POST,
            url,
            None,
            Some(body_str)
        ).await;

        match response {
            Ok(response) => {
                match serde_json::from_str::<DataPost>(response.as_str()) {
                    Ok(data_node) => {
                        log::info!("odpowiedź z serwera {:?}", data_node);
                        Ok(data_node)
                    },
                    Err(err) => {
                        log::error!("Error parsing response: {}", err);
                        Err(err.to_string())
                    }
                }
            },
            Err(_) => {
                log::error!("Error fetch");
                Err("Error fetch".into())
            }
        }
    }
}

#[derive(PartialEq)]
pub enum Resource<T: PartialEq> {
    Loading,
    Ready(T),
    Failed(String),
}

#[derive(PartialEq)]
struct Node {
    data: Value<DataPost>,
    fetch: NodeFetch,
    flag: Value<bool>,
}

impl Node {
    fn new(data: Value<DataPost>, fetch: NodeFetch, flag: Value<bool>) -> Node {
        Node {
            data,
            fetch,
            flag,
        }
    }

    async fn refresh(&self) {
        let flag = self.flag.get_value();
        if *flag {
            return;
        }

        self.flag.set_value(true);

        let new_data = self.fetch.get().await;

        match new_data {
            Ok(new_data) => {
                self.data.set_value(new_data);
            },
            Err(err) => {
                log::error!("Refresh error: {}", err);
            }
        }

        self.flag.set_value(false);
    }

    fn title(&self) -> Rc<String> {
        let node = self.data.get_value();
        Rc::new(node.node.title())
    }
}
/*
    root - 1

        sub1
            root[sub1] --- daje id kolejnego wezla
    
    Vec<numerki>

    current node --> DataNodeIdType

    jesli nie jest wybrana zadna sciezka, to zwracaj 1

    przestawienie sciezki, to tak naprawde odrzucenie wszystkiego co jest za wskazanym elementem z prawej strony
*/

#[derive(PartialEq)]
pub struct State {
    pub driver: DomDriver,
    pub current_path: Value<Vec<DataNodeIdType>>,
    data: AutoMap<DataNodeIdType, Resource<Node>>,
}

impl State {
    pub fn new(root: &Dependencies, driver: &DomDriver) -> Computed<State> {
        let feth_node = {
            let root = root.clone();
            let driver = driver.clone();

            move |path: &DataNodeIdType| -> Computed<Resource<Node>> {
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
                                let node = Node::new(inner_value, node_fetch, flag);
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

    //TODO - do celow testowych

    pub fn push_path(&self) {

        //TODO - dorobić do Value funkcję change ...

        //self.current_path.change
        let mut current = (&*self.current_path.get_value()).clone();
        
        current.push(1);

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
