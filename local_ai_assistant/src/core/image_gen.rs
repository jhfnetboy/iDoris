//! Image Generation Implementation
//!
//! This module provides functionality for generating images from text prompts
//! using the Wuerstchen model via Kalosm vision.
//!
//! Phase 2.2: Image Generation Support

use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use once_cell::sync::Lazy;

/// Global storage for image generation model
static IMAGE_MODEL: Lazy<Mutex<Option<ImageGenState>>> = Lazy::new(|| Mutex::new(None));

/// Flag to indicate if the model is currently generating
static IS_GENERATING: AtomicBool = AtomicBool::new(false);

/// State holder for the image generation model
struct ImageGenState {
    // The Wuerstchen model will be stored here when initialized
    _initialized: bool,
}

/// Image generation settings
#[derive(Clone, Debug)]
pub struct ImageGenSettings {
    pub prompt: String,
    pub negative_prompt: Option<String>,
    pub width: u32,
    pub height: u32,
    pub num_steps: u32,
}

impl Default for ImageGenSettings {
    fn default() -> Self {
        Self {
            prompt: String::new(),
            negative_prompt: None,
            width: 512,
            height: 512,
            num_steps: 30,
        }
    }
}

impl ImageGenSettings {
    pub fn new(prompt: &str) -> Self {
        Self {
            prompt: prompt.to_string(),
            ..Default::default()
        }
    }

    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn with_negative_prompt(mut self, negative: &str) -> Self {
        self.negative_prompt = Some(negative.to_string());
        self
    }

    pub fn with_steps(mut self, steps: u32) -> Self {
        self.num_steps = steps;
        self
    }
}

/// Result of image generation
#[derive(Clone, Debug)]
pub struct GeneratedImage {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub format: String,
}

impl GeneratedImage {
    pub fn to_base64(&self) -> String {
        use base64::Engine;
        base64::engine::general_purpose::STANDARD.encode(&self.data)
    }

    pub fn to_data_url(&self) -> String {
        format!("data:image/{};base64,{}", self.format, self.to_base64())
    }
}

/// Initialize the image generation model
pub async fn init_image_model() -> Result<(), String> {
    use kalosm::vision::Wuerstchen;

    println!("Initializing image generation model (Wuerstchen)...");

    let _model = Wuerstchen::new().await.map_err(|e| {
        eprintln!("Error initializing Wuerstchen model: {}", e);
        e.to_string()
    })?;

    // Store initialization state
    {
        let mut model_guard = IMAGE_MODEL.lock().unwrap();
        *model_guard = Some(ImageGenState { _initialized: true });
    }

    println!("Image generation model initialized successfully!");
    Ok(())
}

/// Check if the image model is initialized
pub fn is_initialized() -> bool {
    IMAGE_MODEL.lock()
        .map(|g| g.is_some())
        .unwrap_or(false)
}

/// Check if generation is in progress
pub fn is_generating() -> bool {
    IS_GENERATING.load(Ordering::SeqCst)
}

/// Generate an image from a text prompt
pub async fn generate_image(settings: ImageGenSettings) -> Result<GeneratedImage, String> {
    use kalosm::vision::{Wuerstchen, WuerstchenInferenceSettings};
    use futures::StreamExt;
    use image::ImageFormat;
    use std::io::Cursor;

    // Check if already generating
    if IS_GENERATING.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_err() {
        return Err("Image generation is already in progress".to_string());
    }

    // Use a guard to ensure we reset the flag
    let _guard = scopeguard::guard((), |_| {
        IS_GENERATING.store(false, Ordering::SeqCst);
    });

    println!("Starting image generation...");
    println!("Prompt: {}", settings.prompt);

    // Create a new model instance for this generation
    // (Wuerstchen doesn't persist state like LLM chat sessions)
    let model = Wuerstchen::new().await.map_err(|e| {
        eprintln!("Error creating Wuerstchen model: {}", e);
        e.to_string()
    })?;

    // Build inference settings
    let mut inference = WuerstchenInferenceSettings::new(&settings.prompt);

    // Note: WuerstchenInferenceSettings may have limited configuration options
    // The exact API depends on the kalosm version

    // Run generation and collect the last image
    let mut images = model.run(inference);
    let mut last_image = None;

    while let Some(image_result) = images.next().await {
        if let Some(img) = image_result.generated_image() {
            last_image = Some(img.clone());
        }
    }

    let generated_img = last_image.ok_or("No image generated")?;

    // Convert to PNG bytes
    let mut png_bytes = Vec::new();
    let mut cursor = Cursor::new(&mut png_bytes);
    generated_img.write_to(&mut cursor, ImageFormat::Png)
        .map_err(|e| format!("Failed to encode image: {}", e))?;

    println!("Image generated successfully! Size: {} bytes", png_bytes.len());

    Ok(GeneratedImage {
        data: png_bytes,
        width: generated_img.width(),
        height: generated_img.height(),
        format: "png".to_string(),
    })
}

/// Generate an image and return as base64 encoded string
pub async fn generate_image_base64(prompt: &str) -> Result<String, String> {
    let settings = ImageGenSettings::new(prompt);
    let image = generate_image(settings).await?;
    Ok(image.to_data_url())
}
