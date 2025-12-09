//! Text-to-Speech Implementation
//!
//! This module provides TTS functionality using various backends:
//! - VibeVoice-Realtime-0.5B (Microsoft, MLX optimized)
//! - Kokoro (via MLX-Audio)
//!
//! Phase 2.3: TTS Support

use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use once_cell::sync::Lazy;
use std::process::Command;
use std::path::PathBuf;

/// TTS Engine type
#[derive(Clone, Debug, PartialEq, Default)]
pub enum TtsEngine {
    #[default]
    VibeVoice,
    Kokoro,
    System, // macOS say command fallback
}

impl TtsEngine {
    pub fn display_name(&self) -> &'static str {
        match self {
            TtsEngine::VibeVoice => "VibeVoice-Realtime-0.5B",
            TtsEngine::Kokoro => "Kokoro-82M",
            TtsEngine::System => "System TTS",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            TtsEngine::VibeVoice => "Microsoft real-time TTS, ~300ms latency",
            TtsEngine::Kokoro => "MLX-Audio TTS, high quality",
            TtsEngine::System => "macOS built-in TTS (fallback)",
        }
    }
}

/// TTS settings
#[derive(Clone, Debug)]
pub struct TtsSettings {
    pub text: String,
    pub engine: TtsEngine,
    pub voice: Option<String>,
    pub speed: f32,
    pub pitch: f32,
}

impl Default for TtsSettings {
    fn default() -> Self {
        Self {
            text: String::new(),
            engine: TtsEngine::default(),
            voice: None,
            speed: 1.0,
            pitch: 1.0,
        }
    }
}

impl TtsSettings {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            ..Default::default()
        }
    }

    pub fn with_engine(mut self, engine: TtsEngine) -> Self {
        self.engine = engine;
        self
    }

    pub fn with_speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }
}

/// Generated audio result
#[derive(Clone, Debug)]
pub struct GeneratedAudio {
    pub data: Vec<u8>,
    pub sample_rate: u32,
    pub format: String,
    pub duration_ms: u32,
}

impl GeneratedAudio {
    pub fn to_base64(&self) -> String {
        use base64::Engine;
        base64::engine::general_purpose::STANDARD.encode(&self.data)
    }

    pub fn to_data_url(&self) -> String {
        format!("data:audio/{};base64,{}", self.format, self.to_base64())
    }
}

/// TTS generation status
static IS_GENERATING: AtomicBool = AtomicBool::new(false);
static GEN_STATUS: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));
static GEN_PROGRESS: AtomicU8 = AtomicU8::new(0);

/// Check if TTS is currently generating
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

/// Set generation status
fn set_status(status: &str, progress: u8) {
    if let Ok(mut s) = GEN_STATUS.lock() {
        *s = status.to_string();
    }
    GEN_PROGRESS.store(progress, Ordering::SeqCst);
    println!("[TTS] {}: {}%", status, progress);
}

/// Get the model directory path
fn get_models_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("models")
}

/// Check if VibeVoice model is downloaded
pub fn is_vibevoice_available() -> bool {
    let model_path = get_models_dir().join("VibeVoice-Realtime-0.5B");
    model_path.exists() && model_path.is_dir()
}

