// Video generation module using external API services
// Supports OpenRouter, Together.ai, and other providers

use std::time::Duration;
use std::collections::BTreeMap;
use crate::models::{VideoProvider, VideoModel, VideoConfig, VideoQuality};
use hmac::{Hmac, Mac};
use sha2::{Sha256, Digest};
use hex;

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
    pub access_key_id: String,
    pub secret_access_key: String,
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
            (VideoModel::TongyiWanxiang, VideoQuality::Premium) => 0.021,   // ~0.15 RMB
            
            // Specific AliVGen cases must come before wildcard
            (VideoModel::AliVGen, VideoQuality::Premium) => 0.030,          // ~0.21 RMB
            (VideoModel::AliVGen, _) => 0.020,                              // ~0.14 RMB

            // Baidu 文心一言
            (VideoModel::ErnieVideo, VideoQuality::Standard) => 0.009,  // ~0.06 RMB
            (VideoModel::ErnieVideo, VideoQuality::HD) => 0.015,        // ~0.11 RMB
            (VideoModel::ErnieVideo, VideoQuality::Premium) => 0.022,   // ~0.16 RMB
            (VideoModel::PaddlePaddleVideo, _) => 0.005,                // ~0.035 RMB

            // Tencent 混元
            (VideoModel::HunyuanVideo, VideoQuality::Standard) => 0.012, // ~0.08 RMB
            (VideoModel::HunyuanVideo, VideoQuality::HD) => 0.019,       // ~0.13 RMB
            (VideoModel::HunyuanVideo, VideoQuality::Premium) => 0.025, // ~0.18 RMB

            // Default pricing for uncovered combinations
            (VideoModel::JimengV1, VideoQuality::Premium) => 0.028,    // ~0.20 RMB
            (VideoModel::JimengV2, VideoQuality::Premium) => 0.035,    // ~0.25 RMB
            
            // Local video is free (running on hardware)
            (VideoModel::LocalVideo, _) => 0.0,

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
            access_key_id: String::new(),
            secret_access_key: String::new(),
            base_url: "https://openrouter.ai/api/v1".to_string(),
            timeout: Duration::from_secs(300), // 5 minutes
        });

        // Default Together config
        configs.insert(VideoProvider::Together, ProviderConfig {
            api_key: std::env::var("TOGETHER_API_KEY").unwrap_or_default(),
            access_key_id: String::new(),
            secret_access_key: String::new(),
            base_url: "https://api.together.xyz/v1".to_string(),
            timeout: Duration::from_secs(240),
        });

        // Default Replicate config
        configs.insert(VideoProvider::Replicate, ProviderConfig {
            api_key: std::env::var("REPLICATE_API_TOKEN").unwrap_or_default(),
            access_key_id: String::new(),
            secret_access_key: String::new(),
            base_url: "https://api.replicate.com/v1".to_string(),
            timeout: Duration::from_secs(360),
        });

        // 国内厂商配置
        configs.insert(VideoProvider::ByteDance, ProviderConfig {
            api_key: std::env::var("BYTEDANCE_API_KEY").unwrap_or_default(),
            access_key_id: std::env::var("Access_Key_ID")
                .or_else(|_| std::env::var("JIMENG_ACCESS_KEY"))
                .or_else(|_| std::env::var("VOLC_ACCESS_KEY"))
                .unwrap_or_default(),
            secret_access_key: std::env::var("Secret_Access_Key")
                .or_else(|_| std::env::var("JIMENG_SECRET_KEY"))
                .or_else(|_| std::env::var("VOLC_SECRET_KEY"))
                .unwrap_or_default(),
            base_url: "https://ark.cn-beijing.volces.com/api/v3".to_string(), // Keep this but mostly unused for visual service
            timeout: Duration::from_secs(180),
        });

        configs.insert(VideoProvider::Alibaba, ProviderConfig {
            api_key: std::env::var("DASHSCOPE_API_KEY").unwrap_or_default(),
            access_key_id: String::new(),
            secret_access_key: String::new(),
            base_url: "https://dashscope.aliyuncs.com/api/v1".to_string(),
            timeout: Duration::from_secs(240),
        });

        configs.insert(VideoProvider::Baidu, ProviderConfig {
            api_key: std::env::var("BAIDU_API_KEY").unwrap_or_default(),
            access_key_id: String::new(),
            secret_access_key: String::new(),
            base_url: "https://aip.baidubce.com/rpc/2.0/ai_custom/v1".to_string(),
            timeout: Duration::from_secs(200),
        });

        configs.insert(VideoProvider::Tencent, ProviderConfig {
            api_key: std::env::var("TENCENT_SECRET_ID").unwrap_or_default(),
            access_key_id: String::new(),
            secret_access_key: String::new(),
            base_url: "https://hunyuan.tencentcloudapi.com".to_string(),
            timeout: Duration::from_secs(300),
        });

        // Local provider config (path based)
        configs.insert(VideoProvider::Local, ProviderConfig {
            api_key: String::new(),
            access_key_id: String::new(),
            secret_access_key: String::new(),
            base_url: "local".to_string(),
            timeout: Duration::from_secs(600),
        });
        
        // HuggingFace provider
         configs.insert(VideoProvider::HuggingFace, ProviderConfig {
            api_key: std::env::var("HF_TOKEN").unwrap_or_default(),
            access_key_id: String::new(),
            secret_access_key: String::new(),
            base_url: "https://api-inference.huggingface.co/models".to_string(),
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
            VideoProvider::Local => self.generate_with_local(request, cost_estimate).await,
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

    async fn generate_with_together(&self, _request: VideoRequest, _cost_estimate: f64) -> Result<VideoResponse, anyhow::Error> {
        // Implementation for Together.ai
        todo!("Together.ai implementation pending")
    }

    async fn generate_with_replicate(&self, _request: VideoRequest, _cost_estimate: f64) -> Result<VideoResponse, anyhow::Error> {
        // Implementation for Replicate
        todo!("Replicate implementation pending")
    }

    async fn generate_with_bytedance(&self, request: VideoRequest, cost_estimate: f64) -> Result<VideoResponse, anyhow::Error> {
        println!("Starting generate_with_bytedance...");
        let config = self.configs.get(&VideoProvider::ByteDance)
            .ok_or_else(|| anyhow::anyhow!("ByteDance config not found"))?;

        if config.access_key_id.is_empty() || config.secret_access_key.is_empty() {
            println!("Error: ByteDance keys missing");
            return Err(anyhow::anyhow!("ByteDance Access Key ID or Secret Access Key not configured. Please set Access_Key_ID and Secret_Access_Key in .env file."));
        }

        let client = reqwest::Client::new();
        let region = "cn-north-1";
        let service = "cv";
        let host = "visual.volcengineapi.com";
        let path = "/";
        let version = "2022-08-31";

        // 1. Submit Task
        let action = "CVSync2AsyncSubmitTask";
        // Ensure query params are sorted: Action comes before Version
        let query = format!("Action={}&Version={}", action, version);
        
        let now = chrono::Utc::now();
        let date_iso = now.format("%Y%m%dT%H%M%SZ").to_string();
        
        let seed = match request.seed {
            Some(s) => s as i64,
            None => -1,
        };

        // Construct Body
        let req_body = serde_json::json!({
            "req_key": "jimeng_t2v_v30_1080p", // Video 3.0
            "prompt": request.prompt,
            "seed": seed,
            "frames": 121, // Default to 5s (24*5+1)
            "aspect_ratio": "16:9" // Default
        });
        let payload = req_body.to_string();
        println!("Request Payload: {}", payload);

        // Debug: Print keys (masked)
        println!("Using AccessKey: {}...", &config.access_key_id.chars().take(4).collect::<String>());

        // Prepare headers for signature
        // NOTE: We do NOT include X-Content-Sha256 in the headers map passed to signing, 
        // to match the reference implementation which only signs [content-type, host, x-date].
        let mut headers = BTreeMap::new();
        headers.insert("Host".to_string(), host.to_string());
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("X-Date".to_string(), date_iso.clone());
        // headers.insert("X-Content-Sha256".to_string(), volc_sha256(&payload)); // Removed to match JS reference

        let auth = sign_volc_request(
            &config.access_key_id,
            &config.secret_access_key,
            "POST",
            path,
            &query,
            &headers,
            &payload, // Payload IS used for hash calculation inside the signer
            region,
            service,
            &date_iso
        );
        println!("Generated Authorization: {}", auth);

        let submit_resp = client.post(format!("https://{}?{}", host, query))
            .header("Authorization", auth)
            .header("Content-Type", "application/json")
            .header("Host", host)
            .header("X-Date", date_iso)
            // .header("X-Content-Sha256", ...) // Not sending this header either
            .body(payload)
            .send()
            .await?;

        let status = submit_resp.status();
        println!("Submit Response Status: {}", status);

        if !status.is_success() {
            let error_text = submit_resp.text().await?;
            println!("Submit Response Error Body: {}", error_text);
            return Err(anyhow::anyhow!("ByteDance Submit Task Error: status={}, body={}", status, error_text));
        }

        let submit_data: serde_json::Value = submit_resp.json().await?;
        println!("Submit Response JSON: {:?}", submit_data);

        if submit_data["code"].as_i64().unwrap_or(0) != 10000 {
             return Err(anyhow::anyhow!("ByteDance Submit Failed: {}", submit_data["message"]));
        }

        let task_id = submit_data["data"]["task_id"].as_str()
            .ok_or_else(|| anyhow::anyhow!("Task ID not found in response"))?
            .to_string();
        println!("Task ID received: {}", task_id);

        // 2. Poll Result
        let action_poll = "CVSync2AsyncGetResult";
        let query_poll = format!("Action={}&Version={}", action_poll, version);
        
        let mut attempts = 0;
        let max_attempts = 150; 

        loop {
            if attempts >= max_attempts {
                return Err(anyhow::anyhow!("Video generation timed out"));
            }
            attempts += 1;
            tokio::time::sleep(Duration::from_secs(2)).await;

            let now_poll = chrono::Utc::now();
            let date_iso_poll = now_poll.format("%Y%m%dT%H%M%SZ").to_string();
            
            let poll_body = serde_json::json!({
                "req_key": "jimeng_t2v_v30_1080p",
                "task_id": task_id
            });
            let payload_poll = poll_body.to_string();

            let mut headers_poll = BTreeMap::new();
            headers_poll.insert("Host".to_string(), host.to_string());
            headers_poll.insert("Content-Type".to_string(), "application/json".to_string());
            headers_poll.insert("X-Date".to_string(), date_iso_poll.clone());

            let auth_poll = sign_volc_request(
                &config.access_key_id,
                &config.secret_access_key,
                "POST",
                path,
                &query_poll,
                &headers_poll,
                &payload_poll,
                region,
                service,
                &date_iso_poll
            );

            let poll_resp = client.post(format!("https://{}?{}", host, query_poll))
                .header("Authorization", auth_poll)
                .header("Content-Type", "application/json")
                .header("Host", host)
                .header("X-Date", date_iso_poll)
                .body(payload_poll)
                .send()
                .await;

            match poll_resp {
                Ok(resp) => {
                    if !resp.status().is_success() {
                        let err = resp.text().await.unwrap_or_default();
                        println!("Poll Error ({}): {}", attempts, err);
                        continue; 
                    }

                    match resp.json::<serde_json::Value>().await {
                        Ok(data) => {
                             // println!("Poll Data: {:?}", data); // Verbose, maybe comment out later
                             if data["code"].as_i64().unwrap_or(0) == 10000 {
                                 let status = data["data"]["status"].as_str().unwrap_or("unknown");
                                 println!("Poll Status: {}", status);
                                 if status == "done" || status == "success" {
                                     let video_url = data["data"]["video_url"].as_str().unwrap_or("").to_string();
                                     return Ok(VideoResponse {
                                         video_url,
                                         thumbnail_url: None, 
                                         generation_id: task_id,
                                         duration_seconds: request.config.duration_seconds,
                                         cost_estimate,
                                         status: VideoStatus::Completed,
                                     });
                                 } else if status == "failed" || status == "error" {
                                      return Err(anyhow::anyhow!("Generation failed: status={}", status));
                                 }
                             }
                        },
                        Err(e) => println!("Poll JSON parse error: {}", e),
                    }
                },
                Err(e) => println!("Poll Request error: {}", e),
            }
        }
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

    async fn generate_with_tencent(&self, _request: VideoRequest, _cost_estimate: f64) -> Result<VideoResponse, anyhow::Error> {
        // 腾讯混元视频生成实现
        todo!("Tencent Hunyuan implementation pending")
    }

    async fn generate_with_huggingface(&self, _request: VideoRequest, _cost_estimate: f64) -> Result<VideoResponse, anyhow::Error> {
        // Hugging Face video generation implementation
        todo!("HuggingFace implementation pending")
    }

    async fn generate_with_local(&self, request: VideoRequest, _cost_estimate: f64) -> Result<VideoResponse, anyhow::Error> {
        // Local generation would invoke python script or internal model
        Ok(VideoResponse {
            video_url: "placeholder_local_url.mp4".to_string(),
            thumbnail_url: None,
            generation_id: format!("local_{}", chrono::Utc::now().timestamp()),
            duration_seconds: request.config.duration_seconds,
            cost_estimate: 0.0,
            status: VideoStatus::Completed, // Mocking for now as we don't have the full local pipeline yet
        })
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

// Helper functions for Volcengine Signature
fn volc_hmac_sha256(key: &[u8], data: &str) -> Vec<u8> {
    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC can take key of any size");
    mac.update(data.as_bytes());
    mac.finalize().into_bytes().to_vec()
}

fn volc_sha256(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    hex::encode(hasher.finalize())
}

fn sign_volc_request(
    access_key_id: &str,
    secret_access_key: &str,
    method: &str,
    path: &str,
    query: &str,
    headers: &BTreeMap<String, String>,
    payload: &str,
    region: &str,
    service: &str,
    date_iso: &str, // YYYYMMDDTHHMMSSZ
) -> String {
    let date_short = &date_iso[0..8]; // YYYYMMDD

    // 1. Canonical Param Strings
    // Headers need to be lowercased and sorted
    let mut sorted_headers = BTreeMap::new();
    for (k, v) in headers {
        sorted_headers.insert(k.to_lowercase(), v.trim().to_string());
    }

    let signed_headers_str = sorted_headers.keys()
        .map(|k| k.as_str())
        .collect::<Vec<&str>>()
        .join(";");

    let mut canonical_headers_str = String::new();
    for (k, v) in &sorted_headers {
        canonical_headers_str.push_str(&format!("{}:{}\n", k, v));
    }

    let payload_hash = volc_sha256(payload);
    
    // 2. Canonical Request
    let canonical_request = format!(
        "{}\n{}\n{}\n{}\n{}\n{}",
        method,
        path,
        query,
        canonical_headers_str,
        signed_headers_str,
        payload_hash
    );

    // 3. String to Sign
    let credential_scope = format!("{}/{}/{}/request", date_short, region, service);
    let string_to_sign = format!(
        "HMAC-SHA256\n{}\n{}\n{}",
        date_iso,
        credential_scope,
        volc_sha256(&canonical_request)
    );

    // 4. Calculate Signature
    // kSecret = Secret Access Key
    // kDate = HMAC(kSecret, Date)
    // kRegion = HMAC(kDate, Region)
    // kService = HMAC(kRegion, Service)
    // kSigning = HMAC(kService, "request")
    let k_date = volc_hmac_sha256(secret_access_key.as_bytes(), date_short);
    let k_region = volc_hmac_sha256(&k_date, region);
    let k_service = volc_hmac_sha256(&k_region, service);
    let k_signing = volc_hmac_sha256(&k_service, "request");
    let signature = hex::encode(volc_hmac_sha256(&k_signing, &string_to_sign));

    // 5. Build Authorization Header
    format!(
        "HMAC-SHA256 Credential={}/{}, SignedHeaders={}, Signature={}",
        access_key_id, credential_scope, signed_headers_str, signature
    )
}