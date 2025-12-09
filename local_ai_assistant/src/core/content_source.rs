//! Content Source Management
//!
//! This module handles various content sources for the content workflow:
//! - RSS feeds
//! - Web pages (article extraction)
//! - Local files (txt, md)
//!
//! Phase 2.4: Content Workflow

use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Content source types
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ContentSourceType {
    RssFeed,
    WebPage,
    LocalFile,
}

/// A content source (RSS feed, web page, or local file)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContentSource {
    pub id: String,
    pub name: String,
    pub source_type: ContentSourceType,
    pub url: Option<String>,
    pub path: Option<PathBuf>,
    pub created_at: DateTime<Utc>,
    pub last_fetched: Option<DateTime<Utc>>,
}

impl ContentSource {
    pub fn new_rss(name: &str, url: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            source_type: ContentSourceType::RssFeed,
            url: Some(url.to_string()),
            path: None,
            created_at: Utc::now(),
            last_fetched: None,
        }
    }

    pub fn new_webpage(url: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: url.to_string(),
            source_type: ContentSourceType::WebPage,
            url: Some(url.to_string()),
            path: None,
            created_at: Utc::now(),
            last_fetched: None,
        }
    }

    pub fn new_local_file(path: PathBuf) -> Self {
        let name = path.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "Unnamed File".to_string());

        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            source_type: ContentSourceType::LocalFile,
            url: None,
            path: Some(path),
            created_at: Utc::now(),
            last_fetched: None,
        }
    }
}

/// An article extracted from a content source
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Article {
    pub id: String,
    pub source_id: String,
    pub title: String,
    pub content: String,
    pub summary: Option<String>,
    pub url: Option<String>,
    pub author: Option<String>,
    pub published_at: Option<DateTime<Utc>>,
    pub fetched_at: DateTime<Utc>,
    pub word_count: usize,
}

impl Article {
    pub fn new(source_id: &str, title: &str, content: &str) -> Self {
        let word_count = content.split_whitespace().count();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            source_id: source_id.to_string(),
            title: title.to_string(),
            content: content.to_string(),
            summary: None,
            url: None,
            author: None,
            published_at: None,
            fetched_at: Utc::now(),
            word_count,
        }
    }

    pub fn with_url(mut self, url: &str) -> Self {
        self.url = Some(url.to_string());
        self
    }

    pub fn with_author(mut self, author: &str) -> Self {
        self.author = Some(author.to_string());
        self
    }

    pub fn with_summary(mut self, summary: &str) -> Self {
        self.summary = Some(summary.to_string());
        self
    }
}

/// RSS Feed entry (subset of full article for list display)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FeedEntry {
    pub id: String,
    pub title: String,
    pub url: String,
    pub summary: Option<String>,
    pub published_at: Option<DateTime<Utc>>,
}

/// Fetch and parse an RSS feed
#[cfg(feature = "server")]
pub async fn fetch_rss_feed(url: &str) -> Result<Vec<FeedEntry>, String> {
    use feed_rs::parser;

    let response = reqwest::get(url)
        .await
        .map_err(|e| format!("Failed to fetch RSS feed: {}", e))?;

    let bytes = response.bytes()
        .await
        .map_err(|e| format!("Failed to read response: {}", e))?;

    let feed = parser::parse(&bytes[..])
        .map_err(|e| format!("Failed to parse RSS feed: {}", e))?;

    let entries = feed.entries.into_iter().map(|entry| {
        FeedEntry {
            id: entry.id,
            title: entry.title.map(|t| t.content).unwrap_or_else(|| "Untitled".to_string()),
            url: entry.links.first()
                .map(|l| l.href.clone())
                .unwrap_or_default(),
            summary: entry.summary.map(|s| s.content),
            published_at: entry.published.map(|p| p.into()),
        }
    }).collect();

    Ok(entries)
}

/// Extract article content from a URL using readability
#[cfg(feature = "server")]
pub async fn extract_article(url: &str) -> Result<Article, String> {
    let response = reqwest::get(url)
        .await
        .map_err(|e| format!("Failed to fetch URL: {}", e))?;

    let html = response.text()
        .await
        .map_err(|e| format!("Failed to read response: {}", e))?;

    let readable = readability::extractor::extract(&mut html.as_bytes(), url)
        .map_err(|e| format!("Failed to extract article: {}", e))?;

    let source_id = uuid::Uuid::new_v4().to_string();
    let mut article = Article::new(&source_id, &readable.title, &readable.text);
    article.url = Some(url.to_string());

    Ok(article)
}

/// Read content from a local file
#[cfg(feature = "server")]
pub fn read_local_file(path: &PathBuf) -> Result<Article, String> {
    use std::fs;

    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    let title = path.file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "Untitled".to_string());

    let source_id = uuid::Uuid::new_v4().to_string();
    let article = Article::new(&source_id, &title, &content);

    Ok(article)
}

/// Source manager for handling multiple content sources
#[derive(Default)]
pub struct SourceManager {
    sources: Vec<ContentSource>,
}

impl SourceManager {
    pub fn new() -> Self {
        Self { sources: Vec::new() }
    }

    pub fn add_source(&mut self, source: ContentSource) {
        self.sources.push(source);
    }

    pub fn remove_source(&mut self, id: &str) {
        self.sources.retain(|s| s.id != id);
    }

    pub fn get_source(&self, id: &str) -> Option<&ContentSource> {
        self.sources.iter().find(|s| s.id == id)
    }

    pub fn list_sources(&self) -> &[ContentSource] {
        &self.sources
    }

    pub fn list_sources_by_type(&self, source_type: ContentSourceType) -> Vec<&ContentSource> {
        self.sources.iter()
            .filter(|s| s.source_type == source_type)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_rss_source() {
        let source = ContentSource::new_rss("Tech News", "https://example.com/feed.xml");
        assert_eq!(source.name, "Tech News");
        assert_eq!(source.source_type, ContentSourceType::RssFeed);
        assert!(source.url.is_some());
    }

    #[test]
    fn test_create_article() {
        let article = Article::new("source-1", "Test Article", "This is the content.");
        assert_eq!(article.title, "Test Article");
        assert_eq!(article.word_count, 4);
    }

    #[test]
    fn test_source_manager() {
        let mut manager = SourceManager::new();
        let source = ContentSource::new_rss("Feed 1", "https://example.com/feed.xml");
        let id = source.id.clone();

        manager.add_source(source);
        assert_eq!(manager.list_sources().len(), 1);

        manager.remove_source(&id);
        assert_eq!(manager.list_sources().len(), 0);
    }
}
