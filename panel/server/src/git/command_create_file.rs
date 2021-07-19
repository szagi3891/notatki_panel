use git2::{
    Repository,
    Tree,
    Oid,
};
use crate::utils::ErrorProcess;
use super::utils::{RepoWrapper, create_file_content};

pub fn command_create_file<'repo>(
    repo: &'repo Repository,
    branch_name: &String,
    path: Vec<String>,      //wskazuje na katalog w którym utworzymy nową treść
    new_path: Vec<String>,  //mona od razu utworzyc potrzebne podktalogi
    new_content: String,
) -> Result<String, ErrorProcess> {

    let new_tree_id = RepoWrapper::new(repo, branch_name)?
        .find_and_modify(&path, move |repo: &Repository, tree: Tree| -> Result<Oid, ErrorProcess> {
            if let Some((first_item_name, rest_path)) = new_path.split_first() {

                let child = tree.get_name(first_item_name.as_str());

                if child.is_some() {
                    return ErrorProcess::user_result(format!("this element already exists - {}", first_item_name));
                }

                let (new_content_id, is_file) = create_file_content(repo, rest_path, &new_content)?;

                let mut builder = repo.treebuilder(Some(&tree))?;
                
                if is_file {
                    builder.insert(first_item_name, new_content_id, 0o100644)?;
                } else {
                    builder.insert(first_item_name, new_content_id, 0o040000)?;
                }

                let id = builder.write()?;

                Ok(id)

            } else {
                ErrorProcess::user_result("new_path - must be a non-empty list")
            }

        })?
        .commit()?;

    Ok(new_tree_id.to_string())
}