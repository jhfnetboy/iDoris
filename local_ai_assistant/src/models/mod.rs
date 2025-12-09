//! Data Models Module

mod chat;
mod session;
mod document;
mod settings;
mod model_info;
pub mod content_template;

pub use chat::{ChatMessage, ChatRole};
pub use session::Session;
pub use document::Document;
pub use settings::{AppSettings, ResponseLanguage, Theme, FontSize};
pub use model_info::{ModelInfo, ModelStatus, get_available_models};
pub use content_template::{
    ArticleTemplate, EditorContent, EditorSection, Platform,
    WritingStyle, TemplateSection, get_builtin_templates,
};
