//! RAG Service
//!
//! Retrieval Augmented Generation service.

use anyhow::Result;
use std::path::Path;
use crate::models::Document;
use crate::core::{embedding, vector_store};

/// Index documents from a directory
pub async fn index_directory(dir: &Path) -> Result<usize> {
    println!("Indexing documents from: {:?}", dir);

    // TODO: Read markdown files and chunk them
    // TODO: Generate embeddings
    // TODO: Store in vector database

    Ok(0)
}

/// Search for relevant documents
pub async fn search(query: &str) -> Result<Vec<Document>> {
    println!("Searching for: {}", query);

    // Generate query embedding
    let query_embedding = embedding::embed_text(query).await?;

    // Search vector store
    let documents = vector_store::search(&query_embedding, 5).await?;

    Ok(documents)
}

/// Augment a prompt with relevant context
pub async fn augment_prompt(query: &str) -> Result<String> {
    let documents = search(query).await?;

    if documents.is_empty() {
        return Ok(query.to_string());
    }

    let context = documents.iter()
        .map(|doc| format!("### {}\n{}", doc.title, doc.body))
        .collect::<Vec<_>>()
        .join("\n\n");

    let augmented = format!(
        "Use the following context to answer the question:\n\n{}\n\n---\n\nQuestion: {}",
        context,
        query
    );

    Ok(augmented)
}
