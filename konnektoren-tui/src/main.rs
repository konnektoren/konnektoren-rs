#[cfg(feature = "crossterm")]
use konnektoren_tui::prelude::{App, init, restore};

#[cfg(feature = "crossterm")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let mut app = App::new();
    let mut terminal = init()?;
    app.run(&mut terminal)?;
    restore()?;
    Ok(())
}

#[cfg(not(feature = "crossterm"))]
fn main() {
    eprintln!("This binary requires the 'crossterm' feature to be enabled.");
    eprintln!("Run with: cargo run --features crossterm");
    std::process::exit(1);
}
