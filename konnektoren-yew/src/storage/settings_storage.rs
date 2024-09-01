use crate::model::Settings;
use crate::storage::Storage;
use gloo::storage::{LocalStorage, Storage as _};

#[derive(Debug, Default)]
pub struct SettingsStorage {}

impl Storage for SettingsStorage {
    const NAME: &'static str = "settings";
    type Item = Settings;

    fn get(&self, id: &str) -> Option<Self::Item> {
        let key = format!("{}:{}", Self::NAME, id);
        if let Ok(item) = LocalStorage::get::<String>(key) {
            serde_json::from_str(&item).unwrap_or_default()
        } else {
            None
        }
    }

    fn get_all(&self) -> Vec<Self::Item> {
        unimplemented!("We can not get all settings")
    }

    fn insert(&mut self, item: Self::Item) {
        let key = format!("{}:{}", Self::NAME, item.id);
        let value = serde_json::to_string(&item).unwrap();
        LocalStorage::set(key, value).unwrap();
    }

    fn update(&mut self, item: Self::Item) {
        self.insert(item);
    }

    fn delete(&mut self, id: &str) {
        let key = format!("{}:{}", Self::NAME, id);
        LocalStorage::delete(key);
    }
}
