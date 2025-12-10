use dioxus::prelude::*;
use anyhow::Result;
#[cfg(feature = "server")]
use std::sync::Arc;
#[cfg(feature = "server")]
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};
use crate::models::{VideoProvider, VideoModel, VideoConfig, VideoQuality};

// 仅在 server 特性下导入 video_gen
#[cfg(feature = "server")]
use crate::core::video_gen::*;

// 视频生成请求表单
#[derive(Deserialize, Serialize, Clone)]
pub struct VideoGenForm {
    pub prompt: String,
    pub negative_prompt: Option<String>,
    pub duration_seconds: u32,
    pub width: u32,
    pub height: u32,
    pub quality: VideoQuality,
    pub fps: u8,
    pub provider: VideoProvider,
    pub model: VideoModel,
    pub seed: Option<u32>,
}

impl Default for VideoGenForm {
    fn default() -> Self {
        Self {
            prompt: "a lovely white cat is playing in the garden".to_string(),
            negative_prompt: None,
            duration_seconds: 5,
            width: 1024,
            height: 576,
            quality: VideoQuality::HD,
            fps: 24,
            provider: VideoProvider::ByteDance, // Default to ByteDance (Cost-effective)
            model: VideoModel::JimengV2,
            seed: None,
        }
    }
}

// Create a simplified VideoResponse struct for web mode
#[derive(Serialize, Deserialize, Clone)]
pub struct VideoResponse {
    pub video_url: String,
    pub thumbnail_url: Option<String>,
    pub generation_id: String,
    pub duration_seconds: u32,
    pub cost_estimate: f64,
    pub status: String,
}

// Provider Info Structure
#[derive(Serialize, Deserialize, Clone)]
pub struct VideoProviderInfo {
    pub provider: VideoProvider,
    pub name: String,
    pub models: Vec<(String, VideoModel)>,
    pub description: String,
}

// Provider Config Status
#[derive(Serialize, Deserialize, Clone)]
pub struct ProviderConfigStatus {
    pub provider: VideoProvider,
    pub name: String,
    pub is_configured: bool,
    pub env_key: String,
}

// Task Status
#[derive(Serialize, Deserialize, Clone)]
pub struct VideoTaskStatus {
    pub task_id: String,
    pub status: String,
    pub progress: u8,
    pub video_url: Option<String>,
    pub error: Option<String>,
}

// Enable actual video generation only under
#[cfg(feature = "server")]
use lazy_static::lazy_static;

#[cfg(feature = "server")]
lazy_static::lazy_static! {
    static ref VIDEO_GENERATOR: Arc<Mutex<VideoGenerator>> = Arc::new(Mutex::new(VideoGenerator::new()));
}

#[server]
pub async fn generate_video(form: VideoGenForm) -> Result<VideoResponse, ServerFnError> {
    #[cfg(feature = "server")]
    {
        let generator = VIDEO_GENERATOR.lock().await;

        // Build request
        let request = VideoRequest::new(form.prompt)
            .with_model(form.model)
            .with_provider(form.provider)
            .with_config(VideoConfig {
                width: form.width,
                height: form.height,
                duration_seconds: form.duration_seconds,
                fps: form.fps,
                quality: form.quality,
                style: None,
            });

        // Set negative prompt and seed
        let mut request = request;
        if let Some(negative) = form.negative_prompt {
            request.negative_prompt = Some(negative);
        }
        if let Some(seed) = form.seed {
            request.seed = Some(seed);
        }

        // Generate video
        let response = generator.generate_video(request)
            .await
            .map_err(|e| ServerFnError::new(format!("Video generation failed: {}", e)))?;

        // Convert to simplified response format
        Ok(VideoResponse {
            video_url: response.video_url,
            thumbnail_url: response.thumbnail_url,
            generation_id: response.generation_id,
            duration_seconds: response.duration_seconds,
            cost_estimate: response.cost_estimate,
            status: match response.status {
                crate::core::video_gen::VideoStatus::Completed => "completed".to_string(),
                crate::core::video_gen::VideoStatus::Pending => "pending".to_string(),
                crate::core::video_gen::VideoStatus::Processing => "processing".to_string(),
                crate::core::video_gen::VideoStatus::Failed(msg) => format!("failed: {}", msg),
            },
        })
    }
    #[cfg(not(feature = "server"))]
    {
        Err(ServerFnError::new("Video generation is only available in server mode."))
    }
}

