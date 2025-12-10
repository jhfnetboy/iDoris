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
            prompt: "".to_string(),
            negative_prompt: None,
            duration_seconds: 5,
            width: 1024,
            height: 576,
            quality: VideoQuality::HD,
            fps: 24,
            provider: VideoProvider::ByteDance, // 默认使用即梦（性价比高）
            model: VideoModel::JimengV2,
            seed: None,
        }
    }
}

// 创建一个简化的 VideoResponse 结构体用于 web 模式
#[derive(Serialize, Deserialize, Clone)]
pub struct VideoResponse {
    pub video_url: String,
    pub thumbnail_url: Option<String>,
    pub generation_id: String,
    pub duration_seconds: u32,
    pub cost_estimate: f64,
    pub status: String,
}

// Provider信息结构
#[derive(Serialize, Deserialize, Clone)]
pub struct VideoProviderInfo {
    pub provider: VideoProvider,
    pub name: String,
    pub models: Vec<(String, VideoModel)>,
    pub description: String,
}

// Provider配置状态
#[derive(Serialize, Deserialize, Clone)]
pub struct ProviderConfigStatus {
    pub provider: VideoProvider,
    pub name: String,
    pub is_configured: bool,
    pub env_key: String,
}

// 任务状态
#[derive(Serialize, Deserialize, Clone)]
pub struct VideoTaskStatus {
    pub task_id: String,
    pub status: String,
    pub progress: u8,
    pub video_url: Option<String>,
    pub error: Option<String>,
}

// 仅在 server 特性下启用实际的视频生成功能
#[cfg(feature = "server")]
mod video_gen_impl {
    use super::*;
    use lazy_static::lazy_static;

    lazy_static::lazy_static! {
        static ref VIDEO_GENERATOR: Arc<Mutex<VideoGenerator>> = Arc::new(Mutex::new(VideoGenerator::new()));
    }

    #[server]
    pub async fn generate_video(form: VideoGenForm) -> Result<VideoResponse, ServerFnError> {
        let generator = VIDEO_GENERATOR.lock().await;

        // 构建请求
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

        // 设置负面提示词和种子
        let mut request = request;
        if let Some(negative) = form.negative_prompt {
            request.negative_prompt = Some(negative);
        }
        if let Some(seed) = form.seed {
            request.seed = Some(seed);
        }

        // 生成视频
        let response = generator.generate_video(request)
            .await
            .map_err(|e| ServerFnError::new(format!("视频生成失败: {}", e)))?;

        // 转换为简化的响应格式
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

    #[server]
    pub async fn estimate_video_cost(form: VideoGenForm) -> Result<f64, ServerFnError> {
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
}

// 为 web 模式提供空的实现
#[cfg(not(feature = "server"))]
#[server]
pub async fn generate_video(form: VideoGenForm) -> Result<VideoResponse, ServerFnError> {
    Err(ServerFnError::new("视频生成功能仅在服务器模式下可用。请使用 `dx serve --features server` 启动服务器模式。".to_string()))
}

#[cfg(not(feature = "server"))]
#[server]
pub async fn estimate_video_cost(form: VideoGenForm) -> Result<f64, ServerFnError> {
    // 提供一个基本的成本估算（基于模型类型）
    let base_cost = match form.model {
        VideoModel::JimengV2 => 0.010 * form.duration_seconds as f64,
        VideoModel::JimengV1 => 0.007 * form.duration_seconds as f64,
        VideoModel::Pika2 => 0.03 * form.duration_seconds as f64,
        _ => 0.02 * form.duration_seconds as f64,
    };

    // 根据质量调整成本
    let quality_multiplier = match form.quality {
        VideoQuality::Standard => 0.67,
        VideoQuality::HD => 1.0,
        VideoQuality::Premium => 1.5,
    };

    Ok(base_cost * quality_multiplier)
}

// 重新导出生成函数
#[cfg(feature = "server")]
pub use video_gen_impl::{generate_video, estimate_video_cost};

// 通用的服务器函数（不需要 core 模块）
#[server]
pub async fn get_available_video_providers() -> Result<Vec<VideoProviderInfo>, ServerFnError> {
    let providers = vec![
        VideoProviderInfo {
            provider: VideoProvider::ByteDance,
            name: "字节跳动".to_string(),
            models: vec![
                ("即梦V2".to_string(), VideoModel::JimengV2),
                ("即梦V1".to_string(), VideoModel::JimengV1),
                ("豆包视频".to_string(), VideoModel::DoubaoVideo),
            ],
            description: "性价比最高的选择，特别适合中文内容".to_string(),
        },
        VideoProviderInfo {
            provider: VideoProvider::Alibaba,
            name: "阿里巴巴".to_string(),
            models: vec![
                ("通义万象".to_string(), VideoModel::TongyiWanxiang),
                ("阿里VGen".to_string(), VideoModel::AliVGen),
            ],
            description: "稳定可靠，企业级服务".to_string(),
        },
        VideoProviderInfo {
            provider: VideoProvider::OpenRouter,
            name: "OpenRouter".to_string(),
            models: vec![
                ("Pika 2.0".to_string(), VideoModel::Pika2),
                ("Stable Video".to_string(), VideoModel::StableVideoDiffusion),
                ("Gen-2".to_string(), VideoModel::Gen2),
            ],
            description: "国际主流模型，英文效果更好".to_string(),
        },
        VideoProviderInfo {
            provider: VideoProvider::Baidu,
            name: "百度".to_string(),
            models: vec![
                ("文心视频".to_string(), VideoModel::ErnieVideo),
                ("飞桨视频".to_string(), VideoModel::PaddlePaddleVideo),
            ],
            description: "国内老牌AI厂商，技术成熟".to_string(),
        },
    ];

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