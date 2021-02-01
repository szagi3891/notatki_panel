use common::{
    DataPost,
    DataNodeIdType,
    PostParamsFetchNodePost
};
use vertigo::{
    DomDriver,
    FetchMethod,
    computed::{
        Value,
    },
};
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
pub struct NodeState {
    data: Value<DataPost>,
    fetch: NodeFetch,
    flag: Value<bool>,
}

impl NodeState {
    pub fn new(data: Value<DataPost>, fetch: NodeFetch, flag: Value<bool>) -> NodeState {
        NodeState {
            data,
            fetch,
            flag,
        }
    }

    pub async fn refresh(&self) {
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

    pub fn title(&self) -> Rc<String> {
        let node = self.data.get_value();
        Rc::new(node.node.title())
    }
}
