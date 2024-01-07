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
use std::sync::mpsc;
use anyhow::{anyhow, Result};
use grammers_client::types::Chat;
use crate::app::{App, ApplicationStage};

mod terminal;
mod ui;
mod app;
mod input_event_handler;

pub const SESSION_FILE: &str = "dialogs.session";

#[tokio::main]
async fn main() {
    if let Err(err) = try_main().await {
        eprintln!("{}", err);
        process::exit(2);
    }
}

async fn try_main() -> Result<()> {
    let api_id = 22569658;
    let api_hash=  "16a2120465917eff8ad394778bb8bfbf".to_string();
    let client = Client::connect(Config {
        session: Session::load_file_or_create(SESSION_FILE)?,
        api_id,
        api_hash: api_hash.clone(),
        params: Default::default(),
    }).await?;
    terminal::setup_terminal()?;
    let mut terminal = terminal::start_terminal(stdout())?;
    let is_authorized = client.is_authorized().await?;
    let chats: Vec<Chat> = if is_authorized {
        let mut chats = Vec::new();
        let mut dialogs = client.iter_dialogs();
        while let Some(dialog) = dialogs.next().await? {
            chats.push(dialog.chat().clone());
        }
        chats
    } else {
        Vec::new()
    };

    let mut app = App::new(api_id, api_hash, is_authorized, chats);
    // let (tx, mut rx) = mpsc::channel::<String>();
    // let mut async_update_started = false;
    loop {
        let app_mut_ref = &mut app;
        let should_quit_result = run_app(&mut terminal, &client,  app_mut_ref).await;
        match should_quit_result {
            Ok(should_quit) => {
                if should_quit {
                    drop(client);
                    drop(app);
                    terminal.clear()?; //todo should i clear the terminal like this?
                    drop(terminal);
                    break;
                }
            }
            Err(err) => {
                //todo мб ещё что-то мутить с терминалом
                terminal.clear()?;
                return Err(anyhow!(err))
            }
        }

        // if let ApplicationStage::Authorized = app.get_application_stagee() {
        //     if !async_update_started {
        //         async_update_started = true;
        //         tokio::spawn(async move {
        //
        //
        //             tx.send()
        //
        //         });
        //     }
        // }
        //
        //
        // if should_quit_result {
        //     terminal.clear()?; //todo should i clear the terminal like this?
        //     break;
        // }
    }

    Ok(())
}

async fn run_app(terminal: &mut Terminal<CrosstermBackend<Stdout>>, client: &Client, app: & mut App<'_>) -> Result<bool> {
    ui::ui(terminal, app)?;
    // if let ApplicationStage::Authorized = app.get_application_stagee() {
    //     //todo handle updates in async way
    //     println!("123")
    // }
    input_event_handler::handle_input_event(app, client).await
}

