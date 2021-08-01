use crate::git::GitBlob;

use crate::utils::ErrorProcess;

use super::gitsync::Gitsync;


#[derive(Clone)]
pub struct Git {
    gitsync: Gitsync,
}

impl Git {
    pub fn new(path: String, branch_name: String) -> Git {

        let gitsync = Gitsync::new(path.clone(), branch_name.clone()).unwrap();     //TODO ...

        Git {
            gitsync,
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
        self.gitsync.command_create_file(path, new_path, new_content).await
    }

    pub async fn rename_item(&self, path: Vec<String>, prev_name: String, prev_hash: String, new_name: String) -> Result<String, ErrorProcess> {
        self.gitsync.command_rename_item(path, prev_name, prev_hash, new_name).await
    }
}
