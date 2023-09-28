use std::io::Write;
use grammers_client::{Client, Config, InitParams, SignInError};
use grammers_session::Session;
use console::{Term, style, Style};
use grammers_mtsender::AuthorizationError;
use crate::terminal_colors::GREEN;

mod terminal_colors;

fn main() {
    let term = Term::stdout();
    let a = GREEN.apply_to("Connecting to Telegram...");
    term.write_line(&*a.to_string()).unwrap_or_else(|error| eprintln!("Error: {}", error));




    // if let Err(error)= result {
    //     !eprintln!("Error: {}", error)
    // }
    // println!("This is {} neat", style("quite").red());
}

async fn async_main() -> Result<(), AuthorizationError> {
    let client_config = Config {
        session: Session::new(),
        api_id: 22569658,
        api_hash: "16a2120465917eff8ad394778bb8bfbf".to_string(),
        // api_hash: "16a2120465917eff8ad394778bb8bfbfffff".to_string(), bad value
        params: InitParams::default()
    };

    let client = Client::connect(client_config).await?;

    Ok(())
    // Ok();
}
