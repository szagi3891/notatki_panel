use tokio::sync::{
    oneshot::{
        self,
    },
    mpsc::{
        self,
        Sender,
    }
};
use crate::git::GitBlob;

use std::sync::Arc;
use git2::{Repository};
use crate::{git::utils::RepoWrapper, utils::ErrorProcess};

use super::gitsync::Gitsync;
use super::command_create_file::command_create_file;
use super::command_rename_item::command_rename_item;

#[derive(Debug)]
enum Command {
    CreateFile {
        path: Vec<String>,      //wskazuje na katalog w którym utworzymy nową treść
        new_path: Vec<String>,  //mona od razu utworzyc potrzebne podktalogi
        new_content: String,
        response: oneshot::Sender<Result<String, ErrorProcess>>,
    },
    RenameItem {
        path: Vec<String>,      //wskazuje na katalog w którym utworzymy nową treść
        prev_name: String,
        prev_hash: String,
        new_name: String,
        response: oneshot::Sender<Result<String, ErrorProcess>>,
    }
}

#[derive(Clone)]
pub struct Git {
    gitsync: Gitsync,
    sender: Sender<Command>,
    _thread: Arc<std::thread::JoinHandle<()>>,
}

impl Git {
    pub fn new(path: String, branch_name: String) -> Git {

        let (sender, mut receiver) = mpsc::channel::<Command>(1000);

        let gitsync = Gitsync::new(path.clone(), branch_name.clone()).unwrap();     //TODO ...

        let thread = std::thread::spawn(move || {

            println!("test z watku ... start");

            let repository = match Repository::open(path) {
                Ok(repo) => repo,
                Err(e) => panic!("failed to init: {}", e),
            };
            
            let mut repo_wrapper = RepoWrapper::new(&repository, branch_name).unwrap();
            
            while let Some(command) = receiver.blocking_recv() {
                println!("command ... {:?}", &command);

                match command {
                    Command::CreateFile {
                        path,
                        new_path,
                        new_content,
                        response,
                    } => {
                        let resp = command_create_file(
                            repo_wrapper.clone(),
                            path,
                            new_path,
                            new_content
                        );

                        match resp {
                            Ok(new_repo) => {
                                let id = new_repo.main_id();
                                repo_wrapper = new_repo;
                                response.send(Ok(id)).unwrap();
                            },
                            Err(err) => {
                                response.send(Err(err)).unwrap();
                            }
                        };
                    },

                    Command::RenameItem {
                        path,
                        prev_name,
                        prev_hash,
                        new_name,
                        response,
                    } => {
                        let resp = command_rename_item(
                            repo_wrapper.clone(),
                            path,
                            prev_name,
                            prev_hash,
                            new_name
                        );

                        match resp {
                            Ok(new_repo) => {
                                let id = new_repo.main_id();
                                repo_wrapper = new_repo;
                                response.send(Ok(id)).unwrap();
                            },
                            Err(err) => {
                                response.send(Err(err)).unwrap();
                            }
                        };
                    },
                }

                println!("next command ...");
            }

            println!("test z watku ...");
        });

        Git {
            sender,
            gitsync,
            _thread: Arc::new(thread),
        }
    }

    pub async fn get_main_commit(&self) -> Result<String, ErrorProcess> {
        self.gitsync.command_main_commit().await
    }

    pub async fn get_from_id(&self, id: &String) -> Result<Option<GitBlob>, ErrorProcess> {
        self.gitsync.get_from_id(id.clone()).await
    }

    pub async fn save_content(&self, path: Vec<String>, prev_hash: String, new_content: String) -> Result<String, ErrorProcess> {
        self.gitsync.command_save_change(path, prev_hash, new_content).await
    }

    pub async fn create_file(&self, path: Vec<String>, new_path: Vec<String>, new_content: String) -> Result<String, ErrorProcess> {
        let (sender, receiver) = oneshot::channel();

        let save_command = Command::CreateFile {
            path,
            new_path,
            new_content,
            response: sender,
        };

        self.sender.send(save_command).await.unwrap();
        receiver.await.unwrap()
    }

    pub async fn rename_item(&self, path: Vec<String>, prev_name: String, prev_hash: String, new_name: String) -> Result<String, ErrorProcess> {
        let (sender, receiver) = oneshot::channel();

        let command = Command::RenameItem {
            path,
            prev_name,
            prev_hash,
            new_name,
            response: sender,
        };

        self.sender.send(command).await.unwrap();
        receiver.await.unwrap()
    }
}
