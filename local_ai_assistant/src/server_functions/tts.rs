//! Text-to-Speech Server Functions
//!
//! This module contains Dioxus server functions for TTS functionality.
//! Phase 2.3: TTS Support

use dioxus::prelude::*;

/// Generates speech from text using the specified engine.
///
/// # Arguments
///
/// * `text` - The text to convert to speech
/// * `engine` - The TTS engine to use ("system", "vibevoice", "kokoro")
/// * `speed` - Speech speed multiplier (0.5 to 2.0)
///
/// # Returns
///
/// * `Result<String>` - The generated audio as a data URL or error
#[server]
pub async fn generate_tts(
    text: String,
    engine: String,
    speed: f32,
) -> Result<String, ServerFnError> {
    #[cfg(feature = "server")]
    {
        use crate::core::tts::{TtsSettings, TtsEngine, generate_speech};

        let tts_engine = match engine.as_str() {
            "vibevoice" => TtsEngine::VibeVoice,
            "kokoro" => TtsEngine::Kokoro,
            _ => TtsEngine::System,
        };

        let settings = TtsSettings::new(&text)
            .with_engine(tts_engine)
            .with_speed(speed);

        let audio = generate_speech(settings).await.map_err(|e| {
            ServerFnError::new(&format!("Error generating speech: {}", e))
        })?;

        Ok(audio.to_data_url())
    }
    #[cfg(not(feature = "server"))]
    {
        let _ = (text, engine, speed);
        Err(ServerFnError::new("TTS not available on client"))
    }
}

/// Checks if TTS generation is in progress.
///
/// # Returns
///
/// * `Result<bool>` - Whether generation is in progress
#[server]
pub async fn is_tts_generating() -> Result<bool, ServerFnError> {
    #[cfg(feature = "server")]
    {
        Ok(crate::core::tts::is_generating())
    }
    #[cfg(not(feature = "server"))]
    {
        Ok(false)
    }
}

/// TTS generation status response
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TtsGenStatus {
    pub is_generating: bool,
    pub status: String,
    pub progress: u8,
}

/// Gets the current TTS generation status.
///
/// # Returns
///
/// * `Result<TtsGenStatus>` - Current generation status, message, and progress (0-100)
#[server]
pub async fn get_tts_status() -> Result<TtsGenStatus, ServerFnError> {
    #[cfg(feature = "server")]
    {
        use crate::core::tts::{is_generating, get_generation_status};
        let (status, progress) = get_generation_status();
        Ok(TtsGenStatus {
            is_generating: is_generating(),
            status,
            progress,
        })
    }
    #[cfg(not(feature = "server"))]
    {
        Ok(TtsGenStatus {
            is_generating: false,
            status: "Not available".to_string(),
            progress: 0,
        })
    }
}

/// Checks if VibeVoice model is available.
///
/// # Returns
///
/// * `Result<bool>` - Whether VibeVoice is downloaded and ready
#[server]
pub async fn is_vibevoice_available() -> Result<bool, ServerFnError> {
    #[cfg(feature = "server")]
    {
        Ok(crate::core::tts::is_vibevoice_available())
    }
    #[cfg(not(feature = "server"))]
    {
        Ok(false)
    }
}

/// Gets available TTS engines.
///
/// # Returns
///
/// * `Result<Vec<String>>` - List of available engine names
#[server]
pub async fn get_available_tts_engines() -> Result<Vec<String>, ServerFnError> {
    #[cfg(feature = "server")]
    {
        use crate::core::tts::get_available_engines;
        let engines = get_available_engines();
        Ok(engines.iter().map(|e| e.display_name().to_string()).collect())
    }
    #[cfg(not(feature = "server"))]
    {
        Ok(vec!["System TTS".to_string()])
    }
}
