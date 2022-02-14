use std::io;
use tui::{backend::CrosstermBackend, Terminal};
use crossterm::{
    terminal::{enable_raw_mode, disable_raw_mode}
};

pub fn start() -> Result<(), io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    enable_raw_mode()?;

    Ok(())
}
