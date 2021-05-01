use std::rc::Rc;
use common::{HandlerFetchNodeBody, HandlerFetchNodeResponse};
use vertigo::{
    computed::{
        Computed,
        Dependencies,
        AutoMapBox,
    },
};

use crate::request::{Request, Resource, ResourceError};

#[derive(PartialEq, Clone)]
pub struct TreeItem {
    pub dir: bool,
    pub id: String,
}

#[derive(PartialEq, Clone)]
pub struct NodeContent {
    value: Computed<Resource<Rc<String>>>,
}

impl NodeContent {
    pub fn new(request: &Request, dependencies: &Dependencies, hash: &String) -> NodeContent {
        let value = dependencies.new_value(Err(ResourceError::Loading));
        let value_read = value.to_computed();

        let response = request
            .fetch("/fetch_node")
            .body(&HandlerFetchNodeBody {
                hash: hash.clone(),
            })
            .post::<HandlerFetchNodeResponse>();

        request.spawn_local(async move {
            let response = response.await;
            value.set_value(response.map(|item| Rc::new(item.content)));
        });

        NodeContent {
            value: value_read,
        }
    }

    pub fn get(&self) -> Result<Rc<String>, ResourceError> {
        match &*self.value.get_value() {
            Ok(content) => Ok(content.clone()),
            Err(err) => Err(err.clone()),
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct StateNodeContent {
    data: AutoMapBox<String, NodeContent>,
}

impl StateNodeContent {
    pub fn new(request: &Request, dependencies: &Dependencies) -> StateNodeContent {
        let data = {
            let request = request.clone();
            let dependencies = dependencies.clone();

            AutoMapBox::new(move |id: &String| NodeContent::new(&request, &dependencies, id))
        };

        StateNodeContent {
            data
        }
    }

    pub fn get(&self, id: &String) -> Result<Rc<String>, ResourceError> {
        log::info!("get content --> {}", id);
        self.data.get_value(id).get()
    }
}
