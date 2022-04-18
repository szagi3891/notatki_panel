use common::RootResponse;
use vertigo::{
    Driver,
    Resource,
    Value,
    Computed,
};

pub struct RootNode {
    pub value: Computed<Resource<RootResponse>>,
}

impl RootNode {
    fn new(request: &Driver) -> RootNode {
        let value = request.new_value(Resource::Loading);
        let value_read = value.to_computed();
        let response = request.request("/fetch_root").get();

        request.spawn(async move {
            let response = response.await.into(|status, body| {
                if status == 200 {
                    return Some(body.into::<RootResponse>());
                }
                None
            });

            value.set_value(response);
        });

        RootNode {
            value: value_read,
        }
    }

    pub fn get(&self) -> Resource<String> {
        let handler_root = self.value.get_value();
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
