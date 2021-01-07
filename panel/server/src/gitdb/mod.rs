use std::collections::BTreeMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/*
    element:
        ostatni timestamp zmiany ...
        data: ....      dane binarne

    2018-01-26T18:30:09.453Z

    Z - moze byc markerem oznaczajacym koniec daty.
    ALbo, mozna wybrac inna literke

    
    zapis mozliwy bylby tylko pod warunkiem ze w request podalismy timestamp aktualnej tresci
    nowy timestamp musi byc wiekszy od tego ktory jest obecnie
*/

type TimestampType = u64;
type ItemIdType = u32;

struct ItemInfoInner {
    last_modification: Option<TimestampType>,
}
impl ItemInfoInner {
    pub fn new() -> ItemInfoInner {
        ItemInfoInner {
            last_modification: None,
        }
    }
}

#[derive(Clone)]
struct ItemInfo {
    data: Arc<RwLock<ItemInfoInner>>,
}

impl ItemInfo {
    pub fn new() -> ItemInfo {
        ItemInfo {
            data: Arc::new(RwLock::new(ItemInfoInner::new()))
        }
    }
}

pub struct GitDB {
    dir_path: String,
    data: Arc<RwLock<BTreeMap<ItemIdType, ItemInfo>>>,
}

impl GitDB {
    pub fn new(dir_path: String) -> GitDB {
        GitDB {
            dir_path,
            data: Arc::new(RwLock::new(BTreeMap::new()))
        }
    }

    async fn get_or_create(&self, id: ItemIdType) -> ItemInfo {
        let mut lock = self.data.write().await;

        if let Some(item) = lock.get(&id) {
            return item.clone();
        }

        let new_item = ItemInfo::new();
        lock.insert(id, new_item.clone());
        new_item
    }

    async fn get_path(&self, id: ItemIdType) -> String {
        //trzeba skleic dir_path z id odpowiednio sformatowanym. to zwroci pelna sciezke do tego obiektu
        todo!();
    }

    pub async fn get(&self, id: ItemIdType) -> (TimestampType, Vec<u8>) {
        todo!();
    }

    pub async fn save(&self, id: ItemIdType, timestamp: TimestampType, content: Vec<u64>) {

        //jak dostalismy blokade do item-a
        //to sprawdz po drodze czy istnieja wszystkie zagniezdzone katalogi prowaddzace do tego elementu
    
        todo!();
    }
}