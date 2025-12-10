// Video generation module using external API services
// Supports OpenRouter, Together.ai, and other providers

use std::time::Duration;
use crate::models::{VideoProvider, VideoModel, VideoConfig, VideoQuality};

// Video generation request
#[derive(Debug, Clone)]
pub struct VideoRequest {
    pub prompt: String,
    pub config: VideoConfig,
    pub model: VideoModel,
    pub provider: VideoProvider,
    pub negative_prompt: Option<String>,
    pub seed: Option<u32>,
}

// Video generation response
#[derive(Debug, Clone)]
pub struct VideoResponse {
    pub video_url: String,
    pub thumbnail_url: Option<String>,
    pub generation_id: String,
    pub duration_seconds: u32,
    pub cost_estimate: f64,  // in USD
    pub status: VideoStatus,
}

#[derive(Debug, Clone)]
pub enum VideoStatus {
    Pending,
    Processing,
    Completed,
    Failed(String),
}

// Provider-specific configurations
#[derive(Debug, Clone)]
pub struct ProviderConfig {
    pub api_key: String,
    pub base_url: String,
    pub timeout: Duration,
}

impl VideoRequest {
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            config: VideoConfig::default(),
            model: VideoModel::Pika2,
            provider: VideoProvider::ByteDance,
            negative_prompt: None,
            seed: None,
        }
    }

    pub fn with_model(mut self, model: VideoModel) -> Self {
        self.model = model;
        self
    }

    pub fn with_provider(mut self, provider: VideoProvider) -> Self {
        self.provider = provider;
        self
    }

    pub fn with_config(mut self, config: VideoConfig) -> Self {
        self.config = config;
        self
    }

    pub fn with_duration(mut self, seconds: u32) -> Self {
        self.config.duration_seconds = seconds;
        self
    }

    pub fn with_quality(mut self, quality: VideoQuality) -> Self {
        self.config.quality = quality;
        self
    }
}

// Cost estimation for different models and quality levels
impl VideoModel {
    pub fn get_cost_per_second(&self, quality: &VideoQuality) -> f64 {
        match (self, quality) {
            // OpenRouter pricing
            (VideoModel::StableVideoDiffusion, VideoQuality::Standard) => 0.01,
            (VideoModel::StableVideoDiffusion, VideoQuality::HD) => 0.02,
            (VideoModel::StableVideoDiffusion, VideoQuality::Premium) => 0.03,

            (VideoModel::Pika2, VideoQuality::Standard) => 0.02,
            (VideoModel::Pika2, VideoQuality::HD) => 0.03,
            (VideoModel::Pika2, VideoQuality::Premium) => 0.05,

            (VideoModel::Gen2, VideoQuality::Standard) => 0.03,
            (VideoModel::Gen2, VideoQuality::HD) => 0.04,
            (VideoModel::Gen2, VideoQuality::Premium) => 0.06,

            (VideoModel::OpenRouterPro, VideoQuality::Standard) => 0.03,
            (VideoModel::OpenRouterPro, VideoQuality::HD) => 0.04,
            (VideoModel::OpenRouterPro, VideoQuality::Premium) => 0.05,

            // Together.ai pricing (generally cheaper)
            (VideoModel::StableVideo, _) => 0.015,
            (VideoModel::OpenSora, _) => 0.02,

            // Replicate pricing (varies by model)
            (VideoModel::Zeroscope, _) => 0.025,
            (VideoModel::StableVideoTurbo, _) => 0.01,

            // 国内厂商定价 (RMB/秒，约等于美元的1/7)
            // ByteDance 即梦/豆包 (性价比高)
            (VideoModel::JimengV1, VideoQuality::Standard) => 0.007,  // ~0.05 RMB
            (VideoModel::JimengV1, VideoQuality::HD) => 0.014,        // ~0.10 RMB
            (VideoModel::JimengV2, VideoQuality::Standard) => 0.010,  // ~0.07 RMB
            (VideoModel::JimengV2, VideoQuality::HD) => 0.021,        // ~0.15 RMB
            (VideoModel::DoubaoVideo, _) => 0.017,                    // ~0.12 RMB

            // Alibaba 通义万象
            (VideoModel::TongyiWanxiang, VideoQuality::Standard) => 0.008,  // ~0.06 RMB
            (VideoModel::TongyiWanxiang, VideoQuality::HD) => 0.014,        // ~0.10 RMB
            (VideoModel::AliVGen, _) => 0.020,                            // ~0.14 RMB

            // Baidu 文心一言
            (VideoModel::ErnieVideo, VideoQuality::Standard) => 0.009,  // ~0.06 RMB
            (VideoModel::ErnieVideo, VideoQuality::HD) => 0.015,        // ~0.11 RMB
            (VideoModel::PaddlePaddleVideo, _) => 0.005,                // ~0.035 RMB

            // Tencent 混元
            (VideoModel::HunyuanVideo, VideoQuality::Standard) => 0.012, // ~0.08 RMB
            (VideoModel::HunyuanVideo, VideoQuality::HD) => 0.019,       // ~0.13 RMB

            // Default pricing for uncovered combinations
            (VideoModel::JimengV1, VideoQuality::Premium) => 0.028,    // ~0.20 RMB
            (VideoModel::JimengV2, VideoQuality::Premium) => 0.035,    // ~0.25 RMB
            (VideoModel::TongyiWanxiang, VideoQuality::Premium) => 0.021, // ~0.15 RMB
            (VideoModel::ErnieVideo, VideoQuality::Premium) => 0.022,   // ~0.16 RMB
            (VideoModel::AliVGen, VideoQuality::Premium) => 0.030,      // ~0.21 RMB
            (VideoModel::HunyuanVideo, VideoQuality::Premium) => 0.025, // ~0.18 RMB

            // Default values
            _ => 0.02, // Default to 2 cents per second
        }
    }
}

