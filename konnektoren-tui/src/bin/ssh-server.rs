#[cfg(feature = "ssh")]
use konnektoren_tui::prelude::SshServer;

#[cfg(feature = "ssh")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let host = std::env::var("SSH_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("SSH_PORT")
        .unwrap_or_else(|_| "2222".to_string())
        .parse::<u16>()
        .unwrap_or(2222);

    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║        Konnektoren TUI SSH Server                         ║");
    println!("╚═══════════════════════════════════════════════════════════╝");
    println!();
    log::info!("Starting Konnektoren SSH Server");
    log::info!("Server listening on {}:{}", host, port);
    println!();
    println!("Connect with:");
    println!(
        "  ssh -p {} <username>@{}",
        port,
        if host == "0.0.0.0" {
            "localhost"
        } else {
            &host
        }
    );
    println!();
    log::info!("Press Ctrl+C to stop the server");
    println!("═══════════════════════════════════════════════════════════");

    SshServer::run(&host, port).await?;

    Ok(())
}

#[cfg(not(feature = "ssh"))]
fn main() {
    eprintln!("This binary requires the 'ssh' feature to be enabled.");
    eprintln!("Build with: cargo build --bin ssh-server --features ssh");
    std::process::exit(1);
}
