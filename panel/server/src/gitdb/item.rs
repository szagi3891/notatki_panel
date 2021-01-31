use std::sync::Arc;
use tokio::sync::RwLock;
use common::{
    DataNode,
    DataNodeIdType,
    DataPost,
    TimestampType,
};

use super::{SaveError, dir::{get_dir, get_path}};
use tokio::sync::RwLockWriteGuard;

fn get_timestamp() -> u128 {
    use std::time::{SystemTime};
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis()
}


pub struct ItemInner {
    dir_path: Arc<String>,
    id: DataNodeIdType,
}

impl ItemInner {
    pub async fn create_base_dir(&self) -> Result<(), std::io::Error> {
        let dir = get_dir(&self.dir_path, &self.id);
        tokio::fs::create_dir_all(dir).await?;
        Ok(())
    }

    pub async fn get(&self) -> Result<DataPost, std::io::Error> {
        let file_path = get_path(&self.dir_path, &self.id);

        let data = tokio::fs::read(file_path).await?;

        let result: Result<DataPost, _> = serde_json::from_slice(data.as_ref());
        let result = result?;

        Ok(result)
    }

    pub async fn save(&self, node: DataNode) -> Result<(), std::io::Error> {
        let file = get_path(&self.dir_path, &self.id);

        let data_to_save = serde_json::to_string(&DataPost {
            timestamp: get_timestamp(),
            node,
        })?;

        tokio::fs::write(file, data_to_save).await
    }

    pub async fn create_empty_dir_if_not_exist(&self, name: &str) -> Result<(), SaveError> {
        let file = get_path(&self.dir_path, &self.id);

        log::info!("root check {}", &file);

        let metadata = tokio::fs::metadata(file).await;

        match metadata {
            Ok(metadata) => {
                if metadata.is_dir() || metadata.is_file() {
                    return Ok(());
                }

                return Err(SaveError::IncorrectRootNode);
            },
            Err(_) => {
                self.save(DataNode::Dir {
                    id: self.id,
                    title: name.into(),
                    child: Vec::new()
                }).await?;

                return Ok(());
            }
        }
    }
}


#[derive(Clone)]
pub struct ItemInfo {
    data: Arc<RwLock<ItemInner>>,
}

impl ItemInfo {
    pub fn new(dir_path: Arc<String>, id: DataNodeIdType) -> ItemInfo {
        let inner = ItemInner {
            dir_path,
            id,
        };

        ItemInfo {
            data: Arc::new(RwLock::new(inner)),
        }
    }

    pub async fn lock<'a>(&'a self) -> RwLockWriteGuard<'a, ItemInner> {
        let lock = self.data.write().await;
        lock
    }
}

