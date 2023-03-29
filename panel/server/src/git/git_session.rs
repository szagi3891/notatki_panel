use git2::{BranchType, ObjectType, Oid, Repository, Tree, TreeBuilder, TreeEntry};
use crate::utils::ErrorProcess;
use tokio::sync::{MutexGuard};
use crate::models::GitTreeItem;
use tokio::task;

use crate::git::GitBlob;

#[derive(PartialEq, Eq, Debug)]
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
                Ok(GitId::new_dir(id))
            }, 
            Some(ObjectType::Blob) => {
                Ok(GitId::new_file(id))
            },
            Some(kind) => {
                Err(ErrorProcess::user(format!("Incorrect type object = {id}, {kind}")))
            },
            None => {
                ErrorProcess::user_result(format!("It was not possible to determine the type of this object = {id}"))
            },
        }
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

    pub fn convert_to_string(self) -> String {
        self.id.to_string()
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

fn create_id(hash: &String) -> Result<Oid, ErrorProcess> {
    match Oid::from_str(hash) {
        Ok(id) => Ok(id),
        Err(err) => {
            ErrorProcess::user_result(format!("Invalid hash {hash} {err}"))
        }
    }
}

fn find_id<'repo>(session: &GitSession<'repo>, id: Oid) -> Result<GitId, ErrorProcess> {
    GitId::new(&session.repo, id)
}

fn find_tree<'repo>(session: &'repo GitSession<'repo>, id: Oid) -> Result<Tree<'repo>, ErrorProcess> {
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


fn get_child_tree<'repo>(
    session: &'repo GitSession<'repo>,
    tree: &Tree,
    name: &String
) -> Result<Tree<'repo>, ErrorProcess> {
    
    for item in tree {
        if item.name() == Some(name.as_str()) {
            let tree = find_tree(session, item.id())?;
            return Ok(tree);
        }
    }

    ErrorProcess::user_result(format!("Element not found {name}"))
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
            return Err(ErrorProcess::user(format!("Incorrect type object = {child}, {kind}")));
        },
        None => {
            return ErrorProcess::user_result(format!("It was not possible to determine the type of this object = {child}"));
        },
    };

    let mut builder = session.repo.treebuilder(Some(tree))?;
    builder.insert(filename, child, 0o040000)?;
    let write_result = builder.write()?;

    Ok(write_result)
}


fn find_and_change_path_small<
    'repo,
    R,
    M: FnOnce(&mut GitTreeBuilder<'repo>) -> Result<R, ErrorProcess>
>(
    session: &'repo GitSession<'repo>,
    tree: &Tree<'repo>,
    path: &[String],
    modify: M
) -> Result<(Oid, R), ErrorProcess> {
    if let Some((first, rest_path)) = path.split_first() {
        
        let child_tree = get_child_tree(session, tree, first)?;
        let (child_tree_modify, result) = find_and_change_path_small(session, &child_tree, rest_path, modify)?;
        let tree_modify = put_child_tree(session, tree, first, child_tree_modify)?;

        Ok((tree_modify, result))

    } else {
        let (new_id, result) = {
            let builder = session.repo.treebuilder(Some(tree))?;

            let mut treebuilder = GitTreeBuilder::new(builder);
            let result = modify(&mut treebuilder)?;
            let new_id = treebuilder.id()?;

            (new_id, result)
        };

        Ok((new_id, result))
    }
}


fn find_and_change_path<
    'repo,
    R,
    M: FnOnce(&mut GitTreeBuilder) -> Result<R, ErrorProcess>
>(
    session: &GitSession<'repo>,
    path: &[String],
    modify: M
) -> Result<(Oid, R), ErrorProcess> {

    let result = session.repo.find_object(session.root, None)?;
    let tree = result.peel_to_tree()?;

    find_and_change_path_small(session, &tree, path, modify)
}


pub fn create_file_content<'repo>(
    session: &GitSession<'repo>,
    new_content: &String,
) -> Result<GitId, ErrorProcess> {
    let new_content_id = session.repo.blob(new_content.as_bytes())?;
    let new_content_id = find_id(session, new_content_id)?;
    Ok(new_content_id)
}

pub fn create_empty_dir<'repo>(
    session: &GitSession<'repo>,
) -> Result<GitId, ErrorProcess> {
    let builder = session.repo.treebuilder(None)?;
    
    let write_result = builder.write()?;
    let write_result = find_id(session, write_result)?;

    Ok(write_result)
}

