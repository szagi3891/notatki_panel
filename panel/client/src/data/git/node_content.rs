use std::rc::Rc;
use common::{HandlerFetchNodeBody, HandlerFetchNodeResponse};
use vertigo::{
    Resource,
    Driver,
    AutoMap, LazyCache,
};

#[derive(Clone)]
pub struct NodeContent {
    value: LazyCache<Rc<String>>,
}

impl NodeContent {
    pub fn new(driver: &Driver, hash: &String) -> NodeContent {
        let hash = hash.clone();

        let response = LazyCache::new(driver, 10 * 60 * 60 * 1000, move |driver: Driver| {
            let hash = hash.clone();
            async move {
                let request = driver
                    .request("/fetch_node")
                    .body_json(HandlerFetchNodeBody {
                        hash: hash.clone(),
                    })
                    .post();

                request.await.into(|status, body| {
                    if status == 200 {
                        let response = body.into::<HandlerFetchNodeResponse>();
                        Some(response.map(|inner| {
                            Rc::new(inner.content.clone())
                        }))
                    } else {
                        None
                    }
                })
            }
        });

        NodeContent {
            value: response,
        }
    }

    fn get(&self) -> Resource<Rc<String>> {
        self.value.get_value().ref_map(|inner| inner.clone())
    }
}

#[derive(Clone)]
pub struct Content {
    data: AutoMap<String, NodeContent>,
}

impl Content {
    pub fn new(driver: &Driver) -> Content {
        let data = {
            let request = driver.clone();

            AutoMap::new(move |id: &String| NodeContent::new(&request, id))
        };

        Content {
            data
        }
    }

    pub fn get(&self, id: &String) -> Resource<Rc<String>> {
        self.data.get_value(id).get()
    }
}
