use fantoccini::{Client, Locator, error::CmdError};
use tokio::time::{sleep, Duration};

/// Opens WhatsApp Web and checks if the user is logged in.
pub async fn open_whatsapp(client: &Client) -> Result<bool, CmdError> {
    // Navigate to WhatsApp Web
    client.goto("https://web.whatsapp.com/").await?;
    println!("Navigated to WhatsApp Web. Please scan the QR code if not logged in.");

    // Wait for login status to update
    sleep(Duration::from_secs(15)).await;

    // Check if the user is logged in by looking for a specific element
    match client.find(Locator::Css("div[data-testid='chat-list']")).await {
        Ok(_) => {
            println!("Login detected: You are logged into WhatsApp Web.");
            Ok(true)
        }
        Err(_) => {
            println!("No login detected: Please scan the QR code.");
            Ok(false)
        }
    }
}
