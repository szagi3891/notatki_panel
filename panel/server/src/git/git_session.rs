use git2::{BranchType, ObjectType, Oid, Repository, Tree, TreeBuilder, TreeEntry};
use crate::utils::ErrorProcess;
use tokio::sync::{MutexGuard};
use common::GitTreeItem;
use tokio::task;

use crate::git::GitBlob;
use super::utils::{create_id, tree_entry_is_file};

pub struct GitId {
    pub id: Oid,
    is_file: bool,
}

impl GitId {
    pub fn new(repo: &MutexGuard<Repository>, id: Oid) -> Result<GitId, ErrorProcess> {
        let child_object = repo.find_object(id, None)?;
        Self::new_kind(id, child_object.kind())
    }

    pub fn new_kind(id: Oid, kind: Option<ObjectType>) -> Result<GitId, ErrorProcess> {
        match kind {
            Some(ObjectType::Tree) => {
                return Ok(GitId::new_dir(id));
            }, 
            Some(ObjectType::Blob) => {
                return Ok(GitId::new_file(id));
            },
            Some(kind) => {
                return Err(ErrorProcess::user(format!("Incorrect type object = {}, {}", id, kind)));
            },
            None => {
                return ErrorProcess::user_result(format!("It was not possible to determine the type of this object = {}", id));
            },
        };
    }

    pub fn new_file(id: Oid) -> GitId {
        GitId {
            id,
            is_file: true,
        }
    }

    pub fn new_dir(id: Oid) -> GitId {
        GitId {
            id,
            is_file: false,
        }
    }
}


struct GitTreeBuilder<'repo> {
    builder: TreeBuilder<'repo>,
}

impl<'repo> GitTreeBuilder<'repo> {
    pub fn new(builder: TreeBuilder<'repo>) -> GitTreeBuilder<'repo> {
        GitTreeBuilder {
            builder
        }
    }

    pub fn insert(&mut self, filename: &str, child: GitId) -> Result<(), ErrorProcess> {
        if child.is_file {
            self.builder.insert(filename, child.id, 0o100644)?;
        } else {
            self.builder.insert(filename, child.id, 0o040000)?;
        }

        Ok(())
    }

    pub fn remove(&mut self, filename: &str) -> Result<(), ErrorProcess> {
        self.builder.remove(filename)?;

        Ok(())
    }

    pub fn get_child(&self, name: &str) -> Result<Option<GitId>, ErrorProcess> {
        let child = self.builder.get(name)?;
        match child {
            Some(child) => {
                let git_id = GitId::new_kind(child.id(), child.kind())?;
                Ok(Some(git_id))
            },
            None => {
                Ok(None)
            }
        }
    }

    pub fn id(self) -> Result<Oid, ErrorProcess> {
        let id = self.builder.write()?;
        Ok(id)
    }

    pub fn is_exist(&self, name: &str) -> Result<bool, ErrorProcess> {
        let child = self.get_child(name)?;
        Ok(child.is_some())
    }

}


fn find_id<'repo>(session: &GitSession<'repo>, id: Oid) -> Result<GitId, ErrorProcess> {
    GitId::new(&session.repo, id)
}

fn find_tree<'repo, 'session: 'repo>(session: &'session GitSession<'repo>, id: Oid) -> Result<Tree<'repo>, ErrorProcess> {
    let result = session.repo.find_object(id, None);
    let result = match result {
        Ok(result) => result,
        Err(_) => {
            panic!("dsa");
        }
    };

    let tree = result.peel_to_tree()?;
    Ok(tree)
}


fn get_child_tree<'repo, 'session: 'repo>(
    session: &'session GitSession<'repo>,
    tree: &Tree,
    name: &String
) -> Result<Tree<'repo>, ErrorProcess> {
    
    for item in tree {
        if item.name() == Some(name.as_str()) {
            let tree = find_tree(&session, item.id())?;
            return Ok(tree);
        }
    }

    ErrorProcess::user_result(format!("Element not found {}", name))
}

