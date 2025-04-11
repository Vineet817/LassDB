use wasm_bindgen::prelude::*;
use lassdb::{put_user, get_user, delete_user};
use lassdb::user::User;

#[wasm_bindgen]
pub fn put(key: String, user_json: String) {
    let user: User = serde_json::from_str(&user_json).unwrap();
    put_user(key, user);
}

#[wasm_bindgen]
pub fn get(key: String) -> Option<String> {
    get_user(&key).map(|u| serde_json::to_string(&u).unwrap())
}

#[wasm_bindgen]
pub fn _delete(key: String) -> bool {
    delete_user(&key)
}

#[wasm_bindgen]
pub fn get_schema(name: String) -> Option<String> {
    let registry = lassdb::schema::SchemaRegistry::new();
    registry.get_schema(&name).map(|s| s.to_string())
}
