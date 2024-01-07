use anyhow::{anyhow, Context, Result};
use ratatui::{backend::CrosstermBackend, widgets::{Block,Paragraph, Borders}, Terminal, Frame};
use std::io::Stdout;
use std::rc::Rc;
use Constraint::Percentage;
use ratatui::layout::{Alignment, Direction, Rect};
use ratatui::prelude::{Constraint, Layout, Style, Color};
use ratatui::widgets::block::{Position, Title};
use ratatui::{prelude::*, widgets::*};
use grammers_client::Client;
use grammers_client::types::Chat;
use qrcode::EcLevel::L;
use ratatui::widgets::{List, ListItem};
// use ratatui::style::{self, Color};
use tui_textarea::TextArea;
use crate::app::{App, ApplicationStage, AuthorizationPhase};

pub fn ui(terminal: &mut Terminal<CrosstermBackend<Stdout>>, app: &App) -> Result<()> {
    terminal.draw(|frame| {
        let main_layout = draw_main_layout(frame);
        match app.get_application_stagee() {
            ApplicationStage::Authorization(phase) => draw_authorization_layout(frame, &main_layout, phase),
            ApplicationStage::Authorized => {
                draw_authorized_layout(frame, &main_layout, app.chats())
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

fn draw_authorized_layout(frame: &mut Frame<CrosstermBackend<Stdout>>, main_layout: &Rc<[Rect]>, chats: &Vec<Chat>) {
    let layouts = Layout::default()
        .margin(1)
        .direction(Direction::Horizontal)
        .constraints(vec![Percentage(30), Percentage(70)])
        .split(main_layout[0]);
    let messages_block = Block::default()
        .style(Style::default().bg(Color::Black))
        .borders(Borders::ALL)
        .title(Title::from("Messages")
            .alignment(Alignment::Center)
        );
    draw_chats(frame, layouts[0], chats);
    frame.render_widget(messages_block, layouts[1]);
}

fn draw_chats(frame: &mut Frame<CrosstermBackend<Stdout>>, layout: Rect, chats: &Vec<Chat>) {
    let chats_block = Block::default()
        .style(Style::default().bg(Color::Black))
        .borders(Borders::ALL)
        .title(Title::from("Chats")
            .alignment(Alignment::Center)
        );
    frame.render_widget(chats_block, layout);
    let highlight_symbol = ">>";
    let list_items: Vec<ListItem> = chats.iter()
        .map(|chat| {
            //🤖 <- bot emoji
            let name = match chat {
                Chat::User(user) => "👤 ".to_owned() + user.first_name(),
                Chat::Group(group) => "👥 ".to_owned() + group.title(),
                Chat::Channel(channel) =>"📢 ".to_owned() + channel.title()
            };
            ListItem::new(Line::from(name))
        }
        ).collect();
    let chats_list = List::new(list_items)
        .highlight_symbol(highlight_symbol);
    let chats_list_layout = Layout::default()
        .margin(1)
        // .constraints([Constraint::Length(1), Constraint::Min(0)])
        .constraints(vec![Percentage(100)])
        .direction(Direction::Vertical)
        .split(layout);
    frame.render_widget(chats_list, chats_list_layout[0]);
}