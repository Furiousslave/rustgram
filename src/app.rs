use std::fs::OpenOptions;
use std::ops::Deref;
use anyhow::{anyhow, Error};
use grammers_client::types::{LoginToken, PasswordToken};
use grammers_tl_types::types::account::Password;
use tui_textarea::TextArea;
use ratatui::layout::Alignment;
// use crate::app::ApplicationStage::{Authorization, Authorized};
use crate::app::AuthorizationPhase::{EnteringCode, EnteringPassword, EnteringPhoneNumber};
use anyhow::{Result, Context};
use grammers_client::Client;
use grammers_mtsender::ReadError;
use tokio::task::JoinHandle;

pub struct App<'a> {
    telegram_api_id: i32,
    phone: String,
    api_hash: String,
    is_authorized: bool,
    // phone_number_text_area: Option<TextArea<'a>>, //todo Должны инициализироваться только когда нужны и после удаляться
    // code_text_area: Option<TextArea<'a>>,
    application_stage: ApplicationStage<'a>,
    // authorization_phase: Option<AuthorizationPhase<'a>>,
    login_token: Option<LoginToken>,
    password_token: Option<PasswordToken>,
    client_handle: Option<Client>,
    network_handle: Option<JoinHandle<Result<(), ReadError>>>
    // is_user_authorized: bool
}


impl<'a> App<'a> {
    pub fn new(telegram_api_id: i32, api_hash: String, is_user_authorized: bool) -> Self {
        let application_stage = match is_user_authorized {
            true => ApplicationStage::Authorized,
            false => {
                let mut phone_number_text_area = TextArea::default();
                phone_number_text_area.set_placeholder_text("Enter your phone number (international format)");
                phone_number_text_area.set_alignment(Alignment::Center);
                ApplicationStage::Authorization(EnteringPhoneNumber(phone_number_text_area))
            }
        };

        //todo реализовать инициализацию нужных полей в зависимости от фазы авторизации


        //
        // let mut code_text_area = TextArea::default();
        // code_text_area.set_placeholder_text("Enter the code you received");
        // code_text_area.set_alignment(Alignment::Center);

        Self {
            telegram_api_id,
            phone: String::default(),
            api_hash,
            is_authorized: false,
            // phone_number_text_area,
            // code_text_area: None,
            application_stage,
            // authorization_phase,
            login_token: None,
            password_token: None,
            client_handle: None,
            network_handle: None
            // is_user_authorized,
        }
    }


    // pub fn get_phone_number_text_area<'b>(&'b mut self) -> &mut Option<TextArea<'a>> {
    //     &mut self.phone_number_text_area
    // }
    //
    // pub fn get_code_text_area<'b>(&'b mut self) -> &mut Option<TextArea<'a>> {
    //     &mut self.code_text_area
    // }


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
    //
    // pub fn set_client_handle(&mut self, client_handle: Client) {
    //     self.client_handle = Some(client_handle);
    // }
    //
    // pub fn get_client_handle(&self) -> Result<&Client> {
    //     match &self.client_handle {
    //         None => Err(anyhow!("Client handle isn't obtained")),
    //         Some(client) => Ok(client)
    //     }
    // }
    //
    // pub fn set_network_handle(&mut self, network_handle: JoinHandle<Result<(), ReadError>>) {
    //     self.network_handle = Some(network_handle);
    // }
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


