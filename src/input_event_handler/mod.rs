use std::time::Duration;
use anyhow::Result;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent};
use crate::app::{App, ApplicationStage, AuthorizationPhase};
use grammers_client::Client;
use tui_textarea::TextArea;
use crate::telegram_integrations::request_login_token;


// pub async fn handle_input_event(app: &mut App<'_>, client: &Client) -> Result<bool> {
//     if event::poll(Duration::from_millis(50))? {
//         if let Event::Key(key) = event::read()? {
//             return match app.get_application_stage() {
//                 ApplicationStage::Authorization(authorization_phase) => {
//                     match authorization_phase {
//                         // AuthorizationPhase::EnteringPhoneNumber(ref mut text_area) => Ok(handle_input_at_phone_entering_authorization_phase(&key, client, text_area,  app).await),
//                         AuthorizationPhase::EnteringPhoneNumber(ref mut text_area) => Ok(handle_input_at_phone_entering_authorization_phase(&key, client, text_area,  app).await),
//                         AuthorizationPhase::EnteringCode(ref mut text_area) => Ok(handle_input_at_code_entering_authorization_phase(&key, client, text_area, app).await)
//                     }
//                 }
//                 ApplicationStage::Authorized => Ok(false)
//             };
//         }
//     }
//     Ok(false)
// }

pub async fn handle_input_event(app: &mut App<'_>, client: &Client) -> Result<bool> {
    if event::poll(Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            return match app.get_application_stage() {
                ApplicationStage::Authorization(authorization_phase) => {
                    match authorization_phase {
                        // AuthorizationPhase::EnteringPhoneNumber(ref mut text_area) => Ok(handle_input_at_phone_entering_authorization_phase(&key, client, text_area,  app).await),
                        AuthorizationPhase::EnteringPhoneNumber(ref mut text_area) => {
                            if is_key_suitable_for_phone_number_editing(&key) {
                                text_area.input(key);
                            } else if key.code == KeyCode::Enter {
                                // let token = request_login_token(client, app).await?;
                                // app.set_login_token(token);
                                app.change_authorization_phase_to_code_entering()
                            }
                            Ok(handle_esc_to_close(&key))
                        },
                        AuthorizationPhase::EnteringCode(ref mut text_area) => {
                            if is_key_suitable_for_phone_number_editing(&key) {
                                text_area.input(key);
                            } else if key.code == KeyCode::Enter {}
                            Ok(handle_esc_to_close(&key))
                        }
                    }
                }
                ApplicationStage::Authorized => Ok(false)
            };
        }
    }
    Ok(false)
}

fn handle_esc_to_close(key: &KeyEvent) -> bool {
    if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Esc {
        return true
    }
    false
}


// fn handle_input_at_phone_entering_authorization_phase(key: &KeyEvent,
//                                                       app: &mut App) -> bool {
//     if is_key_suitable_for_phone_number_editing(key) {
//     } else if key.code == KeyCode::Enter {
//         app.change_authorization_phase_to_code_entering()
//     }
//     handle_esc_to_close(key)
// }

async fn handle_input_at_phone_entering_authorization_phase(key: &KeyEvent, client: &Client,
                                                            phone_entering_text_area: &mut TextArea<'_>, app: &mut App<'_>) -> bool {
    if is_key_suitable_for_phone_number_editing(key) {
        phone_entering_text_area.input(*key);
    } else if key.code == KeyCode::Enter {
        // let token = request_login_token(client, app).await?;
        // app.set_login_token(token);
        app.change_authorization_phase_to_code_entering()
    }
    handle_esc_to_close(key)
}

async fn handle_input_at_code_entering_authorization_phase(key: &KeyEvent, client: &Client,
                                                           code_entering_text_area: &mut TextArea<'_>, app: &mut App<'_>) -> bool {
    if is_key_suitable_for_phone_number_editing(key) {
        code_entering_text_area.input(*key);
    } else if key.code == KeyCode::Enter {}
    handle_esc_to_close(key)
}

// KeyCode::Char('')
// && (key.code.ge(&KeyCode::Char('0')) && key.code.le(&KeyCode::Char('9')))
// async fn handle_input_event_at_authorization(app: &mut App, key: &KeyEvent, client: &Client) -> Result<bool> {
//     if key.kind == event::KeyEventKind::Press {
//
//         match app.get_authorization_phase() {
//             None => {}
//             Some(phase) => {}
//         }
//
//         if is_key_suitable_for_phone_number_editing(key) {
//             app.get_phone_number_text_area().input(*key);
//         } else if key.code == KeyCode::Enter {
//             let token = request_login_token(client, app).await?;
//             app.set_login_token(token)
//         }
//
//
//     }
//     handle_esc_to_close(key)
// }
//
// async fn handle_input_event_at_entering_code(app: &mut App, key: &KeyEvent, client: &Client) -> Result<bool> {
//     if key.kind == event::KeyEventKind::Press {
//         if is_key_suitable_for_phone_number_editing(key) {
//             app.get_phone_number_text_area().input(*key);
//         } else if key.code == KeyCode::Enter {
//             let token = request_login_token(client, app).await?;
//             app.set_login_token(token)
//         }
//     }
//     handle_esc_to_close(key)
// }


fn is_key_suitable_for_phone_number_editing(key: &KeyEvent) -> bool {
    key.kind == event::KeyEventKind::Press &&
        (
            (key.code.ge(&KeyCode::Char('0')) && key.code.le(&KeyCode::Char('9'))) ||
                key.code == KeyCode::Backspace || key.code == KeyCode::Delete
        )
}