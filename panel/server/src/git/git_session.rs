use git2::{BranchType, ObjectType, Oid, Repository, Tree, TreeBuilder, TreeEntry};
use crate::utils::ErrorProcess;
use tokio::sync::{MutexGuard};
use common::GitTreeItem;
use tokio::task;

use crate::git::GitBlob;
use super::utils::{create_id, tree_entry_is_file};

pub struct GitId {
    id: Oid,
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

fn put_child_tree<
    'repo
>(
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
    M: FnOnce(&mut GitTreeBuilder<'repo>) -> Result<(), ErrorProcess>
>(
    session: &'session GitSession<'repo>,
    tree: &Tree<'repo>,
    path: &[String],
    modify: M
) -> Result<Oid, ErrorProcess> {
    if let Some((first, rest_path)) = path.split_first() {
        
        let child_tree = get_child_tree(&session, &tree, first)?;
        let child_tree_modify = find_and_change_path_small(&session, &child_tree, rest_path, modify)?;
        let tree_modify = put_child_tree(&session, &tree, first, child_tree_modify)?;

        Ok(tree_modify)

    } else {
        let builder = session.repo.treebuilder(Some(tree))?;

        let mut treebuilder = GitTreeBuilder::new(builder);
        modify(&mut treebuilder)?;
        let new_id = treebuilder.id()?;
        Ok(new_id)
    }
}


fn find_and_change_path<
    'repo,
    'session: 'repo,
    M: FnOnce(&mut GitTreeBuilder<'repo>) -> Result<(), ErrorProcess>
>(
    session: &'session GitSession<'repo>,
    path: &[String],
    modify: M
) -> Result<Oid, ErrorProcess> {

    // let Self { id, branch_name, repo } = self;

    // let new_id = {
        // let refrr: &'repo MutexGuard<'repo, Repository> = &repo;

        //let aa = repo.find_object(id, None)?;

        // let repo = &self.repo;

    let result = session.repo.find_object(session.id, None)?;
    let tree = result.peel_to_tree()?;

    //GitTreeBuilder::new(repo, tree)
    // let tree = find_tree(refrr, id)?;
    find_and_change_path_small(session, &tree, path, modify)
}


pub fn create_file_content<
    'repo,
>(
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

pub fn command_find_blob<
    'repo,
>(
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


pub fn commit<
    'repo
>(
    session: GitSession<'repo>,
) -> Result<String, ErrorProcess> {
    //TODO - odpalenie tej funkcji, powoduje zakomitowanie zmian i zjedzenie instancji

    let new_tree = find_tree(&session, session.id.clone())?;

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

    Ok(session.id.to_string())
}


// treebuilder
// let mut builder = repo.treebuilder(Some(tree))?;
// builder.insert(filename, child, 0o040000)?;
// let write_result = builder.write()?;


fn convert_to_name(item: &TreeEntry) -> Result<String, ErrorProcess> {
    let name = item.name();

    match name {
        Some(str) => Ok(String::from(str)),
        None => ErrorProcess::server_result("One of the tree elements has an invalid utf8 name")
    }
}

pub struct GitSession<'repo> {
    id: Oid,
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
            id,
            branch_name: branch_name.into(),
            repo
        })
    }

    fn create_blob(&self, new_content: String) -> Result<GitId, ErrorProcess> {             //TODO - tp wyciągnąć na zewnętrzną komendę
        let new_content_id = self.repo.blob(new_content.as_bytes())?;
        Ok(GitId::new_file(new_content_id))
    }


    pub async fn commit(self) -> Result<String, ErrorProcess> {
        task::block_in_place(move || {
            commit(self)
        })
    }

    pub async fn command_main_commit(
        self,
    ) -> Result<String, ErrorProcess> {
        Ok(self.id.to_string())
    }

    pub async fn command_save_change(
        mut self,
        mut path: Vec<String>,
        prev_hash: String,
        new_content: String
    ) -> Result<GitSession<'repo>, ErrorProcess> {

        let file_name = path.pop().unwrap();

        let new_content_id = self.create_blob(new_content)?;

        let new_self = task::block_in_place(move || -> Result<GitSession<'repo>, ErrorProcess> {
            let id = find_and_change_path(
                &self,
                &path,
                move |tree_builder: &mut GitTreeBuilder<'repo>| -> Result<(), ErrorProcess> {
                    
                    let child = tree_builder.get_child(file_name.as_str())?;
                    let child = child.ok_or_else(|| {
                        ErrorProcess::user(format!("item not found to be modified = {}", &file_name))
                    })?;

                    if child.id.to_string() != prev_hash {
                        return ErrorProcess::user_result(format!("item not found to be modified = {}, hash mismatch", file_name));
                    }


                    tree_builder.insert(&file_name, new_content_id)?;
                    Ok(())
                }
            )?;

            self.id = id;

            Ok(self)
        })?;

        Ok(new_self)
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


    pub async fn command_create_file(
        mut self,
        path: Vec<String>,      //wskazuje na katalog w którym utworzymy nową treść
        new_path: Vec<String>,  //mona od razu utworzyc potrzebne podktalogi
        new_content: String,
    ) -> Result<GitSession<'repo>, ErrorProcess> {
        
        let new_self = task::block_in_place(move || -> Result<GitSession<'repo>, ErrorProcess> {
            if let Some((first_item_name, rest_path)) = new_path.split_first() {
                let new_content_id = create_file_content(&self, &rest_path, &new_content)?;

                let id = find_and_change_path(&self, &path, move |tree_builder: &mut GitTreeBuilder<'repo>| -> Result<(), ErrorProcess> {
                    let is_exist = tree_builder.is_exist(first_item_name.as_str())?;

                    if is_exist {
                        return ErrorProcess::user_result(format!("this element already exists - {}", first_item_name));
                    }
        
                    let tree_builder = tree_builder.insert(first_item_name, new_content_id)?;

                    Ok(tree_builder)
                })?;

                self.id = id;

                Ok(self)

                //self.commit()
            } else {
                return ErrorProcess::user_result("new_path - must be a non-empty list");
            }
        });

        new_self
    }


    pub async fn command_rename_item(
        mut self,
        path: Vec<String>,          //wskazuje na katalog
        current_name: String,          //mona od razu utworzyc potrzebne podktalogi
        current_hash: String,
        new_name: String,
    ) -> Result<GitSession<'repo>, ErrorProcess> {

        let new_self = task::block_in_place(move || -> Result<GitSession<'repo>, ErrorProcess> {
            let id = find_and_change_path(&self, &path, move |tree_builder: &mut GitTreeBuilder<'repo>| -> Result<(), ErrorProcess> {
                let current_hash = create_id(&current_hash)?;
                let current_child = tree_builder.get_child(current_name.as_str())?.ok_or_else(|| {
                    ErrorProcess::user("this element not exists")
                        .context("command_rename_item prev_name", &current_name)
                })?;

                if current_child.id != current_hash {
                    let current_hash = current_hash.to_string();
                    let child_id = current_child.id.to_string();
                    return ErrorProcess::user_result(format!("'current_hash' does not match - {} {}", current_hash, child_id));
                }

                let new_item_exist = tree_builder.is_exist(new_name.as_str())?;

                if new_item_exist {
                    return ErrorProcess::user_result(format!("New element exists - {}", new_name));
                }

                tree_builder.remove(current_name.as_str())?;
                tree_builder.insert(new_name.as_str(), current_child)?;

                Ok(())
            })?;
    
            self.id = id;

            Ok(self)
        });

        new_self
    }
}


