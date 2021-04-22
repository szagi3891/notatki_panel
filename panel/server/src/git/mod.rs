use std::path::Path;

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
use git2::Repository;

#[derive(Debug)]
pub enum Command {
    GetBlob {
        id: String,
        response: oneshot::Sender<String>,
    }
}


#[derive(Clone)]
pub struct Git {
    sender: Sender<Command>,
    _thread: Arc<std::thread::JoinHandle<()>>,
}


impl Git {
    pub fn new(path: String) -> Git {

        let (sender, mut receiver) = mpsc::channel::<Command>(1000);

        let thread = std::thread::spawn(move || {

            println!("test z watku ... start");

            let repo = match Repository::init(path) {
                Ok(repo) => repo,
                Err(e) => panic!("failed to init: {}", e),
            };
            
            while let Some(command) = receiver.blocking_recv() {
                println!("command ... {:?}", command);
            }

            drop(repo);

            println!("test z watku ...");
        });

        Git {
            sender,
            _thread: Arc::new(thread),
        }
    }

    pub async fn command(&self) -> GitCommand {
        GitCommand::new(self.sender.clone())
    }
}

pub struct GitCommand {
    sender: Sender<Command>,
}

impl GitCommand {
    pub fn new(sender: Sender<Command>) -> GitCommand {
        GitCommand {
            sender,
        }
    }

    pub fn test(self) {
        //...
    }
}