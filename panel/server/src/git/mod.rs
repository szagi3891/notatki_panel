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

fn command_find_blob(repo: &Repository, id: String) -> Result<Option<GitBlob>, ErrorProcess> {
    let oid = create_id(id)?;

    if let Ok(tree) = repo.find_tree(oid) {
        let mut list: Vec<GitTreeItem> = Vec::new();

        for item in tree.iter() {
            let name = item.name();
            let kind = item.kind();
            let id = item.id();

            let name = match name {
                Some(str) => String::from(str),
                _ => {
                    return Err(ErrorProcess::server("One of the tree elements has an invalid utf8 name"));
                }
            };

            let dir = match kind {
                Some(ObjectType::Tree) => true,
                Some(ObjectType::Blob) => false,
                _ => {
                    return Err(ErrorProcess::server("Trees only support 'ObjectType::Tree' and 'ObjectType::Blob'"));
                }
            };

            list.push(GitTreeItem {
                dir,
                id: id.to_string(),
                name,
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
                        let tree = reference.peel_to_tree().unwrap();
                        let commit = reference.peel_to_commit().unwrap();
                        //TODO - coś rób

                        let file_name = path.pop().unwrap();

                        fn get_sub_tree<'repo>(tree: git2::Tree<'repo>, name: &String) -> Result<git2::Tree<'repo>, ErrorProcess> {
                            todo!();
                        }

                        fn find_and_change<'repo, M: Fn(git2::Tree<'repo>) -> git2::Tree<'repo>>(tree: git2::Tree, path: &[String], modify: M) -> Result<git2::Tree<'repo>, ErrorProcess> {
                            if let Some((first, rest_path)) = path.split_first() {
                                
                                let sub_tree = get_sub_tree(tree, first)?;
                                let sub_tree2 = find_and_change(sub_tree, rest_path, modify)?;
                                return Ok(sub_tree2);

                            } else {
                                //przetwarzaj
                            }

                            todo!()
                        }

                        println!(".. {:?} ..", commit);

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
