pub mod user;
pub mod schema;
pub mod store;

use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use user::User;

static DB: Lazy<Mutex<HashMap<String, User>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub fn put_user(key: String, user: User) {
    let mut db = DB.lock().unwrap();
    db.insert(key, user);
}

pub fn get_user(key: &str) -> Option<User> {
    let db = DB.lock().unwrap();
    db.get(key).cloned()
}

pub fn delete_user(key: &str) -> bool {
    let mut db = DB.lock().unwrap();
    db.remove(key).is_some()
}
