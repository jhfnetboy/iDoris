//! Language Model Implementation
//!
//! This module provides functionality for interacting with the large language model (LLM).
//! It manages a singleton instance of the Llama chat model and provides methods for
//! generating responses, streaming text output, and resetting conversation state.
//!
//! Phase 2.1+: True runtime model switching with hybrid architecture.
//! - Uses OnceCell<Mutex<Chat<Llama>>> for stream compatibility
//! - Uses Lazy<Mutex<Option<Llama>>> for model storage
//! - Supports runtime model switching by reinitializing both

use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use kalosm::language::{Chat, ChatModelExt, IntoChatMessage, Llama};
use once_cell::sync::{Lazy, OnceCell};
use futures::channel::mpsc;

/// Global storage for the Llama model
static LLAMA_MODEL: Lazy<Mutex<Option<Llama>>> = Lazy::new(|| Mutex::new(None));

/// Global storage for the chat session - uses OnceCell for stream compatibility
pub static CHAT_SESSION: OnceCell<Mutex<Chat<Llama>>> = OnceCell::new();

/// Current model ID
static CURRENT_MODEL_ID: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(DEFAULT_MODEL_ID.to_string()));

/// Flag to indicate if a model switch is in progress
static MODEL_SWITCHING: AtomicBool = AtomicBool::new(false);

/// Default model ID
const DEFAULT_MODEL_ID: &str = "qwen-2.5-1.5b";

/// Initializes the language model and creates a chat session
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
    // Check if already initialized with the same model
    if CHAT_SESSION.get().is_some() {
        let current = CURRENT_MODEL_ID.lock().unwrap();
        if *current == model_id {
            println!("Model {} is already initialized", model_id);
            return Ok(());
        }
    }

    // Load the model
    load_model(model_id).await
}

/// Internal function to load a model
async fn load_model(model_id: &str) -> Result<(), String> {
    use kalosm::language::LlamaSource;

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

    // Create chat session
    let chat = llama.chat();

    // Store model
    {
        let mut model_guard = LLAMA_MODEL.lock().unwrap();
        *model_guard = Some(llama);
    }

    // Store model ID
    {
        let mut id_guard = CURRENT_MODEL_ID.lock().unwrap();
        *id_guard = model_id.to_string();
    }

    // Initialize chat session (only if not already set)
    // Note: OnceCell can only be set once, so we need to handle this carefully
    let _ = CHAT_SESSION.set(Mutex::new(chat));

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
pub async fn get_current_model_id() -> String {
    get_current_model_id_sync()
}

/// Synchronous version for compatibility with existing code
pub fn get_current_model_id_sync() -> String {
    CURRENT_MODEL_ID
        .lock()
        .map(|g| g.clone())
        .unwrap_or_else(|_| DEFAULT_MODEL_ID.to_string())
}

/// Switch to a different model at runtime
///
/// Note: Due to OnceCell limitations, this requires a server restart.
/// This function will update the model ID but actual switching happens on restart.
pub async fn switch_model(model_id: &str) -> Result<(), String> {
    // Check if switching is already in progress
    if MODEL_SWITCHING.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_err() {
        return Err("Model switching is already in progress".to_string());
    }

    // Use a guard to ensure we reset the flag even on error
    let _guard = scopeguard::guard((), |_| {
        MODEL_SWITCHING.store(false, Ordering::SeqCst);
    });

    // Check if already using the requested model
    let current_id = get_current_model_id_sync();
    if current_id == model_id {
        println!("Model {} is already loaded", model_id);
        return Ok(());
    }

    // Verify model ID is valid
    get_model_source(model_id)?;

    println!("Switching to model {}...", model_id);

    // If chat session is not yet initialized, we can do a full switch
    if CHAT_SESSION.get().is_none() {
        return load_model(model_id).await;
    }

    // For now, we need to reload the model manually
    // This will create a new chat session but cannot replace the OnceCell
    println!("Loading new model {}...", model_id);

    let source = get_model_source(model_id)?;
    let llama = Llama::builder()
        .with_source(source)
        .build()
        .await
        .map_err(|e| e.to_string())?;

    // Create new chat session
    let new_chat = llama.chat();

    // Store new model
    {
        let mut model_guard = LLAMA_MODEL.lock().unwrap();
        *model_guard = Some(llama);
    }

    // Update model ID
    {
        let mut id_guard = CURRENT_MODEL_ID.lock().unwrap();
        *id_guard = model_id.to_string();
    }

    // Replace the chat session content (Mutex allows this)
    if let Some(chat_mutex) = CHAT_SESSION.get() {
        let mut chat_guard = chat_mutex.lock().unwrap();
        *chat_guard = new_chat;
    }

    println!("Successfully switched to model {}", model_id);
    Ok(())
}

/// Check if model switching is supported
pub fn is_model_switching_supported() -> bool {
    true
}

/// Check if model switching is in progress
pub fn is_model_switching() -> bool {
    MODEL_SWITCHING.load(Ordering::SeqCst)
}

/// Creates a stream for generating text responses from the language model
///
/// This version uses a channel-based approach to avoid lifetime issues with MutexGuard.
///
/// # Parameters
/// * `prompt` - The user's input message
///
/// # Returns
/// * `Result<impl Stream<Item=String>, &'static str>` - A text generation stream or an error
pub fn try_get_stream(prompt: &str) -> Result<mpsc::UnboundedReceiver<String>, &'static str> {
    use kalosm::language::GenerationParameters;
    use futures::StreamExt;

    // Check if switching is in progress
    if MODEL_SWITCHING.load(Ordering::SeqCst) {
        return Err("Model switching in progress, please wait");
    }

    let chat_mutex = CHAT_SESSION.get().ok_or("Chat session not initialized")?;

    // Create channel for streaming tokens
    let (tx, rx) = mpsc::unbounded();

    // Clone prompt to move into async block
    let prompt_owned = prompt.to_string();

    // Spawn task to handle streaming within the mutex lock
    std::thread::spawn(move || {
        // Lock the chat session within the thread
        let mut chat = match chat_mutex.lock() {
            Ok(guard) => guard,
            Err(_) => {
                let _ = tx.unbounded_send("Error: Failed to lock chat session".to_string());
                return;
            }
        };

        // Create the stream while holding the lock
        let mut stream = chat.add_message(prompt_owned.into_chat_message())
            .with_sampler(GenerationParameters::default()
                .with_temperature(0.7)
                .with_top_p(0.9)
                .with_max_length(600)
            );

        // Use a runtime to poll the stream
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        rt.block_on(async {
            while let Some(token) = stream.next().await {
                if tx.unbounded_send(token).is_err() {
                    break;
                }
            }
        });
    });

    Ok(rx)
}

/// Resets the chat session to start a new conversation
///
/// # Returns
/// * `Result<(), String>` - Success or an error message
pub async fn reset_chat() -> Result<(), String> {
    // Get the model
    let model_guard = LLAMA_MODEL.lock().map_err(|_| "Failed to lock model")?;
    let llama = model_guard.as_ref().ok_or("Model not initialized")?;

    // Create new chat session
    let new_chat = llama.chat();

    // Replace the chat session
    if let Some(chat_mutex) = CHAT_SESSION.get() {
        let mut chat_guard = chat_mutex.lock().map_err(|_| "Failed to lock chat session")?;
        *chat_guard = new_chat;
    }

    Ok(())
}

/// Check if the model is initialized
pub fn is_initialized() -> bool {
    CHAT_SESSION.get().is_some()
}

/// Check if the model is initialized (async version)
pub async fn is_initialized_async() -> bool {
    is_initialized()
}
