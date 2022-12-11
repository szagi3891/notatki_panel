use std::{collections::HashMap, rc::Rc};
use common::{GitTreeItem, HandlerFetchDirBody, HandlerFetchDirResponse};
use vertigo::{
    Resource,
    Computed,
    AutoMap, LazyCache, Context, RequestBuilder,
};

use super::models::{GitDirList, TreeItem};

fn convert(list: Rc<HandlerFetchDirResponse>) -> GitDirList {
    let mut out: HashMap<String, TreeItem> = HashMap::new();

    for item in list.list.iter() {
        let GitTreeItem {id, dir, name} = item;
        out.insert(name.clone(), TreeItem {
            dir: *dir,
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

impl PartialEq for NodeDir {
    fn eq(&self, other: &Self) -> bool {
        self.list.eq(&other.list)
    }
}

impl NodeDir {
    pub fn new(id: &String) -> NodeDir {
        let response = RequestBuilder::post("/fetch_tree_item")
            .body_json(HandlerFetchDirBody {
                id: id.to_string(),
            })
            .lazy_cache(|status, body| {
                if status == 200 {
                    Some(body.into::<HandlerFetchDirResponse>())
                } else {
                    None
                }
            });

        let response2 = response.clone();

        let list  = Computed::from(move |context| {
            let resource = response2.get(context);
            resource.map(convert)
        });

        NodeDir {
            _response: response,
            list,
        }
    }

    pub fn get(&self, context: &Context) -> Resource<GitDirList> {
        self.list.get(context)
    }

    pub fn get_list(&self, context: &Context) -> Resource<GitDirList> {
        self.get(context).ref_clone()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Dir {
    data: AutoMap<String, NodeDir>,
}

impl Dir {
    pub fn new() -> Dir {
        let data = AutoMap::new(NodeDir::new);

        Dir {
            data
        }
    }

    pub fn get_list(&self, context: &Context, id: &String) -> Resource<GitDirList> {
        self.data.get(id).get_list(context)
    }
}
