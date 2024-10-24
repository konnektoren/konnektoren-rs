mod certificate_repository;
mod game_state_persistence_impl;
mod inbox_repository;
mod profile_repository;
mod session_repository;
mod settings_repository;

mod local_storage;
mod memory_storage;

mod repository;
mod repository_error;

mod storage;
mod storage_error;

pub use certificate_repository::{
    CertificateRepository, CertificateRepositoryTrait, CERTIFICATE_STORAGE_KEY,
};
pub use game_state_persistence_impl::GameStatePersistenceImpl;
pub use inbox_repository::{InboxRepository, InboxRepositoryTrait, INBOX_STORAGE_KEY};
pub use profile_repository::{ProfileRepository, ProfileRepositoryTrait, PROFILE_STORAGE_KEY};
pub use session_repository::{SessionRepository, SessionRepositoryTrait, SESSION_STORAGE_KEY};
pub use settings_repository::{SettingsRepository, SettingsRepositoryTrait, SETTINGS_STORAGE_KEY};

pub use local_storage::LocalStorage;
pub use memory_storage::MemoryStorage;

pub use repository::Repository;
pub use repository_error::RepositoryError;

pub use storage::Storage;
pub use storage_error::StorageError;
