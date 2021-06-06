use common::GitTreeItem;
use serde::{Deserialize, Serialize};
use tokio::sync::{
    oneshot::{
        self,
    },
    mpsc::{
        self,
        Sender,
    }
};

use std::sync::Arc;
use git2::{
    Repository,
    BranchType,
    ObjectType,
    TreeEntry,
    TreeBuilder,
    Tree,
    Object,
    Oid,
};
use crate::utils::ErrorProcess;

#[derive(Debug, Serialize, Deserialize)]
pub enum GitBlob {
    Blob {
        content: Vec<u8>,
    },
    Tree {
        list: Vec<GitTreeItem>,
    }
}

#[derive(Debug)]
enum Command {
    FindMainCommit {
        branch: String,
        response: oneshot::Sender<String>,
    },
    FindBlob {
        id: String,
        response: oneshot::Sender<Result<Option<GitBlob>, ErrorProcess>>,
    },
    SaveChangeInContent {
        branch: String,
        path: Vec<String>,      //wskazuje na plik do zapisania
        prev_hash: String,
        new_content: String,
        response: oneshot::Sender<()>,
    }
}

fn create_id(hash: String) -> Result<Oid, ErrorProcess> {
    match Oid::from_str(&hash) {
        Ok(id) => Ok(id),
        Err(err) => {
            Err(ErrorProcess::user(format!("Invalid hash {} {}", hash, err)))
        }
    }
}

fn convert_to_name(item: &TreeEntry) -> Result<String, ErrorProcess> {
    let name = item.name();

    match name {
        Some(str) => Ok(String::from(str)),
        None => Err(ErrorProcess::server("One of the tree elements has an invalid utf8 name"))
    }
}

fn convert_to_type(item: &TreeEntry) -> Result<bool, ErrorProcess> {
    let kind = item.kind();

    match kind {
        Some(ObjectType::Tree) => Ok(true),
        Some(ObjectType::Blob) => Ok(false),
        _ => Err(ErrorProcess::server("Trees only support 'ObjectType::Tree' and 'ObjectType::Blob'"))
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

    Err(ErrorProcess::user(format!("Element not found {}", name)))
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
            return Err(ErrorProcess::user(format!("Incorrect type object = {}, {}", child, kind)));
        },
        None => {
            return Err(ErrorProcess::user(format!("It was not possible to determine the type of this object = {}", child)));
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

fn command_find_blob(repo: &Repository, id: String) -> Result<Option<GitBlob>, ErrorProcess> {
    let oid = create_id(id)?;

    if let Ok(tree) = repo.find_tree(oid) {
        let mut list: Vec<GitTreeItem> = Vec::new();

        for item in tree.iter() {
            list.push(GitTreeItem {
                dir: convert_to_type(&item)?,
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


#[derive(Clone)]
pub struct Git {
    branch: String,
    sender: Sender<Command>,
    _thread: Arc<std::thread::JoinHandle<()>>,
}


impl Git {
    pub fn new(path: String, branch: String) -> Git {

        let (sender, mut receiver) = mpsc::channel::<Command>(1000);

        let thread = std::thread::spawn(move || {

            println!("test z watku ... start");

            let repo = match Repository::open(path) {
                Ok(repo) => repo,
                Err(e) => panic!("failed to init: {}", e),
            };
            
            while let Some(command) = receiver.blocking_recv() {
                println!("command ... {:?}", &command);

                match command {
                    Command::FindMainCommit { branch, response } => {
                        let branch = repo.find_branch(branch.as_str(), BranchType::Local).unwrap();
                        let reference = branch.get();
                        let tree = reference.peel_to_tree().unwrap();
                        response.send(tree.id().to_string()).unwrap();
                    },

                    Command::FindBlob { id, response } => {
                        let res = command_find_blob(&repo, id);
                        response.send(res).unwrap();
                    }

                    Command::SaveChangeInContent {
                        branch,
                        mut path,
                        prev_hash,
                        new_content,
                        response
                    } => {

                        let branch = repo.find_branch(branch.as_str(), BranchType::Local).unwrap();
                        let reference = branch.get();
                        let curret_root_tree = reference.peel_to_tree().unwrap();
                        let commit = reference.peel_to_commit().unwrap();
                        //TODO - coś rób

                        let file_name = path.pop().unwrap();

                        println!(".. {:?} ..", commit);

                        let new_tree_id = find_and_change(
                            &repo,
                            curret_root_tree,
                            &path,
                            move |repo: &Repository, tree: Tree| -> Result<Oid, ErrorProcess> {
                                
                                let child = tree.get_name(file_name.as_str());

                                match child {
                                    Some(child) => {
                                        if child.id().to_string() != prev_hash {
                                            return Err(ErrorProcess::user(format!("item not found to be modified = {}, hash mismatch", file_name)));
                                        }
                                    },
                                    None => {
                                        return Err(ErrorProcess::user(format!("item not found to be modified = {}", &file_name)));
                                    }
                                };

                                let mut builder = repo.treebuilder(Some(&tree))?;
                                let new_content_id = repo.blob(new_content.as_bytes())?;
                                println!("insert inner");
                                builder.insert(&file_name, new_content_id, 0o100755)?;

                                let id = builder.write()?;

                                println!("najnizsze id {}", id);
                                Ok(id)
                            }
                        );

                        println!("aaaa vvvvv {:?}", new_tree_id);

                        /*
                            Odczytaj Tree
                            wykonaj operację find_and_change ---> to zwróci Oid

                            Etap komitowania.
                            Trzeba zapisać to drzewo jako nowy komit
                            odczytujemy najnowszy komit.
                            odczytujemy autora
                            odczytujemy obiekt Tree na podstawie Oid

                        */
                        response.send(());
                    }
                }

                println!("next command ...");
            }

            drop(repo);

            println!("test z watku ...");
        });

        Git {
            branch: branch,
            sender,
            _thread: Arc::new(thread),
        }
    }

    pub async fn get_main_commit(&self) -> String {
        let (sender, receiver) = oneshot::channel();

        let command = Command::FindMainCommit {
            branch: self.branch.clone(),
            response: sender,
        };

        self.sender.send(command).await.unwrap();

        let response = receiver.await.unwrap();
        response
    }

    pub async fn get_from_id(&self, id: &String) -> Result<Option<GitBlob>, ErrorProcess> {
        let (sender, receiver) = oneshot::channel();

        let command = Command::FindBlob {
            id: id.clone(),
            response: sender,
        };
        self.sender.send(command).await.unwrap();
        receiver.await.unwrap()
    }

    pub async fn save_content(&self, path: Vec<String>, prev_hash: String, new_content: String) {
        let (sender, receiver) = oneshot::channel();

        let save_command = Command::SaveChangeInContent {
            branch: self.branch.clone(),
            path,
            prev_hash,
            new_content,
            response: sender,
        };

        self.sender.send(save_command).await.unwrap();
        receiver.await.unwrap();
    }
}
