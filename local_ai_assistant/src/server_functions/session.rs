//! Session Server Functions
//!
//! Session management with SQLite persistence.

use dioxus::prelude::*;
use crate::models::{Session, ChatMessage};

/// Creates a new chat session and persists to database
#[server]
pub async fn create_session(title: Option<String>) -> Result<Session, ServerFnError> {
    use crate::storage::database;

    let session = Session::new(title.unwrap_or_else(|| "New Chat".to_string()));

    if let Err(e) = database::create_session(&session).await {
        println!("Error creating session in database: {:?}", e);
        // Still return the session even if persistence fails
    }

    Ok(session)
}

/// Gets all chat sessions from database
#[server]
pub async fn get_sessions() -> Result<Vec<Session>, ServerFnError> {
    use crate::storage::database;

    match database::get_all_sessions().await {
        Ok(sessions) => Ok(sessions),
        Err(e) => {
            println!("Error loading sessions: {:?}", e);
            Ok(vec![])
        }
    }
}

/// Gets a specific session by ID
#[server]
pub async fn get_session(id: String) -> Result<Option<Session>, ServerFnError> {
    use crate::storage::database;
    use uuid::Uuid;

    let uuid = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => return Ok(None),
    };

    match database::get_all_sessions().await {
        Ok(sessions) => Ok(sessions.into_iter().find(|s| s.id == uuid)),
        Err(e) => {
            println!("Error loading session: {:?}", e);
            Ok(None)
        }
    }
}

/// Deletes a chat session
#[server]
pub async fn delete_session(id: String) -> Result<(), ServerFnError> {
    use crate::storage::database;
    use uuid::Uuid;

    let uuid = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => return Err(ServerFnError::new("Invalid session ID")),
    };

    if let Err(e) = database::delete_session(uuid).await {
        println!("Error deleting session: {:?}", e);
    }

    Ok(())
}

/// Updates session title
#[server]
pub async fn update_session_title(id: String, title: String) -> Result<(), ServerFnError> {
    use crate::storage::database;
    use uuid::Uuid;

    let uuid = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => return Err(ServerFnError::new("Invalid session ID")),
    };

    if let Err(e) = database::update_session_title(uuid, &title).await {
        println!("Error updating session title: {:?}", e);
    }

    Ok(())
}

/// Save a message to database
#[server]
pub async fn save_message(message: ChatMessage) -> Result<(), ServerFnError> {
    use crate::storage::database;

    if let Err(e) = database::save_message(&message).await {
        println!("Error saving message: {:?}", e);
    }

    Ok(())
}

/// Get all messages for a session
#[server]
pub async fn get_session_messages(session_id: String) -> Result<Vec<ChatMessage>, ServerFnError> {
    use crate::storage::database;
    use uuid::Uuid;

    let uuid = match Uuid::parse_str(&session_id) {
        Ok(u) => u,
        Err(_) => return Ok(vec![]),
    };

    match database::get_session_messages(uuid).await {
        Ok(messages) => Ok(messages),
        Err(e) => {
            println!("Error loading messages: {:?}", e);
            Ok(vec![])
        }
    }
}
