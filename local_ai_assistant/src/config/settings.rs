//! Application Settings

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub server: ServerSettings,
    pub llm: LlmSettings,
    pub embedding: EmbeddingSettings,
    pub rag: RagSettings,
    pub database: DatabaseSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmSettings {
    pub model_path: String,
    pub context_size: u32,
    pub temperature: f32,
    pub top_p: f32,
    pub max_tokens: u32,
    pub gpu_layers: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingSettings {
    pub model_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RagSettings {
    pub knowledge_dir: String,
    pub chunk_size: usize,
    pub chunk_overlap: usize,
    pub top_k: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseSettings {
    pub path: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            server: ServerSettings {
                host: "127.0.0.1".to_string(),
                port: 8080,
            },
            llm: LlmSettings {
                model_path: "./models/qwen2.5-7b-instruct-q4_k_m.gguf".to_string(),
                context_size: 4096,
                temperature: 0.7,
                top_p: 0.9,
                max_tokens: 2048,
                gpu_layers: 0,
            },
            embedding: EmbeddingSettings {
                model_name: "BAAI/bge-small-en-v1.5".to_string(),
            },
            rag: RagSettings {
                knowledge_dir: "./context".to_string(),
                chunk_size: 512,
                chunk_overlap: 50,
                top_k: 5,
            },
            database: DatabaseSettings {
                path: "./data/assistant.db".to_string(),
            },
        }
    }
}

impl Settings {
    /// Load settings from config file
    pub fn load() -> Result<Self, config::ConfigError> {
        let settings = config::Config::builder()
            .add_source(config::File::with_name("config/default").required(false))
            .add_source(config::Environment::with_prefix("APP"))
            .build()?;

        settings.try_deserialize()
    }
}
