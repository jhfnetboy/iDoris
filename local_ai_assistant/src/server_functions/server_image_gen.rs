//! Image Generation Server Functions
//!
//! This module contains Dioxus server functions for image generation functionality.
//! Phase 2.2: Image Generation Support

use dioxus::prelude::*;

/// Result of image generation returned to client
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ImageResult {
    pub data_url: String,
    pub width: u32,
    pub height: u32,
}

/// Initializes the image generation model.
///
/// # Returns
///
/// * `Result<()>` - Success or error with detailed message
#[server]
pub async fn init_image_model() -> Result<(), ServerFnError> {
    #[cfg(feature = "server")]
    {
        use crate::core::image_gen;
        image_gen::init_image_model().await.map_err(|e| {
            ServerFnError::new(&format!("Error initializing image model: {}", e))
        })
    }
    #[cfg(not(feature = "server"))]
    {
        Ok(())
    }
}

/// Checks if the image model is initialized.
///
/// # Returns
///
/// * `Result<bool>` - Whether the model is initialized
#[server]
pub async fn is_image_model_ready() -> Result<bool, ServerFnError> {
    #[cfg(feature = "server")]
    {
        Ok(crate::core::image_gen::is_initialized())
    }
    #[cfg(not(feature = "server"))]
    {
        Ok(false)
    }
}

/// Checks if image generation is currently in progress.
///
/// # Returns
///
/// * `Result<bool>` - Whether generation is in progress
#[server]
pub async fn is_image_generating() -> Result<bool, ServerFnError> {
    #[cfg(feature = "server")]
    {
        Ok(crate::core::image_gen::is_generating())
    }
    #[cfg(not(feature = "server"))]
    {
        Ok(false)
    }
}

/// Generation status response
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ImageGenStatus {
    pub is_generating: bool,
    pub status: String,
    pub progress: u8,
}

/// Gets the current image generation status.
///
/// # Returns
///
/// * `Result<ImageGenStatus>` - Current generation status, message, and progress (0-100)
#[server]
pub async fn get_image_gen_status() -> Result<ImageGenStatus, ServerFnError> {
    #[cfg(feature = "server")]
    {
        use crate::core::image_gen::{is_generating, get_generation_status};
        let (status, progress) = get_generation_status();
        Ok(ImageGenStatus {
            is_generating: is_generating(),
            status,
            progress,
        })
    }
    #[cfg(not(feature = "server"))]
    {
        Ok(ImageGenStatus {
            is_generating: false,
            status: "Not available".to_string(),
            progress: 0,
        })
    }
}

/// Generates an image from a text prompt using MFLUX.
///
/// # Arguments
///
/// * `prompt` - The text prompt describing the image to generate
/// * `negative_prompt` - Optional negative prompt to avoid certain elements
/// * `width` - Image width (default: 1024)
/// * `height` - Image height (default: 1024)
/// * `steps` - Number of inference steps (uses model default if None)
/// * `model` - MFLUX model: "schnell" (fast), "dev" (quality), "z-image-turbo" (very fast)
/// * `quantize` - Quantization bits: 4 or 8 (default: 8)
///
/// # Returns
///
/// * `Result<ImageResult>` - The generated image as a data URL or error
#[server]
pub async fn generate_image(
    prompt: String,
    negative_prompt: Option<String>,
    width: Option<u32>,
    height: Option<u32>,
    steps: Option<u32>,
    model: Option<String>,
    quantize: Option<u8>,
) -> Result<ImageResult, ServerFnError> {
    #[cfg(feature = "server")]
    {
        use crate::core::image_gen::{ImageGenSettings, MfluxModel, generate_image as gen_img};

        let mut settings = ImageGenSettings::new(&prompt);

        if let Some(neg) = negative_prompt {
            settings = settings.with_negative_prompt(&neg);
        }

        if let (Some(w), Some(h)) = (width, height) {
            settings = settings.with_size(w, h);
        }

        if let Some(s) = steps {
            settings = settings.with_steps(s);
        }

        // Parse model selection
        if let Some(m) = model {
            let mflux_model = match m.as_str() {
                "dev" => MfluxModel::Dev,
                "z-image-turbo" => MfluxModel::ZImageTurbo,
                _ => MfluxModel::Schnell, // Default to schnell
            };
            settings = settings.with_model(mflux_model);
        }

        if let Some(q) = quantize {
            settings = settings.with_quantize(q);
        }

        let image = gen_img(settings).await.map_err(|e| {
            ServerFnError::new(&format!("Error generating image: {}", e))
        })?;

        Ok(ImageResult {
            data_url: image.to_data_url(),
            width: image.width,
            height: image.height,
        })
    }
    #[cfg(not(feature = "server"))]
    {
        let _ = (prompt, negative_prompt, width, height, steps, model, quantize);
        Err(ServerFnError::new("Image generation not available on client"))
    }
}

/// Generates an image with default settings.
///
/// Simplified version of generate_image for quick generation.
///
/// # Arguments
///
/// * `prompt` - The text prompt describing the image to generate
///
/// # Returns
///
/// * `Result<String>` - The generated image as a data URL or error
#[server]
pub async fn generate_image_simple(prompt: String) -> Result<String, ServerFnError> {
    #[cfg(feature = "server")]
    {
        use crate::core::image_gen::generate_image_base64;

        generate_image_base64(&prompt).await.map_err(|e| {
            ServerFnError::new(&format!("Error generating image: {}", e))
        })
    }
    #[cfg(not(feature = "server"))]
    {
        let _ = prompt;
        Err(ServerFnError::new("Image generation not available on client"))
    }
}
