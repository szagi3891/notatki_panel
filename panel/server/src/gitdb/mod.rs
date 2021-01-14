use std::collections::BTreeMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use common::{
    DataNodeIdType,
    TimestampType,
    DataPost,
    DataNode,
};

mod item;
mod dir;

use item::ItemInfo;

#[derive(Debug)]
pub enum SaveError {
    Error(std::io::Error),
    OutdatedTimestamp,
    IncorrectRootNode
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

#[derive(Clone)]
pub struct GitDB {
    dir_path: Arc<String>,
    data: Arc<RwLock<BTreeMap<DataNodeIdType, ItemInfo>>>,
}

impl GitDB {
    pub fn new(dir_path: String) -> GitDB {
        GitDB {
            dir_path: Arc::new(dir_path),
            data: Arc::new(RwLock::new(BTreeMap::new()))
        }
    }

    async fn get_or_create(&self, id: DataNodeIdType) -> ItemInfo {
        let mut lock = self.data.write().await;

        if let Some(item) = lock.get(&id) {
            return item.clone();
        }

        let new_item = ItemInfo::new(self.dir_path.clone(), id);
        lock.insert(id, new_item.clone());
        new_item
    }

    pub async fn get(&self, id: DataNodeIdType) -> Result<DataPost, std::io::Error> {
        let item = self.get_or_create(id).await;
        let lock = item.lock().await;
        let result = lock.get().await?;
        Ok(result)
    }

    pub async fn save(&self, id: DataNodeIdType, timestamp: TimestampType, node: DataNode) -> Result<(), SaveError> {
        let data_post = self.get(id).await?;

        let item = self.get_or_create(id).await;
        let lock = item.lock().await;

        if data_post.timestamp != timestamp {
            return Err(SaveError::OutdatedTimestamp);
        }

        lock.create_base_dir().await?;
        lock.save(node).await?;

        Ok(())
    }

    pub async fn check_root(&self) -> Result<(), SaveError> {
        let root_id = 1;
    
        let item = self.get_or_create(root_id).await;
        let inner = item.lock().await;

        inner.create_base_dir().await?;
        inner.create_empty_dir_if_not_exist("root").await?;

        Ok(())
    }
}
