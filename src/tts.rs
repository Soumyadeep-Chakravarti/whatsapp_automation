use tts_rust::GoogleTTSClient;

pub fn speak_text(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the Google TTS client
    let client = GoogleTTSClient::new("en", "com"); // "en" for English, "com" for TLD

    // Speak the provided text
    client.speak(text)?;

    Ok(())
}
