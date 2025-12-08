//! Chat Server Functions
//!
//! This module contains Dioxus server functions for chat functionality.
//! It leverages Dioxus server functions to bridge client-server communication.

use dioxus::prelude::*;
use dioxus::fullstack::TextStream;
use crate::models::{ModelInfo, ModelStatus};

/// Initializes the language model for chat functionality.
///
/// This server function loads and prepares the chat model for use.
///
/// # Returns
///
/// * `Result<()>` - Success or error with detailed message
#[server]
pub async fn init_llm_model() -> Result<(), ServerFnError> {
    #[cfg(feature = "server")]
    {
        use crate::core::llm::init_chat_model;
        init_chat_model().await.map_err(|e| {
            ServerFnError::new(&format!("Error initializing model: {}", e))
        })
    }
    #[cfg(not(feature = "server"))]
    {
        Ok(())
    }
}

/// Initializes the embedding model for text vectorization.
///
/// This server function loads and prepares the embedding model for use.
///
/// # Returns
///
/// * `Result<()>` - Success or error with detailed message
#[server]
pub async fn init_embedding_model() -> Result<(), ServerFnError> {
    #[cfg(feature = "server")]
    {
        use crate::core::embedding::init_embedding_model as init_embed;
        init_embed().await.map_err(|e| {
            ServerFnError::new(&format!("Error initializing embedding model: {}", e))
        })
    }
    #[cfg(not(feature = "server"))]
    {
        Ok(())
    }
}

/// Generates embedding vectors for the provided text.
///
/// # Arguments
///
/// * `txt` - The text to embed
///
/// # Returns
///
/// * `Result<Vec<f32>>` - Embedding vector or error message
#[server]
pub async fn get_embedding(txt: String) -> Result<Vec<f32>, ServerFnError> {
    #[cfg(feature = "server")]
    {
        let result = tokio::task::spawn_blocking(move || {
            futures::executor::block_on(crate::core::embedding::embed_text(&txt))
        })
            .await
            .map_err(|e| ServerFnError::new(&e.to_string()))?;

        result.map_err(|e| ServerFnError::new(&format!("Error embedding text: {}", e)))
    }
    #[cfg(not(feature = "server"))]
    {
        Ok(vec![])
    }
}

/// Resets the current chat session.
///
/// Clears conversation history and resets the chat model's state.
///
/// # Returns
///
/// * `Result<()>` - Success or error with detailed message
#[server]
pub async fn reset_chat() -> Result<(), ServerFnError> {
    #[cfg(feature = "server")]
    {
        use crate::core::llm::reset_chat as do_reset;
        do_reset().await.map_err(|e| ServerFnError::new(&format!("Error trying to reset chat: {}", e)))
    }
    #[cfg(not(feature = "server"))]
    {
        Ok(())
    }
}

/// Processes a user prompt and returns a streaming text response.
///
/// This function streams model responses token by token, allowing
/// for real-time display to users.
///
/// # Arguments
///
/// * `prompt` - The user's input text
///
/// # Returns
///
/// * `Result<TextStream>` - Stream of response tokens or error
#[get("/api/get_response?prompt")]
pub async fn get_response(prompt: String) -> Result<TextStream> {
    use crate::core::llm;

    // Check if the model is initialized
    if !llm::is_initialized() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Model not initialized"
        ).into());
    }

    let time = std::time::Instant::now();
    println!("Processing prompt: {}", prompt);

    // Try to get a stream (now returns an UnboundedReceiver which is a Stream)
    let rx = llm::try_get_stream(&prompt).map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::Other, e)
    })?;

    println!("\nTotal response time: {:?}", time.elapsed());
    Ok(TextStream::new(rx))
}

