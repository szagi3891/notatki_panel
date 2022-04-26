use std::{collections::HashMap, rc::Rc};
use common::{GitTreeItem, HandlerFetchDirBody, HandlerFetchDirResponse};
use vertigo::{
    Resource,
    Driver,
    Computed,
    AutoMap,
};

use super::models::{GitDirList, TreeItem};

fn convert(list: HandlerFetchDirResponse) -> GitDirList {
    let mut out: HashMap<String, TreeItem> = HashMap::new();

    for item in list.list.into_iter() {
        let GitTreeItem {id, dir, name} = item;
        out.insert(name, TreeItem { dir, id });
    }

    GitDirList::new(Rc::new(out))
}

#[derive(Clone)]
pub struct NodeDir {
    value: Computed<Resource<GitDirList>>,
}

impl NodeDir {
    pub fn new(driver: &Driver, id: &String) -> NodeDir {
        let value = driver.new_value(Resource::Loading);
        let value_read = value.to_computed();

        let response = driver
            .request("/fetch_tree_item")
            .body_json(HandlerFetchDirBody {
                id: id.clone(),
            })
            .post();

        driver.spawn(async move {
            let response = response.await.into(|status, body| {
                if status == 200 {
                    return Some(body.into::<HandlerFetchDirResponse>());
                }
                None
            });
            value.set_value(response.map(convert));
        });

        NodeDir {
            value: value_read,
        }
    }

    pub fn get(&self) -> Rc<Resource<GitDirList>> {
        self.value.get_value()
    }

    pub fn get_list(&self) -> Resource<GitDirList> {
        self.get().ref_clone()
    }
}

#[derive(Clone)]
pub struct Dir {
    data: AutoMap<String, NodeDir>,
}

impl Dir {
    pub fn new(driver: &Driver) -> Dir {
        let data = {
            let request = driver.clone();

            AutoMap::new(move |id: &String| NodeDir::new(&request, id))
        };

        Dir {
            data
        }
    }

    pub fn get_list(&self, id: &String) -> Resource<GitDirList> {
        self.data.get_value(id).get_list()
    }
}
