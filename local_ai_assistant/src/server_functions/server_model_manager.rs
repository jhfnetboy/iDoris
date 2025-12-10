//! Server Functions for Model Management
//!
//! Provides server-side functions for managing HuggingFace models

#[cfg(feature = "server")]
use crate::core::model_manager::ModelManager;
use crate::models::{ModelInfo, CacheInfo};
use dioxus::prelude::*;

#[server]
pub async fn list_cached_models() -> Result<Vec<ModelInfo>, ServerFnError> {
    let mut models = ModelManager::get_available_models().await?;
    ModelManager::check_cached_status(&mut models).await?;
    Ok(models)
}

#[server]
pub async fn download_model(model_id: String) -> Result<String, ServerFnError> {
    ModelManager::download_model(&model_id).await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(format!("Model {} downloaded successfully", model_id))
}

#[server]
pub async fn delete_model(model_id: String) -> Result<String, ServerFnError> {
    ModelManager::delete_model(&model_id).await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(format!("Model {} deleted from cache", model_id))
}

#[server]
pub async fn get_cache_info() -> Result<CacheInfo, ServerFnError> {
    ModelManager::get_cache_info().await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server]
pub async fn init_hf_cache() -> Result<String, ServerFnError> {
    ModelManager::init_hf_cache()
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok("HuggingFace cache initialized successfully".to_string())
}

#[server]
pub async fn ensure_model_cached(model_id: String) -> Result<String, ServerFnError> {
    ModelManager::ensure_model_cached(&model_id).await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(format!("Model {} is now cached", model_id))
}