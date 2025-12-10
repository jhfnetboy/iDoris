//! Error Types for iDoris
//!
//! Unified error handling across the application.

use std::fmt;

/// Main error type for iDoris application
#[derive(Debug)]
pub enum IDorisError {
    /// API-related errors (network, authentication, rate limits)
    ApiError(String),
    
    /// Model-related errors (loading, initialization, inference)
    ModelError(String),
    
    /// Configuration errors (missing keys, invalid values)
    ConfigError(String),
    
    /// Database errors
    DatabaseError(String),
    
    /// File I/O errors
    IoError(std::io::Error),
    
    /// JSON serialization/deserialization errors
    JsonError(serde_json::Error),
    
    /// HTTP request errors
    HttpError(reqwest::Error),
    
    /// Generic error with message
    Other(String),
}

impl fmt::Display for IDorisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IDorisError::ApiError(msg) => write!(f, "API Error: {}", msg),
            IDorisError::ModelError(msg) => write!(f, "Model Error: {}", msg),
            IDorisError::ConfigError(msg) => write!(f, "Configuration Error: {}", msg),
            IDorisError::DatabaseError(msg) => write!(f, "Database Error: {}", msg),
            IDorisError::IoError(err) => write!(f, "I/O Error: {}", err),
            IDorisError::JsonError(err) => write!(f, "JSON Error: {}", err),
            IDorisError::HttpError(err) => write!(f, "HTTP Error: {}", err),
            IDorisError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for IDorisError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            IDorisError::IoError(err) => Some(err),
            IDorisError::JsonError(err) => Some(err),
            IDorisError::HttpError(err) => Some(err),
            _ => None,
        }
    }
}

// Automatic conversions from common error types
impl From<std::io::Error> for IDorisError {
    fn from(err: std::io::Error) -> Self {
        IDorisError::IoError(err)
    }
}

impl From<serde_json::Error> for IDorisError {
    fn from(err: serde_json::Error) -> Self {
        IDorisError::JsonError(err)
    }
}

impl From<reqwest::Error> for IDorisError {
    fn from(err: reqwest::Error) -> Self {
        IDorisError::HttpError(err)
    }
}

impl From<anyhow::Error> for IDorisError {
    fn from(err: anyhow::Error) -> Self {
        IDorisError::Other(err.to_string())
    }
}

impl From<String> for IDorisError {
    fn from(msg: String) -> Self {
        IDorisError::Other(msg)
    }
}

impl From<&str> for IDorisError {
    fn from(msg: &str) -> Self {
        IDorisError::Other(msg.to_string())
    }
}

/// Result type alias for iDoris operations
pub type Result<T> = std::result::Result<T, IDorisError>;

/// Extension trait for user-friendly error messages
pub trait UserFriendlyError {
    /// Convert technical error to user-friendly message
    fn user_message(&self) -> String;
}

impl UserFriendlyError for IDorisError {
    fn user_message(&self) -> String {
        match self {
            IDorisError::ApiError(msg) => {
                if msg.contains("401") || msg.contains("authentication") {
                    "Authentication failed. Please check your API keys in Settings.".to_string()
                } else if msg.contains("429") || msg.contains("rate limit") {
                    "API rate limit exceeded. Please try again in a few minutes.".to_string()
                } else if msg.contains("timeout") {
                    "Request timed out. Please check your internet connection and try again.".to_string()
                } else {
                    format!("API request failed: {}", msg)
                }
            }
            IDorisError::ModelError(msg) => {
                if msg.contains("not found") || msg.contains("not cached") {
                    "Model not found. Please download it from Settings > Models.".to_string()
                } else if msg.contains("out of memory") || msg.contains("OOM") {
                    "Not enough memory to load the model. Try a smaller model or close other applications.".to_string()
                } else {
                    format!("Model error: {}", msg)
                }
            }
            IDorisError::ConfigError(msg) => {
                if msg.contains("missing") || msg.contains("not configured") {
                    format!("Configuration missing: {}. Please check Settings.", msg)
                } else {
                    format!("Configuration error: {}", msg)
                }
            }
            IDorisError::DatabaseError(msg) => {
                format!("Database error: {}. Your data is safe, but the operation failed.", msg)
            }
         IDorisError::IoError(err) => {
                format!("File operation failed: {}", err)
            }
            IDorisError::JsonError(_) => {
                "Data format error. The response from the server was invalid.".to_string()
            }
            IDorisError::HttpError(err) => {
                if err.is_timeout() {
                    "Connection timed out. Please check your internet connection.".to_string()
                } else if err.is_connect() {
                    "Could not connect to the service. Please check your internet connection.".to_string()
                } else {
                    format!("Network error: {}", err)
                }
            }
            IDorisError::Other(msg) => msg.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = IDorisError::ApiError("test".to_string());
        assert_eq!(err.to_string(), "API Error: test");
    }

    #[test]
    fn test_user_friendly_auth_error() {
        let err = IDorisError::ApiError("401 authentication failed".to_string());
        assert!(err.user_message().contains("API keys"));
    }

    #[test]
    fn test_user_friendly_model_error() {
        let err = IDorisError::ModelError("model not found".to_string());
        assert!(err.user_message().contains("download it from Settings"));
    }

    #[test]
    fn test_from_string() {
        let err: IDorisError = "test error".into();
        assert_eq!(err.to_string(), "Error: test error");
    }
}
