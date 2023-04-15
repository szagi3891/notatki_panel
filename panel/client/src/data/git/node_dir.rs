use std::{collections::HashMap, rc::Rc};
use common::{GitTreeItem, HandlerFetchDirBody, HandlerFetchDirResponse};
use vertigo::{
    Resource,
    Computed,
    AutoMap, LazyCache, Context, RequestBuilder,
};

use super::models::{TreeItem};

fn convert(list: Rc<HandlerFetchDirResponse>) -> Rc<HashMap<String, TreeItem>> {
    let mut out: HashMap<String, TreeItem> = HashMap::new();

    for item in list.list.iter() {
        let GitTreeItem {id, dir, name} = item;
        out.insert(name.clone(), TreeItem {
            dir: *dir,
            id: id.clone(),
        });
    }

    Rc::new(out)
}

#[derive(Clone)]
struct NodeDir {
    _response: LazyCache<HandlerFetchDirResponse>,
    list: Computed<Resource<Rc<HashMap<String, TreeItem>>>>,
}

impl PartialEq for NodeDir {
    fn eq(&self, other: &Self) -> bool {
        self.list.eq(&other.list)
    }
}

impl NodeDir {
    pub fn new(_: &AutoMap<String, NodeDir>, id: &String) -> NodeDir {
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

    pub fn get(&self, context: &Context) -> Resource<Rc<HashMap<String, TreeItem>>> {
        self.list.get(context)
    }

    pub fn get_list(&self, context: &Context) -> Resource<Rc<HashMap<String, TreeItem>>> {
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

    pub fn get_list(&self, context: &Context, id: &String) -> Resource<Rc<HashMap<String, TreeItem>>> {
        self.data.get(id).get_list(context)
    }
}
