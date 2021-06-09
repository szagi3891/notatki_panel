use git2::{
    Repository,
    BranchType,
    ObjectType,
    Tree,
    Oid,
};
use crate::utils::ErrorProcess;


pub fn create_id(hash: String) -> Result<Oid, ErrorProcess> {
    match Oid::from_str(&hash) {
        Ok(id) => Ok(id),
        Err(err) => {
            ErrorProcess::user(format!("Invalid hash {} {}", hash, err))
        }
    }
}


fn find_tree<'repo>(repo: &'repo Repository, id: Oid) -> Result<Tree<'repo>, ErrorProcess> {
    let result = repo.find_object(id, None)?;
    let tree = result.peel_to_tree()?;
    Ok(tree)
}

fn get_child_tree<'repo>(
    repo: &'repo Repository, 
    tree: &Tree<'repo>,
    name: &String
) -> Result<Tree<'repo>, ErrorProcess> {
    
    for item in tree {
        if item.name() == Some(name.as_str()) {
            let tree = find_tree(repo, item.id())?;
            return Ok(tree);
        }
    }

    ErrorProcess::user(format!("Element not found {}", name))
}

fn put_child_tree<'repo>(
    repo: &Repository,
    tree: &Tree<'repo>,
    filename: &String,
    child: Oid
) -> Result<Oid, ErrorProcess> {

    let child_object = repo.find_object(child, None)?;

    match child_object.kind() {
        Some(ObjectType::Tree) => {}, 
        Some(ObjectType::Blob) => {},
        Some(kind) => {
            return ErrorProcess::user(format!("Incorrect type object = {}, {}", child, kind));
        },
        None => {
            return ErrorProcess::user(format!("It was not possible to determine the type of this object = {}", child));
        },
    };

    let mut builder = repo.treebuilder(Some(tree))?;
    builder.insert(filename, child, 0o040000)?;
    let write_result = builder.write()?;

    Ok(write_result)
}

fn find_and_change<
    'repo,
    M: Fn(&Repository, Tree<'repo>) -> Result<Oid, ErrorProcess>
>(
    repo: &'repo Repository,
    tree: Tree<'repo>, 
    path: &[String],
    modify: M
) -> Result<Oid, ErrorProcess> {
    if let Some((first, rest_path)) = path.split_first() {
        
        let child_tree = get_child_tree(repo, &tree, first)?;
        let child_tree_modify = find_and_change(repo, child_tree, rest_path, modify)?;
        let tree_modify = put_child_tree(repo, &tree, first, child_tree_modify)?;

        Ok(tree_modify)

    } else {
        modify(repo, tree)
    }
}

pub fn create_file_content<'repo>(
    repo: &'repo Repository,
    path: &[String],
    new_content: &String,
) -> Result<(Oid, bool), ErrorProcess> {
    if let Some((name_item, rest_path)) = path.split_first() {
        
        let (rest_id, is_file) = create_file_content(repo, rest_path, new_content)?;

        let mut builder = repo.treebuilder(None)?;

        if is_file {
            builder.insert(name_item, rest_id, 0o100644)?;
        } else {
            builder.insert(name_item, rest_id, 0o040000)?;
        }
        
        let write_result = builder.write()?;

        Ok((write_result, false))

    } else {
        let new_content_id = repo.blob(new_content.as_bytes())?;
        Ok((new_content_id, true))
    }
}

pub fn find_and_commit<
    'repo,
    M: Fn(&Repository, Tree<'repo>) -> Result<Oid, ErrorProcess>
>(
    repo: &'repo Repository,
    branch_name: &String,
    path: &[String],
    modify: M
) -> Result<Oid, ErrorProcess> {
    let branch = repo.find_branch(branch_name.as_str(), BranchType::Local).unwrap();
    let reference = branch.get();
    let curret_root_tree = reference.peel_to_tree()?;

    let new_tree_id = find_and_change(
        &repo,
        curret_root_tree,
        &path,
        modify
    )?;

    let new_tree = find_tree(&repo, new_tree_id)?;

    let commit = reference.peel_to_commit()?;

    let update_ref = format!("refs/heads/{}", branch_name);
    //HEAD

    repo.commit(
        Some(update_ref.as_str()),   //"heads/master"),
        &commit.author(),
        &commit.committer(),
        "auto save",
        &new_tree,
        &[&commit]
    )?;

    Ok(new_tree_id)
}
