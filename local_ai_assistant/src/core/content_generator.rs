use anyhow::{Result, Context};
use crate::models::{
    ContentPackage, Article, ArticleSection, ImageAsset, VideoClip,
    SocialPlatform, SeoMetadata, GenerationProgress, GenerationStage,
};
use crate::core::{llm, image_gen, video_gen};
use chrono::Utc;

/// Content generator for creating multi-modal content packages
pub struct ContentGenerator {
    llm_provider: String,
    image_provider: String,
    video_provider: String,
}

impl ContentGenerator {
    /// Create a new content generator
    pub fn new(
        llm_provider: String,
        image_provider: String,
        video_provider: String,
    ) -> Self {
        Self {
            llm_provider,
            image_provider,
            video_provider,
        }
    }

    /// Generate a complete content package from a topic
    pub async fn generate_from_topic(
        &self,
        topic: &str,
        progress_callback: Option<Box<dyn Fn(GenerationProgress) + Send + Sync>>,
    ) -> Result<ContentPackage> {
        let mut package = ContentPackage::new(topic.to_string());

        // Stage 1: Generate article outline
        self.notify_progress(&progress_callback, GenerationStage::GeneratingOutline, 10, "Creating article outline...");
        let outline = self.generate_article_outline(topic).await?;
        
        // Stage 2: Generate full article
        self.notify_progress(&progress_callback, GenerationStage::GeneratingArticle, 20, "Writing article content...");
        package.article = self.generate_article_from_outline(topic, &outline).await?;
        package.update_reading_time();

        // Stage 3: Generate header image
        self.notify_progress(&progress_callback, GenerationStage::GeneratingHeaderImage, 40, "Creating header image...");
        if let Ok(header_image) = self.generate_header_image(&package.article.title).await {
            package.header_image = Some(header_image);
        }

        // Stage 4: Generate section images
        self.notify_progress(&progress_callback, GenerationStage::GeneratingSectionImages, 55, "Creating section images...");
        package.section_images = self.generate_section_images(&package.article).await?;

        // Stage 5: Generate social media clips (optional, can fail)
        self.notify_progress(&progress_callback, GenerationStage::GeneratingSocialClips, 70, "Creating social media clips...");
        if let Ok(clips) = self.generate_social_clips(topic, &package.article).await {
            package.social_clips = clips;
        }

        // Stage 6: Generate SEO metadata
        self.notify_progress(&progress_callback, GenerationStage::GeneratingSeoMetadata, 85, "Generating SEO metadata...");
        package.seo_metadata = self.generate_seo_metadata(&package.article).await?;

        // Stage 7: Finalize
        self.notify_progress(&progress_callback, GenerationStage::Finalizing, 95, "Finalizing content package...");
        package.updated_at = Utc::now();

        self.notify_progress(&progress_callback, GenerationStage::Complete, 100, "Content package complete!");
        Ok(package)
    }

    /// Generate an article outline from a topic
    async fn generate_article_outline(&self, topic: &str) -> Result<Vec<String>> {
        let prompt = format!(
            "Create a detailed article outline for the topic: '{}'\n\n\
            Generate a list of 5-7 main section headings. Each heading should be:\n\
            - Clear and descriptive\n\
            - Follow a logical flow\n\
            - Cover different aspects of the topic\n\n\
            Return only the section headings, one per line, without numbering.",
            topic
        );

        let response = llm::generate_text(&prompt, &self.llm_provider, None)
            .await
            .context("Failed to generate article outline")?;

        let headings: Vec<String> = response
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.trim().to_string())
            .collect();

