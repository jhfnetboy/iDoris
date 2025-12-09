//! Content Workflow Server Functions
//!
//! Server functions for Phase 2.4 Content Workflow.
//! Handles RSS fetching, article extraction, and content generation.

use dioxus::prelude::*;
use server_fn::ServerFnError;

/// Fetch RSS feed entries
/// Returns a list of (title, url, summary) tuples
#[server]
pub async fn fetch_rss_entries(url: String) -> Result<Vec<(String, String, String)>, ServerFnError> {
    use crate::core::content_source::fetch_rss_feed;

    let entries = fetch_rss_feed(&url)
        .await
        .map_err(|e| ServerFnError::new(e))?;

    Ok(entries.into_iter().map(|e| {
        (e.title, e.url, e.summary.unwrap_or_default())
    }).collect())
}

/// Extract article content from a URL
/// Returns (title, content)
#[server]
pub async fn extract_article_content(url: String) -> Result<(String, String), ServerFnError> {
    use crate::core::content_source::extract_article;

    let article = extract_article(&url)
        .await
        .map_err(|e| ServerFnError::new(e))?;

    Ok((article.title, article.content))
}

/// Generate an article outline based on title and template
/// Returns a list of (section_title, section_prompt) tuples
#[server]
pub async fn generate_outline(
    title: String,
    template_name: String,
) -> Result<Vec<(String, String)>, ServerFnError> {
    use crate::services::llm_service::get_llm_response;

    let prompt = format!(
        r#"Generate an article outline for: "{}"

Template style: {}

Create 4-6 sections with clear titles. For each section, provide a brief description of what should be covered.

Format your response as:
## Section Title 1
Brief description of what this section should cover.

## Section Title 2
Brief description of what this section should cover.

(Continue for all sections)

Only output the sections, no introduction or conclusion about the outline itself."#,
        title, template_name
    );

    let response = get_llm_response(prompt, None)
        .await
        .map_err(|e| ServerFnError::new(format!("LLM error: {:?}", e)))?;

    // Parse the response into sections
    let sections = parse_outline_response(&response);

    if sections.is_empty() {
        // Fallback to default sections
        Ok(vec![
            ("Introduction".to_string(), "Write an engaging introduction".to_string()),
            ("Background".to_string(), "Provide context and background".to_string()),
            ("Main Content".to_string(), "Elaborate on the main topic".to_string()),
            ("Conclusion".to_string(), "Summarize key points".to_string()),
        ])
    } else {
        Ok(sections)
    }
}

/// Expand a section with AI-generated content
#[server]
pub async fn expand_section(
    section_title: String,
    context: String,
) -> Result<String, ServerFnError> {
    use crate::services::llm_service::get_llm_response;

    let prompt = format!(
        r#"Write content for the section "{}" in an article titled "{}".

Requirements:
- Write 2-4 paragraphs of well-structured content
- Be informative and engaging
- Use clear, professional language
- Include specific details and examples where appropriate
- Do not include the section title in your response

Write the section content now:"#,
        section_title, context
    );

    let response = get_llm_response(prompt, None)
        .await
        .map_err(|e| ServerFnError::new(format!("LLM error: {:?}", e)))?;

    Ok(response.trim().to_string())
}

/// Generate an image prompt based on article content
#[server]
pub async fn generate_image_prompt(text: String) -> Result<String, ServerFnError> {
    use crate::services::llm_service::get_llm_response;

    let prompt = format!(
        r#"Based on the following article content, generate a single image prompt that would create a visually appealing illustration.

Article content:
{}

Generate a concise image prompt (1-2 sentences) that describes a scene, concept, or visual that would complement this content. Focus on visual elements, style, and mood.

Image prompt:"#,
        text.chars().take(500).collect::<String>()
    );

    let response = get_llm_response(prompt, None)
        .await
        .map_err(|e| ServerFnError::new(format!("LLM error: {:?}", e)))?;

    Ok(response.trim().to_string())
}

/// Export content to markdown format
#[server]
pub async fn export_to_markdown(
    title: String,
    sections: Vec<(String, String)>,
) -> Result<String, ServerFnError> {
    let mut md = format!("# {}\n\n", title);

    for (section_title, content) in sections {
        md.push_str(&format!("## {}\n\n", section_title));
        md.push_str(&content);
        md.push_str("\n\n");
    }

    Ok(md)
}

/// Export content to HTML format
#[server]
pub async fn export_to_html(
    title: String,
    sections: Vec<(String, String)>,
) -> Result<String, ServerFnError> {
    use comrak::{markdown_to_html, Options};

    let mut md = format!("# {}\n\n", title);

    for (section_title, content) in sections {
        md.push_str(&format!("## {}\n\n", section_title));
        md.push_str(&content);
        md.push_str("\n\n");
    }

    let html = markdown_to_html(&md, &Options::default());

    Ok(html)
}

/// Parse the LLM response into section tuples
fn parse_outline_response(response: &str) -> Vec<(String, String)> {
    let mut sections = Vec::new();
    let mut current_title: Option<String> = None;
    let mut current_content = String::new();

    for line in response.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("## ") {
            // Save previous section
            if let Some(title) = current_title.take() {
                sections.push((title, current_content.trim().to_string()));
                current_content.clear();
            }
            // Start new section
            current_title = Some(trimmed[3..].trim().to_string());
        } else if current_title.is_some() && !trimmed.is_empty() {
            if !current_content.is_empty() {
                current_content.push(' ');
            }
            current_content.push_str(trimmed);
        }
    }

    // Save last section
    if let Some(title) = current_title {
        sections.push((title, current_content.trim().to_string()));
    }

    sections
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_outline_response() {
        let response = r#"## Introduction
This section introduces the topic.

## Main Content
This section covers the main points.

## Conclusion
This section wraps up."#;

        let sections = parse_outline_response(response);
        assert_eq!(sections.len(), 3);
        assert_eq!(sections[0].0, "Introduction");
        assert_eq!(sections[1].0, "Main Content");
        assert_eq!(sections[2].0, "Conclusion");
    }
}
