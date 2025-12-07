//! Embedding Engine Module
//!
//! Handles text embedding using fastembed.

use std::sync::OnceLock;
use tokio::sync::Mutex;
use anyhow::Result;

static EMBEDDING_MODEL: OnceLock<Mutex<Option<EmbeddingEngine>>> = OnceLock::new();

pub struct EmbeddingEngine {
    // TODO: Add fastembed model handle
    _initialized: bool,
}

pub struct EmbeddingConfig {
    pub model_name: String,
}

impl Default for EmbeddingConfig {
    fn default() -> Self {
        Self {
            model_name: "BAAI/bge-small-en-v1.5".to_string(),
        }
    }
}

/// Initialize the embedding model
pub async fn init_model() -> Result<()> {
    let config = EmbeddingConfig::default();
    println!("Initializing embedding model: {}", config.model_name);

    // TODO: Initialize fastembed model
    // let model = fastembed::TextEmbedding::new(config.model_name)?;

    let engine = EmbeddingEngine {
        _initialized: true,
    };

    EMBEDDING_MODEL.get_or_init(|| Mutex::new(Some(engine)));
    println!("Embedding model initialized successfully");
    Ok(())
}

/// Check if the embedding model is initialized
pub fn is_initialized() -> bool {
    EMBEDDING_MODEL.get()
        .map(|m| m.try_lock().map(|g| g.is_some()).unwrap_or(false))
        .unwrap_or(false)
}

/// Generate embeddings for text
pub async fn embed_text(text: &str) -> Result<Vec<f32>> {
    println!("Generating embedding for: {}...", &text[..text.len().min(50)]);

    // TODO: Implement actual embedding
    // For now, return a mock embedding (384 dimensions for bge-small)
    let embedding = vec![0.0f32; 384];
    Ok(embedding)
}

/// Generate embeddings for multiple texts
pub async fn embed_batch(texts: &[String]) -> Result<Vec<Vec<f32>>> {
    let mut embeddings = Vec::with_capacity(texts.len());
    for text in texts {
        embeddings.push(embed_text(text).await?);
    }
    Ok(embeddings)
}
