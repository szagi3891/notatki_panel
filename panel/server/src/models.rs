#![allow(clippy::new_without_default)]

use serde::{Deserialize, Serialize};
use poem_openapi::{Object};

// pub type TimestampType = u128;


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Object)]
pub struct RootResponse {
    pub root: String,
}


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Object)]
pub struct HandlerFetchDirBody {
    pub id: String,
}


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Object)]
pub struct GitTreeItem {
    pub dir: bool,
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Object)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Object)]
pub struct HandlerFetchNodeBody {
    pub hash: String,
}


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Object)]
pub struct HandlerFetchNodeResponse {
    pub content: String,
}


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Object)]
pub struct HandlerSaveContentBody {
    pub path: Vec<String>,
    pub prev_hash: String,
    pub new_content: String,
}


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Object)]
pub struct HandlerCreateFileBody {
    pub path: Vec<String>,
    pub new_name: String,
    pub new_content: String,
}



#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Object)]
pub struct HandlerCreateDirBody {
    pub path: Vec<String>,
    pub dir: String,
}


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Object)]
pub struct HandlerRenameItemBody {
    pub path: Vec<String>,
    pub prev_name: String,
    pub prev_hash: String,
    pub new_name: String,
}



#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Object)]
pub struct HandlerDeleteItemBody {
    pub path: Vec<String>,
    pub hash: String,
}


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Object)]
pub struct HandlerMoveItemBody {
    pub path: Vec<String>,
    pub hash: String,
    pub new_path: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Object)]
pub struct HandlerAddFiles {
    pub path: Vec<String>,
    pub files: Vec<HandlerAddFilesFile>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Object)]
pub struct HandlerAddFilesFile {
    pub name: String,
    pub blob_id: String,
}