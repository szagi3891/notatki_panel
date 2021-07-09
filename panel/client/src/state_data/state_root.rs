use common::HandlerFetchRootResponse;
use vertigo::{
    computed::{
        Dependencies,
        Value,
        Computed,
    },
};
use crate::request::{Request, Resource, ResourceError};

use super::StateNodeDir;

#[derive(PartialEq)]
pub struct RootNode {
    state_node_dir: StateNodeDir,
    pub value: Computed<Resource<HandlerFetchRootResponse>>,
}

impl RootNode {
    fn new(request: &Request, dependencies: &Dependencies, state_node_dir: StateNodeDir) -> RootNode {
        let value = dependencies.new_value(Err(ResourceError::Loading));
        let value_read = value.to_computed();
        let response = request.fetch("/fetch_root").get::<HandlerFetchRootResponse>();

        request.spawn_local(async move {
            let response = response.await;
            value.set_value(response);
        });

        RootNode {
            state_node_dir,
            value: value_read,
        }
    }

    pub fn get(&self) -> Result<String, ResourceError> {
        let handler_root = self.value.get_value();

        match handler_root.as_ref() {
            Ok(inner) => Ok(inner.root.clone()),
            Err(err) => return Err(err.clone()),
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct StateRoot {
    request: Request,
    dependencies: Dependencies,
    state_node_dir: StateNodeDir,
    pub current: Value<RootNode>,
    //list: Value<VecDeque<RootNode>>,      //todo zaimplementowach historie, zeby zniwelowac ilosc migań
}

impl StateRoot {
    pub fn new(request: &Request, dependencies: &Dependencies, state_node_dir: StateNodeDir) -> StateRoot {
        let current = RootNode::new(request, dependencies, state_node_dir.clone());
        let current = dependencies.new_value(current);
       
        StateRoot {
            request: request.clone(),
            dependencies: dependencies.clone(),
            state_node_dir,
            current,
        }
    }
    
    pub fn get_current_root(&self) -> Result<String, ResourceError> {
        let current = self.current.get_value();
        current.get()
    }


    pub fn refresh(&self) {
        let current = RootNode::new(&self.request, &self.dependencies, self.state_node_dir.clone());
        self.current.set_value(current);
    }
}
