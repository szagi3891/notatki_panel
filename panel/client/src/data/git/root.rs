use common::RootResponse;
use vertigo::{
    Driver,
    Resource,
    Value,
    Computed,
};

use super::node_dir::StateDataGitNodeDir;

#[derive(PartialEq)]
pub struct RootNode {
    state_node_dir: StateDataGitNodeDir,
    pub value: Computed<Resource<RootResponse>>,
}

impl RootNode {
    fn new(request: &Driver, state_node_dir: StateDataGitNodeDir) -> RootNode {
        let value = request.new_value(Resource::Loading);
        let value_read = value.to_computed();
        let response = request.request("/fetch_root").get();

        request.spawn(async move {
            let response = response.await.into(|status, body| {
                if status == 200 {
                    return Some(body.into::<RootResponse>());
                }
                None
            });

            value.set_value(response);
        });

        RootNode {
            state_node_dir,
            value: value_read,
        }
    }

    pub fn get(&self) -> Resource<String> {
        let handler_root = self.value.get_value();
        handler_root.ref_map(|item| item.root.clone())
    }
}

#[derive(PartialEq, Clone)]
pub struct StateDataGitRoot {
    driver: Driver,
    state_node_dir: StateDataGitNodeDir,
    pub current: Value<RootNode>,
    //list: Value<VecDeque<RootNode>>,      //todo zaimplementowach historie, zeby zniwelowac ilosc migań
}

impl StateDataGitRoot {
    pub fn new(driver: &Driver, state_node_dir: StateDataGitNodeDir) -> StateDataGitRoot {
        let current = RootNode::new(driver, state_node_dir.clone());
        let current = driver.new_value(current);
       
        StateDataGitRoot {
            driver: driver.clone(),
            state_node_dir,
            current,
        }
    }

    pub fn get_current_root(&self) -> Resource<String> {
        let current = self.current.get_value();
        current.get()
    }

    pub fn refresh(&self) {
        let current = RootNode::new(&self.driver, self.state_node_dir.clone());
        self.current.set_value(current);
    }
}
