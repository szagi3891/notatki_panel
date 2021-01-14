
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

pub fn get_dir(dir_path: &String, id: &DataNodeIdType) -> String {
    let mut result: Vec<String> = get_path_chunks(dir_path, id);
    result.pop();
    result.join("/")
}

pub fn get_path(dir_path: &String, id: &DataNodeIdType) -> String {
    let mut result: Vec<String> = get_path_chunks(dir_path, id);
    result.join("/")
}

#[test]
fn test_get_path() {
    assert_eq!(get_path(&"/bazowy/katalog".into(), &(0 as DataNodeIdType)), String::from("/bazowy/katalog/f000"));
    assert_eq!(get_path(&"/bazowy/katalog".into(), &(43 as DataNodeIdType)), String::from("/bazowy/katalog/f043"));
    assert_eq!(get_path(&"/bazowy/katalog".into(), &(234222 as DataNodeIdType)), String::from("/bazowy/katalog/d234/f222"));
}