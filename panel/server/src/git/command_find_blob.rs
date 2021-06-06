use common::GitTreeItem;
use serde::{Deserialize, Serialize};

use git2::{
    Repository,
    ObjectType,
    TreeEntry,
};
use crate::utils::ErrorProcess;
use super::utils::create_id;


#[derive(Debug, Serialize, Deserialize)]
pub enum GitBlob {
    Blob {
        content: Vec<u8>,
    },
    Tree {
        list: Vec<GitTreeItem>,
    }
}


fn convert_to_name(item: &TreeEntry) -> Result<String, ErrorProcess> {
    let name = item.name();

    match name {
        Some(str) => Ok(String::from(str)),
        None => Err(ErrorProcess::server("One of the tree elements has an invalid utf8 name"))
    }
}

fn convert_to_type(item: &TreeEntry) -> Result<bool, ErrorProcess> {
    let kind = item.kind();

    match kind {
        Some(ObjectType::Tree) => Ok(true),
        Some(ObjectType::Blob) => Ok(false),
        _ => Err(ErrorProcess::server("Trees only support 'ObjectType::Tree' and 'ObjectType::Blob'"))
    }
}

pub fn command_find_blob(repo: &Repository, id: String) -> Result<Option<GitBlob>, ErrorProcess> {
    let oid = create_id(id)?;

    if let Ok(tree) = repo.find_tree(oid) {
        let mut list: Vec<GitTreeItem> = Vec::new();

        for item in tree.iter() {
            list.push(GitTreeItem {
                dir: convert_to_type(&item)?,
                id: item.id().to_string(),
                name: convert_to_name(&item)?,
            });
        }

        return Ok(Some(GitBlob::Tree { list }));
    }

    if let Ok(blob) = repo.find_blob(oid) {
        let content = blob.content();
        let content = Vec::from(content);

        return Ok(Some(GitBlob::Blob { content }));
    }

    Ok(None)
}

