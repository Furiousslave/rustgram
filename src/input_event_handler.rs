use std::time::Duration;
use anyhow::{anyhow, Result};
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent};
use crate::app::{App, ApplicationStage, AuthorizationPhase};
use grammers_client::{Client, SignInError};
use tokio::task;
use tui_textarea::TextArea;
use crate::SESSION_FILE;

pub async fn handle_input_event(app: &mut App<'_>, client: &Client) -> Result<bool> {
    if event::poll(Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            return match app.get_application_stage() {
                ApplicationStage::Authorization(authorization_phase) => {
                    match authorization_phase {
                        AuthorizationPhase::EnteringPhoneNumber(ref mut text_area) => {
                            if is_key_suitable_for_phone_number_editing(&key, text_area) {
                                text_area.input(key);
                            } else if key.code == KeyCode::Enter && text_area.lines()[0].len() > 0 {
                                let telegram_api_id = app.telegram_api_id();
                                let telegram_api_hash = app.api_hash();
                                let phone = app.get_entered_phone_number()?;
                                let token = client.request_login_code(phone, telegram_api_id, telegram_api_hash).await?;
                                app.set_login_token(token);
                                app.change_authorization_phase_to_code_entering()
                            }
                            Ok(handle_esc_to_close(&key))
                        }
                        AuthorizationPhase::EnteringCode(ref mut text_area) => {
                            if is_key_suitable_for_code_editing(&key, text_area) {
                                text_area.input(key);
                            } else if key.code == KeyCode::Enter && text_area.lines()[0].len() > 0 {
                                let code = app.get_entered_code()?;
                                let signed_in = client.sign_in(app.get_login_token()?, code).await;
                                let should_return = match signed_in {
                                    Err(SignInError::PasswordRequired(password_token)) => {
                                        app.change_authorization_phase_to_password_entering(password_token);
                                        Ok(true)
                                    }

                                    Err(e) => Err(anyhow!("An error occurred when trying to sign in: {}", e)),
                                    _ => Ok(false)
                                }?;
                                if should_return {
                                    return Ok(false)
                                }

                                match client.session().save_to_file(SESSION_FILE) {
                                    Ok(_) => Ok(()),
                                    Err(e) => Err(anyhow!("An error occurred while saving the session to a file: {}", e))
                                }?;

                                app.change_application_stage_to_authorized();
                            }
                            Ok(handle_esc_to_close(&key))
                        }
                        AuthorizationPhase::EnteringPassword(ref mut text_area) => {
                            if is_key_event_press(&key) && key.code != KeyCode::Enter {
                                text_area.input(key);
                            } else if key.code == KeyCode::Enter && text_area.lines()[0].len() > 0 {
                                let token = app.get_password_token()?;
                                client.check_password(token, app.get_entered_password()?).await?;
                                match client.session().save_to_file(SESSION_FILE) {
                                    Ok(_) => Ok(()),
                                    Err(e) => Err(anyhow!("An error occurred while saving the session to a file: {}", e))
                                }?;
                                app.change_application_stage_to_authorized();
                            }
                            Ok(handle_esc_to_close(&key))
                        }
                    }
                }
                ApplicationStage::Authorized => {
                    // let mut dialogs = app.get_client_handle().iter_dialogs();
                    //
                    // println!("Showing up to {} dialogs:", dialogs.total().await?);
                    // while let Some(dialog) = dialogs.next().await? {
                    //     let chat = dialog.chat();
                    //     println!("- {: >10} {}", chat.id(), chat.name());
                    // }
                    //
                    Ok(handle_esc_to_close(&key))
                }
            };
        }
    }
    Ok(false)
}

fn handle_esc_to_close(key: &KeyEvent) -> bool {
    if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Esc {
        return true;
    }
    false
}

fn is_key_suitable_for_phone_number_editing(key: &KeyEvent, text_area: &TextArea) -> bool {
    is_key_event_press(key) &&
        (
            (key.code.ge(&KeyCode::Char('0')) && key.code.le(&KeyCode::Char('9')) && text_area.lines()[0].len() < 15) ||
                key.code == KeyCode::Backspace || key.code == KeyCode::Delete
        )
}

fn is_key_suitable_for_code_editing(key: &KeyEvent, text_area: &TextArea) -> bool {
    is_key_event_press(key) &&
        (
            (key.code.ge(&KeyCode::Char('0')) && key.code.le(&KeyCode::Char('9')) && text_area.lines()[0].len() < 5) ||
                key.code == KeyCode::Backspace || key.code == KeyCode::Delete
        )
}

// fn is_key_suitable_for_password_editing(key: &KeyEvent, text_area: &TextArea) -> bool {
//     is_key_event_press(key) &&
//         (
//             (key.code.ge(&KeyCode::Char('0')) && key.code.le(&KeyCode::Char('9')) && text_area.lines()[0].len() < 5) ||
//                 key.code == KeyCode::Backspace || key.code == KeyCode::Delete
//         )
// }

fn is_key_event_press(key: &KeyEvent) -> bool {
    key.kind == event::KeyEventKind::Press
}