use git2::{BranchType, ObjectType, Repository, Tree, Oid, TreeEntry};
use crate::utils::ErrorProcess;
use tokio::sync::{MutexGuard};
use common::GitTreeItem;
use tokio::task;

use crate::git::GitBlob;
use super::utils::{create_id, tree_entry_is_file};


fn find_tree<'repo>(repo: &'repo MutexGuard<'repo, Repository>, id: Oid) -> Result<Tree, ErrorProcess> {
    let result = repo.find_object(id, None)?;
    let tree = result.peel_to_tree()?;
    Ok(tree)
}

fn get_child_tree<'repo>(
    repo: &'repo MutexGuard<'repo, Repository>,
    tree: &Tree,
    name: &String
) -> Result<Tree<'repo>, ErrorProcess> {
    
    for item in tree {
        if item.name() == Some(name.as_str()) {
            let tree = find_tree(repo, item.id())?;
            return Ok(tree);
        }
    }

    ErrorProcess::user_result(format!("Element not found {}", name))
}

fn put_child_tree<'repo>(
    repo: &Repository,
    tree: &Tree,
    filename: &String,
    child: Oid
) -> Result<Oid, ErrorProcess> {

    let child_object = repo.find_object(child, None)?;

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

    let mut builder = repo.treebuilder(Some(tree))?;
    builder.insert(filename, child, 0o040000)?;
    let write_result = builder.write()?;

    Ok(write_result)
}

