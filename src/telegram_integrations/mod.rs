use grammers_client::{Client, Config, InitParams};
use grammers_session::Session;
use anyhow::Result;

pub async fn connect_to_telegram() -> Result<Client> {
    let client_config = Config {
        session: Session::new(),
        api_id: 22569658,
        api_hash: "16a2120465917eff8ad394778bb8bfbf".to_string(),
        params: InitParams::default()
    };

    let client = Client::connect(client_config).await?;
    Ok(client)
}