use std::sync::Arc;
use git2::{Repository, Oid};
use crate::{utils::ErrorProcess, models::HandlerAddFilesFile};
use tokio::sync::{Mutex, Notify};
use super::git_session::{GitSession, GitId};
use crate::git::GitBlob;

fn split_last(path: &[String]) -> Result<(&[String], &String), ErrorProcess> {    
    if let Some((last, begin)) = path.split_last() {
        Ok((begin, last))
    } else {
        ErrorProcess::user_result("missing last element to split")
    }
}

#[derive(Clone)]
pub struct Git {
    notify: Arc<Notify>,
    branch_name: String,
    repo: Arc<Mutex<Repository>>,
}

impl Git {
    pub fn new(notify: Arc<Notify>, path: String, branch_name: String) -> Result<Git, ErrorProcess> {
        let repository = match Repository::open(&path) {
            Ok(repo) => repo,
            Err(e) => {
                return ErrorProcess::server_result(format!("Problem with init repo: {path} {e}"));
            },
        };

        Ok(Git {
            notify,
            branch_name,
            repo: Arc::new(Mutex::new(repository))
        })
    }

    async fn session<'repo>(&'repo self) -> Result<GitSession<'repo>, ErrorProcess> {
        let mutex_guard = self.repo.lock().await;

        GitSession::new(self.notify.clone(), mutex_guard, self.branch_name.as_str())
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
        let message = format!("save {}", path.join("/"));

        let file_name = path.pop();

        let file_name = match file_name {
            Some(file_name) => file_name,
            None => {
                return Err(ErrorProcess::user("Incorrect path to file - non-empty list expected"));
            }
        };

        let (session, prev_content_id) = session.extract_child(&path, &file_name).await?;

        if prev_content_id.id.to_string() != prev_hash {
            return ErrorProcess::user_result(format!("item not found to be modified = {file_name}, hash mismatch"));
        }

        let (session, new_content_id) = session.create_blob(new_content).await?;

        let session = session.insert_child(&path, &file_name, new_content_id).await?;

        session.commit(message).await
    }

    pub async fn create_blob(&self, data: Vec<u8>) -> Result<String, ErrorProcess> {
        let session = self.session().await?;
        let (session, id) = session.create_blob_vec_u8(data).await?;
        session.end();

        Ok(id.convert_to_string())
    }

    pub async fn get_from_id(&self, id: &String) -> Result<Option<GitBlob>, ErrorProcess> {
        let session = self.session().await?;
        let (_, result) = session.get_from_id(id).await?;
        Ok(result)
    }

    pub async fn create_file(
        &self,
        path: Vec<String>,      //wskazuje na katalog w którym utworzymy nową treść
        new_name: String,
        new_content: String,
    ) -> Result<String, ErrorProcess> {
        let session = self.session().await?;

        let (session, new_content_id) = session.create_file_content(&new_content).await?;

        let (session, old_child) = session.remove_child(&path, &new_name).await?;

        if old_child.is_some() {
            return Err(ErrorProcess::user(format!("File exists in this location: {}", &new_name)));
        }

        let session = session.insert_child(&path, &new_name, new_content_id).await?;

        let message = format!("create file {}/{}", path.join("/"), new_name);
        session.commit(message).await
    }

    pub async fn create_dir(
        &self,
        path: Vec<String>,
        dir: String,
    ) -> Result<String, ErrorProcess> {
        let session = self.session().await?;

        let (session, empty_dir) = session.create_empty_dir().await?;

        let session = session.insert_child(&path, &dir, empty_dir).await?;

        let message = format!("create dir {}/{}", path.join("/"), dir);
        session.commit(message).await
    }


    pub async fn rename_item(
        &self,
        path: Vec<String>,          //wskazuje na katalog
        prev_name: String,          //mona od razu utworzyc potrzebne podktalogi
        prev_hash: String,
        new_name: String,
    ) -> Result<String, ErrorProcess> {
        let session = self.session().await?;

        let (session, child) = session.extract_child(&path, &prev_name).await?;
        session.should_eq(&child, &prev_hash)?;
        let session = session.insert_child(&path, &new_name, child).await?;

        let message = format!("rename {} {prev_name} -> {new_name}", path.join("/"));
        session.commit(message).await
    }

    pub async fn move_item(
        &self,
        path: Vec<String>,          //dir lub file
        hash: String,
        new_path: Vec<String>,
    ) -> Result<String, ErrorProcess> {
        let (path_base, path_last) = split_last(&path)?;
        let (new_path_base, new_path_last) = split_last(&new_path)?;

        let session = self.session().await?;
        let (session, child) = session.extract_child(path_base, path_last).await?;

        session.should_eq(&child, &hash)?;

        let session = session.insert_child(new_path_base, new_path_last, child).await?;
        
        let message = format!("move item from={} to={}", path.join("/"), new_path.join("/"));
        let new_root_id = session.commit(message).await?;
        Ok(new_root_id)
    }

    pub async fn add_files(
        &self,
        path: Vec<String>,
        files: Vec<HandlerAddFilesFile>,
    ) -> Result<String, ErrorProcess> {

        let mut session = self.session().await?;

        for file in files.iter() {
            let id_oid = Oid::from_str(file.blob_id.as_str())?;
            let id = GitId::new_file(id_oid);

            session = session.insert_child(&path, &file.name, id).await?;
        }

        let mut files_name = Vec::new();
        for file in files {
            files_name.push(file.name);
        }

        let message = format!("Add files to path={}, files={}", path.join("/"), files_name.join(","));
        session.commit(message).await
    }

    pub async fn delete_item(
        &self,
        path: Vec<String>,
        item_hash: String,
    ) -> Result<String, ErrorProcess> {
        let (path_base, path_last) = split_last(&path)?;
        
        let session = self.session().await?;
        let (session, child) = session.extract_child(path_base, path_last).await?;

        session.should_eq(&child, &item_hash)?;
        
        let (session, result) = session.get_from_id(&child.id.to_string()).await?;

        match result {
            Some(GitBlob::Tree { list }) => {
                if list.len() > 0 {
                    return Err(ErrorProcess::user(format!("non-empty directory cannot be deleted {path:?}")));
                }
                //ok
            },
            Some(GitBlob::Blob { .. }) => {
                //ok
            },
            None => {
                return Err(ErrorProcess::user(format!("Missing hash {item_hash}")));
            }
        };

        let commit_message = format!("delete {}", path.join("/"));
        let new_root_id = session.commit(commit_message).await?;
        Ok(new_root_id)
    }
}
