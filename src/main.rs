use lassdb::{LassDB, file_storage::FileStorage, config::Config};
use std::fs;
use toml;

fn load_config() -> Config {
    let content = fs::read_to_string("lassdb.toml").expect("Config file missing!");
    toml::from_str(&content).expect("Invalid config format!")
}

fn main() {
    let config = load_config();
    let storage = FileStorage::new(&config.snapshot_path);
    let mut db = LassDB::with_config(storage, config);

    db.put("cool".into(), "feature!".into());
}
