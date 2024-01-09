use anyhow::Result;
use grammers_client::Client;
use grammers_client::types::Chat;

pub async fn get_chats(client: &Client) -> Result<Vec<Chat>> {
    let mut chats = Vec::new();
    let mut dialogs = client.iter_dialogs();
    while let Some(dialog) = dialogs.next().await? {
        chats.push(dialog.chat().clone());
    }
    Ok(chats)
}