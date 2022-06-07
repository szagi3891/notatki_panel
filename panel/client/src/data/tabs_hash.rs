use serde::{Serialize, Deserialize};
use vertigo::{router::HashRouter, Computed};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
struct RouterValue {
    dir: Vec<String>,
    item: Option<String>,
}

impl From<String> for RouterValue {
    fn from(path: String) -> Self {
        log::info!("from --- {path}");

        match serde_json::from_str(path.as_str()) {
            Ok(value) => value,
            Err(err) => {
                log::warn!("Invalid url -  select the default values - {err}");

                RouterValue {
                    dir: Vec::new(),
                    item: None,
                }
            }
        }
    }
}

impl ToString for RouterValue {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Clone)]
pub struct Router {
    route: HashRouter<RouterValue>,
    pub path: Computed<Vec<String>>,
}

impl Router {
    pub fn new() -> Router {
        let route = HashRouter::<RouterValue>::new();

        let path = {
            let route = route.clone();

            Computed::from(move || {
                route.get().dir.clone()
            })
        };

        Router {
            route,
            path
        }
    }

    pub fn get_dir(&self) -> Vec<String> {
        self.route.get().dir
    }

    pub fn set_dir(&self, dir: Vec<String>) {
        let mut route = self.route.get();
        route.dir = dir;
        self.route.set(route);
    }

    pub fn get_item(&self) -> Option<String> {
        self.route.get().item
    }

    pub fn set_item(&self, item: Option<String>) {
        let mut route = self.route.get();
        route.item = item;
        self.route.set(route);
    }
}