use std::sync::Arc;
use tokio::sync::RwLock;
use common::{
    DataNode,
    DataNodeIdType,
    DataPost,
    TimestampType,
};

use super::{NodeError, dir::get_path, disk::save_node};
use tokio::sync::RwLockWriteGuard;


pub struct ItemInner {
    dir_path: Arc<String>,
    id: DataNodeIdType,
}

impl ItemInner {    
    async fn save(&self, node: DataNode) {
        save_node(&self.dir_path, &self.id, node).await;
    }

    pub async fn get(&self) -> Result<DataPost, NodeError> {
        let file_path = get_path(&self.dir_path, &self.id);

        let data = tokio::fs::read(&file_path).await;

        let data = match data {
            Ok(data) => data,
            Err(err) => {
                return Err(NodeError::new(
                    format!("read path: {}", file_path),
                    format!("{}", err))
                );
            }
        };

        let result = serde_json::from_slice::<DataPost>(data.as_ref());
        let result = match result {
            Ok(result) => result,
            Err(err) => {
                return Err(NodeError::new(
                    format!("path: {}", file_path),
                    format!("{}", err))
                );
            }
        };

        Ok(result)
    }

    pub async fn save_with_check_timestamp(&self, timestamp: TimestampType, node: DataNode) -> Result<(), NodeError> {
        let data_post = self.get().await?;

        if data_post.timestamp != timestamp {
            return Err(NodeError::new(format!("node: {}", self.id), "OutdatedTimestamp"));
        }

        self.save(node).await;

        Ok(())
    }

    pub async fn add_child(&self, child_id: DataNodeIdType) -> Result<(), NodeError> {
        let mut data = self.get().await?.node;

        match &mut data {
            DataNode::Dir { child, .. } => {
                child.push(child_id);
            },
            DataNode::File { .. } => {
                panic!("nie mozna dodac dziecka do pliku");
            }
        }

        self.save(data).await;

        Ok(())
    }

    pub async fn create_empty_dir_if_not_exist(&self, name: &str) {
        let file = get_path(&self.dir_path, &self.id);

        log::info!("root check {}", &file);

        let metadata = tokio::fs::metadata(file).await;

        match metadata {
            Ok(metadata) => {
                if metadata.is_dir() || metadata.is_file() {
                    return;
                }

                panic!("IncorrectRootNode");
            },
            Err(_) => {
                self.save(DataNode::Dir {
                    id: self.id,
                    title: name.into(),
                    child: Vec::new()
                }).await;

                return;
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

