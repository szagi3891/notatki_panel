use common::{DataNode, ServerFetchNodePost};
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
use std::sync::Arc;

#[derive(PartialEq)]
struct NodeFetch {
    path: Vec<String>,
    driver: DomDriver
}

impl NodeFetch {
    fn new(path: Vec<String>, driver: DomDriver) -> NodeFetch {
        NodeFetch {
            path,
            driver
        }
    }

    async fn get(&self) -> Result<DataNode, String> {
        let url = format!("/fetch_node");
        let body = ServerFetchNodePost {
            path: self.path.clone(),
        };
        
        let body_str = serde_json::to_string(&body).unwrap();

        let response = self.driver.fetch(FetchMethod::GET, url, None, Some(body_str)).await;

        match response {
            Ok(response) => {
                match serde_json::from_str::<DataNode>(response.as_str()) {
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
    data: Value<DataNode>,
    fetch: NodeFetch,
    // flag: BoxRefCell<bool>,
}

impl Node {
    fn new(data: Value<DataNode>, fetch: NodeFetch) -> Node {
        Node {
            data,
            fetch,
        }
    }

    async fn refresh(&self) {
        //TODO - tutaj trzeba zrobić odświeenie tego elementu
        let new_data = self.fetch.get().await;
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
    pub current_path: Value<Vec<Arc<String>>>,
    data: AutoMap<Vec<String>, Resource<Node>>,
}

impl State {
    pub fn new(root: &Dependencies, driver: &DomDriver) -> Computed<State> {
        let feth_node = {
            let root = root.clone();
            let driver = driver.clone();

            move |path: &Vec<String>| -> Computed<Resource<Node>> {
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
                                let node = Node::new(inner_value, node_fetch);
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

        let start_path = Vec::new();

        root.new_computed_from(State {
            driver: driver.clone(),
            current_path: root.new_value(start_path),
            data: AutoMap::new(feth_node)
        })
    }

    pub fn set_path(&self, new_path: &Vec<Arc<String>>) {
        self.current_path.set_value(new_path.clone());
    }

    //TODO - do celow testowych

    pub fn push_path(&self) {

        //TODO - dorobić do Value funkcję change ...

        //self.current_path.change
        let mut current = (&*self.current_path.get_value()).clone();
        
        current.push(Arc::new("cokolwiek".into()));

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
