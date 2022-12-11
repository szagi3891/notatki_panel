use std::rc::Rc;
use common::{ HandlerFetchNodeBody, HandlerFetchNodeResponse };
use vertigo::{ Resource, AutoMap, LazyCache, Context, RequestBuilder };

#[derive(Clone, Debug)]
pub struct NodeContent {
    value: LazyCache<String>,
}

impl NodeContent {
    pub fn new(hash: &String) -> NodeContent {
        let response = RequestBuilder::post("/fetch_node")
            .body_json(HandlerFetchNodeBody {
                hash: hash.clone(),
            })
            .lazy_cache(|status, body| {
                if status == 200 {
                    let response = body.into::<HandlerFetchNodeResponse>();
                    Some(response.map(|inner| {
                        inner.content
                    }))
                } else {
                    None
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

#[derive(Clone, Debug, PartialEq)]
pub struct Content {
    data: AutoMap<String, NodeContent>,
}

impl Content {
    pub fn new() -> Content {
        let data = AutoMap::new(NodeContent::new);

        Content {
            data
        }
    }

    pub fn get(&self, context: &Context, id: &String) -> Resource<Rc<String>> {
        self.data.get(id).get(context)
    }
}
