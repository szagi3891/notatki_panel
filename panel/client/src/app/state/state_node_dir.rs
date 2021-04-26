use common::{GitTreeItem, HandlerHetchDirBody, HashIdType};
use vertigo::{
    computed::{
        Computed,
        Dependencies,
        AutoMapBox,
    },
};

use crate::request::{Request, Resource};

#[derive(PartialEq, Clone)]
struct NodeDir {
    value: Computed<Resource<Vec<GitTreeItem>>>,
}

impl NodeDir {
    pub fn new(request: &Request, dependencies: &Dependencies, id: &HashIdType) -> NodeDir {
        let value = dependencies.new_value(Resource::Loading);
        let value_read = value.to_computed();

        let response = request
            .fetch("/fetch_tree_item")
            .body(&HandlerHetchDirBody {
                id: id.clone(),
            })
            .post::<Vec<GitTreeItem>>();

        request.spawn_local(async move {
            let response = response.await;
            value.set_value(response);
        });

        NodeDir {
            value: value_read,
        }
    }
}

#[derive(PartialEq)]
pub struct StateNodeDir {
    data: AutoMapBox<HashIdType, NodeDir>,
}

impl StateNodeDir {
    pub fn new(request: &Request, dependencies: &Dependencies) -> StateNodeDir {
        let data = {
            let request = request.clone();
            let dependencies = dependencies.clone();

            let feth_node = move |id: &HashIdType| -> NodeDir {
                NodeDir::new(&request, &dependencies, id)
            };
    
            AutoMapBox::new(feth_node)
        };

        StateNodeDir {
            data
        }
    }
}
