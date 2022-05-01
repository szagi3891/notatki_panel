use common::RootResponse;
use vertigo::{
    Driver,
    Resource,
    Value,
    LazyCache,
};

pub struct RootNode {
    root: LazyCache<RootResponse>,
}

impl RootNode {
    fn new(driver: &Driver) -> RootNode {
        let root = LazyCache::new(driver, 10 * 60 * 60 * 1000, move |driver: Driver| async move {
            let request = driver
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
    driver: Driver,
    pub current: Value<RootNode>,
    //list: Value<VecDeque<RootNode>>,      //todo zaimplementowach historie, zeby zniwelowac ilosc migań
}

impl Root {
    pub fn new(driver: &Driver) -> Root {
        let current = RootNode::new(driver);
        let current = driver.new_value(current);
       
        Root {
            driver: driver.clone(),
            current,
        }
    }

    pub fn get_current_root(&self) -> Resource<String> {
        let current = self.current.get_value();
        current.get()
    }

    pub fn refresh(&self) {
        let current = RootNode::new(&self.driver);
        self.current.set_value(current);
    }
}