fn find_and_change_path<
    'repo,
    M: Fn(&MutexGuard<'repo, Repository>, &Tree) -> Result<Oid, ErrorProcess>
>(
    repo: &MutexGuard<'repo, Repository>,
    tree: &Tree,
    path: &[String],
    modify: M
) -> Result<Oid, ErrorProcess> {
    if let Some((first, rest_path)) = path.split_first() {
        
        let child_tree = get_child_tree(repo, &tree, first)?;
        let child_tree_modify = find_and_change_path(repo, &child_tree, rest_path, modify)?;
        let tree_modify = put_child_tree(repo, &tree, first, child_tree_modify)?;

        Ok(tree_modify)

    } else {
        modify(repo, tree)
    }
}


fn convert_to_name(item: &TreeEntry) -> Result<String, ErrorProcess> {
    let name = item.name();

    match name {
        Some(str) => Ok(String::from(str)),
        None => ErrorProcess::server_result("One of the tree elements has an invalid utf8 name")
    }
}


pub fn create_file_content<'repo>(
    repo: &MutexGuard<'repo, Repository>,
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

    // fn find_and_change<
    //     M: Fn(&MutexGuard<'repo, Repository>, &Tree) -> Result<Oid, ErrorProcess>
    // >(
    //     self,
    //     modify: M
    // ) ->  Result<GitsyncSession<'repo>, ErrorProcess> {
    //     let Self { id, branch_name, repo } = self;

    //     let new_id = {
    //         let tree = find_tree(&repo, id)?;
    //         let new_id = modify(&repo, &tree)?;
    //         new_id
    //     };

    //     Ok(GitsyncSession {
    //         id: new_id,
    //         branch_name,
    //         repo,
    //     })
    // }

    fn find_and_change_path_sync<
        M: Fn(&MutexGuard<'repo, Repository>, &Tree) -> Result<Oid, ErrorProcess>
    >(
        self,
        path: &[String],
        modify: M
    ) -> Result<GitSession<'repo>, ErrorProcess> {

        let Self { id, branch_name, repo } = self;

        let new_id = {
            let tree = find_tree(&repo, id)?;
            let new_id = find_and_change_path(&repo, &tree, path, modify)?;
            new_id
        };

        Ok(GitSession {
            id: new_id,
            branch_name,
            repo,
        })
    }

    pub async fn find_and_change_path<
        M: Fn(&MutexGuard<'repo, Repository>, &Tree) -> Result<Oid, ErrorProcess>
    >(
        self,
        path: &[String],
        modify: M
    ) -> Result<GitSession<'repo>, ErrorProcess> {
        task::block_in_place(move || {
            self.find_and_change_path_sync(path, modify)
        })
    }

    pub fn create_new_blob_sync(self, path: &[String], new_content: &String) -> Result<(GitSession<'repo>, Oid, bool), ErrorProcess> {
        let (id, is_file) = create_file_content(&self.repo, path, new_content)?;
        Ok((self, id, is_file))
    }

    pub async fn create_new_blob(self, path: &[String], new_content: &String) -> Result<(GitSession<'repo>, Oid, bool), ErrorProcess> {
        task::block_in_place(move || {
            self.create_new_blob_sync(path, new_content)
        })
    }


    pub fn commit_sync(self) -> Result<String, ErrorProcess> {
        //TODO - odpalenie tej funkcji, powoduje zakomitowanie zmian i zjedzenie instancji

        let Self { repo, branch_name, id } = self;

        let new_tree = find_tree(&repo, id)?;

        let branch = repo.find_branch(branch_name.as_str(), BranchType::Local)?;
        let reference = branch.get();

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

        Ok(id.to_string())
    }


    pub fn commit(self) -> Result<String, ErrorProcess> {
        task::block_in_place(move || {
            self.commit_sync()
        })
    }

    //.......
    //logika aplikacji
    //.......

    pub async fn command_main_commit(
        self,
    ) -> Result<String, ErrorProcess> {
        Ok(self.id.to_string())
    }

    pub async fn command_save_change(
        self,
        mut path: Vec<String>,
        prev_hash: String,
        new_content: String
    ) -> Result<String, ErrorProcess> {

        let file_name = path.pop().unwrap();

        let new_repo = self.find_and_change_path(&path, move |repo: &MutexGuard<'repo, Repository>, tree: &Tree| -> Result<Oid, ErrorProcess> {
            
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
        }).await?;
        
        new_repo.commit()
    }

    pub async fn command_find_blob(
        self,
        id: &String
    ) -> Result<Option<GitBlob>, ErrorProcess> {
        let Self { repo, .. } = self;

        let oid = create_id(id)?;

        if let Ok(tree) = repo.find_tree(oid) {
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
    
        if let Ok(blob) = repo.find_blob(oid) {
            let content = blob.content();
            let content = Vec::from(content);
    
            return Ok(Some(GitBlob::Blob { content }));
        }
    
        Ok(None)
    }

    pub async fn command_create_file(
        self,
        path: Vec<String>,      //wskazuje na katalog w którym utworzymy nową treść
        new_path: Vec<String>,  //mona od razu utworzyc potrzebne podktalogi
        new_content: String,
    ) -> Result<String, ErrorProcess> {
        
        if let Some((first_item_name, rest_path)) = new_path.split_first() {
            let (session, new_content_id, is_file) = self.create_new_blob(&rest_path, &new_content).await?;

            let new_repo = session.find_and_change_path(&path, move |repo: &MutexGuard<'repo, Repository>, tree: &Tree| -> Result<Oid, ErrorProcess> {
                let child = tree.get_name(first_item_name.as_str());
    
                if child.is_some() {
                    return ErrorProcess::user_result(format!("this element already exists - {}", first_item_name));
                }
    
                let mut builder = repo.treebuilder(Some(&tree))?;
                
                if is_file {
                    builder.insert(first_item_name, new_content_id, 0o100644)?;
                } else {
                    builder.insert(first_item_name, new_content_id, 0o040000)?;
                }
    
                let id = builder.write()?;
    
                Ok(id)
            }).await?;

            new_repo.commit()
        } else {
            return ErrorProcess::user_result("new_path - must be a non-empty list");
        }
    }


    pub async fn command_rename_item(
        self,
        path: Vec<String>,          //wskazuje na katalog
        prev_name: String,          //mona od razu utworzyc potrzebne podktalogi
        prev_hash: String,
        new_name: String,
    ) -> Result<String, ErrorProcess> {

        let new_repo = self.find_and_change_path(&path, move |repo: &MutexGuard<'repo, Repository>, tree: &Tree| -> Result<Oid, ErrorProcess> {
            let child = tree.get_name(prev_name.as_str())
                .ok_or_else(|| ErrorProcess::user("this element not exists")
                    .context("command_rename_item prev_name", &prev_name)
                )?;

            let child_is_file = tree_entry_is_file(&child)
                .map_err(|err| err.context("command_rename_item prev_name", &prev_name))?;

            let prev_hash = create_id(&prev_hash)?;

            if child.id() != prev_hash {
                let prev_hash = prev_hash.to_string();
                let child_id = child.id().to_string();
                return ErrorProcess::user_result(format!("'prev_hash' does not match - {} {}", prev_hash, child_id));
            }

            let new_item_exist = {
                let new_item = tree.get_name(new_name.as_str());
                new_item.is_some()
            };

            if new_item_exist {
                return ErrorProcess::user_result(format!("New element exists - {}", new_name));
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
        }).await?;
        
        new_repo.commit()
    }
}


