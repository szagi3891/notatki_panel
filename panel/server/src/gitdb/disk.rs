use std::sync::Arc;
use common::{
    DataNode,
    DataNodeIdType,
    DataPost,
};

use crate::utils::time::get_current;
use super::dir::{get_path, get_dir};

pub async fn save_node(dir_path: &Arc<String>, id: &DataNodeIdType, node: DataNode) {
    let dir = get_dir(&dir_path, &id);
    tokio::fs::create_dir_all(dir).await.unwrap();

    let file = get_path(&dir_path, &id);

    let data_to_save = serde_json::to_string(&DataPost {
        timestamp: get_current(),
        node,
    }).unwrap();

    tokio::fs::write(file, data_to_save).await.unwrap();
}

