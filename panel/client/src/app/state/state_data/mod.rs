mod state_root;
mod state_node_dir;
mod state_node_content;

use state_node_dir::StateNodeDir;
use state_node_content::StateNodeContent;
use state_root::StateRoot;

use vertigo::{
    DomDriver,
    computed::{
        Dependencies,
    },
};
use crate::request::{Request};

#[derive(Clone)]
pub struct StateData {
    pub state_node_dir: StateNodeDir,
    pub state_node_content: StateNodeContent,
    pub state_root: StateRoot,
}

impl StateData {
    pub fn new(root: &Dependencies, driver: &DomDriver) -> StateData {

        let request = Request::new(driver);

        let state_node_dir = StateNodeDir::new(&request, root);
        let state_node_content = StateNodeContent::new(&request, root);
        let state_root = StateRoot::new(&request, root, state_node_dir.clone());

        StateData {
            state_node_dir,
            state_node_content,
            state_root,
        }
    }
}

pub use state_node_dir::{TreeItem};
