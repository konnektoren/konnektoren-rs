use log::info;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::Rect;
use ratatui::{Terminal, TerminalOptions, Viewport};
use russh::keys::Algorithm;
use russh::keys::ssh_key::PublicKey;
use russh::server::*;
use russh::{Channel, ChannelId, Pty};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::mpsc::{UnboundedSender, unbounded_channel};

use crate::app::App;
use crate::error::{Error, Result};

type SshTerminal = Terminal<CrosstermBackend<TerminalHandle>>;

struct TerminalHandle {
    sender: UnboundedSender<Vec<u8>>,
    sink: Vec<u8>,
}

impl TerminalHandle {
    async fn start(handle: Handle, channel_id: ChannelId) -> Self {
        let (sender, mut receiver) = unbounded_channel::<Vec<u8>>();
        tokio::spawn(async move {
            while let Some(data) = receiver.recv().await {
                let result = handle.data(channel_id, data.into()).await;
                if result.is_err() {
                    eprintln!("Failed to send data: {result:?}");
                }
            }
        });
        Self {
            sender,
            sink: Vec::new(),
        }
    }
}

impl std::io::Write for TerminalHandle {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.sink.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let result = self.sender.send(self.sink.clone());
        if result.is_err() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::BrokenPipe,
                result.unwrap_err(),
            ));
        }

        self.sink.clear();
        Ok(())
    }
}

#[derive(Clone)]
pub struct SshServer {
    clients: Arc<Mutex<HashMap<usize, (SshTerminal, App)>>>,
    id: usize,
}

impl SshServer {
    pub fn new() -> Self {
        Self {
            clients: Arc::new(Mutex::new(HashMap::new())),
            id: 0,
        }
    }

    fn load_or_generate_key() -> russh::keys::PrivateKey {
        // Use /app/data if it exists (Docker), otherwise current directory
        let key_dir = if std::path::Path::new("/app/data").exists() {
            "/app/data"
        } else {
            "."
        };
        let key_path = format!("{}/ssh_host_key", key_dir);

        // Try to load existing key
        if Path::new(&key_path).exists() {
            info!("Loading existing SSH host key from {}", key_path);
            match russh::keys::PrivateKey::read_openssh_file(Path::new(&key_path)) {
                Ok(key) => return key,
                Err(e) => {
                    log::warn!("Failed to load key, generating new one: {}", e);
                }
            }
        }

        // Generate new key
        info!("Generating new SSH host key");
        let key =
            russh::keys::PrivateKey::random(&mut rand_core::OsRng, Algorithm::Ed25519).unwrap();

        // Save the key
        if let Err(e) = key.write_openssh_file(Path::new(&key_path), Default::default()) {
            log::warn!("Failed to save SSH host key: {}", e);
        } else {
            info!("SSH host key saved to {}", key_path);
        }

        key
    }

    pub async fn run(addr: &str, port: u16) -> Result<()> {
        let mut server = Self::new();

        // Start a background task to handle periodic updates if needed
        let clients = server.clients.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

                for (_, (terminal, app)) in clients.lock().await.iter_mut() {
                    // Optionally do periodic updates here
                    let _ = terminal.draw(|f| {
                        f.render_widget(&*app, f.area());
                    });
                }
            }
        });

        let config = Config {
            inactivity_timeout: Some(std::time::Duration::from_secs(3600)),
            auth_rejection_time: std::time::Duration::from_secs(3),
            auth_rejection_time_initial: Some(std::time::Duration::from_secs(0)),
            keys: vec![Self::load_or_generate_key()],
            nodelay: true,
            ..Default::default()
        };

        info!("Starting SSH server on {}:{}", addr, port);
        server
            .run_on_address(Arc::new(config), (addr, port))
            .await
            .map_err(|e| Error::UiError(format!("SSH server error: {}", e)))
    }
}

impl Server for SshServer {
    type Handler = Self;

    fn new_client(&mut self, _: Option<std::net::SocketAddr>) -> Self {
        let s = self.clone();
        self.id += 1;
        info!("New client connection: {}", self.id);
        s
    }
}

impl Handler for SshServer {
    type Error = anyhow::Error;

    async fn channel_open_session(
        &mut self,
        channel: Channel<Msg>,
        session: &mut Session,
    ) -> std::result::Result<bool, Self::Error> {
        let terminal_handle = TerminalHandle::start(session.handle(), channel.id()).await;

        let backend = CrosstermBackend::new(terminal_handle);

        let options = TerminalOptions {
            viewport: Viewport::Fixed(Rect::default()),
        };

        let terminal = Terminal::with_options(backend, options)?;
        let app = App::new();

        let mut clients = self.clients.lock().await;
        clients.insert(self.id, (terminal, app));

        Ok(true)
    }

