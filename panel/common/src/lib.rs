#![allow(clippy::new_without_default)]

pub type TimestampType = u128;

#[cfg_attr(feature = "api", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "client", derive(vertigo::AutoJsJson))]
#[derive(Debug, PartialEq, Eq)]
pub struct RootResponse {
    pub root: String,
}


#[cfg_attr(feature = "api", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "client", derive(vertigo::AutoJsJson))]
#[derive(Debug, PartialEq, Eq)]
pub struct HandlerFetchDirBody {
    pub id: String,
}

#[cfg_attr(feature = "api", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "client", derive(vertigo::AutoJsJson))]
#[derive(Debug, PartialEq, Eq)]
pub struct GitTreeItem {
    pub dir: bool,
    pub id: String,
    pub name: String,
}

#[cfg_attr(feature = "api", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "client", derive(vertigo::AutoJsJson))]
#[derive(Debug, PartialEq, Eq)]
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

#[cfg_attr(feature = "api", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "client", derive(vertigo::AutoJsJson))]
#[derive(Debug, PartialEq, Eq)]
pub struct HandlerFetchNodeBody {
    pub hash: String,
}


#[cfg_attr(feature = "api", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "client", derive(vertigo::AutoJsJson))]
#[derive(Debug, PartialEq, Eq)]
pub struct HandlerFetchNodeResponse {
    pub content: String,
}


#[cfg_attr(feature = "api", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "client", derive(vertigo::AutoJsJson))]
#[derive(Debug, PartialEq, Eq)]
pub struct HandlerSaveContentBody {
    pub path: Vec<String>,
    pub prev_hash: String,
    pub new_content: String,
}


#[cfg_attr(feature = "api", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "client", derive(vertigo::AutoJsJson))]
#[derive(Debug, PartialEq, Eq)]
pub struct HandlerCreateFileBody {
    pub path: Vec<String>,
    pub new_name: String,
    pub new_content: String,
}



#[cfg_attr(feature = "api", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "client", derive(vertigo::AutoJsJson))]
#[derive(Debug, PartialEq, Eq)]
pub struct HandlerCreateDirBody {
    pub path: Vec<String>,
    pub dir: String,
}


#[cfg_attr(feature = "api", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "client", derive(vertigo::AutoJsJson))]
#[derive(Debug, PartialEq, Eq)]
pub struct HandlerRenameItemBody {
    pub path: Vec<String>,
    pub prev_name: String,
    pub prev_hash: String,
    pub new_name: String,
}



#[cfg_attr(feature = "api", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "client", derive(vertigo::AutoJsJson))]
#[derive(Debug, PartialEq, Eq)]
pub struct HandlerDeleteItemBody {
    pub path: Vec<String>,
    pub hash: String,
}

#[cfg_attr(feature = "api", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "client", derive(vertigo::AutoJsJson))]
#[derive(Debug, PartialEq, Eq)]
pub struct HandlerMoveItemBody {
    pub path: Vec<String>,
    pub hash: String,
    pub new_path: Vec<String>,
}