pub struct VideoGenerator {
    configs: std::collections::HashMap<VideoProvider, ProviderConfig>,
}

impl VideoGenerator {
    pub fn new() -> Self {
        let mut configs = std::collections::HashMap::new();

        // Default OpenRouter config
        configs.insert(VideoProvider::OpenRouter, ProviderConfig {
            api_key: std::env::var("OPENROUTER_API_KEY").unwrap_or_default(),
            base_url: "https://openrouter.ai/api/v1".to_string(),
            timeout: Duration::from_secs(300), // 5 minutes
        });

        // Default Together config
        configs.insert(VideoProvider::Together, ProviderConfig {
            api_key: std::env::var("TOGETHER_API_KEY").unwrap_or_default(),
            base_url: "https://api.together.xyz/v1".to_string(),
            timeout: Duration::from_secs(240),
        });

        // Default Replicate config
        configs.insert(VideoProvider::Replicate, ProviderConfig {
            api_key: std::env::var("REPLICATE_API_TOKEN").unwrap_or_default(),
            base_url: "https://api.replicate.com/v1".to_string(),
            timeout: Duration::from_secs(360),
        });

        // 国内厂商配置
        configs.insert(VideoProvider::ByteDance, ProviderConfig {
            api_key: std::env::var("BYTEDANCE_API_KEY").unwrap_or_default(),
            base_url: "https://ark.cn-beijing.volces.com/api/v3".to_string(),
            timeout: Duration::from_secs(180),
        });

        configs.insert(VideoProvider::Alibaba, ProviderConfig {
            api_key: std::env::var("DASHSCOPE_API_KEY").unwrap_or_default(),
            base_url: "https://dashscope.aliyuncs.com/api/v1".to_string(),
            timeout: Duration::from_secs(240),
        });

        configs.insert(VideoProvider::Baidu, ProviderConfig {
            api_key: std::env::var("BAIDU_API_KEY").unwrap_or_default(),
            base_url: "https://aip.baidubce.com/rpc/2.0/ai_custom/v1".to_string(),
            timeout: Duration::from_secs(200),
        });

        configs.insert(VideoProvider::Tencent, ProviderConfig {
            api_key: std::env::var("TENCENT_SECRET_ID").unwrap_or_default(),
            base_url: "https://hunyuan.tencentcloudapi.com".to_string(),
            timeout: Duration::from_secs(300),
        });

        Self { configs }
    }

    pub fn add_provider_config(&mut self, provider: VideoProvider, config: ProviderConfig) {
        self.configs.insert(provider, config);
    }

    pub fn estimate_cost(&self, request: &VideoRequest) -> f64 {
        let cost_per_second = request.model.get_cost_per_second(&request.config.quality);
        cost_per_second * request.config.duration_seconds as f64
    }

    pub async fn generate_video(&self, request: VideoRequest) -> Result<VideoResponse, anyhow::Error> {
        let cost_estimate = self.estimate_cost(&request);

        match request.provider {
            VideoProvider::OpenRouter => self.generate_with_openrouter(request, cost_estimate).await,
            VideoProvider::Together => self.generate_with_together(request, cost_estimate).await,
            VideoProvider::Replicate => self.generate_with_replicate(request, cost_estimate).await,
            VideoProvider::ByteDance => self.generate_with_bytedance(request, cost_estimate).await,
            VideoProvider::Alibaba => self.generate_with_alibaba(request, cost_estimate).await,
            VideoProvider::Baidu => self.generate_with_baidu(request, cost_estimate).await,
            VideoProvider::Tencent => self.generate_with_tencent(request, cost_estimate).await,
            VideoProvider::HuggingFace => self.generate_with_huggingface(request, cost_estimate).await,
        }
    }