        Ok(headings)
    }

    /// Generate full article from outline
    async fn generate_article_from_outline(
        &self,
        topic: &str,
        outline: &[String],
    ) -> Result<Article> {
        let mut article = Article::new(topic.to_string());

        // Generate introduction
        let intro_prompt = format!(
            "Write an engaging introduction (150-200 words) for an article about: '{}'\n\n\
            The article will cover these sections:\n{}\n\n\
            Make it compelling and set the stage for what's to come.",
            topic,
            outline.join("\n- ")
        );

        let intro = llm::generate_text(&intro_prompt, &self.llm_provider, None)
            .await
            .context("Failed to generate introduction")?;

        article.add_section("Introduction".to_string(), intro);

        // Generate each section
        for (idx, heading) in outline.iter().enumerate() {
            let section_prompt = format!(
                "Write a detailed section (200-300 words) for the heading: '{}'\n\n\
                This is section {} of {} in an article about: '{}'\n\n\
                Previous sections covered:\n{}\n\n\
                Provide informative, engaging content with specific details.",
                heading,
                idx + 1,
                outline.len(),
                topic,
                outline[..idx].join("\n- ")
            );

            let content = llm::generate_text(&section_prompt, &self.llm_provider, None)
                .await
                .unwrap_or_else(|_| format!("Content for {}", heading));

            article.add_section(heading.clone(), content);
        }

        // Generate conclusion
        let conclusion_prompt = format!(
            "Write a strong conclusion (100-150 words) for an article about: '{}'\n\n\
            The article covered these main points:\n{}\n\n\
            Summarize the key takeaways and provide a call to action or final thought.",
            topic,
            outline.join("\n- ")
        );

        let conclusion = llm::generate_text(&conclusion_prompt, &self.llm_provider, None)
            .await
            .unwrap_or_else(|_| "Conclusion content".to_string());

        article.add_section("Conclusion".to_string(), conclusion);

        Ok(article)
    }

    /// Generate header image for the article
    async fn generate_header_image(&self, title: &str) -> Result<ImageAsset> {
        let prompt = format!(
            "A professional, eye-catching header image for an article titled: '{}'. \
            High quality, modern, visually appealing.",
            title
        );

        let alt_text = format!("Header image for: {}", title);
        let mut asset = ImageAsset::new(
            prompt.clone(),
            self.image_provider.clone(),
            alt_text,
        );

        // Generate the image
        let image_url = image_gen::generate_image(&prompt, &self.image_provider)
            .await
            .context("Failed to generate header image")?;

        asset.url = Some(image_url);
        Ok(asset)
    }

    /// Generate images for article sections
    async fn generate_section_images(&self, article: &Article) -> Result<Vec<ImageAsset>> {
        let mut images = Vec::new();

        // Generate image for key sections (skip intro/conclusion, max 3-4 images)
        let sections_to_illustrate: Vec<&ArticleSection> = article
            .sections
            .iter()
            .filter(|s| s.heading != "Introduction" && s.heading != "Conclusion")
            .take(4)
            .collect();

        for section in sections_to_illustrate {
            let prompt = format!(
                "An illustrative image for the section '{}' in an article about '{}'. \
                Professional, informative, relevant to the content.",
                section.heading, article.title
            );

            let alt_text = format!("Illustration for section: {}", section.heading);
            let mut asset = ImageAsset::new(
                prompt.clone(),
                self.image_provider.clone(),
                alt_text,
            );

            // Try to generate image, but don't fail if it doesn't work
            match image_gen::generate_image(&prompt, &self.image_provider).await {
                Ok(url) => {
                    asset.url = Some(url);
                    images.push(asset);
                }
                Err(e) => {
                    log::warn!("Failed to generate section image: {}", e);
                }
            }
        }

        Ok(images)
    }

    /// Generate social media video clips
    async fn generate_social_clips(
        &self,
        topic: &str,
        article: &Article,
    ) -> Result<Vec<VideoClip>> {
        let mut clips = Vec::new();

        // Generate 1-2 short promotional clips
        let platforms = vec![SocialPlatform::Twitter, SocialPlatform::Instagram];

        for platform in platforms {
            let prompt = format!(
                "A short, engaging {} promotional video about: '{}'. \
                Quick, attention-grabbing, suitable for social media.",
                format!("{:?}", platform).to_lowercase(),
                topic
            );

            let caption = format!(
                "Check out our latest article: {}! #content #{}",
                article.title,
                topic.replace(' ', "")
            );

            let mut clip = VideoClip::new(
                prompt.clone(),
                self.video_provider.clone(),
                platform,
            );
            clip.caption = Some(caption);

            // Try to generate video, but it's optional
            match video_gen::generate_video(&prompt, &self.video_provider).await {
                Ok(url) => {
                    clip.url = Some(url);
                    clips.push(clip);
                }
                Err(e) => {
                    log::warn!("Failed to generate social media clip: {}", e);
                }
            }
        }

        Ok(clips)
    }

    /// Generate SEO metadata for the content
    async fn generate_seo_metadata(&self, article: &Article) -> Result<SeoMetadata> {
        // Generate meta description
        let meta_description_prompt = format!(
            "Write a compelling 150-160 character meta description for an article titled: '{}'\n\n\
            First section content:\n{}\n\n\
            Make it SEO-friendly and engaging.",
            article.title,
            article.sections.first().map(|s| s.content.as_str()).unwrap_or("")
        );

        let meta_description = llm::generate_text(
            &meta_description_prompt,
            &self.llm_provider,
            None,
        )
        .await
        .unwrap_or_else(|_| article.title.clone());

        // Generate keywords
        let keywords_prompt = format!(
            "Generate 5-10 SEO keywords for an article titled: '{}'\n\n\
            Article covers:\n{}\n\n\
            Return only the keywords, comma-separated.",
            article.title,
            article.sections.iter().map(|s| &s.heading).take(5).cloned().collect::<Vec<_>>().join(", ")
        );

        let keywords_text = llm::generate_text(&keywords_prompt, &self.llm_provider, None)
            .await
            .unwrap_or_default();

        let keywords: Vec<String> = keywords_text
            .split(',')
            .map(|k| k.trim().to_string())
            .filter(|k| !k.is_empty())
            .collect();

        // Create SEO metadata
        Ok(SeoMetadata {
            meta_title: article.title.clone(),
            meta_description: meta_description.chars().take(160).collect(),
            keywords: keywords.clone(),
            canonical_url: None,
            og_image: None,
            schema_markup: Some(self.generate_schema_markup(article)),
            focus_keyword: keywords.first().cloned(),
            readability_score: None, // Will be calculated by SEO analyzer
        })
    }

    /// Generate Schema.org JSON-LD markup
    fn generate_schema_markup(&self, article: &Article) -> String {
        format!(
            r#"{{
  "@context": "https://schema.org",
  "@type": "Article",
  "headline": "{}",
  "wordCount": {},
  "datePublished": "{}",
  "author": {{
    "@type": "Person",
    "name": "{}"
  }}
}}"#,
            article.title,
            article.word_count,
            Utc::now().to_rfc3339(),
            article.author.as_deref().unwrap_or("iDoris AI Assistant")
        )
    }

    /// Notify progress callback
    fn notify_progress(
        &self,
        callback: &Option<Box<dyn Fn(GenerationProgress) + Send + Sync>>,
        stage: GenerationStage,
        progress_percent: u8,
        current_task: &str,
    ) {
        if let Some(cb) = callback {
            cb(GenerationProgress {
                stage,
                progress_percent,
                current_task: current_task.to_string(),
                estimated_time_remaining: None,
            });
        }
    }
}

