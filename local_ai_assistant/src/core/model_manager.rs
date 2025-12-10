//! Model Manager
//!
//! Manages HuggingFace model downloads, caching, and switching.
//! Uses huggingface-cli for model management.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::fs;
use tokio::process::Command as AsyncCommand;
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};
use crate::models::{ModelInfo, ModelType, CacheInfo, get_available_models};

/// Model Manager for handling HuggingFace models
pub struct ModelManager {
    cache_dir: PathBuf,
}

impl ModelManager {
    /// Create a new ModelManager instance
    pub fn new() -> Self {
        let cache_dir = Self::get_cache_dir();
        Self { cache_dir }
    }

    /// Get HuggingFace cache directory
    fn get_cache_dir() -> PathBuf {
        // Use the symlinked cache directory
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("~"))
            .join(".cache/huggingface/hub")
    }

    /// Initialize environment variables for HuggingFace cache
    /// This uses system default paths which will follow the symlink to external disk
    pub fn init_hf_cache() -> Result<()> {
        // Use system default - it will automatically follow the symlink
        // No hardcoded paths needed
        Ok(())
    }

    /// Get list of available models
    pub async fn get_available_models() -> Result<Vec<ModelInfo>> {
        Ok(get_available_models())
    }

    /// Check if model is cached and update cache status
    pub async fn check_cached_status(models: &mut [ModelInfo]) -> Result<()> {
        let cache_dir = Self::get_cache_dir();

        for model in models.iter_mut() {
            let model_cache_dir = cache_dir.join(format!("models--{}", model.id.replace('/', "--")));
            model.is_cached = model_cache_dir.exists();
            model.cache_path = if model.is_cached {
                Some(model_cache_dir)
            } else {
                None
            };
        }

        Ok(())
    }

    /// Download a model using huggingface-cli
    /// Download a model using hf command
    pub async fn download_model(model_id: &str) -> Result<String> {
        println!("Downloading model: {}", model_id);

        // Check if hf command is available
        let output = Command::new("which")
            .arg("hf")
            .output();

        if !output.map_or(false, |o| o.status.success()) {
            return Err(anyhow::anyhow!(
                "hf command not found. Please install huggingface_hub: pip install -U huggingface_hub"
            ));
        }

        // Use hf to download
        let output = AsyncCommand::new("hf")
            .arg("download")
            .arg(model_id)
            .output()
            .await
            .context("Failed to execute hf download")?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Failed to download model {}: {}", model_id, error));
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        Ok(output_str.to_string())
    }

    /// Check if model is cached, download if not
    pub async fn ensure_model_cached(model_id: &str) -> Result<()> {
        let cache_dir = Self::get_cache_dir();
        let model_cache_dir = cache_dir.join(format!("models--{}", model_id.replace('/', "--")));

        if !model_cache_dir.exists() {
            println!("Model {} not found in cache, downloading...", model_id);
            Self::download_model(model_id).await?;
            println!("Model {} downloaded successfully", model_id);
        } else {
            println!("Model {} found in cache", model_id);
        }

        Ok(())
    }

    /// Delete a model from cache
    pub async fn delete_model(model_id: &str) -> Result<()> {
        let cache_dir = Self::get_cache_dir();
        let model_cache_dir = cache_dir.join(format!("models--{}", model_id.replace('/', "--")));

        if model_cache_dir.exists() {
            fs::remove_dir_all(&model_cache_dir)
                .context("Failed to delete model cache directory")?;
            println!("Model {} deleted from cache", model_id);
        }

        Ok(())
    }

    /// Get cache size information
    pub async fn get_cache_info() -> Result<CacheInfo> {
        let cache_dir = Self::get_cache_dir();

        if !cache_dir.exists() {
            return Ok(CacheInfo {
                path: cache_dir,
                total_size_mb: 0,
                model_count: 0,
            });
        }

        // Calculate total size
        let total_size = Self::calculate_dir_size(&cache_dir)?;

        // Count model directories
        let model_count = fs::read_dir(&cache_dir)?
            .filter_map(Result::ok)
            .filter(|entry| {
                entry.file_name()
                    .to_str()
                    .map(|s| s.starts_with("models--"))
                    .unwrap_or(false)
            })
            .count();

        Ok(CacheInfo {
            path: cache_dir,
            total_size_mb: total_size / (1024 * 1024),
            model_count,
        })
    }

    /// Calculate directory size recursively
    fn calculate_dir_size(path: &Path) -> Result<u64> {
        let mut total_size = 0u64;

        if path.is_dir() {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let entry_path = entry.path();

                if entry_path.is_dir() {
                    total_size += Self::calculate_dir_size(&entry_path)?;
                } else {
                    total_size += entry.metadata()?.len();
                }
            }
        }

        Ok(total_size)
    }

    /// Move cache to external disk
    pub async fn move_cache_to_external() -> Result<()> {
        let old_cache = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("~"))
            .join(".cache/huggingface");
        let new_cache = PathBuf::from("/Volumes/UltraDisk/Dev2/.cache/huggingface");

        if !old_cache.exists() {
            println!("No existing cache to move");
            return Ok(());
        }

        // Create new cache directory
        fs::create_dir_all(&new_cache)?;

        // Move contents
        println!("Moving cache to external disk...");
        let output = AsyncCommand::new("rsync")
            .arg("-avh")
            .arg("--progress")
            .arg(format!("{}/", old_cache.display()))
            .arg(format!("{}/", new_cache.display()))
            .output()
            .await
            .context("Failed to move cache with rsync")?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Failed to move cache: {}", error));
        }

        // Remove old cache
        fs::remove_dir_all(&old_cache)?;

        // Create symlink
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(&new_cache, &old_cache)?;
        }

        println!("Cache moved successfully to: {}", new_cache.display());
        Ok(())
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_model_manager_creation() {
        let manager = ModelManager::new();
        assert!(manager.cache_dir.exists());
    }
}