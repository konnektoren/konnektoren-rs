use crate::AssetLoadError;
use futures::future::{FutureExt, LocalBoxFuture, Shared};
use konnektoren_core::assets::{AssetFormat, AssetSource};
use std::{
    any::{Any, TypeId},
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
    sync::Arc,
};

type ErasedAsset = Arc<dyn Any + Send + Sync>;
type InFlight = Shared<LocalBoxFuture<'static, Result<ErasedAsset, AssetLoadError>>>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct CacheKey {
    path: String,
    asset_type: TypeId,
    format_type: TypeId,
    settings: String,
}
#[derive(Clone)]
struct Pending {
    identity: Rc<()>,
    future: InFlight,
}

#[derive(Default)]
struct State {
    cached: HashMap<CacheKey, ErasedAsset>,
    in_flight: HashMap<CacheKey, Pending>,
}

/// Bevy-`AssetServer`-inspired cache and request coordinator.
///
/// It is local (`Rc`) because core sources may return non-`Send` browser
/// futures. This keeps the same server usable in CSR and SSR code.
pub struct AssetServer<S: AssetSource + ?Sized> {
    source: Rc<S>,
    state: Rc<RefCell<State>>,
}

impl<S: AssetSource + ?Sized> Clone for AssetServer<S> {
    fn clone(&self) -> Self {
        Self {
            source: self.source.clone(),
            state: self.state.clone(),
        }
    }
}

impl<S: AssetSource + ?Sized> AssetServer<S> {
    pub fn new(source: Rc<S>) -> Self {
        Self {
            source,
            state: Rc::default(),
        }
    }
    pub fn source(&self) -> &Rc<S> {
        &self.source
    }
    /// Evicts all entries. Pending callers still receive their results, but
    /// their requests cannot repopulate this cache after eviction.
    pub fn clear(&self) {
        let mut state = self.state.borrow_mut();
        state.cached.clear();
        state.in_flight.clear();
    }
    /// Evicts every format of a path, including pending cache entries.
    pub fn invalidate(&self, path: &str) {
        let mut state = self.state.borrow_mut();
        state.cached.retain(|key, _| key.path != path);
        state.in_flight.retain(|key, _| key.path != path);
    }

    pub async fn load<F>(
        &self,
        format: F,
        path: impl Into<String>,
    ) -> Result<Arc<F::Asset>, AssetLoadError>
    where
        S: 'static,
        F: AssetFormat + 'static,
        F::Asset: Send + Sync + 'static,
    {
        let path = path.into();
        let Some(settings) = format.cache_key() else {
            return load_asset(self.source.clone(), format, path.clone())
                .await
                .and_then(|asset| downcast(asset, &path));
        };
        let key = CacheKey {
            path: path.clone(),
            asset_type: TypeId::of::<F::Asset>(),
            format_type: TypeId::of::<F>(),
            settings,
        };
        if let Some(asset) = self.state.borrow().cached.get(&key).cloned() {
            return downcast(asset, &path);
        }
        let pending = self.state.borrow().in_flight.get(&key).cloned();
        let pending = match pending {
            Some(pending) => pending,
            None => {
                let pending = Pending {
                    identity: Rc::new(()),
                    future: load_asset(self.source.clone(), format, path.clone())
                        .boxed_local()
                        .shared(),
                };
                self.state
                    .borrow_mut()
                    .in_flight
                    .insert(key.clone(), pending.clone());
                pending
            }
        };
        let result = pending.future.await;
        let mut state = self.state.borrow_mut();
        // An evicted request may finish after a newer request for the same key.
        if state
            .in_flight
            .get(&key)
            .is_some_and(|current| Rc::ptr_eq(&current.identity, &pending.identity))
        {
            state.in_flight.remove(&key);
            if let Ok(asset) = &result {
                state.cached.insert(key, asset.clone());
            }
        }
        drop(state);
        result.and_then(|asset| downcast(asset, &path))
    }
}

impl<S: AssetSource + Default> Default for AssetServer<S> {
    fn default() -> Self {
        Self::new(Rc::new(S::default()))
    }
}

async fn load_asset<S, F>(
    source: Rc<S>,
    format: F,
    path: String,
) -> Result<ErasedAsset, AssetLoadError>
where
    S: AssetSource + ?Sized,
    F: AssetFormat,
    F::Asset: Send + Sync + 'static,
{
    let bytes = source
        .load_bytes(&path)
        .await
        .map_err(|error| match error {
            konnektoren_core::assets::AssetError::NotFound { .. } => {
                AssetLoadError::NotFound { path: path.clone() }
            }
            error => AssetLoadError::Load {
                path: path.clone(),
                message: error.to_string(),
            },
        })?;
    let asset = format
        .parse(&bytes)
        .map_err(|error| AssetLoadError::Parse {
            path,
            message: error.to_string(),
        })?;
    Ok(Arc::new(asset))
}

fn downcast<T: Send + Sync + 'static>(
    asset: ErasedAsset,
    path: &str,
) -> Result<Arc<T>, AssetLoadError> {
    asset
        .downcast::<T>()
        .map_err(|_| AssetLoadError::TypeMismatch {
            path: path.to_owned(),
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use konnektoren_core::assets::EmbeddedSource;
    #[derive(Clone)]
    struct Text;
    impl AssetFormat for Text {
        type Asset = String;
        fn cache_key(&self) -> Option<String> {
            Some(String::new())
        }
        type Error = std::string::FromUtf8Error;
        fn parse(&self, bytes: &[u8]) -> Result<String, Self::Error> {
            String::from_utf8(bytes.to_vec())
        }
    }
    static ASSETS: EmbeddedSource = EmbeddedSource::new(&[("hello.txt", b"hello")]);
    #[test]
    fn returns_the_same_cached_instance() {
        futures::executor::block_on(async {
            let server = AssetServer::new(Rc::new(ASSETS));
            let first = server.load(Text, "hello.txt").await.unwrap();
            let second = server.load(Text, "hello.txt").await.unwrap();
            assert!(Arc::ptr_eq(&first, &second));
        });
    }
}
