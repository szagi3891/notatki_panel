use serde::{Serialize, Deserialize};
use vertigo::{router::Router as HashRouter, Context, transaction, Value};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
struct RouterValue {
    dir: Vec<String>,
    item: Option<String>,
}

impl From<String> for RouterValue {
    fn from(path: String) -> Self {
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

#[derive(Clone, PartialEq)]
pub struct Router {
    route: HashRouter<RouterValue>,

    ///Element nad którym znajduje się hover
    pub item_hover: Value<Option<String>>,
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}

impl Router {
    pub fn new() -> Router {
        let route = HashRouter::<RouterValue>::new_hash_router();

        Router {
            route,
            item_hover: Value::default(),
        }
    }

    pub fn get_dir(&self, context: &Context) -> Vec<String> {
        self.route.route.get(context).dir
    }

    pub fn get_hover(&self, context: &Context) -> Option<String> {
        self.item_hover.get(context)
    }

    pub fn set_only_item(&self, item: Option<String>) {
        transaction(|context| {
            let mut route = self.route.route.get(context);
            route.item = item;
            self.route.set(route);
            self.item_hover.set(None);
        });
    }

    pub fn set(&self, dir: Vec<String>, item: Option<String>) {
        transaction(|_| {
            self.route.set(RouterValue {
                dir,
                item,
            });
            self.item_hover.set(None);
        });
    }

    pub fn get_item(&self, context: &Context) -> Option<String> {
        self.route.route.get(context).item
    }

    pub fn hover_on(&self, name: &str) {
        self.item_hover.set(Some(name.to_string()));
    }

    pub fn hover_off(&self, name: &str) {
        transaction(|context| {
            let item_hover = self.item_hover.get(context);

            if let Some(item_hover) = item_hover.as_ref() {
                if item_hover == name {
                    self.item_hover.set(None);
                }
            }
        });
    }
}
