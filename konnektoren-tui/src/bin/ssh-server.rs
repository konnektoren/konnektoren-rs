#[cfg(feature = "ssh")]
use konnektoren_tui::prelude::SshServer;
#[cfg(feature = "ssh")]
use log::info;

#[cfg(feature = "ssh")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    info!("Starting Konnektoren SSH Server");
    info!("Listening on 0.0.0.0:2222");
    info!("Connect with: ssh -p 2222 localhost");

    SshServer::run("0.0.0.0", 2222).await?;

    Ok(())
}

#[cfg(not(feature = "ssh"))]
fn main() {
    eprintln!("This binary requires the 'ssh' feature to be enabled.");
    eprintln!("Build with: cargo build --bin ssh-server --features ssh");
    std::process::exit(1);
}
