use grammers_client::types::LoginToken;
use tui_textarea::TextArea;
use ratatui::layout::Alignment;
// use crate::app::ApplicationStage::{Authorization, Authorized};
use crate::app::AuthorizationPhase::{EnteringCode, EnteringPhoneNumber};

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
    // is_user_authorized: bool
}


impl<'a> App<'a> {
    pub fn new(telegram_api_id: i32, api_hash: String, is_user_authorized: bool) -> Self {
        // let authorization_phase = match is_user_authorized {
        //     true => None,
        //     false => {
        //         let mut phone_number_text_area = TextArea::default();
        //         phone_number_text_area.set_placeholder_text("Enter your phone number (international format)");
        //         phone_number_text_area.set_alignment(Alignment::Center);
        //         Some(EnteringPhoneNumber(phone_number_text_area))
        //     }
        // };

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
            login_token: None
            // is_user_authorized
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


    //todo Ранее делал так, разобраться в чём разница
    // pub fn get_application_stage(&mut self) -> &'a mut ApplicationStage {
    //     &mut self.application_stage
    // }

    // pub fn get_authorization_phase(&self) -> &Option<AuthorizationPhase> {
    //     &self.authorization_phase
    // }



    // pub fn set_application_stage(&mut self, application_stage: ApplicationStage) {
    //     self.application_stage = application_stage
    // }

    pub fn set_login_token(&mut self, login_token: LoginToken) {
        self.login_token = Some(login_token)
    }
    pub fn telegram_api_id(&self) -> i32 {
        self.telegram_api_id
    }
    pub fn phone(&self) -> &str {
        &self.phone
    }
    pub fn api_hash(&self) -> &str {
        &self.api_hash
    }

    pub fn change_authorization_phase_to_code_entering(&mut self) {
        let mut code_text_area = TextArea::default();
        code_text_area.set_placeholder_text("Enter the code you received");
        code_text_area.set_alignment(Alignment::Center);
        self.application_stage = ApplicationStage::Authorization(EnteringCode(code_text_area))
    }
    // pub fn set_authorization_phase(&mut self, authorization_phase: Option<AuthorizationPhase>) {
    //     self.authorization_phase = authorization_phase;
    // }

    // pub fn set_is_user_authorized(&mut self, is_user_authorized: bool) {
    //     self.is_user_authorized = is_user_authorized;
    // }
    // pub fn is_user_authorized(&self) -> bool {
    //     self.is_user_authorized
    // }
}

pub enum ApplicationStage<'a>{
    Authorization(AuthorizationPhase<'a>),
    Authorized
}

pub enum AuthorizationPhase<'a> {
    EnteringPhoneNumber(TextArea<'a>),
    EnteringCode(TextArea<'a>)
}


