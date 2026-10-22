use crate::{AssetLoadError, AssetServer};
use konnektoren_core::assets::{AssetFormat, AssetSource};
use std::sync::Arc;
pub trait AssetCollection {
    const FOLDER: &'static str;
    fn asset_path(name: &str) -> String {
        let folder = Self::FOLDER.trim_end_matches('/');
        let name = name.trim_start_matches('/');
        if folder.is_empty() {
            name.to_owned()
        } else {
            format!("{folder}/{name}")
        }
    }
    fn load<'a, S, F>(
        server: &'a AssetServer<S>,
        format: F,
        name: &'a str,
    ) -> impl std::future::Future<Output = Result<Arc<F::Asset>, AssetLoadError>> + 'a
    where
        S: AssetSource + ?Sized + 'static,
        F: AssetFormat + 'static,
        F::Asset: Send + Sync + 'static,
    {
        async move { server.load(format, Self::asset_path(name)).await }
    }
}
