//! Document Model for RAG
//!
//! This module defines data structures for representing document search results.
//! These structures are used to store and transport document data retrieved from
//! the database when providing context for conversations.

use serde::{Deserialize, Serialize};

/// Represents a simplified document search result
///
/// This structure contains the essential information of a document retrieved
/// during context search operations, including:
/// - The document title
/// - The document body text
/// - A relevance score indicating how well the document matches the search query
///
/// The score is used to rank and filter documents based on their relevance to
/// the current conversation context.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Document {
    /// The title of the document
    pub title: String,

    /// The main text content of the document
    pub body: String,

    /// A floating-point score representing the document's relevance
    /// Higher values indicate greater relevance to the search query
    #[serde(default)]
    pub score: f32,
}

impl Document {
    pub fn new(title: String, body: String) -> Self {
        Self {
            title,
            body,
            score: 0.0,
        }
    }

    pub fn with_score(mut self, score: f32) -> Self {
        self.score = score;
        self
    }
}
