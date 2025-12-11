//! Data Models Module

mod chat;
mod session;
mod document;
mod settings;
mod model_info;
pub mod content_template;
pub mod video_gen;
pub mod blog_post;

pub use chat::{ChatMessage, ChatRole};
pub use session::Session;
pub use document::Document;
pub use settings::{AppSettings, ResponseLanguage, Theme, FontSize};
pub use model_info::{ModelInfo, ModelStatus, ModelType, CacheInfo, get_available_models};
// Commented out unused template exports - will be used in Phase 3.2
// pub use content_template::{
//     ArticleTemplate, EditorContent, EditorSection, Platform,
//     WritingStyle, TemplateSection, get_builtin_templates,
// };
pub use video_gen::{
    VideoProvider, VideoModel, VideoConfig, VideoQuality,
};
pub use blog_post::{BlogPost, BlogSection, HeadingLevel};
pub mod content_package;
pub use content_package::{
    ContentPackage, Article, ArticleSection, ImageAsset, VideoClip,
    SocialPlatform, SeoMetadata, ExportFormat, GenerationProgress, GenerationStage,
};
