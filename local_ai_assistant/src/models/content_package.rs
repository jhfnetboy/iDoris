use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Represents a complete content package with article, images, videos, and SEO metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentPackage {
    pub id: String,
    pub topic: String,
    pub article: Article,
    pub header_image: Option<ImageAsset>,
    pub section_images: Vec<ImageAsset>,
    pub social_clips: Vec<VideoClip>,
    pub seo_metadata: SeoMetadata,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Article structure with sections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub title: String,
    pub subtitle: Option<String>,
    pub author: Option<String>,
    pub sections: Vec<ArticleSection>,
    pub word_count: usize,
    pub reading_time_minutes: u32,
}

/// Individual section of an article
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleSection {
    pub heading: String,
    pub content: String,
    pub order: u32,
    pub image_ref: Option<String>, // Reference to ImageAsset id
}

/// Image asset metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageAsset {
    pub id: String,
    pub prompt: String,
    pub url: Option<String>,
    pub local_path: Option<String>,
    pub provider: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub alt_text: String,
    pub created_at: DateTime<Utc>,
}

/// Video clip for social media
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoClip {
    pub id: String,
    pub prompt: String,
    pub url: Option<String>,
    pub local_path: Option<String>,
    pub provider: String,
    pub duration_seconds: Option<u32>,
    pub platform: SocialPlatform,
    pub caption: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Target social media platform
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SocialPlatform {
    Twitter,
    Instagram,
    TikTok,
    YouTube,
    LinkedIn,
    Generic,
}

/// SEO metadata for the content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeoMetadata {
    pub meta_title: String,
    pub meta_description: String,
    pub keywords: Vec<String>,
    pub canonical_url: Option<String>,
    pub og_image: Option<String>,
    pub schema_markup: Option<String>, // JSON-LD
    pub focus_keyword: Option<String>,
    pub readability_score: Option<f32>,
}

/// Export format options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportFormat {
    Markdown,
    Html,
    Json,
    WordPress,
    Medium,
}

/// Content package generation progress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationProgress {
    pub stage: GenerationStage,
    pub progress_percent: u8,
    pub current_task: String,
    pub estimated_time_remaining: Option<u32>, // seconds
}

/// Stages of content generation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GenerationStage {
    Initializing,
    GeneratingOutline,
    GeneratingArticle,
    GeneratingHeaderImage,
    GeneratingSectionImages,
    GeneratingSocialClips,
    GeneratingSeoMetadata,
    Finalizing,
    Complete,
    Failed,
}

impl ContentPackage {
    /// Create a new empty content package
    pub fn new(topic: String) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            topic,
            article: Article {
                title: String::new(),
                subtitle: None,
                author: None,
                sections: Vec::new(),
                word_count: 0,
                reading_time_minutes: 0,
            },
            header_image: None,
            section_images: Vec::new(),
            social_clips: Vec::new(),
            seo_metadata: SeoMetadata {
                meta_title: String::new(),
                meta_description: String::new(),
                keywords: Vec::new(),
                canonical_url: None,
                og_image: None,
                schema_markup: None,
                focus_keyword: None,
                readability_score: None,
            },
            created_at: now,
            updated_at: now,
        }
    }

    /// Calculate reading time based on word count (average 200 words per minute)
    pub fn update_reading_time(&mut self) {
        let words_per_minute = 200;
        self.article.word_count = self.article.sections.iter()
            .map(|s| s.content.split_whitespace().count())
            .sum();
        self.article.reading_time_minutes = 
            ((self.article.word_count as f32 / words_per_minute as f32).ceil() as u32).max(1);
    }

    /// Get total number of assets (images + videos)
    pub fn total_assets(&self) -> usize {
        let image_count = self.section_images.len() + if self.header_image.is_some() { 1 } else { 0 };
        image_count + self.social_clips.len()
    }
}

impl Article {
    /// Create a new article from title
    pub fn new(title: String) -> Self {
        Self {
            title,
            subtitle: None,
            author: None,
            sections: Vec::new(),
            word_count: 0,
            reading_time_minutes: 0,
        }
    }

    /// Add a section to the article
    pub fn add_section(&mut self, heading: String, content: String) {
        let order = self.sections.len() as u32;
        self.sections.push(ArticleSection {
            heading,
            content,
            order,
            image_ref: None,
        });
    }
}

impl ImageAsset {
    /// Create a new image asset
    pub fn new(prompt: String, provider: String, alt_text: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            prompt,
            url: None,
            local_path: None,
            provider,
            width: None,
            height: None,
            alt_text,
            created_at: Utc::now(),
        }
    }
}

impl VideoClip {
    /// Create a new video clip
    pub fn new(prompt: String, provider: String, platform: SocialPlatform) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            prompt,
            url: None,
            local_path: None,
            provider,
            duration_seconds: None,
            platform,
            caption: None,
            created_at: Utc::now(),
        }
    }
}
