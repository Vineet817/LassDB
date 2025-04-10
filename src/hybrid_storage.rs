use std::collections::HashMap;
use crate::{StorageBackend, file_storage::FileStorage, memory_storage::InMemoryStorage};

pub enum HybridStorage {
    File(FileStorage),
    Memory(InMemoryStorage),
}

impl HybridStorage {
    pub fn new(path: &str) -> Self {
        if std::path::Path::new(path).exists() {
            HybridStorage::File(FileStorage::new(path))
        } else {
            HybridStorage::Memory(InMemoryStorage::new(None))
        }
    }
}

impl StorageBackend for HybridStorage {
    fn load(&self) -> HashMap<String, String> {
        match self {
            HybridStorage::File(f) => f.load(),
            HybridStorage::Memory(m) => m.load(),
        }
    }

    fn save(&self, store: &HashMap<String, String>) {
        match self {
            HybridStorage::File(f) => f.save(store),
            HybridStorage::Memory(_) => {}, // no-op
        }
    }
}