    async fn generate_with_openrouter(&self, request: VideoRequest, cost_estimate: f64) -> Result<VideoResponse, anyhow::Error> {
        let config = self.configs.get(&VideoProvider::OpenRouter)
            .ok_or_else(|| anyhow::anyhow!("OpenRouter config not found"))?;

        if config.api_key.is_empty() {
            return Err(anyhow::anyhow!("OpenRouter API key not configured"));
        }

        // Prepare OpenRouter API request
        let mut api_request = serde_json::json!({
            "model": self.get_openrouter_model_name(&request.model),
            "prompt": request.prompt,
            "width": request.config.width,
            "height": request.config.height,
            "duration_seconds": request.config.duration_seconds,
            "quality": self.get_quality_string(&request.config.quality),
        });

        // Add optional parameters
        if let Some(negative_prompt) = &request.negative_prompt {
            api_request["negative_prompt"] = serde_json::Value::String(negative_prompt.clone());
        }
        if let Some(seed) = request.seed {
            api_request["seed"] = serde_json::Value::Number(seed.into());
        }

        let client = reqwest::Client::new();
        let response = client
            .post(&format!("{}/video/generations", config.base_url))
            .header("Authorization", format!("Bearer {}", config.api_key))
            .header("HTTP-Referer", "https://idoris.local")
            .header("X-Title", "iDoris Content Creator")
            .json(&api_request)
            .timeout(config.timeout)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("OpenRouter API error: {}", error_text));
        }

        let api_response: serde_json::Value = response.json().await?;

        // Parse response
        Ok(VideoResponse {
            video_url: api_response["data"][0]["url"].as_str().unwrap_or("").to_string(),
            thumbnail_url: api_response["data"][0]["thumbnail_url"].as_str().map(|s| s.to_string()),
            generation_id: api_response["id"].as_str().unwrap_or("").to_string(),
            duration_seconds: request.config.duration_seconds,
            cost_estimate,
            status: VideoStatus::Completed,
        })
    }

    async fn generate_with_together(&self, request: VideoRequest, cost_estimate: f64) -> Result<VideoResponse, anyhow::Error> {
        // Implementation for Together.ai
        todo!("Together.ai implementation pending")
    }

    async fn generate_with_replicate(&self, request: VideoRequest, cost_estimate: f64) -> Result<VideoResponse, anyhow::Error> {
        // Implementation for Replicate
        todo!("Replicate implementation pending")
    }

    async fn generate_with_bytedance(&self, request: VideoRequest, cost_estimate: f64) -> Result<VideoResponse, anyhow::Error> {
        let config = self.configs.get(&VideoProvider::ByteDance)
            .ok_or_else(|| anyhow::anyhow!("ByteDance config not found"))?;

        if config.api_key.is_empty() {
            return Err(anyhow::anyhow!("ByteDance API key not configured"));
        }

        // 即梦/豆包 API 请求
        let mut api_request = serde_json::json!({
            "model": self.get_bytedance_model_name(&request.model),
            "input": {
                "prompt": request.prompt,
                "duration": request.config.duration_seconds,
                "width": request.config.width,
                "height": request.config.height,
                "fps": request.config.fps,
            }
        });

        // 添加负面提示词
        if let Some(negative_prompt) = &request.negative_prompt {
            api_request["input"]["negative_prompt"] = serde_json::Value::String(negative_prompt.clone());
        }

        let client = reqwest::Client::new();
        let response = client
            .post(&format!("{}/video/generations", config.base_url))
            .header("Authorization", format!("Bearer {}", config.api_key))
            .header("Content-Type", "application/json")
            .json(&api_request)
            .timeout(config.timeout)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("ByteDance API error: {}", error_text));
        }

        let api_response: serde_json::Value = response.json().await?;

        Ok(VideoResponse {
            video_url: api_response["output"]["video_url"].as_str().unwrap_or("").to_string(),
            thumbnail_url: api_response["output"]["thumbnail_url"].as_str().map(|s| s.to_string()),
            generation_id: api_response["id"].as_str().unwrap_or("").to_string(),
            duration_seconds: request.config.duration_seconds,
            cost_estimate,
            status: VideoStatus::Completed,
        })
    }

    async fn generate_with_alibaba(&self, request: VideoRequest, cost_estimate: f64) -> Result<VideoResponse, anyhow::Error> {
        let config = self.configs.get(&VideoProvider::Alibaba)
            .ok_or_else(|| anyhow::anyhow!("Alibaba config not found"))?;

        if config.api_key.is_empty() {
            return Err(anyhow::anyhow!("Alibaba DashScope API key not configured"));
        }

        // 通义万象 API 请求
        let api_request = serde_json::json!({
            "model": self.get_alibaba_model_name(&request.model),
            "input": {
                "prompt": request.prompt,
                "video_length": request.config.duration_seconds,
                "resolution": format!("{}x{}", request.config.width, request.config.height),
            }
        });

        let client = reqwest::Client::new();
        let response = client
            .post(&format!("{}/services/aigc/text2video/video-synthesis", config.base_url))
            .header("Authorization", format!("Bearer {}", config.api_key))
            .header("X-DashScope-SSE", "disable")
            .json(&api_request)
            .timeout(config.timeout)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("Alibaba API error: {}", error_text));
        }

        let api_response: serde_json::Value = response.json().await?;

        Ok(VideoResponse {
            video_url: api_response["output"]["video_url"].as_str().unwrap_or("").to_string(),
            thumbnail_url: api_response["output"]["cover_image_url"].as_str().map(|s| s.to_string()),
            generation_id: api_response["request_id"].as_str().unwrap_or("").to_string(),
            duration_seconds: request.config.duration_seconds,
            cost_estimate,
            status: VideoStatus::Completed,
        })
    }

    async fn generate_with_baidu(&self, request: VideoRequest, cost_estimate: f64) -> Result<VideoResponse, anyhow::Error> {
        // 百度文心一言视频生成实现
        let config = self.configs.get(&VideoProvider::Baidu)
            .ok_or_else(|| anyhow::anyhow!("Baidu config not found"))?;

        if config.api_key.is_empty() {
            return Err(anyhow::anyhow!("Baidu API key not configured"));
        }

        // 首先获取 access_token
        let client = reqwest::Client::new();
        let token_response = client
            .get(&format!(
                "https://aip.baidubce.com/oauth/2.0/token?grant_type=client_credentials&client_id={}",
                config.api_key
            ))
            .send()
            .await?;

        let token_data: serde_json::Value = token_response.json().await?;
        let access_token = token_data["access_token"].as_str().ok_or_else(|| anyhow::anyhow!("Failed to get access token"))?;

        // 生成视频的请求
        let api_request = serde_json::json!({
            "prompt": request.prompt,
            "video_width": request.config.width,
            "video_height": request.config.height,
            "video_duration": request.config.duration_seconds,
        });

        let response = client
            .post(&format!(
                "{}/wenxinworkshop/video/generation/v1?access_token={}",
                config.base_url, access_token
            ))
            .json(&api_request)
            .timeout(config.timeout)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("Baidu API error: {}", error_text));
        }

        let api_response: serde_json::Value = response.json().await?;

        Ok(VideoResponse {
            video_url: api_response["data"]["video_url"].as_str().unwrap_or("").to_string(),
            thumbnail_url: api_response["data"]["cover_url"].as_str().map(|s| s.to_string()),
            generation_id: api_response["log_id"].as_str().unwrap_or("").to_string(),
            duration_seconds: request.config.duration_seconds,
            cost_estimate,
            status: VideoStatus::Completed,
        })
    }

    async fn generate_with_tencent(&self, request: VideoRequest, cost_estimate: f64) -> Result<VideoResponse, anyhow::Error> {
        // 腾讯混元视频生成实现
        todo!("Tencent Hunyuan implementation pending")
    }

    async fn generate_with_huggingface(&self, request: VideoRequest, cost_estimate: f64) -> Result<VideoResponse, anyhow::Error> {
        // Hugging Face video generation implementation
        todo!("HuggingFace implementation pending")
    }

    fn get_openrouter_model_name(&self, model: &VideoModel) -> &str {
        match model {
            VideoModel::Pika2 => "pika/pika-2.0",
            VideoModel::Gen2 => "runway/gen-2",
            VideoModel::StableVideoDiffusion => "stability-ai/stable-video-diffusion",
            VideoModel::OpenRouterPro => "openrouter/video-pro",
            _ => "pika/pika-2.0", // default
        }
    }

    fn get_bytedance_model_name(&self, model: &VideoModel) -> &str {
        match model {
            VideoModel::JimengV1 => "jimeng-1.0",
            VideoModel::JimengV2 => "jimeng-2.0",
            VideoModel::DoubaoVideo => "doubao-video-gen",
            _ => "jimeng-2.0", // default to newer version
        }
    }

    fn get_alibaba_model_name(&self, model: &VideoModel) -> &str {
        match model {
            VideoModel::TongyiWanxiang => "wanx-v1",
            VideoModel::AliVGen => "ali-v-gen",
            _ => "wanx-v1", // default
        }
    }

    fn get_quality_string(&self, quality: &VideoQuality) -> &str {
        match quality {
            VideoQuality::Standard => "480p",
            VideoQuality::HD => "720p",
            VideoQuality::Premium => "1080p",
        }
    }
}

impl Default for VideoGenerator {
    fn default() -> Self {
        Self::new()
    }
}