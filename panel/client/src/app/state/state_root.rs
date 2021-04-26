use common::HandlerRoot;
use vertigo::{
    computed::{
        Dependencies,
        Value,
        Computed,
    },
};
use crate::request::{Request, Resource};

#[derive(PartialEq)]
struct RootNode {
    value: Computed<Resource<HandlerRoot>>,
}

impl RootNode {
    fn new(request: &Request, dependencies: &Dependencies) -> RootNode {
        let value = dependencies.new_value(Resource::Loading);
        let value_read = value.to_computed();
        let response = request.fetch("/fetch_root").get::<HandlerRoot>();

        request.spawn_local(async move {
            let response = response.await;
            value.set_value(response);
        });

        RootNode { 
            value: value_read,
        }
    }
}

#[derive(PartialEq)]
pub struct StateRoot {
    request: Request,
    dependencies: Dependencies,
    current: Value<RootNode>,
    //list: Value<VecDeque<RootNode>>,      //todo zaimplementowach historie, zeby zniwelowac ilosc migań
}

impl StateRoot {
    pub fn new(request: &Request, dependencies: &Dependencies) -> StateRoot {
        // let mut list = VecDeque::new();
        // list.push_back(RootNode::new(&driver, dependencies));

        // let list = dependencies.new_value(list);

        let current = RootNode::new(request, dependencies);
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

        match &*resource {
            Resource::Loading => None,
            Resource::Ready(value) => Some(value.root.clone()),
            Resource::Error(_) => None,
        }
    }

    pub fn get_hash_view(&self) -> String {
        match self.get_hash() {
            Some(hash) => format!("hash={}", &hash),
            None => "none".into(),
        }
    }
}