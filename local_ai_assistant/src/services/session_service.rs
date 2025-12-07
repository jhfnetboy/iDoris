//! Session Service
//!
//! Manages chat sessions and their persistence.

use anyhow::Result;
use uuid::Uuid;
use crate::models::Session;

/// Create a new session
pub async fn create(session: &Session) -> Result<()> {
    // TODO: Persist to SQLite
    println!("Creating session: {} - {}", session.id, session.title);
    Ok(())
}

/// List all sessions
pub async fn list() -> Result<Vec<Session>> {
    // TODO: Query from SQLite
    println!("Listing all sessions");
    Ok(vec![])
}

/// Get a session by ID
pub async fn get(id: Uuid) -> Result<Option<Session>> {
    // TODO: Query from SQLite
    println!("Getting session: {}", id);
    Ok(None)
}

/// Delete a session
pub async fn delete(id: Uuid) -> Result<()> {
    // TODO: Delete from SQLite
    println!("Deleting session: {}", id);
    Ok(())
}

/// Update session title
pub async fn update_title(id: Uuid, title: String) -> Result<()> {
    // TODO: Update in SQLite
    println!("Updating session {} title to: {}", id, title);
    Ok(())
}
