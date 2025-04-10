use serde::Deserialize;
#[derive(Debug, Clone,Deserialize)]
pub struct Config {
    pub autosave: bool,
    pub flush_on_exit: bool,
    pub snapshot_path: String,
}