fn put_child_tree<'repo>(
    session: &GitSession<'repo>,
    tree: &Tree,
    filename: &String,
    child: Oid
) -> Result<Oid, ErrorProcess> {

    let child_object = session.repo.find_object(child, None)?;

    match child_object.kind() {
        Some(ObjectType::Tree) => {}, 
        Some(ObjectType::Blob) => {},
        Some(kind) => {
            return Err(ErrorProcess::user(format!("Incorrect type object = {}, {}", child, kind)));
        },
        None => {
            return ErrorProcess::user_result(format!("It was not possible to determine the type of this object = {}", child));
        },
    };

    let mut builder = session.repo.treebuilder(Some(tree))?;
    builder.insert(filename, child, 0o040000)?;
    let write_result = builder.write()?;

    Ok(write_result)
}


fn find_and_change_path_small<
    'repo,
    'session: 'repo,
    R,
    M: FnOnce(&mut GitTreeBuilder<'repo>) -> Result<R, ErrorProcess>
>(
    session: &'session GitSession<'repo>,
    tree: &Tree<'repo>,
    path: &[String],
    modify: M
) -> Result<(Oid, R), ErrorProcess> {
    if let Some((first, rest_path)) = path.split_first() {
        
        let child_tree = get_child_tree(&session, &tree, first)?;
        let (child_tree_modify, result) = find_and_change_path_small(&session, &child_tree, rest_path, modify)?;
        let tree_modify = put_child_tree(&session, &tree, first, child_tree_modify)?;

        Ok((tree_modify, result))

    } else {
        let builder = session.repo.treebuilder(Some(tree))?;

        let mut treebuilder = GitTreeBuilder::new(builder);
        let result = modify(&mut treebuilder)?;
        let new_id = treebuilder.id()?;
        Ok((new_id, result))
    }
}


fn find_and_change_path<
    'repo,
    'session: 'repo,
    R,
    M: FnOnce(&mut GitTreeBuilder<'repo>) -> Result<R, ErrorProcess>
>(
    session: &'session GitSession<'repo>,
    path: &[String],
    modify: M
) -> Result<(Oid, R), ErrorProcess> {

    let result = session.repo.find_object(session.root, None)?;
    let tree = result.peel_to_tree()?;

    find_and_change_path_small(session, &tree, path, modify)
}


pub fn create_file_content<'repo>(
    session: &GitSession<'repo>,
    path: &[String],
    new_content: &String,
) -> Result<GitId, ErrorProcess> {
    if let Some((name_item, rest_path)) = path.split_first() {
        
        let rest_id = create_file_content(&session, rest_path, new_content)?;

        let mut builder = session.repo.treebuilder(None)?;

        if rest_id.is_file {
            builder.insert(name_item, rest_id.id, 0o100644)?;
        } else {
            builder.insert(name_item, rest_id.id, 0o040000)?;
        }
        
        let write_result = builder.write()?;
        let write_result = find_id(session, write_result)?;

        Ok(write_result)

    } else {
        let new_content_id = session.repo.blob(new_content.as_bytes())?;
        let new_content_id = find_id(session, new_content_id)?;
        Ok(new_content_id)
    }
}

pub fn command_find_blob<'repo>(
    session: &GitSession<'repo>,
    id: &String
) -> Result<Option<GitBlob>, ErrorProcess> {
    let oid = create_id(id)?;

    if let Ok(tree) = session.repo.find_tree(oid) {
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

    if let Ok(blob) = session.repo.find_blob(oid) {
        let content = blob.content();
        let content = Vec::from(content);

        return Ok(Some(GitBlob::Blob { content }));
    }

    Ok(None)
}


pub fn commit<'repo>(
    session: GitSession<'repo>,
) -> Result<String, ErrorProcess> {
    let new_tree = find_tree(&session, session.root.clone())?;

    let branch = session.repo.find_branch(session.branch_name.as_str(), BranchType::Local)?;
    let reference = branch.get();

    let commit = reference.peel_to_commit()?;

    let update_ref = format!("refs/heads/{}", session.branch_name);
    //HEAD

    session.repo.commit(
        Some(update_ref.as_str()),   //"heads/master"),
        &commit.author(),
        &commit.committer(),
        "auto save",
        &new_tree,
        &[&commit]
    )?;

    Ok(session.root.to_string())
}


