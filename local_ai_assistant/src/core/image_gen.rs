//! Image Generation Implementation
//!
//! This module provides functionality for generating images from text prompts
//! using MFLUX (MLX-based Flux model) via subprocess.
//!
//! Phase 2.2: Image Generation Support (MFLUX backend)

use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use once_cell::sync::Lazy;
use std::process::Command;
use std::path::PathBuf;

/// Flag to indicate if the model is currently generating
static IS_GENERATING: AtomicBool = AtomicBool::new(false);

/// Current generation status message
static GEN_STATUS: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));

/// Generation progress (0-100)
static GEN_PROGRESS: AtomicU8 = AtomicU8::new(0);

/// MFLUX model types
#[derive(Clone, Debug, Default)]
pub enum MfluxModel {
    #[default]
    Schnell,       // Fast, 4 steps
    Dev,           // Higher quality, 20 steps
    ZImageTurbo,   // Very fast, 9 steps
}

impl MfluxModel {
    pub fn name(&self) -> &'static str {
        match self {
            MfluxModel::Schnell => "schnell",
            MfluxModel::Dev => "dev",
            MfluxModel::ZImageTurbo => "mlx-community/Z-Image-Turbo",
        }
    }

    /// Returns the base model for custom models (needed for mflux --base-model parameter)
    pub fn base_model(&self) -> Option<&'static str> {
        match self {
            MfluxModel::Schnell => None,  // Built-in, no base model needed
            MfluxModel::Dev => None,      // Built-in, no base model needed
            MfluxModel::ZImageTurbo => Some("schnell"),  // Based on schnell
        }
    }

    pub fn default_steps(&self) -> u32 {
        match self {
            MfluxModel::Schnell => 4,
            MfluxModel::Dev => 20,
            MfluxModel::ZImageTurbo => 9,
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            MfluxModel::Schnell => "FLUX.1 Schnell (Fast)",
            MfluxModel::Dev => "FLUX.1 Dev (Quality)",
            MfluxModel::ZImageTurbo => "Z-Image Turbo (Very Fast)",
        }
    }
}

/// Image generation settings
#[derive(Clone, Debug)]
pub struct ImageGenSettings {
    pub prompt: String,
    pub negative_prompt: Option<String>,
    pub width: u32,
    pub height: u32,
    pub num_steps: Option<u32>,
    pub model: MfluxModel,
    pub quantize: Option<u8>,  // 4 or 8 bit quantization
    pub seed: Option<u64>,
}

