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
    pub current: Value<RootNode>,
    //list: Value<VecDeque<RootNode>>,      //todo zaimplementowach historie, zeby zniwelowac ilosc migań
}

impl StateRoot {
    pub fn new(request: &Request, dependencies: &Dependencies, state_node_dir: StateNodeDir) -> StateRoot {
        // let mut list = VecDeque::new();
        // list.push_back(RootNode::new(&driver, dependencies));

        // let list = dependencies.new_value(list);

        let current = RootNode::new(request, dependencies, state_node_dir);
        let current = dependencies.new_value(current);
       
        StateRoot {
            request: request.clone(),
            dependencies: dependencies.clone(),
            current,
        }
    }

    // fn refresh(&self) {
    //     let new_root = RootNode::new(&self.driver, &self.dependencies);

    //     //TODO - sprawdzic czy nie ma wiecej niz 10 elementow
    //     //zepsute usuwac
    //     //dobre, powyzej 10ciu usuwac
    //     //zostawic przynajmniej jeden element na lisce

    //     // let mut list = *(self.list.get_value());
    //     // list.push_back(new_root);
    //     // self.list.set_value(list);

    //     let new_current = RootNode::new(&self.driver, &self.dependencies);
    //     self.current.set_value(new_current);
    // }

    pub fn get_hash(&self) -> Option<String> {
        let value = self.current.get_value();

        let resource = value.value.get_value();

        match resource.as_ref() {
            Err(_) => None,
            Ok(value) => Some(value.root.clone()),
        }
    }

    pub fn get_hash_view(&self) -> String {
        match self.get_hash() {
            Some(hash) => format!("hash={}", &hash),
            None => "none".into(),
        }
    }

    //path --> ...
        //bierzemy z kilku rootow path o ktory pytamy ...
    
    pub fn get_current_root(&self) -> Result<String, ResourceError> {
        let current = self.current.get_value();
        current.get()
    }
}