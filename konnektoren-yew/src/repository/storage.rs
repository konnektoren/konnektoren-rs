use super::storage_error::StorageError;
use async_trait::async_trait;

#[async_trait]
pub trait Storage {
    async fn get(&self, key: &str) -> Result<Option<String>, StorageError>;
    async fn set(&self, key: &str, value: &str) -> Result<(), StorageError>;
    async fn remove(&self, key: &str) -> Result<(), StorageError>;
}
