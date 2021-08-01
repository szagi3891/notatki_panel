use git2::{ObjectType, Oid, TreeEntry};
use crate::utils::ErrorProcess;

pub fn create_id(hash: &String) -> Result<Oid, ErrorProcess> {
    match Oid::from_str(&hash) {
        Ok(id) => Ok(id),
        Err(err) => {
            ErrorProcess::user_result(format!("Invalid hash {} {}", hash, err))
        }
    }
}

pub fn tree_entry_is_file(child: &TreeEntry) -> Result<bool, ErrorProcess> {
    let child_kind = child.kind()
        .ok_or_else(|| ErrorProcess::user("Problem with reading the 'kind' for"))?;

    if child_kind == ObjectType::Tree {
        Ok(false)
    } else if child_kind == ObjectType::Blob {
        Ok(true)
    } else {
        Err(
            ErrorProcess::user("tree_entry_is_file - unsupported type")
                .context("child.id", child.id())
                .context("kind", child_kind)
        )
    }
}
