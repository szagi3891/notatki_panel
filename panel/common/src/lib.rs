use serde::{Deserialize, Serialize};
use vertigo::{RequestTrait, make_serde_request_trait};

pub type TimestampType = u128;


make_serde_request_trait!(RootResponse);
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RootResponse {
    pub root: String,
}


make_serde_request_trait!(HandlerFetchDirBody);
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

make_serde_request_trait!(HandlerFetchDirResponse);
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

make_serde_request_trait!(HandlerFetchNodeBody);
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct HandlerFetchNodeBody {
    pub hash: String,
}


make_serde_request_trait!(HandlerFetchNodeResponse);
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct HandlerFetchNodeResponse {
    pub content: String,
}


make_serde_request_trait!(HandlerSaveContentBody);
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct HandlerSaveContentBody {
    pub path: Vec<String>,
    pub prev_hash: String,
    pub new_content: String,
}


make_serde_request_trait!(HandlerCreateFileBody);
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct HandlerCreateFileBody {
    pub path: Vec<String>,
    pub new_name: String,
    pub new_content: String,
}



make_serde_request_trait!(HandlerCreateDirBody);
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct HandlerCreateDirBody {
    pub path: Vec<String>,
    pub dir: String,
}


make_serde_request_trait!(HandlerRenameItemBody);
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct HandlerRenameItemBody {
    pub path: Vec<String>,
    pub prev_name: String,
    pub prev_hash: String,
    pub new_name: String,
}



make_serde_request_trait!(HandlerDeleteItemBody);
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct HandlerDeleteItemBody {
    pub path: Vec<String>,
    pub hash: String,
}

