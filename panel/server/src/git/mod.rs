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

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeItem {
    dir: bool,
    id: String,
    name: String,
}

impl Into<GitTreeItem> for TreeItem {
    fn into(self) -> GitTreeItem {
        let TreeItem { dir, id, name } = self;
        GitTreeItem {
            dir,
            id,
            name,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GitBlob {
    Blob {
        content: Vec<u8>,
    },
    Tree {
        list: Vec<TreeItem>,
    }
}

#[derive(Debug)]
pub enum Command {
    FindMainCommit {
        branch: String,
        response: oneshot::Sender<String>,
    },
    FindBlob {
        id: String,
        response: oneshot::Sender<Option<GitBlob>>,
    }
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
                        let oid = match Oid::from_str(&id) {
                            Ok(id) => id,
                            Err(err) => {
                                println!("error id {}", err);
                                response.send(None).unwrap();
                                continue;
                            }
                        };

                        if let Ok(tree) = repo.find_tree(oid) {
                            let mut list: Vec<TreeItem> = Vec::new();

                            for item in tree.iter() {
                                let name = item.name();
                                let kind = item.kind();
                                let id = item.id();

                                let name = match name {
                                    Some(str) => String::from(str),
                                    _ => {
                                        panic!("todo1");
                                    }
                                };

                                let dir = match kind {
                                    Some(ObjectType::Tree) => true,
                                    Some(ObjectType::Blob) => false,
                                    _ => {
                                        panic!("todo");
                                    }
                                };

                                list.push(TreeItem {
                                    dir,
                                    id: id.to_string(),
                                    name,
                                });
                            }

                            response.send(Some(GitBlob::Tree { list })).unwrap();
                            continue;
                        }

                        if let Ok(blob) = repo.find_blob(oid) {
                            let content = blob.content();
                            let content = Vec::from(content);

                            response.send(Some(GitBlob::Blob { content })).unwrap();
                            continue;
                        }

                        response.send(None).unwrap();
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

    pub async fn get_from_id(&self, id: &String) -> Option<GitBlob> {
        let (sender, receiver) = oneshot::channel();

        let command = Command::FindBlob {
            id: id.clone(),
            response: sender,
        };
        self.sender.send(command).await.unwrap();
        receiver.await.unwrap()
    }
}
