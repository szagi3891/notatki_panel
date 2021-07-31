use std::sync::Arc;
use git2::{BranchType, ObjectType, Repository, Tree, Oid};
use crate::utils::ErrorProcess;
use tokio::sync::{Mutex, MutexGuard};
use tokio::task;
use super::gitsync_session::GitsyncSession;

#[derive(Clone)]
pub struct Gitsync {
    branch_name: String,
    repo: Arc<Mutex<Repository>>,
}

impl Gitsync {
    pub fn new(path: String, branch_name: String) -> Result<Gitsync, ErrorProcess> {
        let repository = match Repository::open(&path) {
            Ok(repo) => repo,
            Err(e) => {
                return ErrorProcess::server_result(format!("Problem with init repo: {}", path));
            },
        };

        Ok(Gitsync {
            branch_name,
            repo: Arc::new(Mutex::new(repository))
        })
    }

    async fn session<'repo>(&'repo self) -> Result<GitsyncSession<'repo>, ErrorProcess> {
        let mutex_guard = self.repo.lock().await;

        GitsyncSession::new(mutex_guard, self.branch_name.as_str())
    }

    pub async fn command_save_change(
        &self,
        path: Vec<String>,
        prev_hash: String,
        new_content: String
    ) -> Result<String, ErrorProcess> {
        let session = self.session().await?;
        session.command_save_change(path, prev_hash, new_content).await
    }
}
