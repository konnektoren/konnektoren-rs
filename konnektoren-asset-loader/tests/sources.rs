use futures::executor::block_on;
use konnektoren_asset_loader::{
    AssetError, AssetSource, EmbeddedSource, LayeredSource, SourceLayer,
};
#[cfg(feature = "zip")]
use std::cell::Cell;
use std::rc::Rc;

static PRIMARY: EmbeddedSource = EmbeddedSource::new(&[("same", b"primary")]);
static FALLBACK: EmbeddedSource =
    EmbeddedSource::new(&[("same", b"fallback"), ("other", b"other")]);

struct Broken;
impl AssetSource for Broken {
    async fn load_bytes(&self, path: &str) -> Result<Vec<u8>, AssetError> {
        Err(AssetError::Load {
            path: path.to_owned(),
            message: "unavailable".to_owned(),
        })
    }
}

#[test]
fn layered_sources_obey_priority_and_fall_back_only_when_missing() {
    block_on(async {
        let source = LayeredSource::new(vec![Rc::new(PRIMARY), Rc::new(FALLBACK)]);
        assert_eq!(source.load_bytes("same").await.unwrap(), b"primary");
        assert_eq!(source.load_bytes("other").await.unwrap(), b"other");
        assert!(matches!(
            source.load_bytes("missing").await,
            Err(AssetError::NotFound { .. })
        ));
        let source = LayeredSource::new(vec![Rc::new(Broken), Rc::new(FALLBACK)]);
        assert!(matches!(
            source.load_bytes("same").await,
            Err(AssetError::Load { .. })
        ));
    });
}

#[test]
fn source_can_be_selected_at_runtime_without_wrapping_it_in_a_chain() {
    block_on(async {
        let source: Rc<dyn SourceLayer> = Rc::new(PRIMARY);
        assert_eq!(source.load_bytes("same").await.unwrap(), b"primary");
        let server = konnektoren_asset_loader::AssetServer::new(source);
        let clone = server.clone();
        assert!(Rc::ptr_eq(server.source(), clone.source()));
        struct Bytes;
        impl konnektoren_asset_loader::AssetFormat for Bytes {
            type Asset = Vec<u8>;
            type Error = std::convert::Infallible;
            fn parse(&self, bytes: &[u8]) -> Result<Vec<u8>, Self::Error> {
                Ok(bytes.to_vec())
            }
        }
        assert_eq!(&*clone.load(Bytes, "same").await.unwrap(), b"primary");
    });
}

#[cfg(feature = "derive")]
#[test]
fn collections_support_generics_and_root_paths() {
    use konnektoren_asset_loader::AssetCollection;
    #[derive(AssetCollection)]
    #[asset(folder = "")]
    struct Root;
    #[derive(AssetCollection)]
    #[asset(folder = "challenges/")]
    struct Challenges<'a, T: 'a, const N: usize>
    where
        T: AsRef<str>,
    {
        _value: &'a [T; N],
    }
    assert_eq!(Root::asset_path("/level.yml"), "level.yml");
    assert_eq!(
        Challenges::<'_, String, 2>::asset_path("/level.yml"),
        "challenges/level.yml"
    );
}

#[cfg(feature = "zip")]
#[test]
fn zip_source_reads_once_and_reuses_entries_across_clones() {
    use konnektoren_asset_loader::ZipSource;
    use std::io::{Cursor, Write};
    struct Archive {
        bytes: Vec<u8>,
        calls: Cell<usize>,
    }
    impl AssetSource for Archive {
        async fn load_bytes(&self, path: &str) -> Result<Vec<u8>, AssetError> {
            assert_eq!(path, "bundle.zip");
            self.calls.set(self.calls.get() + 1);
            Ok(self.bytes.clone())
        }
    }
    let mut writer = zip::ZipWriter::new(Cursor::new(Vec::new()));
    writer
        .start_file("a.txt", zip::write::SimpleFileOptions::default())
        .unwrap();
    writer.write_all(b"archive").unwrap();
    let archive = Rc::new(Archive {
        bytes: writer.finish().unwrap().into_inner(),
        calls: Cell::new(0),
    });
    block_on(async {
        let source = ZipSource::new(archive.clone(), "bundle.zip");
        let cloned = source.clone();
        assert_eq!(source.load_bytes("a.txt").await.unwrap(), b"archive");
        assert_eq!(cloned.load_bytes("a.txt").await.unwrap(), b"archive");
        assert!(matches!(
            source.load_bytes("missing").await,
            Err(AssetError::NotFound { .. })
        ));
        assert_eq!(archive.calls.get(), 1);
    });
}

#[cfg(feature = "zip")]
#[test]
fn invalid_zip_is_reported_as_a_load_error() {
    block_on(async {
        let source = konnektoren_asset_loader::ZipSource::new(Rc::new(PRIMARY), "same");
        assert!(matches!(
            source.load_bytes("a.txt").await,
            Err(AssetError::Load { .. })
        ));
    });
}
