use anyhow::{anyhow, Context, Result};
use ratatui::{backend::CrosstermBackend, widgets::{Block,Paragraph, Borders}, Terminal, Frame};
use std::io::Stdout;
use std::rc::Rc;
use Constraint::Percentage;
use ratatui::layout::{Alignment, Direction, Rect};
use ratatui::prelude::{Constraint, Layout, Style, Color};
use ratatui::widgets::block::{Position, Title};
use grammers_client::Client;
use qrcode::EcLevel::L;
// use ratatui::style::{self, Color};
use tui_textarea::TextArea;
use crate::app::{App, ApplicationStage, AuthorizationPhase};

pub fn ui(terminal: &mut Terminal<CrosstermBackend<Stdout>>, application_stage: &ApplicationStage<'_>) -> Result<()> {
    terminal.draw(|frame| {
        let main_layout = draw_main_layout(frame);
        match application_stage {
            ApplicationStage::Authorization(phase) => draw_authorization_layout(frame, &main_layout, phase),
            ApplicationStage::Authorized => {
                draw_authorized_layout(frame, &main_layout)
            }
        }
    })?;
    Ok(())
}

fn draw_main_layout(frame: &mut Frame<CrosstermBackend<Stdout>>) -> Rc<[Rect]> {
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Percentage(100)
        ])
        .split(frame.size());
    let main_block = Block::default()
        .style(Style::default().bg(Color::Black))
        .borders(Borders::ALL)
        .title(Title::from("Rustgram")
            .alignment(Alignment::Center)
        );
    frame.render_widget(main_block, main_layout[0]);
    main_layout
}

fn draw_authorization_layout(frame: &mut Frame<CrosstermBackend<Stdout>>, main_layout: &Rc<[Rect]>, authorization_phase: &AuthorizationPhase) {
    let vertical_margin = (main_layout[0].height as f64 * 0.25).round() as u16;
    let horizontal_margin = (main_layout[0].width as f64 * 0.25).round() as u16;
    let layout = Layout::default()
        .vertical_margin(vertical_margin)
        .horizontal_margin(horizontal_margin)
        .direction(Direction::Vertical)
        .constraints(vec![Percentage(100)])
        .split(main_layout[0]);
    let widget_to_render = match authorization_phase {
        AuthorizationPhase::EnteringPhoneNumber(text_area) => text_area.widget(),
        AuthorizationPhase::EnteringCode(text_area) => text_area.widget(),
        AuthorizationPhase::EnteringPassword(text_area) => text_area.widget()
    };
    frame.render_widget(widget_to_render, layout[0]);
}

fn draw_authorized_layout(frame: &mut Frame<CrosstermBackend<Stdout>>, main_layout: &Rc<[Rect]>) {
    let layouts = Layout::default()
        .margin(1)
        .direction(Direction::Horizontal)
        .constraints(vec![Percentage(30), Percentage(70)])
        .split(main_layout[0]);
    let chats_block = Block::default()
        .style(Style::default().bg(Color::Black))
        .borders(Borders::ALL)
        .title(Title::from("Chats")
            .alignment(Alignment::Center)
        );
    let messages_block = Block::default()
        .style(Style::default().bg(Color::Black))
        .borders(Borders::ALL)
        .title(Title::from("Messages")
            .alignment(Alignment::Center)
        );
    frame.render_widget(chats_block, layouts[0]);
    frame.render_widget(messages_block, layouts[1]);
}