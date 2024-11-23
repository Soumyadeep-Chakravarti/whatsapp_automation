// src/voice_playback.rs

use rodio::{OutputStream, Decoder, Sink};
use std::fs::File;
use std::io::BufReader;

pub fn play_voice_message(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize audio output stream
    let (_stream, stream_handle) = OutputStream::try_default()
        .map_err(|e| format!("Audio output device not available: {}", e))?;
    let sink = Sink::try_new(&stream_handle)?;

    // Open the audio file
    let file = File::open(file_path)
        .map_err(|e| format!("Failed to open file '{}': {}", file_path, e))?;
    let source = Decoder::new(BufReader::new(file))
        .map_err(|e| format!("Failed to decode audio file '{}': {}", file_path, e))?;

    // Play audio
    sink.append(source);
    sink.sleep_until_end(); // Ensure playback completes before exiting
    Ok(())
}
