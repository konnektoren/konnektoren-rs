use konnektoren_asset_loader::AssetFormat;
use serde::de::DeserializeOwned;
use std::marker::PhantomData;

/// Default YAML parser. Applications can use it for their own Serde types.
pub struct Yaml<T>(PhantomData<fn() -> T>);
impl<T> Yaml<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}
impl<T> Default for Yaml<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T> Clone for Yaml<T> {
    fn clone(&self) -> Self {
        Self::new()
    }
}
impl<T: DeserializeOwned> AssetFormat for Yaml<T> {
    type Asset = T;
    type Error = serde_yaml::Error;

    fn extensions(&self) -> &[&str] {
        &["yml", "yaml"]
    }
    fn cache_key(&self) -> Option<String> {
        Some(String::new())
    }
    fn parse(&self, bytes: &[u8]) -> Result<T, Self::Error> {
        serde_yaml::from_slice(bytes)
    }
}
