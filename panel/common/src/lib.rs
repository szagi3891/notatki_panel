use serde::{Deserialize, Serialize};
use vertigo::RequestTrait;

pub type TimestampType = u128;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RootResponse {
    pub root: String,
}

impl RequestTrait for RootResponse {
    fn into_string(self) -> Result<String, String> {
        serde_json::to_string(&self)
            .map_err(|err| format!("error serialize {}", err))
    }

    fn from_string(data: &str) -> Result<Self, String> {
        serde_json::from_str::<Self>(data)
            .map_err(|err| format!("error deserialize {}", err))
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct HandlerFetchDirBody {
    pub id: String,
}

impl RequestTrait for HandlerFetchDirBody {
    fn into_string(self) -> Result<String, String> {
        serde_json::to_string(&self)
            .map_err(|err| format!("error serialize {}", err))
    }

    fn from_string(data: &str) -> Result<Self, String> {
        serde_json::from_str::<Self>(data)
            .map_err(|err| format!("error deserialize {}", err))
    }
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
impl RequestTrait for HandlerFetchDirResponse {
    fn into_string(self) -> Result<String, String> {
        serde_json::to_string(&self)
            .map_err(|err| format!("error serialize {}", err))
    }

    fn from_string(data: &str) -> Result<Self, String> {
        serde_json::from_str::<Self>(data)
            .map_err(|err| format!("error deserialize {}", err))
    }
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
impl RequestTrait for HandlerFetchNodeBody {
    fn into_string(self) -> Result<String, String> {
        serde_json::to_string(&self)
            .map_err(|err| format!("error serialize {}", err))
    }

    fn from_string(data: &str) -> Result<Self, String> {
        serde_json::from_str::<Self>(data)
            .map_err(|err| format!("error deserialize {}", err))
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct HandlerFetchNodeResponse {
    pub content: String,
}
impl RequestTrait for HandlerFetchNodeResponse {
    fn into_string(self) -> Result<String, String> {
        serde_json::to_string(&self)
            .map_err(|err| format!("error serialize {}", err))
    }

    fn from_string(data: &str) -> Result<Self, String> {
        serde_json::from_str::<Self>(data)
            .map_err(|err| format!("error deserialize {}", err))
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct HandlerSaveContentBody {
    pub path: Vec<String>,
    pub prev_hash: String,
    pub new_content: String,
}

impl RequestTrait for HandlerSaveContentBody {
    fn into_string(self) -> Result<String, String> {
        serde_json::to_string(&self)
            .map_err(|err| format!("error serialize {}", err))
    }

    fn from_string(data: &str) -> Result<Self, String> {
        serde_json::from_str::<Self>(data)
            .map_err(|err| format!("error deserialize {}", err))
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct HandlerCreateFileBody {
    pub path: Vec<String>,
    pub new_name: String,
    pub new_content: String,
}

impl RequestTrait for HandlerCreateFileBody {
    fn into_string(self) -> Result<String, String> {
        serde_json::to_string(&self)
            .map_err(|err| format!("error serialize {}", err))
    }

    fn from_string(data: &str) -> Result<Self, String> {
        serde_json::from_str::<Self>(data)
            .map_err(|err| format!("error deserialize {}", err))
    }
}


#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct HandlerCreateDirBody {
    pub path: Vec<String>,
    pub dir: String,
}


impl RequestTrait for HandlerCreateDirBody {
    fn into_string(self) -> Result<String, String> {
        serde_json::to_string(&self)
            .map_err(|err| format!("error serialize {}", err))
    }

    fn from_string(data: &str) -> Result<Self, String> {
        serde_json::from_str::<Self>(data)
            .map_err(|err| format!("error deserialize {}", err))
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct HandlerRenameItemBody {
    pub path: Vec<String>,
    pub prev_name: String,
    pub prev_hash: String,
    pub new_name: String,
}

impl RequestTrait for HandlerRenameItemBody {
    fn into_string(self) -> Result<String, String> {
        serde_json::to_string(&self)
            .map_err(|err| format!("error serialize {}", err))
    }

    fn from_string(data: &str) -> Result<Self, String> {
        serde_json::from_str::<Self>(data)
            .map_err(|err| format!("error deserialize {}", err))
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct HandlerDeleteItemBody {
    pub path: Vec<String>,
    pub hash: String,
}

impl RequestTrait for HandlerDeleteItemBody {
    fn into_string(self) -> Result<String, String> {
        serde_json::to_string(&self)
            .map_err(|err| format!("error serialize {}", err))
    }

    fn from_string(data: &str) -> Result<Self, String> {
        serde_json::from_str::<Self>(data)
            .map_err(|err| format!("error deserialize {}", err))
    }
}
