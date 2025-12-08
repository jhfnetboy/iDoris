//! Context Management Server Functions
//!
//! Functions for managing RAG context documents.

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Get the context folder path - uses the same path as vector_store
#[cfg(feature = "server")]
fn get_context_dir() -> PathBuf {
    crate::core::vector_store::get_context_folder()
}

/// Context file info
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ContextFile {
    pub name: String,
    pub size: u64,
    pub preview: String,
}

/// List all context files
#[server]
pub async fn list_context_files() -> Result<Vec<ContextFile>, ServerFnError> {
    use std::fs;

    let context_dir = get_context_dir();

    // Create directory if it doesn't exist
    if !context_dir.exists() {
        fs::create_dir_all(&context_dir)
            .map_err(|e| ServerFnError::new(&format!("Failed to create context directory: {}", e)))?;
    }

    let mut files = Vec::new();

    let entries = fs::read_dir(context_dir)
        .map_err(|e| ServerFnError::new(&format!("Failed to read context directory: {}", e)))?;

    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    // Only include text files
                    if ext == "md" || ext == "txt" || ext == "json" {
                        let name = path.file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("unknown")
                            .to_string();

                        let metadata = fs::metadata(&path).ok();
                        let size = metadata.map(|m| m.len()).unwrap_or(0);

                        let content = fs::read_to_string(&path).unwrap_or_default();
                        let preview = content.chars().take(100).collect::<String>();
                        let preview = if content.len() > 100 {
                            format!("{}...", preview)
                        } else {
                            preview
                        };

                        files.push(ContextFile { name, size, preview });
                    }
                }
            }
        }
    }

    files.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(files)
}

/// Add a new context document
#[server]
pub async fn add_context_document(title: String, content: String) -> Result<(), ServerFnError> {
    use std::fs;

    let context_dir = get_context_dir();

    // Create directory if it doesn't exist
    if !context_dir.exists() {
        fs::create_dir_all(&context_dir)
            .map_err(|e| ServerFnError::new(&format!("Failed to create context directory: {}", e)))?;
    }

    // Sanitize filename
    let safe_title: String = title
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
        .collect();

    let filename = if safe_title.ends_with(".md") || safe_title.ends_with(".txt") {
        safe_title
    } else {
        format!("{}.md", safe_title)
    };

    let path = context_dir.join(&filename);

    fs::write(&path, &content)
        .map_err(|e| ServerFnError::new(&format!("Failed to write file: {}", e)))?;

    println!("Added context document: {:?}", path);

    // Note: The vector store would need to be reinitialized to include the new document
    // For now, we just save the file

    Ok(())
}

/// Delete a context document
#[server]
pub async fn delete_context_document(filename: String) -> Result<(), ServerFnError> {
    use std::fs;

    // Security check - prevent directory traversal
    if filename.contains("..") || filename.contains("/") {
        return Err(ServerFnError::new("Invalid filename"));
    }

    let context_dir = get_context_dir();
    let path = context_dir.join(&filename);

    fs::remove_file(&path)
        .map_err(|e| ServerFnError::new(&format!("Failed to delete file: {}", e)))?;

    println!("Deleted context document: {:?}", path);

    Ok(())
}

/// Get content of a context document
#[server]
pub async fn get_context_document(filename: String) -> Result<String, ServerFnError> {
    use std::fs;

    // Security check - prevent directory traversal
    if filename.contains("..") || filename.contains("/") {
        return Err(ServerFnError::new("Invalid filename"));
    }

    let context_dir = get_context_dir();
    let path = context_dir.join(&filename);

    let content = fs::read_to_string(&path)
        .map_err(|e| ServerFnError::new(&format!("Failed to read file: {}", e)))?;

    Ok(content)
}

/// Reload the vector store with updated documents
/// This adds new documents to the existing database instead of rebuilding
#[server]
pub async fn reload_context_database() -> Result<String, ServerFnError> {
    #[cfg(feature = "server")]
    {
        // Add new documents to existing vector store
        match crate::core::vector_store::reload_documents().await {
            Ok(msg) => {
                println!("Vector store documents reloaded: {}", msg);
                Ok(msg)
            }
            Err(e) => {
                println!("Failed to reload documents: {}", e);
                Err(ServerFnError::new(&format!("Failed to reload: {}", e)))
            }
        }
    }

    #[cfg(not(feature = "server"))]
    {
        Ok("Reload not supported in this build".to_string())
    }
}
