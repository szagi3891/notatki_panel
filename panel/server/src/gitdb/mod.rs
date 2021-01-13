use std::collections::BTreeMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use common::{
    DataNodeIdType,
    TimestampType,
    DataPost,
    DataNode,
};

fn get_path_chunks(dir_path: &String, id: &DataNodeIdType) -> Vec<String> {
    let mut id = *id as u64;

    if id == 0 {
        return vec!(dir_path.clone(), "f000".into());
    }

    let mut chunks: Vec<String> = Vec::new();
    let mut is_first_exec = false;

    while id > 0 {
        let prefix = if is_first_exec == false {
            is_first_exec = true;
            "f"
        } else {
            "d"
        };
    
        chunks.push(format!("{}{:03}", prefix, id % 1000));
        id = id / 1000;
    }

    let mut result: Vec<String> = Vec::new();

    result.push(dir_path.clone());

    for item in chunks.iter().rev() {
        result.push(item.clone());
    }

    result
}

fn get_dir(dir_path: &String, id: &DataNodeIdType) -> String {
    let mut result: Vec<String> = get_path_chunks(dir_path, id);
    result.pop();
    result.join("/")
}

fn get_path(dir_path: &String, id: &DataNodeIdType) -> String {
    let mut result: Vec<String> = get_path_chunks(dir_path, id);
    result.join("/")
}

#[test]
fn test_get_path() {
    assert_eq!(get_path(&"/bazowy/katalog".into(), &(0 as DataNodeIdType)), String::from("/bazowy/katalog/f000"));
    assert_eq!(get_path(&"/bazowy/katalog".into(), &(43 as DataNodeIdType)), String::from("/bazowy/katalog/f043"));
    assert_eq!(get_path(&"/bazowy/katalog".into(), &(234222 as DataNodeIdType)), String::from("/bazowy/katalog/d234/f222"));
}

struct ItemInfoInner {
    _last_modification: Option<TimestampType>,
}

impl ItemInfoInner {
    pub fn new() -> ItemInfoInner {
        ItemInfoInner {
            _last_modification: None,
        }
    }
}

#[derive(Clone)]
struct ItemInfo {
    data: Arc<RwLock<ItemInfoInner>>,
}

impl ItemInfo {
    pub fn new() -> ItemInfo {
        ItemInfo {
            data: Arc::new(RwLock::new(ItemInfoInner::new()))
        }
    }
}

pub enum SaveError {
    Error(std::io::Error),
    OutdatedTimestamp,
}

impl From<std::io::Error> for SaveError {
    fn from(error: std::io::Error) -> Self {
        SaveError::Error(error)
    }
}

impl From<serde_json::error::Error> for SaveError {
    fn from(error: serde_json::error::Error) -> Self {
        SaveError::Error(error.into())
    }
}


// SaveError

pub struct GitDB {
    dir_path: String,
    data: Arc<RwLock<BTreeMap<DataNodeIdType, ItemInfo>>>,
}

impl GitDB {
    pub fn new(dir_path: String) -> GitDB {
        GitDB {
            dir_path,
            data: Arc::new(RwLock::new(BTreeMap::new()))
        }
    }

    async fn get_or_create(&self, id: DataNodeIdType) -> ItemInfo {
        let mut lock = self.data.write().await;

        if let Some(item) = lock.get(&id) {
            return item.clone();
        }

        let new_item = ItemInfo::new();
        lock.insert(id, new_item.clone());
        new_item
    }

    pub async fn get(&self, id: DataNodeIdType) -> Result<DataPost, std::io::Error> {
        let item = self.get_or_create(id).await;
        let lock = item.data.write().await;
        
        let file_path = get_path(&self.dir_path, &id);

        let data = tokio::fs::read(file_path).await?;

        let result: Result<DataPost, _> = serde_json::from_slice(data.as_ref());
        let result = result?;

        std::mem::forget(lock);

        Ok(result)
    }

    pub async fn save(&self, id: DataNodeIdType, timestamp: TimestampType, mut node: DataNode) -> Result<(), SaveError> {
        let item = self.get_or_create(id).await;
        let lock = item.data.write().await;

        let data_post = self.get(id).await?;

        if data_post.timestamp != timestamp {
            return Err(SaveError::OutdatedTimestamp);
        }

        let dir = get_dir(&self.dir_path, &id);
        let file = get_path(&self.dir_path, &id);

        tokio::fs::create_dir_all(dir).await?;


        let data_to_save = DataPost {
            timestamp,
            node,
        };

        let data_to_save = serde_json::to_string(&data_to_save)?;

        tokio::fs::write(file, data_to_save).await?;

        std::mem::forget(lock);
        Ok(())
    }
}
