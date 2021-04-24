use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub type TimestampType = u128;
pub type DataNodeIdType = Arc<String>;           //40chars

                                  //TODO - po stronie serwera pasowałoby zeby string byl opakowany w Arc
                                  //TODO - a po stronie przegladarki w Rc

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub enum DataNode {
    File {
        id: DataNodeIdType,
        title: String,
        content: String,
    },
    Dir {
        id: DataNodeIdType,
        title: String,
        child: Vec<DataNodeIdType>,         //rozjazdowka na kolejne dzieci z trescia
    }
}

impl DataNode {
    pub fn title(&self) -> String {
        match self {
            DataNode::File { title, .. } => title.clone(),
            DataNode::Dir { title, .. } => title.clone(),
        }
    }
}

//Ta struktura będzie latać na handlerach http ...
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct DataPost {
    pub timestamp: TimestampType,
    pub node: DataNode,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PostParamsFetchNodePost {
    pub node_id: DataNodeIdType
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PostParamsCreateDir {
    pub parent_node: DataNodeIdType,
    pub name: String,
}
