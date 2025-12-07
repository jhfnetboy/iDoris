//! Document Model for RAG

use serde::{Deserialize, Serialize};

/// Represents a document chunk for RAG
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    pub title: String,
    pub body: String,
    pub source: String,
    pub embedding: Option<Vec<f32>>,
}

impl Document {
    pub fn new(title: String, body: String, source: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            body,
            source,
            embedding: None,
        }
    }

    pub fn with_embedding(mut self, embedding: Vec<f32>) -> Self {
        self.embedding = Some(embedding);
        self
    }
}