fn tree_entry_is_file(child: &TreeEntry) -> Result<bool, ErrorProcess> {
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

fn command_find_blob<'repo>(
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
    message: String,
) -> Result<String, ErrorProcess> {
    let new_tree = find_tree(&session, session.root)?;

    let branch = session.repo.find_branch(session.branch_name.as_str(), BranchType::Local)?;
    let reference = branch.get();

    let commit = reference.peel_to_commit()?;

    let update_ref = format!("refs/heads/{}", session.branch_name);
    //HEAD

    session.repo.commit(
        Some(update_ref.as_str()),   //"heads/master"),
        &commit.author(),
        &commit.committer(),
        message.as_str(),
        &new_tree,
        &[&commit]
    )?;

    session.repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
    Ok(session.root.to_string())
}

fn create_blob<'repo>(session: &GitSession<'repo>, new_content: String) -> Result<GitId, ErrorProcess> {
    let new_content_id = session.repo.blob(new_content.as_bytes())?;
    Ok(GitId::new_file(new_content_id))
}

fn create_blob_vec_u8<'repo>(session: &GitSession<'repo>, new_content: Vec<u8>) -> Result<GitId, ErrorProcess> {
    let new_content_id = session.repo.blob(new_content.as_slice())?;
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

    pub async fn commit(self, message: String) -> Result<String, ErrorProcess> {
        task::block_in_place(move || {
            commit(self, message)
        })
    }

    pub fn end(self) {}

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

    pub async fn create_blob_vec_u8(self, new_content: Vec<u8>) -> Result<(GitSession<'repo>, GitId), ErrorProcess> {
        task::block_in_place(move || {
            let new_content_id = create_blob_vec_u8(&self, new_content)?;
            Ok((self, new_content_id))
        })
    }

    pub async fn get_from_id(
        self,
        id: &String
    ) -> Result<(GitSession<'repo>, Option<GitBlob>), ErrorProcess> {
        
        task::block_in_place(move || {
            let result = command_find_blob(&self, id)?;
            Ok((self, result))
        })
    }

    pub async fn create_file_content(self, content: &String) -> Result<(GitSession<'repo>, GitId), ErrorProcess> {
        task::block_in_place(move || {
            let new_content_id = create_file_content(&self, content)?;
            Ok((self, new_content_id))
        })
    }

    pub async fn create_empty_dir(self) -> Result<(GitSession<'repo>, GitId), ErrorProcess> {
        task::block_in_place(move || {
            let new_content_id = create_empty_dir(&self)?;
            Ok((self, new_content_id))
        })
    }

    pub async fn insert_child(self, path: &[String], new_child_item: &String, new_content_id: GitId) -> Result<GitSession<'repo>, ErrorProcess> {
        task::block_in_place(move || -> Result<GitSession<'repo>, ErrorProcess> {
            let mut session = self;

            let (new_root, _) = find_and_change_path(&session, path, move |tree_builder: &mut GitTreeBuilder| -> Result<(), ErrorProcess> {
                let is_exist = tree_builder.is_exist(new_child_item.as_str())?;

                if is_exist {
                    return ErrorProcess::user_result(format!("this element already exists - {new_child_item}"));
                }

                tree_builder.insert(new_child_item, new_content_id)?;

                Ok(())
            })?;

            session.root = new_root;

            Ok(session)
        })
    }

    pub async fn remove_child(self, path: &[String], child_name: &String) -> Result<(GitSession<'repo>, Option<GitId>), ErrorProcess> {
        task::block_in_place(move || {
            let mut session = self;

            let (new_root, result)  = find_and_change_path(&session, path, move |tree_builder: &mut GitTreeBuilder| -> Result<Option<GitId>, ErrorProcess> {
                let child_id = tree_builder.get_child(child_name.as_str())?;

                match child_id {
                    Some(child_id) => {
                        tree_builder.remove(child_name.as_str())?;
                        Ok(Some(child_id))
                    },
                    None => {
                        Ok(None)
                    }
                }
                
            })?;

            session.root = new_root;

            Ok((session, result))
        })
    }

    pub async fn extract_child(self, path: &[String], child_name: &String) -> Result<(GitSession<'repo>, GitId), ErrorProcess> {
        let (session, child) = self.remove_child(path, child_name).await?;

        let child = match child {
            Some(child) => child,
            None => {
                return Err(ErrorProcess::user(format!("No file exists in the location: {}/{}", path.join("/"), child_name)));
            }
        };

        Ok((session, child))
    }

    fn create_id(&self, hash: &String) -> Result<GitId, ErrorProcess> {
        let id = create_id(hash)?;
        let git_id = GitId::new(&self.repo, id)?;
        Ok(git_id)
    }

    pub fn should_eq(&self, child: &GitId, hash: &String) -> Result<(), ErrorProcess> {
        let hash = self.create_id(hash)?;

        if hash != *child {
            return ErrorProcess::user_result(format!("'hash' does not match - child={child:?} hash={hash:?}"));
        }

        Ok(())
    }
}


