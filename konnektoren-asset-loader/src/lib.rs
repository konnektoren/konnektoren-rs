#![doc = include_str!("../README.md")]
mod collection;
mod error;
mod server;
mod source;
pub use collection::AssetCollection;
pub use error::AssetLoadError;
#[cfg(feature = "derive")]
pub use konnektoren_asset_loader_derive::AssetCollection;
#[cfg(feature = "csr")]
pub use konnektoren_core::assets::UrlSource;
pub use konnektoren_core::assets::{
    AssetError, AssetFormat, AssetSource, DefaultAssetSource, EmbeddedSource, FileSource,
};
pub use server::AssetServer;
#[cfg(feature = "zip")]
pub use source::ZipSource;
pub use source::{LayeredSource, SourceLayer};
