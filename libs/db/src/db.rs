use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::sync::{Arc, RwLock};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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

    pub fn store(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        let serialized_records = bincode::serialize(&*self.records.read().unwrap())?;
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(filename.to_owned() + ".hoya")?;

        file.write_all(&serialized_records)?;

        Ok(())
    }

    pub fn load(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        let records = fs::read(filename.to_owned() + ".hoya")?;
        let tree = bincode::deserialize::<Collection>(&records)?;

        let mut old_db = self.records.write().unwrap();
        *old_db = tree;
        Ok(())
    }
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}
