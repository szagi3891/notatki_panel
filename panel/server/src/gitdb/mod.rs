use std::collections::BTreeMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::fs;
use common::{
    DataNodeIdType,
    TimestampType,
    DataPost,
    DataNode,
};
use crate::utils::time::get_current;

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

struct Autoid {
    path: String,
}

impl Autoid {
    pub fn new(path: String) -> Autoid {
        println!("path={}", path);
        
        Autoid {
            path
        }
    }

    pub async fn get_next_id(&mut self) -> DataNodeIdType {
        let data = fs::read_to_string(&self.path).await.unwrap();

        let mut current_counter = serde_json::from_str::<DataNodeIdType>(&data).unwrap();

        current_counter += 1;

        let data_to_save = serde_json::to_string(&current_counter).unwrap();

        fs::write(&self.path, data_to_save).await.unwrap();

        current_counter
    }
}

// SaveError

#[derive(Clone)]
pub struct GitDB {
    dir_path: Arc<String>,
    data: Arc<RwLock<BTreeMap<DataNodeIdType, ItemInfo>>>,
    autoid: Arc<RwLock<Autoid>>,
}

impl GitDB {
    pub fn new(dir_path: String) -> GitDB {
        let autoid = Autoid::new(format!("{}/autoid", dir_path));

        GitDB {
            dir_path: Arc::new(dir_path),
            data: Arc::new(RwLock::new(BTreeMap::new())),
            autoid: Arc::new(RwLock::new(autoid))
        }
    }

    async fn get_next_id(&self) -> DataNodeIdType {
        let mut lock = self.autoid.write().await;
        let next_id = (*lock).get_next_id().await;
        next_id
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

    pub async fn create_file(&self, parent_id: DataNodeIdType, name: String) -> Result<(), SaveError> {
        let next_id = self.get_next_id().await;

        let node = DataNode::File {
            id: next_id,
            title: name,
            content: "".into(),
        };

        let timestamp = get_current();

        self.save(next_id, timestamp, node).await
    }
}
