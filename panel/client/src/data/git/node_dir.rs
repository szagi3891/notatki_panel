use std::{collections::HashMap, rc::Rc};
use common::{GitTreeItem, HandlerFetchDirBody, HandlerFetchDirResponse};
use vertigo::{
    Resource,
    Computed,
    AutoMap, LazyCache, get_driver,
};

use super::models::{GitDirList, TreeItem};

fn convert(list: &HandlerFetchDirResponse) -> GitDirList {
    let mut out: HashMap<String, TreeItem> = HashMap::new();

    for item in list.list.iter() {
        let GitTreeItem {id, dir, name} = item;
        out.insert(name.clone(), TreeItem {
            dir: dir.clone(),
            id: id.clone(),
        });
    }

    GitDirList::new(Rc::new(out))
}

#[derive(Clone)]
struct NodeDir {
    _response: LazyCache<HandlerFetchDirResponse>,
    list: Computed<Resource<GitDirList>>,
}

impl NodeDir {
    pub fn new(id: &String) -> NodeDir {
        let id = id.clone();

        let response = LazyCache::new(10 * 60 * 60 * 1000, move || {
            let id = id.clone();
            async move {
                let request = get_driver()
                    .request("/fetch_tree_item")
                    .body_json(HandlerFetchDirBody {
                        id,
                    })
                    .post();

                request.await.into(|status, body| {
                    if status == 200 {
                        Some(body.into::<HandlerFetchDirResponse>())
                    } else {
                        None
                    }
                })
            }
        });

        let response2 = response.clone();

        let list = Computed::from(move || {
            let resource = response2.get_value();
            resource.ref_map(convert)
        });

        NodeDir {
            _response: response,
            list,
        }
    }

    pub fn get(&self) -> Rc<Resource<GitDirList>> {
        self.list.get_value()
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
    pub fn new() -> Dir {
        let data = AutoMap::new(move |id: &String| NodeDir::new(id));

        Dir {
            data
        }
    }

    pub fn get_list(&self, id: &String) -> Resource<GitDirList> {
        self.data.get_value(id).get_list()
    }
}
