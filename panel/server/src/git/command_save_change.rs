use git2::{
    Repository,
    Tree,
    Oid,
};
use crate::utils::ErrorProcess;

use super::utils::RepoWrapper;
// use super::utils::find_and_commit;

pub fn command_save_change<'repo>(
    repo: &'repo Repository,
    branch_name: &String,
    mut path: Vec<String>,
    prev_hash: String,
    new_content: String
) -> Result<String, ErrorProcess> {

    let file_name = path.pop().unwrap();

    let new_tree_id = RepoWrapper::new(repo, branch_name)?
        .find_and_modify(&path, move |repo: &Repository, tree: Tree| -> Result<Oid, ErrorProcess> {
            
            let child = tree.get_name(file_name.as_str())
                .ok_or_else(|| ErrorProcess::user(format!("item not found to be modified = {}", &file_name)))?;

            if child.id().to_string() != prev_hash {
                return ErrorProcess::user_result(format!("item not found to be modified = {}, hash mismatch", file_name));
            }

            let mut builder = repo.treebuilder(Some(&tree))?;
            let new_content_id = repo.blob(new_content.as_bytes())?;
            builder.insert(&file_name, new_content_id, 0o100644)?;
            //0o100755

            let id = builder.write()?;

            Ok(id)
        })?
        .commit()?;

    Ok(new_tree_id.to_string())
}