/// Export content package to various formats
pub async fn export_content_package(
    package: &ContentPackage,
    format: crate::models::ExportFormat,
) -> Result<String> {
    match format {
        crate::models::ExportFormat::Markdown => export_to_markdown(package),
        crate::models::ExportFormat::Html => export_to_html(package),
        crate::models::ExportFormat::Json => export_to_json(package),
        crate::models::ExportFormat::WordPress => export_to_wordpress(package),
        crate::models::ExportFormat::Medium => export_to_medium(package),
    }
}

/// Export to Markdown format
fn export_to_markdown(package: &ContentPackage) -> Result<String> {
    let mut md = String::new();
    
    md.push_str(&format!("# {}\n\n", package.article.title));
    
    if let Some(subtitle) = &package.article.subtitle {
        md.push_str(&format!("## {}\n\n", subtitle));
    }
    
    md.push_str(&format!("*Reading time: {} min*\n\n", package.article.reading_time_minutes));
    md.push_str("---\n\n");
    
    for section in &package.article.sections {
        md.push_str(&format!("## {}\n\n", section.heading));
        md.push_str(&format!("{}\n\n", section.content));
    }
    
    md.push_str("---\n\n");
    md.push_str(&format!("**Keywords**: {}\n", package.seo_metadata.keywords.join(", ")));
    
    Ok(md)
}

/// Export to HTML format
fn export_to_html(package: &ContentPackage) -> Result<String> {
    let mut html = String::from("<!DOCTYPE html>\n<html>\n<head>\n");
    html.push_str(&format!("  <title>{}</title>\n", package.article.title));
    html.push_str(&format!("  <meta name=\"description\" content=\"{}\">\n", package.seo_metadata.meta_description));
    html.push_str(&format!("  <meta name=\"keywords\" content=\"{}\">\n", package.seo_metadata.keywords.join(", ")));
    html.push_str("</head>\n<body>\n");
    
    html.push_str(&format!("  <h1>{}</h1>\n", package.article.title));
    
    for section in &package.article.sections {
        html.push_str(&format!("  <h2>{}</h2>\n", section.heading));
        html.push_str(&format!("  <p>{}</p>\n", section.content));
    }
    
    html.push_str("</body>\n</html>");
    Ok(html)
}

/// Export to JSON format
fn export_to_json(package: &ContentPackage) -> Result<String> {
    serde_json::to_string_pretty(package)
        .context("Failed to serialize content package to JSON")
}

/// Export to WordPress format (XML-RPC compatible)
fn export_to_wordpress(package: &ContentPackage) -> Result<String> {
    // For now, return HTML suitable for WordPress
    export_to_html(package)
}

/// Export to Medium format (Markdown variant)
fn export_to_medium(package: &ContentPackage) -> Result<String> {
    // Medium uses a specific Markdown variant
    export_to_markdown(package)
}
