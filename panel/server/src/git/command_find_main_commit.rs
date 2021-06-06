use git2::{
    Repository,
    BranchType,
};
use crate::utils::ErrorProcess;


pub fn command_find_main_commit<'repo>(repo: &'repo Repository, branch: String) -> Result<String, ErrorProcess> {
    let branch = repo.find_branch(branch.as_str(), BranchType::Local)?;
    let reference = branch.get();
    let tree = reference.peel_to_tree()?;

    Ok(tree.id().to_string())
}