    async fn auth_password(
        &mut self,
        user: &str,
        _password: &str,
    ) -> std::result::Result<Auth, Self::Error> {
        info!("Password auth for user: {}", user);

        // Set username in the app
        let mut clients = self.clients.lock().await;
        if let Some((_, app)) = clients.get_mut(&self.id) {
            app.set_username(user.to_string());
        }

        Ok(Auth::Accept)
    }

    async fn auth_publickey(
        &mut self,
        user: &str,
        _: &PublicKey,
    ) -> std::result::Result<Auth, Self::Error> {
        info!("Public key auth for user: {}", user);

        // Set username in the app
        let mut clients = self.clients.lock().await;
        if let Some((_, app)) = clients.get_mut(&self.id) {
            app.set_username(user.to_string());
        }

        Ok(Auth::Accept)
    }

    async fn auth_none(&mut self, user: &str) -> std::result::Result<Auth, Self::Error> {
        info!("Auth none for user: {}", user);

        // Set username in the app
        let mut clients = self.clients.lock().await;
        if let Some((_, app)) = clients.get_mut(&self.id) {
            app.set_username(user.to_string());
        }

        Ok(Auth::Accept)
    }

    async fn data(
        &mut self,
        channel: ChannelId,
        data: &[u8],
        session: &mut Session,
    ) -> std::result::Result<(), Self::Error> {
        let mut should_close = false;
        let mut should_redraw = false;

        {
            let mut clients = self.clients.lock().await;
            if let Some((_, app)) = clients.get_mut(&self.id) {
                for byte in data {
                    match byte {
                        b'q' | 27 => {
                            app.exit();
                            should_close = true;
                            break;
                        }
                        b'm' => {
                            app.toggle_map();
                            should_redraw = true;
                        }
                        b'h' => {
                            app.previous_question();
                            should_redraw = true;
                        }
                        b'l' => {
                            app.next_question();
                            should_redraw = true;
                        }
                        b'\t' => {
                            app.next_challenge();
                            should_redraw = true;
                        }
                        b'0'..=b'9' => {
                            let option_id = (byte - b'0') as usize;
                            let _ = app.solve_option(option_id);
                            should_redraw = true;
                        }
                        _ => {}
                    }
                }
            }
        }

        if should_close {
            self.clients.lock().await.remove(&self.id);
            session.close(channel)?;
        } else if should_redraw {
            let mut clients = self.clients.lock().await;
            if let Some((terminal, app)) = clients.get_mut(&self.id) {
                terminal.draw(|f| {
                    f.render_widget(&*app, f.area());
                })?;
            }
        }

        Ok(())
    }

    async fn window_change_request(
        &mut self,
        _: ChannelId,
        col_width: u32,
        row_height: u32,
        _: u32,
        _: u32,
        _: &mut Session,
    ) -> std::result::Result<(), Self::Error> {
        let rect = Rect {
            x: 0,
            y: 0,
            width: col_width as u16,
            height: row_height as u16,
        };

        let mut clients = self.clients.lock().await;
        if let Some((terminal, app)) = clients.get_mut(&self.id) {
            terminal.resize(rect)?;
            terminal.draw(|f| {
                f.render_widget(&*app, f.area());
            })?;
        }

        Ok(())
    }

    async fn pty_request(
        &mut self,
        channel: ChannelId,
        _: &str,
        col_width: u32,
        row_height: u32,
        _: u32,
        _: u32,
        _: &[(Pty, u32)],
        session: &mut Session,
    ) -> std::result::Result<(), Self::Error> {
        let rect = Rect {
            x: 0,
            y: 0,
            width: col_width as u16,
            height: row_height as u16,
        };

        let mut clients = self.clients.lock().await;
        if let Some((terminal, app)) = clients.get_mut(&self.id) {
            terminal.resize(rect)?;
            terminal.draw(|f| {
                f.render_widget(&*app, f.area());
            })?;
        }

        session.channel_success(channel)?;

        Ok(())
    }
}

impl Drop for SshServer {
    fn drop(&mut self) {
        let id = self.id;
        let clients = self.clients.clone();
        tokio::spawn(async move {
            let mut clients = clients.lock().await;
            clients.remove(&id);
        });
    }
}
