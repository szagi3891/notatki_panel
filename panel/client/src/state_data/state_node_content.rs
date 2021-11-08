use std::rc::Rc;
use common::{HandlerFetchNodeBody, HandlerFetchNodeResponse};
use vertigo::{
    Resource,
    DomDriver,
    computed::{
        Computed,
        Dependencies,
        AutoMap,
    },
};

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
    pub fn new(request: &DomDriver, dependencies: &Dependencies, hash: &String) -> NodeContent {
        let value = dependencies.new_value(Resource::Loading);
        let value_read = value.to_computed();

        let response = request
            .request("/fetch_node")
            .body_json(HandlerFetchNodeBody {
                hash: hash.clone(),
            })
            .post();


        request.spawn(async move {
            let response = response.await.into(|status, body| {
                if status == 200 {
                    return Some(body.into::<HandlerFetchNodeResponse>());
                }
                None
            });
            value.set_value(response.map(|item| Rc::new(item.content)));
        });

        NodeContent {
            value: value_read,
        }
    }

    fn get(&self) -> Resource<Rc<String>> {
        self.value.get_value().ref_clone()
    }
}

#[derive(PartialEq, Clone)]
pub struct StateNodeContent {
    data: AutoMap<String, NodeContent>,
}

impl StateNodeContent {
    pub fn new(request: &DomDriver, dependencies: &Dependencies) -> StateNodeContent {
        let data = {
            let request = request.clone();
            let dependencies = dependencies.clone();

            AutoMap::new(move |id: &String| NodeContent::new(&request, &dependencies, id))
        };

        StateNodeContent {
            data
        }
    }

    pub fn get(&self, id: &String) -> Resource<Rc<String>> {
        self.data.get_value(id).get()
    }
}
