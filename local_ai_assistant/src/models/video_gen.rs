// 视频生成相关的共享类型定义
// 这些类型在 web 和 server 端都需要使用

use serde::{Deserialize, Serialize};

// Video generation providers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum VideoProvider {
    OpenRouter,
    Together,
    Replicate,
    HuggingFace,
    // 国内厂商
    ByteDance,    // 豆包/即梦
    Alibaba,      // 阿里通义
    Baidu,        // 百度
    Tencent,      // 腾讯混元
}

// Video generation models
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Copy)]
pub enum VideoModel {
    // OpenRouter models
    Pika2,
    Gen2,
    StableVideoDiffusion,
    OpenRouterPro,

    // Together.ai models
    StableVideo,
    OpenSora,

    // Replicate models
    Zeroscope,
    StableVideoTurbo,

    // ByteDance models
    JimengV1,       // 即梦 1.0
    JimengV2,       // 即梦 2.0
    DoubaoVideo,    // 豆包视频生成

    // Alibaba models
    TongyiWanxiang, // 通义万象视频
    AliVGen,        // 阿里视频生成

    // Baidu models
    ErnieVideo,     // 文心一言视频
    PaddlePaddleVideo,

    // Tencent models
    HunyuanVideo,   // 混元视频
}

// Video quality settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoConfig {
    pub width: u32,
    pub height: u32,
    pub duration_seconds: u32,
    pub fps: u8,
    pub quality: VideoQuality,
    pub style: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Copy)]
pub enum VideoQuality {
    Standard,  // 480p
    HD,        // 720p
    Premium,   // 1080p+
}

impl Default for VideoConfig {
    fn default() -> Self {
        Self {
            width: 1024,
            height: 576,  // 16:9 aspect ratio
            duration_seconds: 5,
            fps: 24,
            quality: VideoQuality::HD,
            style: None,
        }
    }
}

impl Default for VideoQuality {
    fn default() -> Self {
        VideoQuality::HD
    }
}