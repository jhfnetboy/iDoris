//! Outline generation for blog posts
//!
//! Uses AI to create structured outlines from topics.

use crate::models::{BlogSection, HeadingLevel};
use crate::core::llm::get_llm_response;

/// Generate a blog post outline from a topic
pub async fn generate_outline(topic: &str) -> Result<Vec<BlogSection>, String> {
    let prompt = format!(
        "Create a detailed blog post outline for the topic: \"{}\"\n\n\
         Instructions:\n\
         - Start with a compelling H1 title\n\
         - Include 5-7 main H2 sections\n\
         - Add 2-3 H3 subsections under key H2s\n\
         - Use clear, descriptive headings\n\
         - Format as markdown headings (# for H1, ## for H2, ### for H3)\n\
         - Only return the outline, no explanation\n\n\
         Example format:\n\
         # Main Title Here\n\
         ## Introduction\n\
         ### Background\n\
         ### Why This Matters\n\
         ## Main Section 1\n\
         ### Subsection A\n\
         ## Conclusion\n\n\
         Now create the outline:",
        topic
    );
    
    println!("🤖 Generating outline for: {}", topic);
    
    let response = get_llm_response(prompt, None)
        .await
        .map_err(|e| format!("LLM error: {}", e))?;
    
    println!("📝 LLM response received, parsing outline...");
    
    parse_outline(&response)
}

/// Parse LLM response into structured outline
fn parse_outline(text: &str) -> Result<Vec<BlogSection>, String> {
    let mut sections = Vec::new();
    
    for line in text.lines() {
        let trimmed = line.trim();
        
        // Skip empty lines
        if trimmed.is_empty() {
            continue;
        }
        
        // Try to parse as heading
        if let Some((level, heading_text)) = HeadingLevel::from_markdown_line(trimmed) {
            let section = BlogSection::new(heading_text, level);
            sections.push(section);
        }
    }
    
    // Validate outline
    if sections.is_empty() {
        return Err("No headings found in LLM response".to_string());
    }
    
    // Ensure we have at least one H1 (title)
    if !sections.iter().any(|s| matches!(s.level, HeadingLevel::H1)) {
        return Err("Outline must have an H1 title".to_string());
    }
    
    // Ensure we have some H2 sections
    let h2_count = sections.iter().filter(|s| matches!(s.level, HeadingLevel::H2)).count();
    if h2_count < 3 {
        return Err(format!("Outline should have at least 3 main sections (H2), found {}", h2_count));
    }
    
    println!("✅ Parsed {} sections ({} H2s)", sections.len(), h2_count);
    
    Ok(sections)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_outline() {
        let markdown = r#"
# The Future of AI
## Introduction
### What is AI
### Why It Matters
## Current State
## Future Trends
### Healthcare
### Education
## Conclusion
"#;
        
        let result = parse_outline(markdown);
        assert!(result.is_ok());
        
        let sections = result.unwrap();
        assert_eq!(sections.len(), 8);
        
        // Check H1
        assert!(matches!(sections[0].level, HeadingLevel::H1));
        assert_eq!(sections[0].heading, "The Future of AI");
        
        // Check H2s
        let h2_count = sections.iter().filter(|s| matches!(s.level, HeadingLevel::H2)).count();
        assert_eq!(h2_count, 4);
    }

    #[test]
    fn test_parse_no_h1() {
        let markdown = r#"
## Section 1
## Section 2
"#;
        
        let result = parse_outline(markdown);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("H1 title"));
    }

    #[test]
    fn test_parse_too_few_sections() {
        let markdown = r#"
# Title
## Section 1
## Section 2
"#;
        
        let result = parse_outline(markdown);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("at least 3 main sections"));
    }

    #[test]
    fn test_heading_level_parsing() {
        let (level, text) = HeadingLevel::from_markdown_line("## Test Heading").unwrap();
        assert!(matches!(level, HeadingLevel::H2));
        assert_eq!(text, "Test Heading");
    }
}
