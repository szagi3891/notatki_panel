use std::sync::Arc;
use git2::Repository;
use crate::utils::ErrorProcess;
use tokio::sync::{Mutex};
use super::git_session::GitSession;
use crate::git::GitBlob;

#[derive(Clone)]
pub struct Git {
    branch_name: String,
    repo: Arc<Mutex<Repository>>,
}

impl Git {
    pub fn new(path: String, branch_name: String) -> Result<Git, ErrorProcess> {
        let repository = match Repository::open(&path) {
            Ok(repo) => repo,
            Err(e) => {
                return ErrorProcess::server_result(format!("Problem with init repo: {} {}", path, e));
            },
        };

        Ok(Git {
            branch_name,
            repo: Arc::new(Mutex::new(repository))
        })
    }

    async fn session<'repo>(&'repo self) -> Result<GitSession<'repo>, ErrorProcess> {
        let mutex_guard = self.repo.lock().await;

        GitSession::new(mutex_guard, self.branch_name.as_str())
    }

    pub async fn main_commit(
        &self,
    ) -> Result<String, ErrorProcess> {
        let session = self.session().await?;
        session.command_main_commit().await
    }

    pub async fn save_content(
        &self,
        path: Vec<String>,
        prev_hash: String,
        new_content: String
    ) -> Result<String, ErrorProcess> {
        let session = self.session().await?;
        let session = session.command_save_change(path, prev_hash, new_content).await?;
        session.commit()
    }

    pub async fn get_from_id(&self, id: &String) -> Result<Option<GitBlob>, ErrorProcess> {
        let session = self.session().await?;
        let (_, result) = session.command_find_blob(id).await?;
        Ok(result)
    }

    pub async fn create_file(
        &self,
        path: Vec<String>,      //wskazuje na katalog w którym utworzymy nową treść
        new_path: Vec<String>,  //mona od razu utworzyc potrzebne podktalogi
        new_content: String,
    ) -> Result<String, ErrorProcess> {
        let session = self.session().await?;
        let session = session.command_create_file(path, new_path, new_content).await?;
        session.commit()
    }


    pub async fn rename_item(
        &self,
        path: Vec<String>,          //wskazuje na katalog
        prev_name: String,          //mona od razu utworzyc potrzebne podktalogi
        prev_hash: String,
        new_name: String,
    ) -> Result<String, ErrorProcess> {
        let session = self.session().await?;
        let session = session.command_rename_item(path, prev_name, prev_hash, new_name).await?;
        session.commit()
    }
}
