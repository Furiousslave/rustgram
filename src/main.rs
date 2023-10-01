use std::io::{Stdout, Write};
use grammers_client::{Client, Config, InitParams, SignInError};
use grammers_session::Session;
use console::{Term, style, Style};
use grammers_mtsender::AuthorizationError;
use std::io::{self, stdout};
use std::process;
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders},
    Terminal,
};
use crossterm::{event::{self, Event, KeyCode}, ExecutableCommand};
use ratatui::style::Stylize;
use ratatui::widgets::Paragraph;
use std::{thread, time::Duration};
use anyhow::Result;

mod telegram_integrations;
mod terminal;
mod ui;


#[tokio::main]
async fn main() {
    if let Err(err) = try_main().await {
        eprintln!("{}", err);
        process::exit(2);
    }
}

async fn try_main() -> Result<()> {
    let client = telegram_integrations::connect_to_telegram().await?;
    terminal::setup_terminal()?;
    let mut terminal = terminal::start_terminal(stdout())?;

    loop {
        let should_quit = run_app(&mut terminal, &client).await?;
        if should_quit {
            terminal.clear()?; //todo should i clear the terminal like this?
            break;
        }
    }
    Ok(())
}

async fn run_app(terminal: &mut Terminal<CrosstermBackend<Stdout>>, client: &Client) -> Result<bool> {
    ui::ui(terminal, client).await?;

    if event::poll(Duration::from_millis(50))? {
    if let Event::Key(key) = event::read()? {
        if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
            return Ok(true);
        }
    }
}
    Ok(false)
}


// terminal.draw(|f| {
//     f.render_widget(Paragraph::new("Hello World!")
//                         .block(Block::default().title("Greeting").borders(Borders::ALL)),
//                     f.size(), );
// })?;
//
// if event::poll(std::time::Duration::from_millis(50))? {
//     if let Event::Key(key) = event::read()? {
//         if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
//             return Ok(true);
//         }
//     }
// }

