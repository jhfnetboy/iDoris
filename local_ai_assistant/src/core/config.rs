//! Configuration validation and management
//!
//! Validates environment variables and configuration on startup.

use super::error::{IDorisError, Result};
use std::env;

/// Validates all required environment variables
pub fn validate_env_config() -> Result<()> {
    println!("Validating environment configuration...");
    
    // Check for model paths (optional but recommended)
    if env::var("HF_HOME").is_err() && env::var("TRANSFORMERS_CACHE").is_err() {
        println!("Info: HF_HOME or TRANSFORMERS_CACHE not set. Models will use default cache location.");
    }
    
    // Validate API keys for external services (all optional)
    validate_optional_api_keys();
    
    println!("✅ Environment configuration validated");
    Ok(())
}

/// Validate optional API keys and provide helpful warnings
fn validate_optional_api_keys() {
    // ByteDance/Jimeng keys
    if env::var("Access_Key_ID").is_err() && env::var("JIMENG_ACCESS_KEY").is_err() {
        println!("Info: ByteDance/Jimeng API keys not configured. Video generation with ByteDance will not be available.");
    }
    
    // Together.ai
    if env::var("TOGETHER_API_KEY").is_err() {
        println!("Info: Together.ai API key not configured.");
    }
    
    // Replicate
    if env::var("REPLICATE_API_TOKEN").is_err() {
        println!("Info: Replicate API token not configured.");
    }
}

/// Validates a specific API key is present and not empty
pub fn validate_api_key(key_name: &str) -> Result<String> {
    env::var(key_name)
        .map_err(|_| IDorisError::ConfigError(
            format!("{} not configured. Please set it in your .env file.", key_name)
        ))
        .and_then(|val| {
            if val.trim().is_empty() {
                Err(IDorisError::ConfigError(
                    format!("{} is empty. Please provide a valid value.", key_name)
                ))
            } else {
                Ok(val)
            }
        })
}

/// Validates API keys with multiple possible names (fallback support)
pub fn validate_api_key_with_fallbacks(preferred: &str, fallbacks: &[&str]) -> Result<String> {
    // Try preferred first
    if let Ok(val) = validate_api_key(preferred) {
        return Ok(val);
    }
    
    // Try fallbacks
    for key in fallbacks {
        if let Ok(val) = validate_api_key(key) {
            println!("Using {} (fallback for {})", key, preferred);
            return Ok(val);
        }
    }
    
    Err(IDorisError::ConfigError(
        format!("None of the API keys found: {}, {}", preferred, fallbacks.join(", "))
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validate_env_config() {
        // Should not error even with missing keys
        assert!(validate_env_config().is_ok());
    }
}
