use std::collections::BTreeMap;
use std::sync::Arc;
use tokio::sync::RwLock;

type TimestampType = u64;
type ItemIdType = u64;

fn get_path_chunks(dir_path: &String, id: &ItemIdType) -> Vec<String> {
    let mut id = *id as u64;

    if id == 0 {
        return vec!(dir_path.clone(), "f000".into());
    }

    let mut chunks: Vec<String> = Vec::new();
    let mut is_first_exec = false;

    while id > 0 {
        let prefix = if is_first_exec == false {
            is_first_exec = true;
            "f"
        } else {
            "d"
        };
    
        chunks.push(format!("{}{:03}", prefix, id % 1000));
        id = id / 1000;
    }

    let mut result: Vec<String> = Vec::new();

    result.push(dir_path.clone());

    for item in chunks.iter().rev() {
        result.push(item.clone());
    }

    result
}

fn get_dir(dir_path: &String, id: &ItemIdType) -> String {
    let mut result: Vec<String> = get_path_chunks(dir_path, id);
    result.pop();
    result.join("/")
}

fn get_path(dir_path: &String, id: &ItemIdType) -> String {
    let mut result: Vec<String> = get_path_chunks(dir_path, id);
    result.join("/")
}

#[test]
fn test_get_path() {
    assert_eq!(get_path(&"/bazowy/katalog".into(), &(0 as ItemIdType)), String::from("/bazowy/katalog/f000"));
    assert_eq!(get_path(&"/bazowy/katalog".into(), &(43 as ItemIdType)), String::from("/bazowy/katalog/f043"));
    assert_eq!(get_path(&"/bazowy/katalog".into(), &(234222 as ItemIdType)), String::from("/bazowy/katalog/d234/f222"));
}

fn decode_timestamp(timestamp: Vec<u8>) -> u64 {
    let mut out: u64 = 0;

    for char in timestamp {
        let digit = char - b'0';
        out = 10 * out + digit as u64;
    }

    out
}

#[test]
fn test_decode_timestamp() {
    assert_eq!(decode_timestamp(vec!(b'9', b'9', b'9', b'5', b'1')), 99951);
    assert_eq!(decode_timestamp(vec!(b'9', b'5', b'1')), 951);
    assert_eq!(decode_timestamp(vec!(b'0')), 0);
}

fn encode_timestamp(mut timestamp: u64) -> Vec<u8> {
    let mut out = Vec::<u8>::new();

    while timestamp > 0 {
        out.push(b'0' + (timestamp % 10) as u8);
        timestamp = timestamp / 10;
    }

    let mut result = Vec::new();

    for item in out.iter().rev() {
        result.push(*item);
    }

    if result.len() == 0 {
        return vec!(b'0');
    }

    result
}

#[test]
fn test_encode_timestamp() {
    assert_eq!(encode_timestamp(99951), vec!(b'9', b'9', b'9', b'5', b'1'));
    assert_eq!(encode_timestamp(951), vec!(b'9', b'5', b'1'));
    assert_eq!(encode_timestamp(0), vec!(b'0'));
}

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

pub enum SaveError {
    Error(std::io::Error),
    OutdatedTimestamp,
}

impl From<std::io::Error> for SaveError {
    fn from(error: std::io::Error) -> Self {
        SaveError::Error(error)
    }
}

// SaveError

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

    pub async fn get(&self, id: ItemIdType) -> Result<(TimestampType, Vec<u8>), std::io::Error> {
        let item = self.get_or_create(id).await;
        let lock = item.data.write().await;
        
        let file_path = get_path(&self.dir_path, &id);

        let mut data = tokio::fs::read(file_path).await?;

        let mut timestamp = Vec::<u8>::new();
    
        //Format
        //data-data-data-dataX0123456789

        loop {
            let char = data.pop().unwrap();

            if char.is_ascii_digit() {
                timestamp.push(char);
                continue;
            }

            if char == b'X' {
                break;
            }

            panic!("data corrupted");
        }

        let timestamp: Vec<u8> = timestamp.iter().rev().cloned().collect();
        
        let timestamp: TimestampType = decode_timestamp(timestamp);
        
        std::mem::forget(lock);

        Ok((timestamp, data))
    }

    pub async fn save(&self, id: ItemIdType, timestamp: TimestampType, mut content: Vec<u8>) -> Result<(), SaveError> {
        let item = self.get_or_create(id).await;
        let lock = item.data.write().await;

        let (file_time, _) = self.get(id).await?;

        if file_time != timestamp {
            return Err(SaveError::OutdatedTimestamp);
        }

        let dir = get_dir(&self.dir_path, &id);
        let file = get_path(&self.dir_path, &id);

        tokio::fs::create_dir_all(dir).await?;


        content.push(b'X');
        content.extend(encode_timestamp(timestamp));

        tokio::fs::write(file, content).await?;

        std::mem::forget(lock);
        Ok(())
    }
}
