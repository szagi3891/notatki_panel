use common::RootResponse;
use vertigo::{
    Driver,
    Resource,
    computed::{
        Dependencies,
        Value,
        Computed,
    }
};

use super::StateNodeDir;

#[derive(PartialEq)]
pub struct RootNode {
    state_node_dir: StateNodeDir,
    pub value: Computed<Resource<RootResponse>>,
}

impl RootNode {
    fn new(request: &Driver, dependencies: &Dependencies, state_node_dir: StateNodeDir) -> RootNode {
        let value = dependencies.new_value(Resource::Loading);
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
pub struct StateRoot {
    driver: Driver,
    dependencies: Dependencies,
    state_node_dir: StateNodeDir,
    pub current: Value<RootNode>,
    //list: Value<VecDeque<RootNode>>,      //todo zaimplementowach historie, zeby zniwelowac ilosc migań
}

impl StateRoot {
    pub fn new(driver: &Driver, dependencies: &Dependencies, state_node_dir: StateNodeDir) -> StateRoot {
        let current = RootNode::new(driver, dependencies, state_node_dir.clone());
        let current = dependencies.new_value(current);
       
        StateRoot {
            driver: driver.clone(),
            dependencies: dependencies.clone(),
            state_node_dir,
            current,
        }
    }

    pub fn get_current_root(&self) -> Resource<String> {
        let current = self.current.get_value();
        current.get()
    }

    pub fn refresh(&self) {
        let current = RootNode::new(&self.driver, &self.dependencies, self.state_node_dir.clone());
        self.current.set_value(current);
    }
}
