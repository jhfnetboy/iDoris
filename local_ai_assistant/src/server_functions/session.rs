//! Session Server Functions
//!
//! Placeholder functions for session management.
//! TODO: Implement SQLite-backed session persistence in Phase 1.

use dioxus::prelude::*;
use crate::models::Session;

/// Creates a new chat session
#[server]
pub async fn create_session(title: Option<String>) -> Result<Session, ServerFnError> {
    let session = Session::new(title.unwrap_or_else(|| "New Chat".to_string()));
    // TODO: Persist to SQLite
    Ok(session)
}

/// Gets all chat sessions
#[server]
pub async fn get_sessions() -> Result<Vec<Session>, ServerFnError> {
    // TODO: Load from SQLite
    Ok(vec![])
}

/// Gets a specific session by ID
#[server]
pub async fn get_session(id: String) -> Result<Option<Session>, ServerFnError> {
    // TODO: Load from SQLite
    let _ = id;
    Ok(None)
}

/// Deletes a chat session
#[server]
pub async fn delete_session(id: String) -> Result<(), ServerFnError> {
    // TODO: Delete from SQLite
    let _ = id;
    Ok(())
}

/// Updates session title
#[server]
pub async fn update_session_title(id: String, title: String) -> Result<(), ServerFnError> {
    // TODO: Update in SQLite
    let _ = (id, title);
    Ok(())
}
