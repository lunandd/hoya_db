use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, PartialEq, Clone)]
pub enum DBTypes {
    Number(isize),
    Float(f64),
    Boolean(bool),
    Text(String),
    List(Vec<DBTypes>),
    Unit(()),
}

type Collection = BTreeMap<String, DBTypes>;
type Records = Arc<RwLock<Collection>>;

#[derive(Debug, Clone)]
pub struct Database {
    records: Records,
}

unsafe impl Sync for Database {}
unsafe impl Send for Database {}

impl Database {
    pub fn new() -> Self {
        Self {
            records: Arc::new(RwLock::new(Collection::new())),
        }
    }

    pub fn get(&self, key: &str) -> Option<DBTypes> {
        self.records.read().unwrap().get(key).cloned()
    }

    pub fn put(&self, key: String, value: DBTypes) -> Option<DBTypes> {
        self.records.write().unwrap().insert(key, value)
    }

    pub fn remove(&self, key: &str) -> Option<DBTypes> {
        self.records.write().unwrap().remove(key)
    }

    pub fn exists(&self, key: &str) -> bool {
        self.records.read().unwrap().contains_key(key)
    }

    // TODO: Storing the database in a file
    pub fn store(&self, _filename: &str) {
        todo!()
    }

    // TODO: Loading the database from a file
    pub fn load(&self, _filename: &str) {
        todo!()
    }
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}
