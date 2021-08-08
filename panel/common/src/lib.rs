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
pub struct HandlerFetchNodeBody {
    pub hash: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct HandlerFetchNodeResponse {
    pub content: String,
}


#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct HandlerSaveContentBody {
    pub path: Vec<String>,
    pub prev_hash: String,
    pub new_content: String,
}


#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct HandlerSaveContentResponse {
    pub new_root: String,
}


#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct HandlerCreateFileBody {
    pub path: Vec<String>,
    pub new_path: Vec<String>,
    pub new_content: String,
}


#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct HandlerCreateFileResponse {
    pub new_root: String,
}



#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct HandlerRenameItemBody {
    pub path: Vec<String>,
    pub prev_name: String,
    pub prev_hash: String,
    pub new_name: String,
}


#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct HandlerRenameItemResponse {
    pub new_root: String,
}


#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct HandlerDeleteFileBody {
    pub path: Vec<String>,
    pub hash: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct HandlerDeleteFileResponse {
    pub new_root: String,
}