use git2::ObjectType;
use git2::{
    Repository,
    Tree,
    Oid,
};
use crate::utils::ErrorProcess;
use super::create_id;
use super::utils::find_and_commit;

pub fn command_rename_item<'repo>(
    repo: &'repo Repository,
    branch_name: &String,
    path: Vec<String>,          //wskazuje na katalog
    prev_name: String,          //mona od razu utworzyc potrzebne podktalogi
    prev_hash: String,
    new_name: String,
) -> Result<String, ErrorProcess> {

    let new_tree_id = find_and_commit(
        &repo,
        branch_name,
        &path,
        move |repo: &Repository, tree: Tree| -> Result<Oid, ErrorProcess> {
        
            let child = tree.get_name(prev_name.as_str());

            let child = match child {
                Some(child) => child,
                None => {
                    return ErrorProcess::user(format!("this element not exists - {}", prev_name));
                },
            };

            let child_is_file = {
                let child_kind = match child.kind() {
                    Some(child_kind) => child_kind,
                    None => {
                        return ErrorProcess::user(format!("Problem with reading the 'kind' for  - {}", prev_name));
                    }
                };

                if child_kind == ObjectType::Tree {
                    false
                } else if child_kind == ObjectType::Blob {
                    true
                } else {
                    return ErrorProcess::user(format!("Unsupported type  - {} {}", prev_name, child_kind));
                }
            };

            let prev_hash = create_id(prev_hash.clone())?;

            if child.id() != prev_hash {
                let prev_hash = prev_hash.to_string();
                let child_id = child.id().to_string();
                return ErrorProcess::user(format!("'prev_hash' does not match - {} {}", prev_hash, child_id));
            }

            let new_item_exist = {
                let new_item = tree.get_name(new_name.as_str());
                new_item.is_some()
            };

            if new_item_exist {
                return ErrorProcess::user(format!("New element exists - {}", new_name));
            }

            let mut builder = repo.treebuilder(Some(&tree))?;
            
            builder.remove(prev_name.as_str())?;

            if child_is_file  {
                builder.insert(new_name.clone(), child.id(), 0o100644)?;
            } else {
                builder.insert(new_name.clone(), child.id(), 0o040000)?;
            }

            let id = builder.write()?;

            Ok(id)

        }
    )?;

    Ok(new_tree_id.to_string())
}