//! Image Generation Implementation
//!
//! This module provides functionality for generating images from text prompts
//! using the Wuerstchen model via Kalosm vision.
//!
//! Phase 2.2: Image Generation Support

use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use once_cell::sync::Lazy;
use kalosm::vision::Wuerstchen;

/// Global storage for image generation model - persists across requests
static IMAGE_MODEL: Lazy<Mutex<Option<Wuerstchen>>> = Lazy::new(|| Mutex::new(None));

/// Flag to indicate if the model is currently generating
static IS_GENERATING: AtomicBool = AtomicBool::new(false);

/// Current generation status message
static GEN_STATUS: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));

/// Generation progress (0-100)
static GEN_PROGRESS: AtomicU8 = AtomicU8::new(0);

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
    use kalosm::vision::ModelLoadingProgress;

    // Check if already initialized
    if is_initialized() {
        println!("Image model already loaded, skipping initialization");
        return Ok(());
    }

    println!("Initializing image generation model (Wuerstchen)...");
    set_status("Downloading model...", 2);

    let model = Wuerstchen::builder()
        .build_with_loading_handler(|progress| {
            let pct_f = progress.progress();
            let pct = ((pct_f * 98.0) as u8 + 2).min(100);

            match &progress {
                ModelLoadingProgress::Downloading { source, .. } => {
                    set_status(&format!("Downloading: {:.0}%", pct_f * 100.0), pct);
                    println!("[ImageGen] Downloading {}: {:.1}%", source, pct_f * 100.0);
                }
                ModelLoadingProgress::Loading { .. } => {
                    set_status(&format!("Loading: {:.0}%", pct_f * 100.0), pct);
                    println!("[ImageGen] Loading into memory: {:.1}%", pct_f * 100.0);
                }
            }
        })
        .await
        .map_err(|e| {
            eprintln!("Error initializing Wuerstchen model: {}", e);
            e.to_string()
        })?;

    // Store the model in global state
    {
        let mut model_guard = IMAGE_MODEL.lock().unwrap();
        *model_guard = Some(model);
    }

    set_status("Ready", 0);
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

/// Get current generation status
pub fn get_generation_status() -> (String, u8) {
    let status = GEN_STATUS.lock()
        .map(|g| g.clone())
        .unwrap_or_default();
    let progress = GEN_PROGRESS.load(Ordering::SeqCst);
    (status, progress)
}

/// Set generation status and progress
fn set_status(status: &str, progress: u8) {
    if let Ok(mut s) = GEN_STATUS.lock() {
        *s = status.to_string();
    }
    GEN_PROGRESS.store(progress, Ordering::SeqCst);
    println!("[ImageGen] {}: {}%", status, progress);
}

/// Generate an image from a text prompt
pub async fn generate_image(settings: ImageGenSettings) -> Result<GeneratedImage, String> {
    use kalosm::vision::WuerstchenInferenceSettings;
    use futures::StreamExt;
    use image::ImageFormat;
    use std::io::Cursor;

    // Check if already generating
    if IS_GENERATING.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_err() {
        return Err("Image generation is already in progress".to_string());
    }

    // Use a guard to ensure we reset the flag and status
    let _guard = scopeguard::guard((), |_| {
        IS_GENERATING.store(false, Ordering::SeqCst);
        set_status("Ready", 0);
    });

    set_status("Starting generation...", 1);
    println!("Prompt: {}", settings.prompt);

    // Initialize model if not already done (downloads only once)
    if !is_initialized() {
        init_image_model().await?;
    }

    set_status("Model loaded, preparing...", 20);

    set_status("Generating image...", 30);

    // Take the model out temporarily for generation (to avoid holding lock across await)
    let model = {
        let mut model_guard = IMAGE_MODEL.lock().map_err(|e| format!("Failed to lock model: {}", e))?;
        model_guard.take().ok_or("Model not initialized")?
    };

    // Build inference settings and run generation
    let inference = WuerstchenInferenceSettings::new(&settings.prompt);
    let total_steps = settings.num_steps;
    let mut images = model.run(inference);
    let mut last_image = None;
    let mut step_count = 0;

    while let Some(image_result) = images.next().await {
        step_count += 1;
        let progress = 30 + ((step_count as f32 / total_steps as f32) * 60.0) as u8;
        let progress = progress.min(90);
        set_status(&format!("Step {}/{}", step_count, total_steps), progress);

        if let Some(img) = image_result.generated_image() {
            last_image = Some(img.clone());
        }
    }

    // Put the model back
    {
        let mut model_guard = IMAGE_MODEL.lock().map_err(|e| format!("Failed to lock model: {}", e))?;
        *model_guard = Some(model);
    }

    set_status("Processing result...", 95);

    let generated_img = last_image.ok_or_else(|| {
        set_status("Failed: No image generated", 0);
        "No image generated".to_string()
    })?;

    // Convert to PNG bytes
    let mut png_bytes = Vec::new();
    let mut cursor = Cursor::new(&mut png_bytes);
    generated_img.write_to(&mut cursor, ImageFormat::Png)
        .map_err(|e| {
            set_status(&format!("Failed: {}", e), 0);
            format!("Failed to encode image: {}", e)
        })?;

    set_status("Complete!", 100);
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
