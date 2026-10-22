use futures::{channel::oneshot, executor::block_on, pin_mut, poll};
use konnektoren_asset_loader::{AssetFormat, AssetLoadError, AssetServer, AssetSource};
use konnektoren_core::assets::{AssetError, EmbeddedSource};
use std::{
    cell::{Cell, RefCell},
    collections::VecDeque,
    rc::Rc,
    sync::Arc,
};

// Deliberately not Clone: formats and sources do not need to be cloneable.
struct Text;
impl AssetFormat for Text {
    type Asset = String;
    type Error = std::string::FromUtf8Error;
    fn cache_key(&self) -> Option<String> {
        Some(String::new())
    }
    fn parse(&self, bytes: &[u8]) -> Result<String, Self::Error> {
        String::from_utf8(bytes.to_vec())
    }
}

struct Uppercase;
impl AssetFormat for Uppercase {
    type Asset = String;
    type Error = std::string::FromUtf8Error;
    fn cache_key(&self) -> Option<String> {
        Some(String::new())
    }
    fn parse(&self, bytes: &[u8]) -> Result<String, Self::Error> {
        Text.parse(bytes).map(|text| text.to_uppercase())
    }
}

struct Prefix(&'static str);
impl AssetFormat for Prefix {
    type Asset = String;
    type Error = std::string::FromUtf8Error;
    fn cache_key(&self) -> Option<String> {
        Some(self.0.to_owned())
    }
    fn parse(&self, bytes: &[u8]) -> Result<String, Self::Error> {
        Text.parse(bytes).map(|text| format!("{}{text}", self.0))
    }
}

struct Uncached;
impl AssetFormat for Uncached {
    type Asset = String;
    type Error = std::string::FromUtf8Error;
    fn parse(&self, bytes: &[u8]) -> Result<String, Self::Error> {
        Text.parse(bytes)
    }
}

static ASSETS: EmbeddedSource = EmbeddedSource::new(&[("hello", b"hello"), ("broken", &[0xff])]);

type Reply = Result<Vec<u8>, AssetError>;
#[derive(Default)]
struct ControlledSource {
    calls: Cell<usize>,
    replies: RefCell<VecDeque<oneshot::Receiver<Reply>>>,
}
impl ControlledSource {
    fn queue(&self) -> oneshot::Sender<Reply> {
        let (send, receive) = oneshot::channel();
        self.replies.borrow_mut().push_back(receive);
        send
    }
}
impl AssetSource for ControlledSource {
    async fn load_bytes(&self, path: &str) -> Reply {
        self.calls.set(self.calls.get() + 1);
        let reply = self.replies.borrow_mut().pop_front();
        match reply {
            Some(reply) => reply.await.map_err(|error| AssetError::Load {
                path: path.to_owned(),
                message: error.to_string(),
            })?,
            None => Err(AssetError::NotFound {
                path: path.to_owned(),
            }),
        }
    }
}

#[test]
fn cache_distinguishes_formats_and_settings() {
    block_on(async {
        let server = AssetServer::new(Rc::new(ASSETS));
        assert_eq!(&*server.load(Text, "hello").await.unwrap(), "hello");
        assert_eq!(&*server.load(Uppercase, "hello").await.unwrap(), "HELLO");
        assert_eq!(
            &*server.load(Prefix("a:"), "hello").await.unwrap(),
            "a:hello"
        );
        assert_eq!(
            &*server.load(Prefix("b:"), "hello").await.unwrap(),
            "b:hello"
        );
        let a = server.load(Prefix("a:"), "hello").await.unwrap();
        let b = server.load(Prefix("a:"), "hello").await.unwrap();
        assert!(Arc::ptr_eq(&a, &b));
    });
}

#[test]
fn formats_without_cache_identity_load_afresh() {
    block_on(async {
        let server = AssetServer::new(Rc::new(ASSETS));
        let a = server.load(Uncached, "hello").await.unwrap();
        let b = server.load(Uncached, "hello").await.unwrap();
        assert_eq!(a, b);
        assert!(!Arc::ptr_eq(&a, &b));
    });
}

#[test]
fn concurrent_requests_and_clones_share_one_load() {
    block_on(async {
        let source = Rc::new(ControlledSource::default());
        let reply = source.queue();
        let server = AssetServer::new(source.clone());
        let other = server.clone();
        let first = server.load(Text, "hello");
        let second = other.load(Text, "hello");
        pin_mut!(first, second);
        assert!(poll!(&mut first).is_pending());
        assert!(poll!(&mut second).is_pending());
        assert_eq!(source.calls.get(), 1);
        reply.send(Ok(b"hello".to_vec())).unwrap();
        let (first, second) = futures::join!(first, second);
        assert!(Arc::ptr_eq(&first.unwrap(), &second.unwrap()));
    });
}

#[test]
fn failed_requests_can_be_retried() {
    block_on(async {
        let source = Rc::new(ControlledSource::default());
        let server = AssetServer::new(source.clone());
        assert!(matches!(
            server.load(Text, "hello").await,
            Err(AssetLoadError::NotFound { .. })
        ));
        source.queue().send(Ok(b"hello".to_vec())).unwrap();
        assert_eq!(&*server.load(Text, "hello").await.unwrap(), "hello");
        assert_eq!(source.calls.get(), 2);
    });
}

#[test]
fn parse_failures_are_not_cached() {
    block_on(async {
        let source = Rc::new(ControlledSource::default());
        let server = AssetServer::new(source.clone());
        source.queue().send(Ok(vec![0xff])).unwrap();
        assert!(matches!(
            server.load(Text, "hello").await,
            Err(AssetLoadError::Parse { .. })
        ));
        source.queue().send(Ok(b"fixed".to_vec())).unwrap();
        assert_eq!(&*server.load(Text, "hello").await.unwrap(), "fixed");
    });
}

#[test]
fn invalidation_evicts_all_formats_for_only_the_requested_path() {
    block_on(async {
        let server = AssetServer::new(Rc::new(ASSETS));
        let text = server.load(Text, "hello").await.unwrap();
        let upper = server.load(Uppercase, "hello").await.unwrap();
        server.invalidate("another");
        assert!(Arc::ptr_eq(
            &text,
            &server.load(Text, "hello").await.unwrap()
        ));
        server.invalidate("hello");
        assert!(!Arc::ptr_eq(
            &text,
            &server.load(Text, "hello").await.unwrap()
        ));
        assert!(!Arc::ptr_eq(
            &upper,
            &server.load(Uppercase, "hello").await.unwrap()
        ));
        let fresh = server.load(Text, "hello").await.unwrap();
        server.clear();
        assert!(!Arc::ptr_eq(
            &fresh,
            &server.load(Text, "hello").await.unwrap()
        ));
    });
}

#[test]
fn evicted_requests_cannot_replace_newer_cached_values() {
    block_on(async {
        for (clear_all, old_finishes_first) in
            [(false, false), (false, true), (true, false), (true, true)]
        {
            let source = Rc::new(ControlledSource::default());
            let old_reply = source.queue();
            let new_reply = source.queue();
            let server = AssetServer::new(source.clone());
            let old = server.load(Text, "hello");
            pin_mut!(old);
            assert!(poll!(&mut old).is_pending());
            if clear_all {
                server.clear();
            } else {
                server.invalidate("hello");
            }
            let new = server.load(Text, "hello");
            pin_mut!(new);
            assert!(poll!(&mut new).is_pending());
            assert_eq!(source.calls.get(), 2);
            let fresh = if old_finishes_first {
                old_reply.send(Ok(b"old".to_vec())).unwrap();
                assert_eq!(&*old.await.unwrap(), "old");
                // The old completion must not remove the newer pending entry.
                let waiter = server.load(Text, "hello");
                pin_mut!(waiter);
                assert!(poll!(&mut waiter).is_pending());
                assert_eq!(source.calls.get(), 2);
                new_reply.send(Ok(b"new".to_vec())).unwrap();
                let fresh = new.await.unwrap();
                assert!(Arc::ptr_eq(&fresh, &waiter.await.unwrap()));
                fresh
            } else {
                new_reply.send(Ok(b"new".to_vec())).unwrap();
                let fresh = new.await.unwrap();
                old_reply.send(Ok(b"old".to_vec())).unwrap();
                assert_eq!(&*old.await.unwrap(), "old");
                fresh
            };
            assert!(Arc::ptr_eq(
                &fresh,
                &server.load(Text, "hello").await.unwrap()
            ));
        }
    });
}

#[test]
fn dropped_waiter_does_not_prevent_later_request_from_completing() {
    block_on(async {
        let source = Rc::new(ControlledSource::default());
        let reply = source.queue();
        let server = AssetServer::new(source.clone());
        {
            let abandoned = server.load(Text, "hello");
            pin_mut!(abandoned);
            assert!(poll!(&mut abandoned).is_pending());
        }
        reply.send(Ok(b"hello".to_vec())).unwrap();
        assert_eq!(&*server.load(Text, "hello").await.unwrap(), "hello");
        assert_eq!(source.calls.get(), 1);
    });
}
