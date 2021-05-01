use serde::{Deserialize, Serialize};

pub type TimestampType = u128;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct HandlerFetchRootResponse {
    pub root: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct HandlerFetchDirBody {
    pub id: String,
}


#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct GitTreeItem {
    pub dir: bool,
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct HandlerFetchDirResponse {
    pub list: Vec<GitTreeItem>,
}

impl HandlerFetchDirResponse {
    pub fn new() -> HandlerFetchDirResponse {
        HandlerFetchDirResponse {
            list: Vec::new(),
        }
    }

    pub fn add(&mut self, item: GitTreeItem) {
        self.list.push(item);
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct HandlerFetchNodeResponse {
    pub content: String,
}

