use crate::user::User;
use sled::{Db, IVec};
use serde::{Serialize, Deserialize};
use anyhow::Result;

pub struct UserStore {
    db: Db,
}

impl UserStore {
    pub fn new(path: &str) -> Result<Self> {
        let db = sled::open(path)?;
        Ok(Self { db })
    }

    pub fn put(&self, key: &str, user: &User) -> Result<()> {
        let value = serde_json::to_vec(user)?;
        self.db.insert(key, value)?;
        self.db.flush()?;
        Ok(())
    }

    pub fn get(&self, key: &str) -> Result<Option<User>> {
        if let Some(ivec) = self.db.get(key)? {
            let user: User = serde_json::from_slice(&ivec)?;
            Ok(Some(user))
        } else {
            Ok(None)
        }
    }

    pub fn delete(&self, key: &str) -> Result<bool> {
        Ok(self.db.remove(key)?.is_some())
    }
}
