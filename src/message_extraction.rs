use fantoccini::{Client, Locator, error::CmdError};
use std::collections::HashSet;
use once_cell::sync::OnceCell;
use std::sync::Mutex;

static SEEN_MESSAGES: OnceCell<Mutex<HashSet<String>>> = OnceCell::new();

pub async fn extract_new_messages(client: &Client) -> Result<Vec<String>, CmdError> {
    // Initialize SEEN_MESSAGES if not already initialized
    SEEN_MESSAGES.get_or_init(|| Mutex::new(HashSet::new()));

    // Locate the chat list
    let chat_list = client.find(Locator::Css("div[data-testid='conversation-panel']")).await?;

    // Locate all visible messages
    let messages = chat_list.find_all(Locator::Css("span.selectable-text")).await?;

    let mut new_messages = Vec::new();

    // Access SEEN_MESSAGES safely
    if let Some(seen) = SEEN_MESSAGES.get() {
        let mut seen = seen.lock().unwrap();
        for message in messages {
            // Extract the text content of the message
            let text = message.text().await?;

            // Check if this message is already seen
            if !seen.contains(&text) {
                // Add new messages to the result list and mark as seen
                new_messages.push(text.clone());
                seen.insert(text);
            }
        }
    }

    Ok(new_messages)
}
