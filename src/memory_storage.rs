use std::collections::HashMap;
use crate::StorageBackend;

pub struct InMemoryStorage {
    store: Option<HashMap<String, String>>,
}

impl InMemoryStorage {
    pub fn new(initial: Option<HashMap<String, String>>) -> Self {
        Self { store: initial }
    }
}

impl StorageBackend for InMemoryStorage {
    fn load(&self) -> HashMap<String, String> {
        self.store.clone().unwrap_or_default()
    }

    fn save(&self, _store: &HashMap<String, String>) {
        // No-op for pure memory
    }
}
