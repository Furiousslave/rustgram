use std::io::Write;
use std::io::{self, stdout};
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders},
    Terminal
};
use crossterm::{ExecutableCommand, terminal::{enable_raw_mode, EnterAlternateScreen}};
use anyhow::Result;
pub fn setup_terminal() -> Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    Ok(())
}

pub fn start_terminal<W: Write>(buf: W, ) -> io::Result<Terminal<CrosstermBackend<W>>> {
    let backend = CrosstermBackend::new(buf);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    terminal.clear()?;

    Ok(terminal)
}