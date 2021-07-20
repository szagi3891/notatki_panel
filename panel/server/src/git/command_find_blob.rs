use common::GitTreeItem;
use serde::{Deserialize, Serialize};

use git2::{
    TreeEntry,
};
use crate::utils::ErrorProcess;
use super::utils::{RepoWrapper, create_id, tree_entry_is_file};


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
        None => ErrorProcess::server_result("One of the tree elements has an invalid utf8 name")
    }
}

pub fn command_find_blob(repo: &RepoWrapper, id: String) -> Result<Option<GitBlob>, ErrorProcess> {
    let oid = create_id(id)?;

    if let Ok(tree) = repo.repo.find_tree(oid) {
        let mut list: Vec<GitTreeItem> = Vec::new();

        for item in tree.iter() {
            list.push(GitTreeItem {
                dir: !tree_entry_is_file(&item)?,
                id: item.id().to_string(),
                name: convert_to_name(&item)?,
            });
        }

        return Ok(Some(GitBlob::Tree { list }));
    }

    if let Ok(blob) = repo.repo.find_blob(oid) {
        let content = blob.content();
        let content = Vec::from(content);

        return Ok(Some(GitBlob::Blob { content }));
    }

    Ok(None)
}