/// Searches the database for relevant context given a query.
///
/// Retrieves documents that match the query from the database.
/// Results are filtered by similarity threshold and include relevance scores.
///
/// # Arguments
///
/// * `q` - The search query
///
/// # Returns
///
/// * `Result<String>` - Formatted context string with relevance scores or error
#[server]
pub async fn search_context(q: String) -> Result<String, ServerFnError> {
    #[cfg(feature = "server")]
    {
        println!("Searching context for query: {}", q);
        let documents = crate::core::vector_store::query(&q).await.map_err(|e| {
            println!("Error querying database: {}", e);
            ServerFnError::new(&format!("Error querying database: {}", e))
        })?;

        if documents.is_empty() {
            println!("No relevant documents found for query");
            return Ok(String::new());
        }

        // Format with reference numbers and relevance scores
        let context = documents.into_iter()
            .enumerate()
            .map(|(i, document)| {
                format!(
                    "[Reference {}] (Relevance: {:.0}%)\nTitle: {}\n{}\n",
                    i + 1,
                    document.score * 100.0,
                    document.title,
                    document.body
                )
            })
            .collect::<Vec<_>>()
            .join("\n---\n");

        println!("Found {} relevant documents for RAG", context.matches("[Reference").count());
        Ok(context)
    }
    #[cfg(not(feature = "server"))]
    {
        Ok(String::new())
    }
}

/// Initializes the vector store database connection.
///
/// Must be called before any vector store operations can be performed.
///
/// # Returns
///
/// * `Result<()>` - Success or error with detailed message
#[server]
pub async fn init_db() -> Result<(), ServerFnError> {
    #[cfg(feature = "server")]
    {
        crate::core::vector_store::connect_to_database()
            .await
            .map_err(|e| {
                eprintln!("Error: {:?}", e);
                ServerFnError::new(e)
            })?;
        Ok(())
    }
    #[cfg(not(feature = "server"))]
    {
        Ok(())
    }
}

/// Initializes the SQLite database for session persistence.
///
/// Must be called before any session/message operations can be performed.
///
/// # Returns
///
/// * `Result<()>` - Success or error with detailed message
#[server]
pub async fn init_sqlite_db() -> Result<(), ServerFnError> {
    #[cfg(feature = "server")]
    {
        crate::storage::database::init()
            .await
            .map_err(|e| {
                eprintln!("Error initializing SQLite: {:?}", e);
                ServerFnError::new(&format!("SQLite init error: {}", e))
            })?;
        Ok(())
    }
    #[cfg(not(feature = "server"))]
    {
        Ok(())
    }
}

// ============================================================================
// Model Management Server Functions (Phase 2.1)
// ============================================================================

/// Returns the list of available LLM models.
///
/// # Returns
///
/// * `Result<Vec<ModelInfo>>` - List of available models or error
#[server]
pub async fn list_available_models() -> Result<Vec<ModelInfo>, ServerFnError> {
    use crate::models::get_available_models;
    Ok(get_available_models())
}

/// Gets the currently active model.
///
/// # Returns
///
/// * `Result<ModelInfo>` - Current model info or error
#[server]
pub async fn get_current_model() -> Result<ModelInfo, ServerFnError> {
    #[cfg(feature = "server")]
    {
        use crate::core::llm::get_current_model_id;
        let current_id = get_current_model_id().await;
        let models = crate::models::get_available_models();

        models.into_iter()
            .find(|m| m.id == current_id)
            .map(|mut m| {
                m.status = ModelStatus::Active;
                m
            })
            .ok_or_else(|| ServerFnError::new("Current model not found in available models"))
    }
    #[cfg(not(feature = "server"))]
    {
        Ok(ModelInfo::new(
            "qwen-2.5-1.5b",
            "Qwen 2.5 1.5B",
            "1.5B",
            "4-6GB",
            "Fast responses, good for simple tasks"
        ))
    }
}

/// Switches to a different LLM model.
///
/// This will unload the current model and load the new one.
/// Note: This operation may take some time as models need to be downloaded/loaded.
///
/// # Arguments
///
/// * `model_id` - The ID of the model to switch to
///
/// # Returns
///
/// * `Result<()>` - Success or error with detailed message
#[server]
pub async fn switch_llm_model(model_id: String) -> Result<(), ServerFnError> {
    #[cfg(feature = "server")]
    {
        use crate::core::llm::switch_model;

        println!("Switching to model: {}", model_id);

        switch_model(&model_id).await.map_err(|e| {
            eprintln!("Error switching model: {}", e);
            ServerFnError::new(&format!("Error switching model: {}", e))
        })
    }
    #[cfg(not(feature = "server"))]
    {
        let _ = model_id;
        Ok(())
    }
}
