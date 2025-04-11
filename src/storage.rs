use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use sled::{Db, IVec};
use crate::schema::User;
pub trait StorageBackend {
    fn load(&self) -> HashMap<String, String>;
    fn save(&self, store: &HashMap<String, String>);
}
pub struct Storage {
    db: Db,
}

impl Storage {
    pub fn new(path: &str) -> Self {
        let db = sled::open(path).expect("Failed to open database");
        Self { db }
    }

    pub fn put_user(&self, key: &str, user: &User) {
        let encoded = bincode::serialize(user).expect("Serialization failed");
        self.db.insert(key, encoded).unwrap();
    }

    pub fn get_user(&self, key: &str) -> Option<User> {
        self.db.get(key).ok().flatten().and_then(|ivec| {
            bincode::deserialize(&ivec).ok()
        })
    }
}