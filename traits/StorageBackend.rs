pub trait StorageBackend {
    fn load(&self) -> HashMap<String, String>;
    fn save(&self, store: &HashMap<String, String>);
}
