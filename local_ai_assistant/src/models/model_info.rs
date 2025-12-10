//! Model Information Data Structures
//!
//! This module defines data structures for managing available AI models.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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

/// Model types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ModelType {
    Language,
    ImageGeneration,
    Embedding,
    Audio,
    Multimodal,
}

/// Information about an available LLM model
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ModelInfo {
    /// Unique identifier for the model
    pub id: String,
    /// Display name
    pub name: String,
    /// Model size string (e.g. "1.5B") - Legacy/Display
    pub size: String,
    /// Memory requirement description - Legacy/Display
    pub memory_required: String,
    /// Current status
    pub status: ModelStatus,
    /// Description of the model's capabilities
    pub description: String,
    
    // Fields from core::ModelInfo
    pub model_type: ModelType,
    pub size_mb: Option<u64>,
    pub is_cached: bool,
    pub cache_path: Option<PathBuf>,
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
            // Default core fields
            model_type: ModelType::Language,
            size_mb: None,
            is_cached: false,
            cache_path: None,
        }
    }
}

/// Cache information structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheInfo {
    pub path: PathBuf,
    pub total_size_mb: u64,
    pub model_count: usize,
}

/// Get the list of available models
pub fn get_available_models() -> Vec<ModelInfo> {
    vec![
        ModelInfo {
            id: "Qwen/Qwen2.5-1.5B-Instruct".to_string(),
            name: "Qwen 2.5 1.5B".to_string(),
            size: "1.5B".to_string(),
            memory_required: "4-6GB".to_string(),
            status: ModelStatus::Available,
            description: "Lightweight Chinese/English LLM".to_string(),
            model_type: ModelType::Language,
            size_mb: Some(3040),
            is_cached: false,
            cache_path: None,
        },
        ModelInfo {
            id: "Qwen/Qwen2.5-3B-Instruct".to_string(),
            name: "Qwen 2.5 3B".to_string(),
            size: "3B".to_string(),
            memory_required: "6-8GB".to_string(),
            status: ModelStatus::Available,
            description: "Medium Chinese/English LLM".to_string(),
            model_type: ModelType::Language,
            size_mb: Some(6100),
            is_cached: false,
            cache_path: None,
        },
        ModelInfo {
            id: "Qwen/Qwen2.5-7B-Instruct".to_string(),
            name: "Qwen 2.5 7B".to_string(),
            size: "7B".to_string(),
            memory_required: "10-12GB".to_string(),
            status: ModelStatus::Available,
            description: "Large Chinese/English LLM".to_string(),
            model_type: ModelType::Language,
            size_mb: Some(14700),
            is_cached: false,
            cache_path: None,
        },
        ModelInfo {
            id: "black-forest-labs/FLUX.1-schnell".to_string(),
            name: "FLUX.1 Schnell".to_string(),
            size: "12B".to_string(), // Metadata
            memory_required: "16GB+".to_string(), 
            status: ModelStatus::Available,
            description: "Fast image generation model".to_string(),
            model_type: ModelType::ImageGeneration,
            size_mb: Some(12420),
            is_cached: false,
            cache_path: None,
        },
        ModelInfo {
            id: "BAAI/bge-large-zh-v1.5".to_string(),
            name: "BGE Large Chinese".to_string(),
            size: "300M".to_string(), // Metadata
            memory_required: "2GB".to_string(),
            status: ModelStatus::Available,
            description: "Chinese text embeddings".to_string(),
            model_type: ModelType::Embedding,
            size_mb: Some(1340),
            is_cached: false,
            cache_path: None,
        },
        ModelInfo {
            id: "meta-llama/Llama-3.2-3B-Instruct".to_string(),
            name: "Llama 3.2 3B".to_string(),
            size: "3B".to_string(),
            memory_required: "6-8GB".to_string(),
            status: ModelStatus::Available,
            description: "Meta's latest small model".to_string(),
            model_type: ModelType::Language,
            size_mb: Some(6000), // Estimate
            is_cached: false,
            cache_path: None,
        },
    ]
}
