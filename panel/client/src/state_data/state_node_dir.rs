use std::{collections::HashMap, rc::Rc};
use common::{GitTreeItem, HandlerFetchDirBody, HandlerFetchDirResponse};
use vertigo::{
    Resource,
    DomDriver,
    computed::{
        Computed,
        Dependencies,
        AutoMap,
    },
};

#[derive(PartialEq, Clone, Debug)]
pub struct TreeItem {
    pub dir: bool,
    pub id: String,
}

fn convert(list: HandlerFetchDirResponse) -> Rc<HashMap<String, TreeItem>> {
    let mut out: HashMap<String, TreeItem> = HashMap::new();

    for item in list.list.into_iter() {
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
    pub fn new(request: &DomDriver, dependencies: &Dependencies, id: &String) -> NodeDir {
        let value = dependencies.new_value(Resource::Loading);
        let value_read = value.to_computed();

        let response = request
            .request("/fetch_tree_item")
            .body_json(HandlerFetchDirBody {
                id: id.clone(),
            })
            .post();

        request.spawn(async move {
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

    pub fn get(&self) -> Rc<Resource<Rc<HashMap<String, TreeItem>>>> {
        self.value.get_value()
    }

    pub fn get_list(&self) -> Resource<Rc<HashMap<String, TreeItem>>> {
        self.get().ref_clone()
    }
}

#[derive(PartialEq, Clone)]
pub struct StateNodeDir {
    data: AutoMap<String, NodeDir>,
}

impl StateNodeDir {
    pub fn new(request: &DomDriver, dependencies: &Dependencies) -> StateNodeDir {
        let data = {
            let request = request.clone();
            let dependencies = dependencies.clone();

            AutoMap::new(move |id: &String| NodeDir::new(&request, &dependencies, id))
        };

        StateNodeDir {
            data
        }
    }

    pub fn get_list(&self, id: &String) -> Resource<Rc<HashMap<String, TreeItem>>> {
        self.data.get_value(id).get_list()
    }
}
