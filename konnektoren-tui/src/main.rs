#[cfg(feature = "crossterm")]
use konnektoren_tui::prelude::{App, init, restore};

#[cfg(feature = "crossterm")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();
    let mut terminal = init()?;
    app.run(&mut terminal)?;
    restore()?;
    Ok(())
}

#[cfg(not(feature = "crossterm"))]
fn main() {
    println!("This example requires the 'crossterm' feature to be enabled.");
}
