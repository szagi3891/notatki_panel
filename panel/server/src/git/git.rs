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
        mut path: Vec<String>,
        prev_hash: String,
        new_content: String
    ) -> Result<String, ErrorProcess> {
        let session = self.session().await?;

        let file_name = path.pop();

        let file_name = match file_name {
            Some(file_name) => file_name,
            None => {
                return Err(ErrorProcess::user("Incorrect path to file - non-empty list expected"));
            }
        };

        let (session, prev_content_id) = session.remove_child(&path, &file_name).await?;

        let prev_content_id = match prev_content_id {
            Some(prev_content_id) => prev_content_id,
            None => {
                return Err(ErrorProcess::user(format!("No file exists in the location: {}/{}", path.join("/"), file_name)));
            }
        };

        if prev_content_id.id.to_string() != prev_hash {
            return ErrorProcess::user_result(format!("item not found to be modified = {}, hash mismatch", file_name));
        }

        let (session, new_content_id) = session.create_blob(new_content).await?;

        let session = session.insert_child(&path, &file_name, new_content_id).await?;

        session.commit().await
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

        if let Some((first_item_name, rest_path)) = new_path.split_first() {
            let (session, new_content_id) = session.create_file_content(rest_path, &new_content).await?;

            let (session, old_child) = session.remove_child(&path, first_item_name).await?;

            if old_child.is_some() {
                return Err(ErrorProcess::user(format!("File exists in this location: {}", first_item_name)));
            }

            let session = session.insert_child(&path, first_item_name, new_content_id).await?;

            session.commit().await
        } else {
            return ErrorProcess::user_result("new_path - must be a non-empty list");
        }
    }


    pub async fn rename_item(
        &self,
        path: Vec<String>,          //wskazuje na katalog
        prev_name: String,          //mona od razu utworzyc potrzebne podktalogi
        prev_hash: String,
        new_name: String,
    ) -> Result<String, ErrorProcess> {
        let session = self.session().await?;

        let (session, current_id) = session.remove_child(&path, &prev_name).await?;

        let current_id = match current_id {
            Some(current_id) => current_id,
            None => {
                return Err(ErrorProcess::user(format!("No file exists in the location: {}/{}", path.join("/"), prev_name)));
            }
        };

        let current_hash = session.create_id(&prev_hash)?;
        if current_id != current_hash {
            return ErrorProcess::user_result(format!("'current_hash' does not match - {:?} {:?}", current_hash, current_id));
        }

        let session = session.insert_child(&path, &new_name, current_id).await?;

        session.commit().await
    }


    //TODO
    //rozbić na operację usuwania elementu
    //oraz dodawania nowego 

}
