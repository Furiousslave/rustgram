use grammers_client::{Client, Config, InitParams};
use grammers_session::Session;
use anyhow::Result;
use grammers_client::types::LoginToken;
use crate::app::App;

pub async fn connect_to_telegram(api_id: i32, api_hash: String) -> Result<Client> {
    let client_config = Config {
        session: Session::new(),
        api_id,
        api_hash,
        params: InitParams::default()
    };

    let client = Client::connect(client_config).await?;
    Ok(client)
}

pub async fn request_login_token(client: &Client, app: &App<'_>) -> Result<LoginToken> {
    let token = client.request_login_code(app.phone(), app.telegram_api_id(), app.api_hash()).await?;
    Ok(token)
}