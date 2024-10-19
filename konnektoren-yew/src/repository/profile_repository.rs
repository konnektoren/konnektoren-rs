use super::repository::Repository;
use super::repository_error::RepositoryError;
use super::storage::Storage;
use async_trait::async_trait;
use konnektoren_core::prelude::PlayerProfile;
use serde_json;

pub const PROFILE_STORAGE_KEY: &str = "konnektoren_profile";

#[derive(Debug, PartialEq)]
pub struct ProfileRepository<S: Storage> {
    storage: S,
}

impl<S: Storage> ProfileRepository<S> {
    pub fn new(storage: S) -> Self {
        Self { storage }
    }
}

#[async_trait]
impl<S: Storage + Send + Sync> Repository<PlayerProfile> for ProfileRepository<S> {
    async fn save(&self, key: &str, profile: &PlayerProfile) -> Result<(), RepositoryError> {
        let serialized =
            serde_json::to_string(profile).map_err(|e| RepositoryError::SerializationError(e))?;
        self.storage
            .set(key, &serialized)
            .await
            .map_err(|e| RepositoryError::StorageError(e.to_string()))
    }

    async fn get(&self, key: &str) -> Result<Option<PlayerProfile>, RepositoryError> {
        match self.storage.get(key).await {
            Ok(Some(serialized)) => {
                let profile = serde_json::from_str(&serialized)
                    .map_err(|e| RepositoryError::SerializationError(e))?;
                Ok(Some(profile))
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

impl<S: Storage + Send + Sync> ProfileRepository<S> {
    pub async fn get_profile(&self, key: &str) -> Result<Option<PlayerProfile>, RepositoryError> {
        self.get(key).await
    }

    pub async fn update_profile(
        &self,
        key: &str,
        profile: &PlayerProfile,
    ) -> Result<(), RepositoryError> {
        self.save(key, profile).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::memory_storage::MemoryStorage;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_profile_repository() {
        let storage = MemoryStorage::default();
        let repo = ProfileRepository::new(storage);
        let key = PROFILE_STORAGE_KEY;

        // Test saving a profile
        let profile = PlayerProfile {
            id: "123".to_string(),
            name: "Alice".to_string(),
            xp: 100,
        };
        repo.update_profile(key, &profile).await.unwrap();

        // Test getting the profile
        let stored_profile = repo.get_profile(key).await.unwrap().unwrap();
        assert_eq!(profile, stored_profile);

        // Test updating the profile
        let updated_profile = PlayerProfile {
            id: "123".to_string(),
            name: "Alice".to_string(),
            xp: 200,
        };
        repo.update_profile(key, &updated_profile).await.unwrap();
        let stored_updated_profile = repo.get_profile(key).await.unwrap().unwrap();
        assert_eq!(updated_profile, stored_updated_profile);

        // Test deleting the profile
        repo.delete(key).await.unwrap();
        let deleted_profile = repo.get_profile(key).await.unwrap();
        assert!(deleted_profile.is_none());
    }
}
