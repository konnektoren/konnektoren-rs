pub mod profile_storage;
mod settings_storage;

pub trait Storage {
    const NAME: &'static str;
    type Item;
    fn get(&self, id: &str) -> Option<Self::Item>;
    fn get_all(&self) -> Vec<Self::Item>;
    fn insert(&mut self, item: Self::Item);
    fn update(&mut self, item: Self::Item);
    fn delete(&mut self, id: &str);
}

pub use profile_storage::ProfileStorage;
pub use settings_storage::SettingsStorage;
