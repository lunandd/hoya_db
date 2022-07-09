use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, PartialEq, Clone)]
pub enum DBTypes {
    Number(isize),
    Float(f64),
    Boolean(bool),
    Text(String),
    List(Vec<DBTypes>),
}

type Collection = BTreeMap<String, DBTypes>;
type Records = Arc<RwLock<Collection>>;

#[derive(Debug, Clone)]
pub struct DB {
    records: Records,
}

unsafe impl Sync for DB {}
unsafe impl Send for DB {}

impl DB {
    pub fn new() -> Self {
        Self {
            records: Arc::new(RwLock::new(BTreeMap::new())),
        }
    }

    // TODO: Not use Option.cloned(), if that's even possible
    pub fn get(&self, key: &str) -> Option<DBTypes> {
        self.records.read().unwrap().get(key).cloned()
    }

    pub fn put(&mut self, key: String, value: DBTypes) -> Option<DBTypes> {
        self.records.write().unwrap().insert(key, value)
    }

    pub fn remove(&mut self, key: &str) -> Option<DBTypes> {
        self.records.write().unwrap().remove(key)
    }

    pub fn exists(&self, key: &str) -> bool {
        self.records.read().unwrap().contains_key(key)
    }
}

impl Default for DB {
    fn default() -> Self {
        Self::new()
    }
}
