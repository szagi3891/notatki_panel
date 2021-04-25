use common::{DataNodeIdType, DataNode, DataPost, /*PostParamsCreateDir,*/ PostParamsFetchNodePost};
use vertigo::{DomDriver, FetchMethod, computed::{Dependencies, Value}};
use std::rc::Rc;

#[derive(PartialEq)]
pub struct NodeFetch {
    node_id: DataNodeIdType,
    driver: DomDriver
}

impl NodeFetch {
    pub fn new(node_id: DataNodeIdType, driver: DomDriver) -> NodeFetch {
        NodeFetch {
            node_id,
            driver
        }
    }

    pub async fn get(&self) -> Result<DataPost, String> {
        let url = format!("/fetch_node");
        let body = PostParamsFetchNodePost {
            node_id: self.node_id.clone(),
        };
        
        let body_str = serde_json::to_string(&body).unwrap();

        let response = self.driver.fetch(url).set_body(body_str).post().await?;

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
    }

    // pub async fn create_dir(&self, name: String) {
    //     let url = format!("/create_dir");
    //     let body = PostParamsCreateDir {
    //         parent_node: self.node_id,
    //         name,
    //     };
        
    //     let body_str = serde_json::to_string(&body).unwrap();

    //     let response = self.driver.fetch(
    //         FetchMethod::POST,
    //         url,
    //         None,
    //         Some(body_str)
    //     ).await;

    //     match response {
    //         Ok(response) => {
    //             match serde_json::from_str::<DataNodeIdType>(response.as_str()) {
    //                 Ok(new_id) => {
    //                     log::info!("Utworzono katalog {:?}", new_id);
    //                 },
    //                 Err(err) => {
    //                     log::error!("Error parsing response: {}", err);
    //                 }
    //             }
    //         },
    //         Err(_) => {
    //             log::error!("Error fetch");
    //         }
    //     }
    // }
}

#[derive(PartialEq)]
pub enum Resource<T: PartialEq> {
    Loading,
    Ready(T),
    Failed(String),
}

impl<T: PartialEq> Resource<T> {
    pub fn map<K: PartialEq, F: Fn(T) -> K>(self, map: F) -> Resource<K> {
        match self {
            Resource::Loading => Resource::Loading,
            Resource::Ready(data) => Resource::Ready(map(data)),
            Resource::Failed(error) => Resource::Failed(error),
        }
    }
}

// #[derive(PartialEq)]
// pub enum CurrentAction {
//     CreateDir,
// }

#[derive(PartialEq)]
pub struct NodeState {
    data: Value<DataPost>,
    fetch: NodeFetch,
    //action: Value<Option<CurrentAction>>,
}

impl NodeState {
    pub fn new(root: Dependencies, fetch: NodeFetch, data: DataPost) -> NodeState {
        let data = root.new_value(data);
        //let action = root.new_value(None);

        NodeState {
            data,
            fetch,
            //action,
        }
    }

    /*
        mozemy bezpiecznie wielokrotnie wywolać tą funkcję. Nieświeze dane zostaną odrzucone
    */
    async fn refresh(&self) {
        let new_data = self.fetch.get().await;

        match new_data {
            Ok(new_data) => {
                let current = self.data.get_value();

                if new_data.timestamp > current.timestamp {
                    self.data.set_value(new_data);
                } else {
                    log::warn!("I reject the data on the node because it is stale");
                }
            },
            Err(err) => {
                log::error!("Refresh error: {}", err);
            }
        }
    }

    pub fn title(&self) -> Rc<String> {
        let node = self.data.get_value();
        Rc::new(node.node.title())
    }

    pub fn child(&self) -> Option<Vec<DataNodeIdType>> {
        let node = self.data.get_value();

        let data_node = &node.node;
        
        match &data_node {
            DataNode::Dir { child, .. } => Some(child.clone()),
            DataNode::File { .. } => None,
        }
    }

    // pub async fn create_dir(&self, name: String) {
    //     if self.action.get_value().is_some() {
    //         return;
    //     }

    //     self.action.set_value(Some(CurrentAction::CreateDir));

    //     self.fetch.create_dir(name).await;
    //     self.refresh().await;

    //     self.action.set_value(None);
    // }
}