fn create_blob<'repo>(session: &GitSession<'repo>, new_content: String) -> Result<GitId, ErrorProcess> {
    let new_content_id = session.repo.blob(new_content.as_bytes())?;
    Ok(GitId::new_file(new_content_id))
}


fn convert_to_name(item: &TreeEntry) -> Result<String, ErrorProcess> {
    let name = item.name();

    match name {
        Some(str) => Ok(String::from(str)),
        None => ErrorProcess::server_result("One of the tree elements has an invalid utf8 name")
    }
}

pub struct GitSession<'repo> {
    root: Oid,
    branch_name: String,
    repo: MutexGuard<'repo, Repository>,
}

impl<'repo> GitSession<'repo> {
    pub fn new(repo: MutexGuard<'repo, Repository>, branch_name: &str) -> Result<GitSession<'repo>, ErrorProcess> {
        let id = {
            let branch = (*repo).find_branch(branch_name, BranchType::Local)?;
            let reference = branch.get();
            let tree = reference.peel_to_tree()?;
            tree.id()
        };

        Ok(GitSession {
            root: id,
            branch_name: branch_name.into(),
            repo
        })
    }

    pub async fn commit(self) -> Result<String, ErrorProcess> {
        task::block_in_place(move || {
            commit(self)
        })
    }

    pub async fn command_main_commit(
        self,
    ) -> Result<String, ErrorProcess> {
        Ok(self.root.to_string())
    }

    pub async fn create_blob(self, new_content: String) -> Result<(GitSession<'repo>, GitId), ErrorProcess> {
        task::block_in_place(move || {
            let new_content_id = create_blob(&self, new_content)?;
            Ok((self, new_content_id))
        })
    }

    pub async fn command_find_blob(
        self,
        id: &String
    ) -> Result<(GitSession<'repo>, Option<GitBlob>), ErrorProcess> {
        
        let new_self = task::block_in_place(move || {
            let result = command_find_blob(&self, id)?;
            Ok((self, result))
        });

        new_self
    }

    pub async fn create_file_content(self, new_path: &[String], content: &String) -> Result<(GitSession<'repo>, GitId), ErrorProcess> {
        task::block_in_place(move || {
            let new_content_id = create_file_content(&self, new_path, content)?;
            Ok((self, new_content_id))
        })
    }

    pub async fn insert_child(mut self, path: &Vec<String>, new_child_item: &String, new_content_id: GitId) -> Result<GitSession<'repo>, ErrorProcess> {
        task::block_in_place(move || -> Result<GitSession<'repo>, ErrorProcess> {
            let (new_root, _) = find_and_change_path(&self, &path, move |tree_builder: &mut GitTreeBuilder<'repo>| -> Result<(), ErrorProcess> {
                let is_exist = tree_builder.is_exist(new_child_item.as_str())?;

                if is_exist {
                    return ErrorProcess::user_result(format!("this element already exists - {}", new_child_item));
                }

                let tree_builder = tree_builder.insert(new_child_item, new_content_id)?;

                Ok(tree_builder)
            })?;

            self.root = new_root;

            Ok(self)
        })
    }

    pub async fn remove_child(mut self, path: &Vec<String>, child_name: &String) -> Result<(GitSession<'repo>, GitId), ErrorProcess> {
        task::block_in_place(move || {
            let (new_root, result)  = find_and_change_path(&self, &path, move |tree_builder: &mut GitTreeBuilder<'repo>| -> Result<GitId, ErrorProcess> {
                let current_child = tree_builder.get_child(child_name.as_str())?.ok_or_else(|| {
                    ErrorProcess::user("this element not exists")
                        .context("command_rename_item prev_name", &child_name)
                })?;

                tree_builder.remove(child_name.as_str())?;

                Ok(current_child)
            })?;

            self.root = new_root;

            Ok((self, result))
        })
    }

    pub fn create_id(&self, hash: &String) -> Result<GitId, ErrorProcess> {
        let id = create_id(hash)?;
        let git_id = GitId::new(&self.repo, id)?;
        Ok(git_id)
    }
}


