//! Core Services Module
//!
//! Low-level services for LLM inference, embedding, vector storage, image generation, TTS, video generation, and content sources.

pub mod error;
pub mod config;
pub mod llm;
pub mod embedding;
pub mod vector_store;

#[cfg(feature = "server")]
pub mod model_manager;

#[cfg(feature = "server")]
pub mod image_gen;

#[cfg(feature = "server")]
pub mod tts;

#[cfg(feature = "server")]
pub mod video_gen;

#[cfg(feature = "server")]
pub mod content_source;

#[cfg(feature = "server")]
pub mod content_generator;

#[cfg(feature = "server")]
pub mod seo;