/// Check if mlx-audio is installed
pub fn is_mlx_audio_available() -> bool {
    Command::new("python3")
        .args(["-c", "import mlx_audio"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Get available TTS engines based on what's installed
pub fn get_available_engines() -> Vec<TtsEngine> {
    let mut engines = vec![TtsEngine::System]; // Always available on macOS

    if is_vibevoice_available() {
        engines.insert(0, TtsEngine::VibeVoice);
    }

    if is_mlx_audio_available() {
        engines.push(TtsEngine::Kokoro);
    }

    engines
}

/// Generate speech using system TTS (macOS say command)
async fn generate_system_tts(text: &str, speed: f32) -> Result<GeneratedAudio, String> {
    use std::fs;
    use std::io::Read;

    set_status("Generating with system TTS...", 30);

    let temp_file = std::env::temp_dir().join("tts_output.aiff");
    let temp_path = temp_file.to_string_lossy().to_string();

    // Calculate rate (default is ~175 words per minute)
    let rate = (175.0 * speed) as i32;

    let output = Command::new("say")
        .args(["-o", &temp_path, "-r", &rate.to_string(), text])
        .output()
        .map_err(|e| format!("Failed to run say command: {}", e))?;

    if !output.status.success() {
        return Err(format!("say command failed: {}", String::from_utf8_lossy(&output.stderr)));
    }

    set_status("Reading audio file...", 80);

    // Read the generated file
    let mut file = fs::File::open(&temp_file)
        .map_err(|e| format!("Failed to open audio file: {}", e))?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)
        .map_err(|e| format!("Failed to read audio file: {}", e))?;

    // Clean up temp file
    let _ = fs::remove_file(&temp_file);

    set_status("Complete!", 100);

    Ok(GeneratedAudio {
        data,
        sample_rate: 22050,
        format: "aiff".to_string(),
        duration_ms: (text.len() as u32 * 50) / speed as u32, // Rough estimate
    })
}

/// Generate speech using VibeVoice (via Python subprocess)
async fn generate_vibevoice_tts(text: &str, _speed: f32) -> Result<GeneratedAudio, String> {
    use std::fs;
    use std::io::Read;

    set_status("Loading VibeVoice model...", 10);

    let model_path = get_models_dir().join("VibeVoice-Realtime-0.5B");
    if !model_path.exists() {
        return Err("VibeVoice model not found. Please download it first.".to_string());
    }

    let temp_file = std::env::temp_dir().join("vibevoice_output.wav");
    let temp_path = temp_file.to_string_lossy().to_string();

    set_status("Generating speech...", 30);

    // Get voice preset path (use default voice if available)
    let voices_dir = model_path.join("voices").join("streaming_model");
    let voice_preset = if voices_dir.exists() {
        // Find first available voice preset
        std::fs::read_dir(&voices_dir)
            .ok()
            .and_then(|entries| {
                entries
                    .filter_map(|e| e.ok())
                    .find(|e| e.path().extension().map_or(false, |ext| ext == "pt"))
                    .map(|e| e.path())
            })
    } else {
        None
    };

    // Python script to run VibeVoice using the correct streaming inference API with voice presets
    let python_script = format!(r#"
import sys
import torch
import copy
import traceback
import os

try:
    from vibevoice.modular.modeling_vibevoice_streaming_inference import VibeVoiceStreamingForConditionalGenerationInference
    from vibevoice.processor.vibevoice_streaming_processor import VibeVoiceStreamingProcessor

    model_path = '{model_path}'
    voice_preset_path = '{voice_preset}'

    # Determine device and dtype
    if torch.backends.mps.is_available():
        device = 'mps'
        load_dtype = torch.float32
        attn_impl = 'sdpa'
    elif torch.cuda.is_available():
        device = 'cuda'
        load_dtype = torch.bfloat16
        attn_impl = 'flash_attention_2'
    else:
        device = 'cpu'
        load_dtype = torch.float32
        attn_impl = 'sdpa'

    print(f'Using device: {{device}}', file=sys.stderr)

    # Load processor and model
    processor = VibeVoiceStreamingProcessor.from_pretrained(model_path)

    try:
        if device == 'mps':
            model = VibeVoiceStreamingForConditionalGenerationInference.from_pretrained(
                model_path,
                torch_dtype=load_dtype,
                attn_implementation=attn_impl,
                device_map=None,
            )
            model.to('mps')
        elif device == 'cuda':
            model = VibeVoiceStreamingForConditionalGenerationInference.from_pretrained(
                model_path,
                torch_dtype=load_dtype,
                device_map='cuda',
                attn_implementation=attn_impl,
            )
        else:
            model = VibeVoiceStreamingForConditionalGenerationInference.from_pretrained(
                model_path,
                torch_dtype=load_dtype,
                device_map='cpu',
                attn_implementation=attn_impl,
            )
    except Exception as e:
        # Fallback to SDPA if flash_attention_2 fails
        print(f'Falling back to SDPA: {{e}}', file=sys.stderr)
        model = VibeVoiceStreamingForConditionalGenerationInference.from_pretrained(
            model_path,
            torch_dtype=load_dtype,
            device_map=(device if device in ('cuda', 'cpu') else None),
            attn_implementation='sdpa'
        )
        if device == 'mps':
            model.to('mps')

    model.eval()
    model.set_ddpm_inference_steps(num_steps=5)

    # Prepare text
    full_script = '''{text}'''.replace("'", "'").replace('"', '"').replace('"', '"')

    # Load voice preset if available
    all_prefilled_outputs = None
    if voice_preset_path and os.path.exists(voice_preset_path):
        print(f'Loading voice preset: {{voice_preset_path}}', file=sys.stderr)
        all_prefilled_outputs = torch.load(voice_preset_path, map_location=device, weights_only=False)

        # Prepare inputs with cached prompt (voice preset)
        inputs = processor.process_input_with_cached_prompt(
            text=full_script,
            cached_prompt=all_prefilled_outputs,
            padding=True,
            return_tensors='pt',
            return_attention_mask=True,
        )
    else:
        print('No voice preset found, using default voice', file=sys.stderr)
        # Prepare inputs without voice preset
        inputs = processor(
            text=full_script,
            padding=True,
            return_tensors='pt',
            return_attention_mask=True,
        )

    # Move tensors to device
    for k, v in inputs.items():
        if torch.is_tensor(v):
            inputs[k] = v.to(device)

    # Generate audio
    outputs = model.generate(
        **inputs,
        max_new_tokens=None,
        cfg_scale=1.5,
        tokenizer=processor.tokenizer,
        generation_config={{'do_sample': False}},
        verbose=False,
        all_prefilled_outputs=copy.deepcopy(all_prefilled_outputs) if all_prefilled_outputs is not None else None,
    )

    # Save audio
    if outputs.speech_outputs and outputs.speech_outputs[0] is not None:
        processor.save_audio(
            outputs.speech_outputs[0],
            output_path='{output}',
        )
        print('SUCCESS')
    else:
        print('ERROR: No audio output generated')
        sys.exit(1)

except Exception as e:
    print(f'ERROR: {{e}}')
    traceback.print_exc()
    sys.exit(1)
"#,
        model_path = model_path.display(),
        voice_preset = voice_preset.as_ref().map(|p| p.display().to_string()).unwrap_or_default(),
        text = text.replace("'", "\\'").replace('\n', "\\n"),
        output = temp_path
    );

    let output = Command::new("python3")
        .args(["-c", &python_script])
        .output()
        .map_err(|e| format!("Failed to run Python: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() || !stdout.contains("SUCCESS") {
        return Err(format!("VibeVoice generation failed: {} {}", stdout, stderr));
    }

    set_status("Reading audio file...", 80);

    let mut file = fs::File::open(&temp_file)
        .map_err(|e| format!("Failed to open audio file: {}", e))?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)
        .map_err(|e| format!("Failed to read audio file: {}", e))?;

    let _ = fs::remove_file(&temp_file);

    set_status("Complete!", 100);

    Ok(GeneratedAudio {
        data,
        sample_rate: 24000,
        format: "wav".to_string(),
        duration_ms: (text.len() as u32 * 60), // Rough estimate
    })
}

/// Main TTS generation function
pub async fn generate_speech(settings: TtsSettings) -> Result<GeneratedAudio, String> {
    // Check if already generating
    if IS_GENERATING.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_err() {
        return Err("TTS generation is already in progress".to_string());
    }

    // Use a guard to ensure we reset the flag
    let _guard = scopeguard::guard((), |_| {
        IS_GENERATING.store(false, Ordering::SeqCst);
        set_status("Ready", 0);
    });

    set_status("Starting TTS generation...", 1);
    println!("[TTS] Text: {} ({})", &settings.text[..settings.text.len().min(50)], settings.engine.display_name());

    match settings.engine {
        TtsEngine::VibeVoice => {
            if !is_vibevoice_available() {
                return Err("VibeVoice model not downloaded. Please download from Settings.".to_string());
            }
            generate_vibevoice_tts(&settings.text, settings.speed).await
        }
        TtsEngine::Kokoro => {
            // TODO: Implement Kokoro via mlx-audio
            Err("Kokoro TTS not yet implemented".to_string())
        }
        TtsEngine::System => {
            generate_system_tts(&settings.text, settings.speed).await
        }
    }
}

/// Quick TTS using default settings
pub async fn speak_text(text: &str) -> Result<GeneratedAudio, String> {
    let engines = get_available_engines();
    let engine = engines.first().cloned().unwrap_or(TtsEngine::System);

    let settings = TtsSettings::new(text).with_engine(engine);
    generate_speech(settings).await
}
