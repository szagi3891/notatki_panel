use std::rc::Rc;
use common::{HandlerFetchNodeBody, HandlerFetchNodeResponse};
use vertigo::{
    Resource,
    Driver,
    Computed,
    AutoMap,
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
    pub fn new(driver: &Driver, hash: &String) -> NodeContent {
        let value = driver.new_value(Resource::Loading);
        let value_read = value.to_computed();

        let response = driver
            .request("/fetch_node")
            .body_json(HandlerFetchNodeBody {
                hash: hash.clone(),
            })
            .post();


        driver.spawn(async move {
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

#[derive(Clone)]
pub struct StateDataGitNodeContent {
    data: AutoMap<String, NodeContent>,
}

impl StateDataGitNodeContent {
    pub fn new(driver: &Driver) -> StateDataGitNodeContent {
        let data = {
            let request = driver.clone();

            AutoMap::new(move |id: &String| NodeContent::new(&request, id))
        };

        StateDataGitNodeContent {
            data
        }
    }

    pub fn get(&self, id: &String) -> Resource<Rc<String>> {
        self.data.get_value(id).get()
    }
}
