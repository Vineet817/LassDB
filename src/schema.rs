use std::collections::HashMap;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserV1 {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserV2 {
    pub id: u32,
    pub name: String,
    pub email: String,
}


#[derive(Debug)]
pub struct SchemaRegistry {
    schemas: HashMap<&'static str, &'static str>,
}

impl SchemaRegistry {
    pub fn new() -> Self {
        let mut registry = HashMap::new();
        registry.insert("UserV1", "{ id: u32, name: String }");
        registry.insert("UserV2", "{ id: u32, name: String, email: String }");

        Self { schemas: registry }
    }

    pub fn get_schema(&self, name: &str) -> Option<&'static str> {
        self.schemas.get(name).copied()
    }

    pub fn all(&self) -> &HashMap<&'static str, &'static str> {
        &self.schemas
    }
}
