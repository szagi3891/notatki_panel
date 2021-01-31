use common::{DataNode, ServerFetchNodePost};
use vertigo::{
    DomDriver,
    FetchMethod,
    computed::{
        Value,
        Computed,
        Dependencies,
        AutoMap
    }
};
use std::sync::Arc;

#[derive(PartialEq)]
pub enum Resource<T: PartialEq> {
    Loading,
    Ready(T),
    Failed(String),
}


#[derive(PartialEq)]
pub struct State {
    pub driver: DomDriver,
    pub current_path: Value<Vec<Arc<String>>>,
    data: AutoMap<Vec<String>, Resource<DataNode>>,
}

impl State {
    pub fn new(root: &Dependencies, driver: &DomDriver) -> Computed<State> {
        let feth_node = {
            let root = root.clone();
            let driver = driver.clone();

            move |path: &Vec<String>| -> Computed<Resource<DataNode>> {
                let value = root.new_value(Resource::Loading);
                let result = value.to_computed();
    
                fetch_path(&path, value, &driver);
    
                result
            }
        };

        // let start_path = vec!("aaa".into(), "bbb".into());
        let start_path = Vec::new();

        root.new_computed_from(State {
            driver: driver.clone(),
            current_path: root.new_value(start_path),
            data: AutoMap::new(feth_node)
        })
    }

    pub fn set_path(&self, new_path: &Vec<Arc<String>>) {
        self.current_path.set_value(new_path.clone());
    }

    //TODO - do celow testowych

    pub fn push_path(&self) {

        //TODO - dorobić do Value funkcję change ...

        //self.current_path.change
        let mut current = (&*self.current_path.get_value()).clone();
        
        current.push(Arc::new("cokolwiek".into()));

        self.current_path.set_value(current);
    }
}

fn fetch_path(path: &Vec<String>, value: Value<Resource<DataNode>>, driver: &DomDriver) {

    let url = format!("/fetch_node");
    let body = ServerFetchNodePost {
        path: path.clone(),
    };

    let body_str = serde_json::to_string(&body).unwrap();

    let driver_fetch = driver.clone();
    driver.spawn_local(async move {
        let response = driver_fetch.fetch(FetchMethod::GET, url, None, Some(body_str)).await;

        match response {
            Ok(response) => {
                match serde_json::from_str::<DataNode>(response.as_str()) {
                    Ok(branch) => {
                        log::info!("odpowiedź z serwera {:?}", branch);
                        value.set_value(Resource::Ready(branch));
                    },
                    Err(err) => {
                        log::error!("Error parsing response: {}", err);
                        value.set_value(Resource::Failed(err.to_string()));
                    }
                }
            },
            Err(_) => {
                log::error!("Error fetch");
                value.set_value(Resource::Failed("Error fetch".into()));
            }
        }
    });
}


/*
dodać zakładki


    zakładka1: notatki
    zakładka2: parsowanie urla ...

    zakładka - zarządzanie gitem
        menu pozwalające usuwać niepotrzebne gałęzie gita ...
    
        pozwoli np. ta funkcja na uruchomienie polecenia rebejsującego
*/
