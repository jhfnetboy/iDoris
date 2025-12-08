//! Language Model Implementation
//!
//! This module provides functionality for interacting with the large language model (LLM).
//! It manages a singleton instance of the Llama chat model and provides methods for
//! generating responses, streaming text output, and resetting conversation state.
//!
//! Phase 2.1: Added multi-model support with dynamic model switching.

use tokio::sync::OnceCell;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use kalosm::language::{Chat, ChatModelExt, IntoChatMessage, Llama};

/// Global singleton for the chat session
/// Uses OnceCell and Mutex for thread-safe access and initialization
pub static CHAT_SESSION: OnceCell<Mutex<Chat<Llama>>> = OnceCell::const_new();

/// Global singleton for the language model
/// Stores the base LLM that can generate new chat sessions when needed
pub static MODEL: OnceCell<Mutex<Llama>> = OnceCell::const_new();

/// Track the currently loaded model ID
static CURRENT_MODEL_ID: OnceCell<Mutex<String>> = OnceCell::const_new();

/// Flag to indicate if a model switch is in progress
static MODEL_SWITCHING: AtomicBool = AtomicBool::new(false);

/// Default model ID
const DEFAULT_MODEL_ID: &str = "qwen-2.5-1.5b";

/// Initializes the language model and creates a chat session
///
/// This function:
/// 1. Checks if the model is already initialized
/// 2. If not, creates a new Llama model instance with the Qwen 2.5 7B model
/// 3. Creates a chat session from the model
/// 4. Stores both in their respective global singletons
///
/// Returns Ok(()) on success or an error message on failure
pub async fn init_chat_model() -> Result<(), String> {
    init_chat_model_with_id(DEFAULT_MODEL_ID).await
}

/// Initializes the language model with a specific model ID
///
/// # Arguments
/// * `model_id` - The ID of the model to load
pub async fn init_chat_model_with_id(model_id: &str) -> Result<(), String> {
    use kalosm::language::LlamaSource;

    if CHAT_SESSION.get().is_none() {
        println!("Initializing chat model: {}...", model_id);

        let source = get_model_source(model_id)?;
        println!("Downloading model {}...", model_id);

        let llama = Llama::builder()
            .with_source(source)
            .build()
            .await
            .map_err(|e| {
                eprintln!("Error building model: {}", e);
                e.to_string()
            })?;

        println!("Model {} loaded successfully!", model_id);
        let chat = llama.chat();

        MODEL.set(Mutex::new(llama))
            .map_err(|_| "Couldn't set model".to_string())?;
        CHAT_SESSION.set(Mutex::new(chat))
            .map_err(|_| "Couldn't set chat session".to_string())?;

        // Initialize and set current model ID
        CURRENT_MODEL_ID.get_or_init(|| async { Mutex::new(model_id.to_string()) }).await;
    }
    Ok(())
}

/// Get the LlamaSource for a given model ID
fn get_model_source(model_id: &str) -> Result<kalosm::language::LlamaSource, String> {
    use kalosm::language::LlamaSource;

    match model_id {
        "qwen-2.5-1.5b" => Ok(LlamaSource::qwen_2_5_1_5b_instruct()),
        "qwen-2.5-7b" => Ok(LlamaSource::qwen_2_5_7b_instruct()),
        "qwen-2.5-3b" => Ok(LlamaSource::qwen_2_5_3b_instruct()),
        "llama-3.2-3b" => Ok(LlamaSource::llama_3_2_3b_chat()),
        _ => Err(format!("Unknown model ID: {}", model_id)),
    }
}

/// Get the currently loaded model ID
pub fn get_current_model_id() -> String {
    CURRENT_MODEL_ID
        .get()
        .and_then(|m| m.lock().ok())
        .map(|id| id.clone())
        .unwrap_or_else(|| DEFAULT_MODEL_ID.to_string())
}

/// Switch to a different model
///
/// Note: Due to the OnceCell pattern, we cannot truly "switch" models at runtime
/// without restarting the application. This function will return an error if
/// a different model is already loaded.
///
/// For full model switching support, the application needs to be restarted.
pub async fn switch_model(model_id: &str) -> Result<(), String> {
    let current = get_current_model_id();

    if current == model_id {
        println!("Model {} is already loaded", model_id);
        return Ok(());
    }

    // Check if model is already initialized
    if CHAT_SESSION.get().is_some() {
        // In the current architecture with OnceCell, we cannot truly switch models
        // This would require architectural changes to use RwLock or similar
        return Err(format!(
            "Cannot switch from {} to {} at runtime. Please restart the application with the new model.",
            current, model_id
        ));
    }

    // If no model is loaded yet, initialize with the requested model
    init_chat_model_with_id(model_id).await
}

/// Check if model switching is supported
/// Currently returns false due to OnceCell limitations
pub fn is_model_switching_supported() -> bool {
    false
}

/// Creates a stream for generating text responses from the language model
///
/// This function:
/// 1. Retrieves the chat session from the global singleton
/// 2. Sends the user's prompt to the model
/// 3. Configures generation parameters (temperature, top_p, etc.)
/// 4. Returns a stream that will yield text chunks as they are generated
///
/// # Parameters
/// * `prompt` - The user's input message
///
/// # Returns
/// * `Result<impl Stream<Item=String>, &'static str>` - A text generation stream or an error
pub fn try_get_stream(prompt: &str) -> Result<impl futures::Stream<Item=String>, &'static str> {
    use kalosm::language::GenerationParameters;

    let chat_session = CHAT_SESSION
        .get()
        .ok_or("Model couldn't be initialized.")?;

    let mut guard = chat_session
        .try_lock()
        .map_err(|_| "Couldn't get model lock")?;

    Ok(guard(&prompt.into_chat_message()).with_sampler(GenerationParameters::default()
        .with_temperature(0.7)     // Controls randomness (higher = more random)
        .with_top_p(0.9)           // Nucleus sampling parameter (higher = more diverse)
        .with_max_length(600)      // Maximum response length in tokens
    ))
}

/// Resets the chat session to start a new conversation
///
/// This function:
/// 1. Retrieves the base language model
/// 2. Creates a fresh chat session
/// 3. Replaces the existing chat session in the global singleton
///
/// This effectively clears the conversation history and starts with a clean state.
///
/// # Returns
/// * `Result<(), String>` - Success or an error message
pub async fn reset_chat() -> Result<(), String> {
    let llama = MODEL
        .get()
        .ok_or("Model not initialized")?
        .lock()
        .map_err(|_| "Error locking model")?;
    let new_chat = llama.chat();
    let session_mutex = CHAT_SESSION
        .get()
        .ok_or("Session not initialized")?;
    *session_mutex
        .lock()
        .map_err(|_| "Error locking session")? = new_chat;
    Ok(())
}

/// Check if the model is initialized
pub fn is_initialized() -> bool {
    CHAT_SESSION.get().is_some()
}
