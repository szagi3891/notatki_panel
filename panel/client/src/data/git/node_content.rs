use std::rc::Rc;
use common::{ HandlerFetchNodeBody, HandlerFetchNodeResponse };
use vertigo::{ Resource, AutoMap, LazyCache, get_driver, Context };

#[derive(Clone, Debug)]
pub struct NodeContent {
    value: LazyCache<String>,
}

impl NodeContent {
    pub fn new(hash: &String) -> NodeContent {
        let hash = hash.clone();

        let response = LazyCache::new(10 * 60 * 60 * 1000, move || {
            let hash = hash.clone();
            async move {
                let request = get_driver()
                    .request("/fetch_node")
                    .body_json(HandlerFetchNodeBody {
                        hash: hash.clone(),
                    })
                    .post();

                request.await.into(|status, body| {
                    if status == 200 {
                        let response = body.into::<HandlerFetchNodeResponse>();
                        Some(response.map(|inner| {
                            inner.content
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

    fn get(&self, context: &Context) -> Resource<Rc<String>> {
        self.value.get(context)
    }
}

#[derive(Clone, Debug)]
pub struct Content {
    data: AutoMap<String, NodeContent>,
}

impl Content {
    pub fn new() -> Content {
        let data = {
            AutoMap::new(move |id: &String| NodeContent::new(id))
        };

        Content {
            data
        }
    }

    pub fn get(&self, context: &Context, id: &String) -> Resource<Rc<String>> {
        self.data.get(id).get(context)
    }
}
