use konnektoren_tui::prelude::{init, restore, App};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();
    let mut terminal = init()?;
    app.run(&mut terminal)?;
    restore()?;
    Ok(())
}
