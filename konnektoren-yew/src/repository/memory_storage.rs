use super::storage::Storage;
use super::storage_error::StorageError;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Default)]
pub struct MemoryStorage {
    storage: Arc<RwLock<HashMap<String, String>>>,
}

#[async_trait]
impl Storage for MemoryStorage {
    async fn get(&self, key: &str) -> Result<Option<String>, StorageError> {
        let storage = self
            .storage
            .read()
            .map_err(|e| StorageError::AccessError(e.to_string()))?;
        Ok(storage.get(key).cloned())
    }

    async fn set(&self, key: &str, value: &str) -> Result<(), StorageError> {
        let mut storage = self
            .storage
            .write()
            .map_err(|e| StorageError::AccessError(e.to_string()))?;
        storage.insert(key.to_string(), value.to_string());
        Ok(())
    }

    async fn remove(&self, key: &str) -> Result<(), StorageError> {
        let mut storage = self
            .storage
            .write()
            .map_err(|e| StorageError::AccessError(e.to_string()))?;
        storage.remove(key);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_memory_storage() {
        let storage = MemoryStorage::default();
        storage.set("key", "value").await.unwrap();
        assert_eq!(storage.get("key").await.unwrap(), Some("value".to_string()));
        storage.remove("key").await.unwrap();
        assert_eq!(storage.get("key").await.unwrap(), None);
    }
}
