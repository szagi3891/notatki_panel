use common::RootResponse;
use vertigo::{
    Resource,
    Value,
    LazyCache, get_driver,
};

pub struct RootNode {
    root: LazyCache<RootResponse>,
}

impl RootNode {
    fn new() -> RootNode {
        let root = LazyCache::new(10 * 60 * 60 * 1000, move || async move {
            let request = get_driver()
                .request("/fetch_root")
                .get();

            request.await.into(|status, body| {
                if status == 200 {
                    Some(body.into::<RootResponse>())
                } else {
                    None
                }
            })
        });

        RootNode {
            root,
        }
    }

    pub fn get(&self) -> Resource<String> {
        let handler_root = self.root.get_value();
        handler_root.ref_map(|item| item.root.clone())
    }
}

#[derive(Clone)]
pub struct Root {
    pub current: Value<RootNode>,
    //list: Value<VecDeque<RootNode>>,      //todo zaimplementowach historie, zeby zniwelowac ilosc migań
}

impl Root {
    pub fn new() -> Root {
        let current = RootNode::new();
        let current = Value::new(current);
       
        Root {
            current,
        }
    }

    pub fn get_current_root(&self) -> Resource<String> {
        let current = self.current.get_value();
        current.get()
    }

    pub fn refresh(&self) {
        let current = RootNode::new();
        self.current.set_value(current);
    }
}