#[server]
pub async fn estimate_video_cost(form: VideoGenForm) -> Result<f64, ServerFnError> {
    #[cfg(feature = "server")]
    {
        let generator = VIDEO_GENERATOR.lock().await;

        let request = VideoRequest::new(form.prompt)
            .with_model(form.model)
            .with_provider(form.provider)
            .with_config(VideoConfig {
                width: form.width,
                height: form.height,
                duration_seconds: form.duration_seconds,
                fps: form.fps,
                quality: form.quality,
                style: None,
            });

        let cost = generator.estimate_cost(&request);
        Ok(cost)
    }
    #[cfg(not(feature = "server"))]
    {
         // Provide a basic cost estimate (based on model type) for client-side fallback (if needed)
         // But usually this function is called from client to server.
         // If called on client accidentally without server, return 0 or error.
         // Actually, let's keep the client-side estimation logic if it was intended to run locally?
         // No, #[server] means it's an API call. If we want client-side logic, it should be a normal function, not #[server].
         // The previous code had a client-side impl for `estimate_video_cost` marked as `#[server]`.
         // This implies it was mocking the server call?
         // If I keep it #[server], the body on client is the network call.
         // The body on server is the implementation.
         // SO: The logic I am writing here IS the server implementation.
         // The macro generates the client stub automatically.
         // I don't need a `cfg(not(server))` block unless I want to override what happens on the server when the feature is disabled.
         
       Err(ServerFnError::new("Server feature disabled"))
    }
}

// Common server functions (do not need core module)
#[server]
// Common server functions (do not need core module)
#[server]
pub async fn get_available_video_providers() -> Result<Vec<VideoProviderInfo>, ServerFnError> {
    let mut providers = vec![
        VideoProviderInfo {
            provider: VideoProvider::ByteDance,
            name: "ByteDance".to_string(),
            models: vec![
                ("Jimeng V2".to_string(), VideoModel::JimengV2),
                ("Jimeng V1".to_string(), VideoModel::JimengV1),
                ("Doubao Video".to_string(), VideoModel::DoubaoVideo),
            ],
            description: "Best value, excellent for Chinese content".to_string(),
        },
        VideoProviderInfo {
            provider: VideoProvider::Alibaba,
            name: "Alibaba".to_string(),
            models: vec![
                ("Tongyi Wanxiang".to_string(), VideoModel::TongyiWanxiang),
                ("Ali VGen".to_string(), VideoModel::AliVGen),
            ],
            description: "Stable and reliable enterprise service".to_string(),
        },
        VideoProviderInfo {
            provider: VideoProvider::OpenRouter,
            name: "OpenRouter".to_string(),
            models: vec![
                ("Pika 2.0".to_string(), VideoModel::Pika2),
                ("Stable Video".to_string(), VideoModel::StableVideoDiffusion),
                ("Gen-2".to_string(), VideoModel::Gen2),
            ],
            description: "International models, better for English".to_string(),
        },
        VideoProviderInfo {
            provider: VideoProvider::Baidu,
            name: "Baidu".to_string(),
            models: vec![
                ("Ernie Video".to_string(), VideoModel::ErnieVideo),
                ("Paddle Video".to_string(), VideoModel::PaddlePaddleVideo),
            ],
            description: "Mature technology from a leading AI company".to_string(),
        },
    ];

    #[cfg(feature = "server")]
    {
        // Add Local Models
        // We reuse the model manager to list downloaded models
        let local_models = crate::models::get_available_models();
        {
             let mut local_video_models = Vec::new();
             
             for model in local_models {
                 // Basic heuristic to identify likely video models
                 let id_lower = model.id.to_lowercase();
                 if id_lower.contains("video") 
                    || id_lower.contains("motion") 
                    || id_lower.contains("animate") 
                    || id_lower.contains("svd") 
                    || id_lower.contains("zeroscope")
                 {
                     local_video_models.push((model.id, VideoModel::LocalVideo));
                 }
             }

             if !local_video_models.is_empty() {
                 providers.push(VideoProviderInfo {
                     provider: VideoProvider::Local,
                     name: "Local Machine".to_string(),
                     models: local_video_models,
                     description: "Run on your own hardware (Requires capable GPU)".to_string(),
                 });
             }
        }
    }

    Ok(providers)
}

// 检查API配置状态
#[server]
pub async fn check_video_api_configs() -> Result<Vec<ProviderConfigStatus>, ServerFnError> {
    let mut statuses = Vec::new();

    // 检查各个厂商的API密钥配置
    let providers = vec![
        (VideoProvider::ByteDance, "BYTEDANCE_API_KEY", "字节跳动"),
        (VideoProvider::Alibaba, "DASHSCOPE_API_KEY", "阿里巴巴"),
        (VideoProvider::OpenRouter, "OPENROUTER_API_KEY", "OpenRouter"),
        (VideoProvider::Baidu, "BAIDU_API_KEY", "百度"),
        (VideoProvider::Tencent, "TENCENT_SECRET_ID", "腾讯"),
    ];

    for (provider, env_key, display_name) in providers {
        let is_configured = std::env::var(env_key).is_ok();
        statuses.push(ProviderConfigStatus {
            provider,
            name: display_name.to_string(),
            is_configured,
            env_key: env_key.to_string(),
        });
    }

    Ok(statuses)
}

// 获取视频生成任务状态
#[server]
pub async fn get_video_generation_status(task_id: String) -> Result<VideoTaskStatus, ServerFnError> {
    Ok(VideoTaskStatus {
        task_id,
        status: "completed".to_string(),
        progress: 100,
        video_url: None,
        error: None,
    })
}