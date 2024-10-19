use super::repository::Repository;
use super::repository_error::RepositoryError;
use super::storage::Storage;
use crate::model::Settings;
use async_trait::async_trait;

pub const SETTINGS_STORAGE_KEY: &str = "konnektoren_settings";

pub struct SettingsRepository<S: Storage> {
    storage: S,
}

impl<S: Storage> SettingsRepository<S> {
    pub fn new(storage: S) -> Self {
        Self { storage }
    }
}

#[async_trait]
impl<S: Storage + Send + Sync> Repository<Settings> for SettingsRepository<S> {
    async fn save(&self, key: &str, settings: &Settings) -> Result<(), RepositoryError> {
        let serialized =
            serde_json::to_string(settings).map_err(RepositoryError::SerializationError)?;
        self.storage
            .set(key, &serialized)
            .await
            .map_err(|e| RepositoryError::StorageError(e.to_string()))
    }

    async fn get(&self, key: &str) -> Result<Option<Settings>, RepositoryError> {
        match self.storage.get(key).await {
            Ok(Some(serialized)) => {
                let settings = serde_json::from_str(&serialized)
                    .map_err(RepositoryError::SerializationError)?;
                Ok(Some(settings))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(RepositoryError::StorageError(e.to_string())),
        }
    }

    async fn delete(&self, key: &str) -> Result<(), RepositoryError> {
        self.storage
            .remove(key)
            .await
            .map_err(|e| RepositoryError::StorageError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Settings;
    use crate::repository::MemoryStorage;

    #[tokio::test]
    async fn test_save_and_get() {
        let storage = MemoryStorage::default();
        let repository = SettingsRepository::new(storage);

        let settings = Settings::default();
        repository
            .save(SETTINGS_STORAGE_KEY, &settings)
            .await
            .unwrap();

        let loaded = repository.get(SETTINGS_STORAGE_KEY).await.unwrap().unwrap();
        assert_eq!(settings, loaded);
    }

    #[tokio::test]
    async fn test_delete() {
        let storage = MemoryStorage::default();
        let repository = SettingsRepository::new(storage);

        let settings = Settings::default();
        repository
            .save(SETTINGS_STORAGE_KEY, &settings)
            .await
            .unwrap();

        repository.delete(SETTINGS_STORAGE_KEY).await.unwrap();

        let loaded = repository.get(SETTINGS_STORAGE_KEY).await.unwrap();
        assert!(loaded.is_none());
    }
}
