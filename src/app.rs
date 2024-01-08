use std::fs::OpenOptions;
use std::ops::Deref;
use anyhow::{anyhow, Error};
use grammers_client::types::{Chat, LoginToken, PasswordToken};
use grammers_tl_types::types::account::Password;
use tui_textarea::TextArea;
// use crate::app::ApplicationStage::{Authorization, Authorized};
use crate::app::AuthorizationPhase::{EnteringCode, EnteringPassword, EnteringPhoneNumber};
use anyhow::{Result, Context};
use grammers_client::Client;
use grammers_mtsender::ReadError;
use ratatui::layout::Alignment;
use ratatui::layout::Alignment::Center;
use tokio::task::JoinHandle;

pub struct App<'a> {
    telegram_api_id: i32,
    phone: String,
    api_hash: String,
    application_stage: ApplicationStage<'a>,
    login_token: Option<LoginToken>,
    password_token: Option<PasswordToken>,
    client_handle: Option<Client>,
    network_handle: Option<JoinHandle<Result<(), ReadError>>>,
    chats: Vec<Chat>
}


impl<'a> App<'a> {
    pub fn new(telegram_api_id: i32, api_hash: String, is_user_authorized: bool, chats: Vec<Chat>) -> Self {
        let application_stage = match is_user_authorized {
            true => ApplicationStage::Authorized,
            false => {
                let mut phone_number_text_area = TextArea::default();
                phone_number_text_area.set_placeholder_text("Enter your phone number (international format)");
                phone_number_text_area.set_alignment(Center);
                ApplicationStage::Authorization(EnteringPhoneNumber(phone_number_text_area))
            }
        };

        Self {
            telegram_api_id,
            phone: String::default(),
            api_hash,
            application_stage,
            login_token: None,
            password_token: None,
            client_handle: None,
            network_handle: None,
            chats
        }
    }

    pub fn get_application_stage(&mut self) -> &mut ApplicationStage<'a> {
        &mut self.application_stage
    }

    pub fn get_application_stagee(&self) -> &ApplicationStage<'a> {
        &self.application_stage
    }

    pub fn get_entered_phone_number(&self) -> Result<&str> {
        match &self.application_stage {
            ApplicationStage::Authorization(phase) => {
                if let EnteringPhoneNumber(_) = phase {
                    Ok(phase.get_content_from_text_area())
                } else {
                    Err(anyhow!("Application must be at entering phone number authorization phase"))
                }
            }
            _ => Err(anyhow!("Application must be at Authorization stage"))
        }
    }

    pub fn get_entered_code(&self) -> Result<&str> {
        match &self.application_stage {
            ApplicationStage::Authorization(phase) => {
                if let EnteringCode(_) = phase {
                    Ok(phase.get_content_from_text_area())
                } else {
                    Err(anyhow!("Application must be at entering code authorization phase"))
                }
            }
            _ => Err(anyhow!("Application must be at Authorization stage"))
        }
    }

    pub fn get_entered_password(&self) -> Result<&str> {
        match &self.application_stage {
            ApplicationStage::Authorization(phase) => {
                if let EnteringPassword(_) = phase {
                    Ok(phase.get_content_from_text_area())
                } else {
                    Err(anyhow!("Application must be at entering password authorization phase"))
                }
            }
            _ => Err(anyhow!("Application must be at Authorization stage"))
        }
    }

    //todo Ранее делал так, разобраться в чём разница
    // pub fn get_application_stage(&mut self) -> &'a mut ApplicationStage {
    //     &mut self.application_stage
    // }


    pub fn set_login_token(&mut self, login_token: LoginToken) {
        self.login_token = Some(login_token)
    }


    pub fn phone(&self) -> &str {
        &self.phone
    }

    pub fn change_authorization_phase_to_code_entering(&mut self) {
        let mut code_text_area = TextArea::default();
        code_text_area.set_placeholder_text("Enter the code you received");
        code_text_area.set_alignment(Alignment::Center);
        self.application_stage = ApplicationStage::Authorization(EnteringCode(code_text_area))
    }

    pub fn change_authorization_phase_to_password_entering(&mut self, password_token: PasswordToken) {
        let hint = password_token.hint().unwrap_or("None");
        let mut password_text_area = TextArea::default();
        password_text_area.set_placeholder_text(format!("Enter the password (hint {}): ", hint));
        password_text_area.set_alignment(Alignment::Center);
        self.application_stage = ApplicationStage::Authorization(EnteringPassword(password_text_area));
        self.password_token = Some(password_token);
    }

    pub fn change_application_stage_to_authorized(&mut self, ) {
        self.application_stage = ApplicationStage::Authorized
    }

    pub fn get_login_token(&self) -> Result<&LoginToken> {
        match &self.login_token {
            None => Err(anyhow!("Login token is missing")),
            Some(token) => Ok(token)
        }
    }
    pub fn get_password_token(&mut self) -> Result<PasswordToken> {
        let option_password_token = std::mem::replace(&mut self.password_token, None);
        match option_password_token {
            None => Err(anyhow!("Password token is missing")),
            Some(token) => Ok(token)
        }
    }
    pub fn telegram_api_id(&self) -> i32 {
        self.telegram_api_id
    }
    pub fn api_hash(&self) -> &str {
        &self.api_hash
    }

    pub fn chats(&self) -> &Vec<Chat> {
        &self.chats
    }
}

pub enum ApplicationStage<'a> {
    Authorization(AuthorizationPhase<'a>),
    Authorized,
}

pub enum AuthorizationPhase<'a> {
    EnteringPhoneNumber(TextArea<'a>),
    EnteringCode(TextArea<'a>),
    EnteringPassword(TextArea<'a>),
}

impl AuthorizationPhase<'_> {
    pub fn get_content_from_text_area(&self) -> &str {
        match &self {
            EnteringPhoneNumber(tex_area) => tex_area.lines()[0].as_str(),
            EnteringCode(text_area) => text_area.lines()[0].as_str(),
            EnteringPassword(text_area) => text_area.lines()[0].as_str()
        }
    }
}


