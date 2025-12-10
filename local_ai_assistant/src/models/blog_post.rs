//! Blog Post Data Models
//!
//! Core data structures for blog post generation.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Heading level in a blog post
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum HeadingLevel {
    /// H1 - Main title
    H1,
    /// H2 - Major sections
    H2,
    /// H3 - Subsections
    H3,
}

impl HeadingLevel {
    /// Convert to markdown heading prefix
    pub fn to_markdown_prefix(&self) -> &str {
        match self {
            HeadingLevel::H1 => "#",
            HeadingLevel::H2 => "##",
            HeadingLevel::H3 => "###",
        }
    }
    
    /// Parse from markdown heading line
    pub fn from_markdown_line(line: &str) -> Option<(Self, String)> {
        let trimmed = line.trim();
        if let Some(text) = trimmed.strip_prefix("### ") {
            Some((HeadingLevel::H3, text.to_string()))
        } else if let Some(text) = trimmed.strip_prefix("## ") {
            Some((HeadingLevel::H2, text.to_string()))
        } else if let Some(text) = trimmed.strip_prefix("# ") {
            Some((HeadingLevel::H1, text.to_string()))
        } else {
            None
        }
    }
}

/// A section within a blog post
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlogSection {
    /// Unique identifier
    pub id: String,
    /// Section heading text
    pub heading: String,
    /// Heading level (H1, H2, H3)
    pub level: HeadingLevel,
    /// Section content (markdown)
    pub content: String,
    /// Optional image prompt for this section
    pub suggested_image: Option<String>,
    /// Word count of this section
    pub word_count: usize,
}

impl BlogSection {
    /// Create a new blog section
    pub fn new(heading: String, level: HeadingLevel) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            heading,
            level,
            content: String::new(),
            suggested_image: None,
            word_count: 0,
        }
    }
    
    /// Update content and recalculate word count
    pub fn set_content(&mut self, content: String) {
        self.word_count = content.split_whitespace().count();
        self.content = content;
    }
    
    /// Convert to markdown
    pub fn to_markdown(&self) -> String {
        format!(
            "{} {}\n\n{}",
            self.level.to_markdown_prefix(),
            self.heading,
            self.content
        )
    }
}

/// Complete blog post with metadata
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlogPost {
    /// Unique identifier
    pub id: String,
    /// Blog post title (H1)
    pub title: String,
    /// SEO meta description
    pub meta_description: String,
    /// SEO keywords
    pub keywords: Vec<String>,
    /// Structured outline (all sections)
    pub outline: Vec<BlogSection>,
    /// Full content as markdown
    pub content: String,
    /// Total word count
    pub word_count: usize,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

impl BlogPost {
    /// Create a new blog post from a topic
    pub fn new(title: String) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            meta_description: String::new(),
            keywords: Vec::new(),
            outline: Vec::new(),
            content: String::new(),
            word_count: 0,
            created_at: now,
            updated_at: now,
        }
    }
    
    /// Generate full markdown content from outline
    pub fn generate_content(&mut self) {
        let mut content = String::new();
        
        // Add title
        if !self.title.is_empty() {
            content.push_str(&format!("# {}\n\n", self.title));
        }
        
        // Add all sections
        for section in &self.outline {
            content.push_str(&section.to_markdown());
            content.push_str("\n\n");
        }
        
        // Calculate total word count
        self.word_count = content.split_whitespace().count();
        self.content = content.trim().to_string();
        self.updated_at = Utc::now();
    }
    
    /// Export as HTML (using comrak for markdown → HTML)
    pub fn to_html(&self) -> String {
        comrak::markdown_to_html(&self.content, &comrak::Options::default())
    }
    
    /// Get a brief summary (first 200 characters)
    pub fn summary(&self) -> String {
        let text: String = self.content
            .lines()
            .filter(|line| !line.starts_with('#'))
            .collect::<Vec<_>>()
            .join(" ");
        
        if text.len() > 200 {
            format!("{}...", &text[..200])
        } else {
            text
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heading_level_markdown() {
        assert_eq!(HeadingLevel::H1.to_markdown_prefix(), "#");
        assert_eq!(HeadingLevel::H2.to_markdown_prefix(), "##");
        assert_eq!(HeadingLevel::H3.to_markdown_prefix(), "###");
    }

    #[test]
    fn test_parse_heading() {
        let (level, text) = HeadingLevel::from_markdown_line("## Introduction").unwrap();
        assert_eq!(level, HeadingLevel::H2);
        assert_eq!(text, "Introduction");
    }

    #[test]
    fn test_blog_section() {
        let mut section = BlogSection::new("Introduction".to_string(), HeadingLevel::H2);
        section.set_content("This is a test section with multiple words.".to_string());
        
        assert_eq!(section.word_count, 8);
        assert!(section.to_markdown().contains("## Introduction"));
    }

    #[test]
    fn test_blog_post_generation() {
        let mut post = BlogPost::new("Test Article".to_string());
        
        let mut section1 = BlogSection::new("Introduction".to_string(), HeadingLevel::H2);
        section1.set_content("First section content.".to_string());
        
        let mut section2 = BlogSection::new("Conclusion".to_string(), HeadingLevel::H2);
        section2.set_content("Final section content.".to_string());
        
        post.outline = vec![section1, section2];
        post.generate_content();
        
        assert!(post.content.contains("# Test Article"));
        assert!(post.content.contains("## Introduction"));
        assert!(post.content.contains("## Conclusion"));
        assert!(post.word_count > 0);
    }
}
