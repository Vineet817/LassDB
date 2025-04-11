use serde::{Deserialize, Serialize};
// use crate::schema::User as UserSchema;
use sled::Db;
use bincode;
use anyhow::Result;
use crate::schema::{UserV1, UserV2};
pub struct UserStore {
    db: Db,
}

impl UserStore {
    pub fn new(path: &str) -> Result<Self> {
        let db = sled::open(path)?;
        Ok(Self { db })
    }

    pub fn put(&self, key: &str, user: &User) -> Result<()> {
        let serialized = bincode::serialize(user)?;
        self.db.insert(key, serialized)?;
        self.db.flush()?;
        Ok(())
    }

    pub fn get(&self, key: &str) -> Result<Option<User>> {
        if let Some(data) = self.db.get(key)? {
            let user: User = bincode::deserialize(&data)?;
            Ok(Some(user))
        } else {
            Ok(None)
        }
    }

    pub fn delete(&self, key: &str) -> Result<bool> {
        Ok(self.db.remove(key)?.is_some())
    }
}


// Enum to represent evolving schema
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "version", content = "data")]
pub enum User {
    V1(UserV1),
    V2(UserV2),
}

impl User {
    pub fn migrate(self) -> User {
        match self {
            User::V1(old) => User::V2(UserV2 {
                id: old.id,
                name: old.name,
                email: "unknown@example.com".to_string(), // default migration
            }),
            User::V2(_) => self,
        }
    }

    pub fn current_version(&self) -> &'static str {
        match self {
            User::V1(_) => "1",
            User::V2(_) => "2",
        }
    }
}