impl Default for ImageGenSettings {
    fn default() -> Self {
        Self {
            prompt: String::new(),
            negative_prompt: None,
            width: 1024,
            height: 1024,
            num_steps: None,  // Use model default
            model: MfluxModel::Schnell,
            quantize: Some(8),  // 8-bit quantization by default for speed
            seed: None,
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
        self.num_steps = Some(steps);
        self
    }

    pub fn with_model(mut self, model: MfluxModel) -> Self {
        self.model = model;
        self
    }

    pub fn with_quantize(mut self, bits: u8) -> Self {
        self.quantize = Some(bits);
        self
    }

    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
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

/// Check if mflux-generate command is available
pub fn is_mflux_available() -> bool {
    Command::new("mflux-generate")
        .arg("--help")
        .output()
        .is_ok()
}

/// Get the output directory for generated images
fn get_output_dir() -> PathBuf {
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    let output_dir = home.join(".local_ai_assistant").join("images");
    std::fs::create_dir_all(&output_dir).ok();
    output_dir
}

/// Initialize MFLUX (check if available)
pub async fn init_image_model() -> Result<(), String> {
    set_status("Checking MFLUX...", 10);

    if !is_mflux_available() {
        set_status("MFLUX not found", 0);
        return Err("MFLUX not installed. Install with: uv tool install mflux".to_string());
    }

    set_status("Ready (MFLUX)", 0);
    println!("[ImageGen] MFLUX is available");
    Ok(())
}

/// Check if MFLUX is available (no model state needed for CLI tool)
pub fn is_initialized() -> bool {
    is_mflux_available()
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

/// Generate an image from a text prompt using MFLUX CLI
pub async fn generate_image(settings: ImageGenSettings) -> Result<GeneratedImage, String> {
    use std::time::{SystemTime, UNIX_EPOCH};

    // Check if already generating
    if IS_GENERATING.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_err() {
        return Err("Image generation is already in progress".to_string());
    }

    // Use a guard to ensure we reset the flag and status
    let _guard = scopeguard::guard((), |_| {
        IS_GENERATING.store(false, Ordering::SeqCst);
        set_status("Ready", 0);
    });

    set_status("Starting generation...", 5);
    println!("[ImageGen] Prompt: {}", settings.prompt);
    println!("[ImageGen] Model: {}", settings.model.display_name());

    // Check if MFLUX is available
    if !is_mflux_available() {
        set_status("MFLUX not installed", 0);
        return Err("MFLUX not installed. Install with: uv tool install mflux".to_string());
    }

    set_status("Preparing MFLUX...", 10);

    // Generate unique output filename
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    let output_dir = get_output_dir();
    let output_file = output_dir.join(format!("image_{}.png", timestamp));

    // Build mflux-generate command
    let mut cmd = Command::new("mflux-generate");

    // Model selection
    cmd.arg("--model").arg(settings.model.name());

    // Add base model if needed (for custom HuggingFace models like Z-Image-Turbo)
    if let Some(base) = settings.model.base_model() {
        cmd.arg("--base-model").arg(base);
    }

    // Prompt
    cmd.arg("--prompt").arg(&settings.prompt);

    // Output path
    cmd.arg("--output").arg(&output_file);

    // Image dimensions
    cmd.arg("--width").arg(settings.width.to_string());
    cmd.arg("--height").arg(settings.height.to_string());

    // Steps (use model default if not specified)
    let steps = settings.num_steps.unwrap_or(settings.model.default_steps());
    cmd.arg("--steps").arg(steps.to_string());

    // Quantization
    if let Some(q) = settings.quantize {
        cmd.arg("--quantize").arg(q.to_string());
    }

    // Seed
    if let Some(seed) = settings.seed {
        cmd.arg("--seed").arg(seed.to_string());
    }

    set_status(&format!("Generating with {}...", settings.model.display_name()), 20);
    println!("[ImageGen] Running: mflux-generate --model {} --prompt \"{}\" --width {} --height {} --steps {}",
        settings.model.name(),
        settings.prompt,
        settings.width,
        settings.height,
        steps
    );

    // Run the command
    let output = cmd.output().map_err(|e| {
        set_status(&format!("Failed: {}", e), 0);
        format!("Failed to run mflux-generate: {}", e)
    })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        set_status("Generation failed", 0);
        eprintln!("[ImageGen] MFLUX stderr: {}", stderr);
        return Err(format!("MFLUX generation failed: {}", stderr));
    }

    set_status("Reading generated image...", 90);

    // Read the generated image
    let png_bytes = std::fs::read(&output_file).map_err(|e| {
        set_status(&format!("Failed: {}", e), 0);
        format!("Failed to read generated image: {}", e)
    })?;

    // Get image dimensions using image crate
    let img = image::load_from_memory(&png_bytes).map_err(|e| {
        set_status(&format!("Failed: {}", e), 0);
        format!("Failed to parse image: {}", e)
    })?;

    set_status("Complete!", 100);
    println!("[ImageGen] Image generated successfully! Size: {} bytes", png_bytes.len());

    Ok(GeneratedImage {
        data: png_bytes,
        width: img.width(),
        height: img.height(),
        format: "png".to_string(),
    })
}

/// Generate an image and return as base64 encoded string
pub async fn generate_image_base64(prompt: &str) -> Result<String, String> {
    let settings = ImageGenSettings::new(prompt);
    let image = generate_image(settings).await?;
    Ok(image.to_data_url())
}
