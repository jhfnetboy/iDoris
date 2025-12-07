//! Chat Server Functions

use dioxus::prelude::*;

#[cfg(feature = "server")]
use dioxus::fullstack::TextStream;

/// Initializes the LLM model
#[server]
pub async fn init_llm_model() -> Result<(), ServerFnError> {
    #[cfg(feature = "server")]
    {
        use crate::core::llm;
        llm::init_model().await.map_err(|e| {
            ServerFnError::new(&format!("Error initializing LLM: {}", e))
        })
    }
    #[cfg(not(feature = "server"))]
    {
        Ok(())
    }
}

/// Initializes the embedding model
#[server]
pub async fn init_embedding_model() -> Result<(), ServerFnError> {
    #[cfg(feature = "server")]
    {
        use crate::core::embedding;
        embedding::init_model().await.map_err(|e| {
            ServerFnError::new(&format!("Error initializing embedding: {}", e))
        })
    }
    #[cfg(not(feature = "server"))]
    {
        Ok(())
    }
}

/// Initializes the database
#[server]
pub async fn init_database() -> Result<(), ServerFnError> {
    #[cfg(feature = "server")]
    {
        use crate::storage::database;
        database::init().await.map_err(|e| {
            ServerFnError::new(&format!("Error initializing database: {}", e))
        })
    }
    #[cfg(not(feature = "server"))]
    {
        Ok(())
    }
}

/// Generates a streaming response for a chat prompt
#[cfg(feature = "server")]
#[get("/api/chat?session_id&prompt")]
pub async fn get_response(session_id: String, prompt: String) -> Result<TextStream> {
    use crate::core::llm;
    use futures::channel::mpsc;

    let (tx, rx) = mpsc::unbounded();

    if !llm::is_initialized() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Model not initialized"
        ).into());
    }

    println!("[{}] Processing prompt: {}", session_id, prompt);
    let time = std::time::Instant::now();

    let mut stream = llm::generate_stream(&prompt).map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
    })?;

    tokio::spawn(async move {
        use futures::StreamExt;
        let _ = tx.unbounded_send(String::new());
        while let Some(token) = stream.next().await {
            if tx.unbounded_send(token).is_err() {
                break;
            }
        }
    });

    println!("Response time: {:?}", time.elapsed());
    Ok(TextStream::new(rx))
}

/// Resets the chat session
#[server]
pub async fn reset_chat() -> Result<(), ServerFnError> {
    #[cfg(feature = "server")]
    {
        use crate::core::llm;
        llm::reset_session().await.map_err(|e| {
            ServerFnError::new(&format!("Error resetting chat: {}", e))
        })
    }
    #[cfg(not(feature = "server"))]
    {
        Ok(())
    }
}

/// Searches for relevant context using RAG
#[server]
pub async fn search_context(query: String) -> Result<String, ServerFnError> {
    #[cfg(feature = "server")]
    {
        use crate::services::rag_service;
        let documents = rag_service::search(&query).await.map_err(|e| {
            ServerFnError::new(&format!("Error searching context: {}", e))
        })?;

        let context = documents.into_iter()
            .map(|doc| format!("Title: {}\nBody: {}\n", doc.title, doc.body))
            .collect::<Vec<_>>()
            .join("\n");

        Ok(context)
    }
    #[cfg(not(feature = "server"))]
    {
        Ok(String::new())
    }
}
