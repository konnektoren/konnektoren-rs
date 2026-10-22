# Konnektoren asset loader

Load typed assets from embedded bytes, files, browser HTTP, ZIP archives or
application-defined sources. The loader does not know about levels, challenges,
FAQ or any UI framework. Applications decide when to request an asset.

## Source, format, server

- `AssetSource` supplies bytes asynchronously. Implement `load_bytes` to add a backend.
- `AssetFormat` parses bytes synchronously into an application-defined type.
- `AssetServer` coordinates loads and optionally caches parsed values.
- `LayeredSource` searches sources in order. Only `AssetError::NotFound` falls
  through; permission, network, server and parse failures remain visible.
- `SourceLayer` is the object-safe bridge for runtime source selection. Any
  `AssetSource` implements it; `Rc<dyn SourceLayer>` can be passed to a server.

```rust
use konnektoren_asset_loader::{AssetFormat, AssetServer, EmbeddedSource};
use std::{rc::Rc, sync::Arc};

struct Text;
impl AssetFormat for Text {
    type Asset = String;
    type Error = std::string::FromUtf8Error;

    fn parse(&self, bytes: &[u8]) -> Result<String, Self::Error> {
        String::from_utf8(bytes.to_vec())
    }

    fn cache_key(&self) -> Option<String> {
        Some(String::new()) // This parser has no settings.
    }
}

static ASSETS: EmbeddedSource = EmbeddedSource::new(&[("intro.txt", b"Hallo")]);

# futures::executor::block_on(async {
let server = AssetServer::new(Rc::new(ASSETS));
let first = server.load(Text, "intro.txt").await?;
let second = server.clone().load(Text, "intro.txt").await?;
assert_eq!(&*first, "Hallo");
assert!(Arc::ptr_eq(&first, &second));
# Ok::<(), konnektoren_asset_loader::AssetLoadError>(())
# }).unwrap();
```

For YAML, JSON or a serialized binary format, implement `AssetFormat::parse`
using the application's serializer. For `rust-embed`, implement `AssetSource`
for your embedded folder type and return its bytes or `AssetError::NotFound`.
Neither extension requires changing this crate. `extensions()` is descriptive;
the server uses the format supplied by the caller, without automatic dispatch.

## Cache contract

Caching is opt-in: `AssetFormat::cache_key()` defaults to `None`, which loads
and parses afresh. Return `Some(key)` to enable both parsed-asset caching and
coalescing of simultaneous requests. Include **all parsing settings** in this
key. The complete identity also contains the path, format type and output type.
Different source configurations should use separate server instances.

Clones of a server share its source, cache and pending requests; the source and
format do not need to implement `Clone`. Successful results are shared as `Arc`;
failures are not cached, so callers can retry. An abandoned request can be resumed
by a later caller requesting the same cache identity. Pending work is polled by
callers, never by a background task.

`invalidate(path)` evicts every format of one path. `clear()` evicts everything.
Existing callers can still receive an evicted request's result, but it cannot
repopulate the cache or replace a newer result. A subsequent load starts fresh.
These methods affect the server cache, not source-owned caches (such as the
ZIP entry cache) or the browser's HTTP cache.

The server uses `Rc` and local futures, supporting browser backends without
requiring `Send` futures. It is not a shared multi-thread server. Parsed assets
currently must be `Send + Sync + 'static`.

## Sources and features

| Source / feature | Behavior |
|---|---|
| `EmbeddedSource` | Static path/byte pairs, including `include_bytes!` data |
| `FileSource` | Searches configured directories on native targets |
| `csr` | Exposes `UrlSource`; forwards core's browser feature |
| `UrlSource` | Relative paths use its base URL; HTTP(S) and protocol-relative URLs are preserved; HTTP 404 means `NotFound` |
| `zip` | Exposes `ZipSource`; reads and caches archive entries on first access |
| `derive` (default) | Enables `#[derive(AssetCollection)]` with `#[asset(folder = "...")]` |

The `csr` feature enables core's browser randomness support as well. Native
file loading does not provide a sandbox: use trusted asset paths.

An embedded-first chain can request remote assets missing from the binary.
A remote-first chain falls back to embedded data on HTTP 404. Offline network
failures are errors; applications wanting an offline fallback policy can add a
custom source that implements that policy explicitly.

## Migration from the initial asset-loader branch

Custom formats must implement `cache_key()` to retain parsed-asset caching;
return a stable empty string for a stateless parser, or encode its settings.
`AssetLoadError::NotFound` now distinguishes missing assets from load failures.
No new runtime or serializer dependency is required.
