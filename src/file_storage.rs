use std::{collections::HashMap, fs, path::PathBuf, fs::File};
use crate::StorageBackend;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct SerializableStore(HashMap<String, String>);

pub struct FileStorage {
    path: PathBuf,
}

impl FileStorage {
    pub fn new(path: &str) -> Self {
        Self {
            path: PathBuf::from(path),
        }
    }
}

impl StorageBackend for FileStorage {
    fn load(&self) -> HashMap<String, String> {
        if let Ok(file) = File::open(&self.path) {
            serde_json::from_reader(file).unwrap_or_default()
        } else {
            HashMap::new()
        }
    }

    fn save(&self, store: &HashMap<String, String>) {
        let tmp_path = self.path.with_extension("tmp");
        let file = File::create(&tmp_path).unwrap();
        serde_json::to_writer(file, store).unwrap();
        fs::rename(tmp_path, &self.path).unwrap();
    }
}
