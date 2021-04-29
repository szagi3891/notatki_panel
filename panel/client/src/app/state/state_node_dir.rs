use std::{collections::HashMap, rc::Rc};
use common::{GitTreeItem, HandlerHetchDirBody};
use vertigo::{
    computed::{
        Computed,
        Dependencies,
        AutoMapBox,
    },
};

use crate::request::{Request, Resource, ResourceError};

#[derive(PartialEq, Clone)]
pub struct TreeItem {
    pub dir: bool,
    pub id: String,
}

fn convert(list: Vec<GitTreeItem>) -> Rc<HashMap<String, TreeItem>> {
    let mut out: HashMap<String, TreeItem> = HashMap::new();

    for item in list.into_iter() {
        let GitTreeItem {id, dir, name} = item;
        out.insert(name, TreeItem { dir, id });
    }

    Rc::new(out)
}

#[derive(PartialEq, Clone)]
pub struct NodeDir {
    value: Computed<Resource<Rc<HashMap<String, TreeItem>>>>,
}

impl NodeDir {
    pub fn new(request: &Request, dependencies: &Dependencies, id: &String) -> NodeDir {
        let value = dependencies.new_value(Err(ResourceError::Loading));
        let value_read = value.to_computed();

        let response = request
            .fetch("/fetch_tree_item")
            .body(&HandlerHetchDirBody {
                id: id.clone(),
            })
            .post::<Vec<GitTreeItem>>();

        request.spawn_local(async move {
            let response = response.await;
            value.set_value(response.map(convert));
        });

        NodeDir {
            value: value_read,
        }
    }

    pub fn get(&self) -> Rc<Resource<Rc<HashMap<String, TreeItem>>>> {
        self.value.get_value()
    }

    pub fn get_list(&self) -> Result<Rc<HashMap<String, TreeItem>>, ResourceError> {
        let list = self.get();

        let value = match &*list {
            Ok(inner) => inner,
            Err(err) => return Err(err.clone()),
        };

        Ok(value.clone())
    }
}

#[derive(PartialEq, Clone)]
pub struct StateNodeDir {
    data: AutoMapBox<String, NodeDir>,
}

impl StateNodeDir {
    pub fn new(request: &Request, dependencies: &Dependencies) -> StateNodeDir {
        let data = {
            let request = request.clone();
            let dependencies = dependencies.clone();

            AutoMapBox::new(move |id: &String| NodeDir::new(&request, &dependencies, id))
        };

        StateNodeDir {
            data
        }
    }

    pub fn get(&self, id: &String) -> NodeDir {
        self.data.get_value(id)
    }

    pub fn get_list(&self, id: &String) -> Result<Rc<HashMap<String, TreeItem>>, ResourceError> {
        log::info!("get list --> {}", id);
        self.data.get_value(id).get_list()
    }
}
