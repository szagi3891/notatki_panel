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
use git2::{Oid, Repository};
use crate::utils::ErrorProcess;

mod utils;
mod command_find_main_commit;
mod command_find_blob;
mod command_save_change;

use command_find_main_commit::command_find_main_commit;
use command_find_blob::command_find_blob;
pub use command_find_blob::GitBlob;
use command_save_change::command_save_change;
pub use utils::create_id;

#[derive(Debug)]
enum Command {
    FindMainCommit {
        branch: String,
        response: oneshot::Sender<Result<std::string::String, ErrorProcess>>,
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
        response: oneshot::Sender<Result<String, ErrorProcess>>,
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
                        let tree = command_find_main_commit(&repo, branch);
                        response.send(tree).unwrap();
                    },

                    Command::FindBlob { id, response } => {
                        let res = command_find_blob(&repo, id);
                        response.send(res).unwrap();
                    }

                    Command::SaveChangeInContent {
                        branch,
                        path,
                        prev_hash,
                        new_content,
                        response
                    } => {
                        let resp = command_save_change(&repo, branch, path, prev_hash, new_content);
                        response.send(resp).unwrap();
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

    pub async fn get_main_commit(&self) -> Result<String, ErrorProcess> {
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

    pub async fn save_content(&self, path: Vec<String>, prev_hash: String, new_content: String) -> Result<String, ErrorProcess> {
        let (sender, receiver) = oneshot::channel();

        let save_command = Command::SaveChangeInContent {
            branch: self.branch.clone(),
            path,
            prev_hash,
            new_content,
            response: sender,
        };

        self.sender.send(save_command).await.unwrap();
        receiver.await.unwrap()
    }
}
