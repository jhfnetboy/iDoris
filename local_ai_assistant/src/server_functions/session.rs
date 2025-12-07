//! Session Server Functions

use dioxus::prelude::*;
use crate::models::Session;

/// Creates a new chat session
#[server]
pub async fn create_session(title: Option<String>) -> Result<Session, ServerFnError> {
    let session = Session::new(title.unwrap_or_else(|| "New Chat".to_string()));

    #[cfg(feature = "server")]
    {
        use crate::services::session_service;
        session_service::create(&session).await.map_err(|e| {
            ServerFnError::new(&format!("Error creating session: {}", e))
        })?;
    }

    Ok(session)
}

/// Gets all chat sessions
#[server]
pub async fn get_sessions() -> Result<Vec<Session>, ServerFnError> {
    #[cfg(feature = "server")]
    {
        use crate::services::session_service;
        session_service::list().await.map_err(|e| {
            ServerFnError::new(&format!("Error getting sessions: {}", e))
        })
    }
    #[cfg(not(feature = "server"))]
    {
        Ok(vec![])
    }
}

/// Gets a specific session by ID
#[server]
pub async fn get_session(id: String) -> Result<Option<Session>, ServerFnError> {
    #[cfg(feature = "server")]
    {
        use crate::services::session_service;
        use uuid::Uuid;

        let uuid = Uuid::parse_str(&id).map_err(|e| {
            ServerFnError::new(&format!("Invalid session ID: {}", e))
        })?;

        session_service::get(uuid).await.map_err(|e| {
            ServerFnError::new(&format!("Error getting session: {}", e))
        })
    }
    #[cfg(not(feature = "server"))]
    {
        Ok(None)
    }
}

/// Deletes a chat session
#[server]
pub async fn delete_session(id: String) -> Result<(), ServerFnError> {
    #[cfg(feature = "server")]
    {
        use crate::services::session_service;
        use uuid::Uuid;

        let uuid = Uuid::parse_str(&id).map_err(|e| {
            ServerFnError::new(&format!("Invalid session ID: {}", e))
        })?;

        session_service::delete(uuid).await.map_err(|e| {
            ServerFnError::new(&format!("Error deleting session: {}", e))
        })
    }
    #[cfg(not(feature = "server"))]
    {
        Ok(())
    }
}

/// Updates session title
#[server]
pub async fn update_session_title(id: String, title: String) -> Result<(), ServerFnError> {
    #[cfg(feature = "server")]
    {
        use crate::services::session_service;
        use uuid::Uuid;

        let uuid = Uuid::parse_str(&id).map_err(|e| {
            ServerFnError::new(&format!("Invalid session ID: {}", e))
        })?;

        session_service::update_title(uuid, title).await.map_err(|e| {
            ServerFnError::new(&format!("Error updating session: {}", e))
        })
    }
    #[cfg(not(feature = "server"))]
    {
        Ok(())
    }
}
