use anyhow::Result;
use ratatui::{backend::CrosstermBackend, widgets::{Block, Borders}, Terminal, Frame};
use std::io::Stdout;
use std::rc::Rc;
use ratatui::layout::{Alignment, Direction, Rect};
use ratatui::prelude::{Constraint, Layout};
use ratatui::widgets::block::{Position, Title};
use grammers_client::Client;
use tui_textarea::TextArea;

pub async fn ui(terminal: &mut Terminal<CrosstermBackend<Stdout>>, client: &Client) -> Result<()> {
    let is_authorized = client.is_authorized().await?;
    terminal.draw(|frame| {
        let main_layout = draw_main_layout(frame)?;

        if !is_authorized {
            draw_authorization_layout(frame, &main_layout)?;
        } else {}

    })?;

    Ok(())
}

fn draw_main_layout(frame: &mut Frame<CrosstermBackend<Stdout>>) -> Result<Rc<[Rect]>> {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(100),
            Constraint::Percentage(100), //todo Точно нужна?
        ])
        .split(frame.size());
    frame.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .title(Title::from("Rustgram")
                .alignment(Alignment::Center)
            ),
        main_layout[0],
    );
    Ok(main_layout)
}

fn draw_authorization_layout(frame: &mut Frame<CrosstermBackend<Stdout>>, main_layout: &Rc<[Rect]>) -> Result<()> {
    let mut textarea = TextArea::default();
    textarea.set_placeholder_text("Enter your phone number (international format)");
    // let layout =
    //     Layout::default().constraints([Constraint::Length(3), Constraint::Min(1)].as_slice());
    // let mut is_valid = validate(&mut textarea);
    frame.render_widget(textarea.widget(), main_layout[0]);



    Ok(())
}

pub fn draw_authorization_method(frame: &mut Frame<CrosstermBackend<Stdout>>) -> Result<()> {
    Ok(())
}



