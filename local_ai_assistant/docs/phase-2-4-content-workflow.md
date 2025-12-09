# Phase 2.4: Content Creator Workflow

## Overview

Content creation workflow system inspired by Floneum/Kalosm, enabling local content creation pipeline with RSS/URL sources, article templates, and AI-powered content generation.

## Features

### Content Sources
- **RSS Feeds**: Subscribe and fetch articles from RSS feeds
- **URL Extraction**: Extract article content using readability algorithm
- **Local Files**: Import txt and md files

### Article Templates
Built-in templates for multiple platforms:
- Blog (标题 + 摘要 + 正文 + 结论)
- WeChat (公众号文章格式)
- 小红书 (hook + 痛点 + 方案 + CTA)
- Twitter Thread (hook + 5 tweets)
- LinkedIn Article
- Medium Post

### Content Editor UI
Three-column layout:
```
┌─────────────────────────────────────────────────────┐
│ [Sources] [Editor] [Preview]                    [X] │
├────────────┬────────────────────────────────────────┤
│ RSS Feeds  │                                        │
│ ├ TechNews │  # Article Title                      │
│ └ AI Daily │                                        │
│            │  [Generate Outline]  [Expand]          │
│ Bookmarks  │                                        │
│ ├ Article1 │  ## Section 1                          │
│ └ Article2 │  {content}                            │
│            │                                        │
│ [+ Add]    │  ## Section 2                          │
│            │  {content}                            │
│            │                                        │
│            │  [Insert Image] [Generate Image]      │
│            │                                        │
├────────────┴────────────────────────────────────────┤
│ [Export MD] [Export HTML] [Copy] [Save Draft]       │
└─────────────────────────────────────────────────────┘
```

## Architecture

### New Files

| File | Description |
|------|-------------|
| `src/core/content_source.rs` | RSS, URL, file source management |
| `src/models/content_template.rs` | Article templates and editor content |
| `src/components/content_editor.rs` | Three-column editor UI |
| `src/server_functions/content.rs` | Server-side API functions |

### Modified Files

| File | Changes |
|------|---------|
| `src/core/mod.rs` | Export content_source module |
| `src/models/mod.rs` | Export content_template types |
| `src/components/mod.rs` | Export ContentEditorPanel |
| `src/components/app.rs` | Add ContentEditor panel variant |
| `src/components/sidebar.rs` | Add Content Editor button (orange) |
| `Cargo.toml` | Add dependencies |

## Dependencies

```toml
feed-rs = "2.0"           # RSS parsing
reqwest = { version = "0.12", features = ["blocking"] }
readability = "0.3"       # Article extraction
comrak = "0.28"           # Markdown to HTML
```

## Server Functions

```rust
// RSS feed operations
fetch_rss_entries(url: String) -> Result<Vec<FeedEntry>, ServerFnError>

// Article extraction
extract_article_content(url: String) -> Result<Article, ServerFnError>

// AI-powered generation
generate_outline(topic: String, template_name: String) -> Result<Vec<EditorSection>, ServerFnError>
expand_section(section_title: String, context: String) -> Result<String, ServerFnError>
generate_image_prompt(text: String) -> Result<String, ServerFnError>

// Export
export_to_markdown(content: EditorContent) -> Result<String, ServerFnError>
export_to_html(content: EditorContent) -> Result<String, ServerFnError>
```

## Data Models

### Platform Enum
```rust
pub enum Platform {
    Blog,
    WeChat,
    XiaoHongShu,
    Twitter,
    LinkedIn,
    Medium,
}
```

### Writing Style Enum
```rust
pub enum WritingStyle {
    Professional,
    Casual,
    Technical,
    Marketing,
    Educational,
}
```

### Article Template
```rust
pub struct ArticleTemplate {
    pub name: String,
    pub platform: Platform,
    pub style: WritingStyle,
    pub sections: Vec<TemplateSection>,
}
```

### Editor Content
```rust
pub struct EditorContent {
    pub title: String,
    pub sections: Vec<EditorSection>,
    pub platform: Platform,
    pub style: WritingStyle,
}

pub struct EditorSection {
    pub title: String,
    pub content: String,
    pub word_limit: Option<usize>,
}
```

## Usage

1. Click the orange "Content Editor" button in the sidebar
2. Add RSS feeds or paste URLs to import content
3. Select a template for your target platform
4. Use "Generate Outline" to create article structure
5. Use "Expand" to generate content for each section
6. Preview and export as Markdown or HTML

## Future Enhancements

- Image generation integration (auto-generate images for sections)
- Draft persistence to database
- Multiple export formats (PDF, DOCX)
- Collaborative editing
- SEO optimization suggestions
