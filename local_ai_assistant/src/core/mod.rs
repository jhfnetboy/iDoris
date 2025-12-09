//! Core Services Module
//!
//! Low-level services for LLM inference, embedding, vector storage, image generation, and TTS.

pub mod llm;
pub mod embedding;
pub mod vector_store;

#[cfg(feature = "server")]
pub mod image_gen;

#[cfg(feature = "server")]
pub mod tts;
