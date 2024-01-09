use std::io::Stdout;
use std::rc::Rc;

use anyhow::Result;
use Constraint::Percentage;
use grammers_client::types::Chat;
use ratatui::{backend::CrosstermBackend, Frame, Terminal, widgets::{Block, Borders}};
use ratatui::{prelude::*, widgets::*};
use ratatui::layout::{Direction, Rect};
use ratatui::layout::Alignment::Center;
use ratatui::prelude::{Constraint, Layout, Style};
use ratatui::widgets::{List, ListItem};
use ratatui::widgets::block::Title;
use unicode_segmentation::UnicodeSegmentation;

use crate::app::{ActiveMessagingTab, App, ApplicationStage, AuthorizationPhase, AuthorizedState};

const MAIN_FRAME_TITLE: &str = "Rustgram🦀";
const MESSAGES_FRAME_TITLE: &str = "Messages";
const CHATS_FRAME_TITLE: &str = "Chats";
const SELECTED_CHAT_HIGHLIGHT_SYMBOL: &str = ">>";
const USER_CHAT_TYPE_EMOJI: &str = "👤";
const BOT_CHAT_TYPE_EMOJI: &str = "🤖";
const GROUP_CHAT_TYPE_EMOJI: &str = "👥";
const CHANNEL_CHAT_TYPE_EMOJI: &str = "📢";
const SCROLL_THUMB_SYMBOL: &str = "▐";

pub fn ui(terminal: &mut Terminal<CrosstermBackend<Stdout>>, app: &App) -> Result<()> {
    terminal.draw(|frame| {
        let main_layout = draw_main_layout(frame);
        match app.get_application_stagee() {
            ApplicationStage::Authorization(phase) => draw_authorization_layout(frame, &main_layout, phase),
            ApplicationStage::Authorized(state) => {
                draw_authorized_layout(frame, &main_layout, state)
            }
        }
    })?;
    Ok(())
}

fn draw_main_layout(frame: &mut Frame) -> Rc<[Rect]> {
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Percentage(100)
        ])
        .split(frame.size());
    let main_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from(MAIN_FRAME_TITLE)
            .alignment(Center)
        );
    frame.render_widget(main_block, main_layout[0]);
    main_layout
}

fn draw_authorization_layout(frame: &mut Frame, main_layout: &Rc<[Rect]>, authorization_phase: &AuthorizationPhase) {
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

fn draw_authorized_layout(frame: &mut Frame, main_layout: &Rc<[Rect]>, state: &AuthorizedState) {
    let layouts = Layout::default()
        .margin(1)
        .direction(Direction::Horizontal)
        .constraints(vec![Percentage(30), Percentage(70)])
        .split(main_layout[0]);
    let messages_block_border_style = match state.active_messaging_tab() {
        ActiveMessagingTab::Messages => Style::new().white(),
        _ => Style::new().dark_gray()
    };
    let messages_block = Block::default()
        .borders(Borders::ALL)
        .border_style(messages_block_border_style)
        .title(Title::from(MESSAGES_FRAME_TITLE).alignment(Center));
    draw_chats(frame, layouts[0], state);
    frame.render_widget(messages_block, layouts[1]);
}

fn draw_chats(frame: &mut Frame, layout: Rect, state: &AuthorizedState) {
    let chats_block_border_style = match state.active_messaging_tab() {
        ActiveMessagingTab::Chats => Style::new().white(),
        _ => Style::new().dark_gray()
    };
    let chats_block = Block::default()
        .borders(Borders::ALL)
        .border_style(chats_block_border_style)
        .title(Title::from(CHATS_FRAME_TITLE).alignment(Center));
    frame.render_widget(chats_block, layout);
    let list_items: Vec<ListItem> = state.chats().iter()
        .map(|chat| {
            let name = match chat {
                Chat::User(user) => {
                    let name_without_emojis = remove_emoji_graphemes(user.first_name());
                    if user.is_bot() {
                        format!("{BOT_CHAT_TYPE_EMOJI} {name_without_emojis}")
                    } else {
                        format!("{USER_CHAT_TYPE_EMOJI} {name_without_emojis}")
                    }
                },
                Chat::Group(group) => {
                    let name_without_emojis = remove_emoji_graphemes(group.title());
                    format!("{GROUP_CHAT_TYPE_EMOJI} {name_without_emojis}")
                },
                Chat::Channel(channel) => {
                    let name_without_emojis = remove_emoji_graphemes(channel.title());
                    format!("{CHANNEL_CHAT_TYPE_EMOJI} {name_without_emojis}")
                }
            };
            ListItem::new(Line::from(name))
        }
        ).collect();
    let selected_chat_index = state.selected_chat_index();
    let chats_number = list_items.len();
    let mut list_state = ListState::default().with_selected(Some(selected_chat_index));
    let chats_list_layout = Layout::default()
        .margin(1)
        .constraints(vec![Constraint::Min(0)])
        .direction(Direction::Vertical)
        .split(layout)[0];
    StatefulWidget::render(
        List::new(list_items)
            .highlight_symbol(SELECTED_CHAT_HIGHLIGHT_SYMBOL),
        chats_list_layout,
        frame.buffer_mut(),
        &mut list_state,
    );
    let mut scrollbar_state = ScrollbarState::default()
        .content_length(chats_number)
        .position(selected_chat_index);
    Scrollbar::default()
        .begin_symbol(None)
        .end_symbol(None)
        .track_symbol(None)
        .thumb_symbol(SCROLL_THUMB_SYMBOL)
        .render(chats_list_layout, frame.buffer_mut(), &mut scrollbar_state);
}

fn remove_emoji_graphemes(input_str: &str) -> String {
    input_str.graphemes(true)
        .filter(|g| g.len() == 1 || g.len() == 2)
        .collect::<String>()
        .trim()
        .to_string()
}