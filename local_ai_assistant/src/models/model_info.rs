//! Model Information Data Structures
//!
//! This module defines data structures for managing available AI models.

use serde::{Deserialize, Serialize};

/// Status of a model
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelStatus {
    /// Model is available and ready to use
    Available,
    /// Model is currently loading
    Loading,
    /// Model is active and being used
    Active,
    /// Model needs to be downloaded first
    NotDownloaded,
    /// Model encountered an error
    Error,
}

impl Default for ModelStatus {
    fn default() -> Self {
        Self::Available
    }
}

/// Information about an available LLM model
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelInfo {
    /// Unique identifier for the model
    pub id: String,
    /// Display name
    pub name: String,
    /// Model size (e.g., "1.5B", "7B", "14B")
    pub size: String,
    /// Memory requirement description
    pub memory_required: String,
    /// Current status
    pub status: ModelStatus,
    /// Description of the model's capabilities
    pub description: String,
}

impl ModelInfo {
    pub fn new(id: &str, name: &str, size: &str, memory: &str, description: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            size: size.to_string(),
            memory_required: memory.to_string(),
            status: ModelStatus::Available,
            description: description.to_string(),
        }
    }
}

/// Get the list of available models
pub fn get_available_models() -> Vec<ModelInfo> {
    vec![
        ModelInfo::new(
            "qwen-2.5-1.5b",
            "Qwen 2.5 1.5B",
            "1.5B",
            "4-6GB",
            "Fast responses, good for simple tasks"
        ),
        ModelInfo::new(
            "qwen-2.5-7b",
            "Qwen 2.5 7B",
            "7B",
            "10-12GB",
            "Balanced performance and quality"
        ),
        ModelInfo::new(
            "qwen-2.5-3b",
            "Qwen 2.5 3B",
            "3B",
            "6-8GB",
            "Good balance for 16GB systems"
        ),
        ModelInfo::new(
            "llama-3.2-3b",
            "Llama 3.2 3B",
            "3B",
            "6-8GB",
            "Meta's latest small model"
        ),
    ]
}
