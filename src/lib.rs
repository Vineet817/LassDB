pub mod config;
pub mod file_storage;
pub mod memory_storage;
pub mod hybrid_storage;

use config::Config;
use std::collections::HashMap;

pub trait StorageBackend {
    fn load(&self) -> HashMap<String, String>;
    fn save(&self, store: &HashMap<String, String>);
}

pub struct LassDB {
    store: HashMap<String, String>,
    backend: Box<dyn StorageBackend>,
    autosave: bool,
    flush_on_exit: bool,
    dirty: bool,config: Config,

}

impl LassDB {
pub fn with_config(backend: Box<dyn StorageBackend>, config: Config) -> Self {
    let store = backend.load();
    Self {
        store,
        backend,

        autosave: config.autosave,
        flush_on_exit: config.flush_on_exit,
        dirty: false,config,
    }
}

pub fn put(&mut self, key: String, value: String) {
    self.store.insert(key, value);
    self.dirty = true;
    if self.autosave {
        self.flush();
    }
}

pub fn get(&self, key: &str) -> Option<&String> {
    self.store.get(key)
}

pub fn flush(&mut self) {
    self.backend.save(&self.store);
    self.dirty = false;
}pub fn delete(&mut self, key: &str) -> bool {
        let removed = self.store.remove(key).is_some();
        if removed {
            self.dirty = true;
            if self.autosave {
                self.flush();
            }
        }
        removed
    }
}

impl Drop for LassDB {
    fn drop(&mut self) {
        if self.flush_on_exit && self.dirty {
            self.flush();
        }
    }
}
