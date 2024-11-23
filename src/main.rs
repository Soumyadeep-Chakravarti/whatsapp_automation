// src/main.rs

mod message_extraction;
mod tts;
mod voice_playback;
mod whatsapp_web;

use tokio::time::{sleep, Duration};
use message_extraction::{fetch_messages, MessageType};

#[tokio::main]
async fn main() {
    loop {
        match fetch_messages() {
            Ok(messages) => {
                for message in messages {
                    match message.message_type {
                        MessageType::Text => {
                            println!("{} says: {}", message.sender, message.content);
                            let announcement = format!("{} says: {}", message.sender, message.content);
                            tts::speak_text(&announcement).unwrap();
                        }
                        MessageType::Voice => {
                            println!("{} sent a voice message", message.sender);
                            tts::speak_text(&format!("{} sent a voice message", message.sender)).unwrap();
                            voice_playback::play_voice_message("path_to_voice_message_file").unwrap();  // Specify file path
                        }
                        MessageType::File(file_type) => {
                            println!("{} sent a {}", message.sender, file_type);
                            let announcement = format!("{} sent a {}", message.sender, file_type);
                            tts::speak_text(&announcement).unwrap();
                        }
                    }
                }
            }
            Err(e) => eprintln!("Error fetching messages: {}", e),
        }

        sleep(Duration::from_secs(30)).await;  // Check every 30 seconds
    }
}
