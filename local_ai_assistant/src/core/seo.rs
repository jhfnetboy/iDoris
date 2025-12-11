use anyhow::{Result, Context};
use std::collections::HashMap;
use crate::core::llm;

/// SEO analysis results
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SeoAnalysis {
    pub keyword_density: HashMap<String, KeywordStats>,
    pub readability_score: f32,
    pub readability_grade: String,
    pub meta_description: String,
    pub suggested_keywords: Vec<String>,
    pub schema_markup: Option<String>,
    pub heading_structure: HeadingStructure,
    pub word_count: usize,
    pub sentence_count: usize,
    pub avg_sentence_length: f32,
    pub issues: Vec<SeoIssue>,
    pub recommendations: Vec<String>,
}

/// Keyword statistics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KeywordStats {
    pub count: usize,
    pub density: f32, // percentage
    pub positions: Vec<usize>, // character positions
}

/// Heading structure analysis
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HeadingStructure {
    pub h1_count: usize,
    pub h2_count: usize,
    pub h3_count: usize,
    pub has_proper_hierarchy: bool,
    pub headings: Vec<Heading>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Heading {
    pub level: u8,
    pub text: String,
    pub position: usize,
}

/// SEO issue severity
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum SeoIssueSeverity {
    Critical,
    Warning,
    Info,
}

/// SEO issue
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SeoIssue {
    pub severity: SeoIssueSeverity,
    pub title: String,
    pub description: String,
    pub fix: Option<String>,
}

/// Analyze content for SEO
pub async fn analyze_content(content: &str, title: Option<&str>, llm_provider: &str) -> Result<SeoAnalysis> {
    let mut analysis = SeoAnalysis {
        keyword_density: HashMap::new(),
        readability_score: 0.0,
        readability_grade: String::new(),
        meta_description: String::new(),
        suggested_keywords: Vec::new(),
        schema_markup: None,
        heading_structure: HeadingStructure {
            h1_count: 0,
            h2_count: 0,
            h3_count: 0,
            has_proper_hierarchy: false,
            headings: Vec::new(),
        },
        word_count: 0,
        sentence_count: 0,
        avg_sentence_length: 0.0,
        issues: Vec::new(),
        recommendations: Vec::new(),
    };

    // Basic text analysis
    analysis.word_count = count_words(content);
    analysis.sentence_count = count_sentences(content);
    analysis.avg_sentence_length = if analysis.sentence_count > 0 {
        analysis.word_count as f32 / analysis.sentence_count as f32
    } else {
        0.0
    };

    // Readability analysis
    let (score, grade) = calculate_readability(content);
    analysis.readability_score = score;
    analysis.readability_grade = grade;

    // Keyword density analysis
    analysis.keyword_density = analyze_keyword_density(content);

    // Heading structure analysis
    analysis.heading_structure = analyze_heading_structure(content);

    // Generate meta description using AI
    analysis.meta_description = generate_meta_description(content, title, llm_provider).await?;

    // Suggest keywords using AI
    analysis.suggested_keywords = suggest_keywords(content, title, llm_provider).await?;

    // Generate schema markup if title provided
    if let Some(title_text) = title {
        analysis.schema_markup = Some(generate_article_schema(title_text, &analysis));
    }

    // Identify issues and recommendations
    analysis.issues = identify_seo_issues(&analysis, title);
    analysis.recommendations = generate_recommendations(&analysis);

    Ok(analysis)
}

/// Count words in text
fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

/// Count sentences in text
fn count_sentences(text: &str) -> usize {
    text.chars()
        .filter(|&c| c == '.' || c == '!' || c == '?')
        .count()
        .max(1)
}

/// Calculate Flesch Reading Ease score
/// Formula: 206.835 - 1.015(total words/total sentences) - 84.6(total syllables/total words)
fn calculate_readability(text: &str) -> (f32, String) {
    let words = count_words(text);
    let sentences = count_sentences(text);
    let syllables = count_syllables(text);

    if words == 0 || sentences == 0 {
        return (0.0, "N/A".to_string());
    }

    let avg_sentence_length = words as f32 / sentences as f32;
    let avg_syllables_per_word = syllables as f32 / words as f32;

    let score = 206.835 - (1.015 * avg_sentence_length) - (84.6 * avg_syllables_per_word);
    let score = score.max(0.0).min(100.0);

    let grade = match score as u32 {
        90..=100 => "Very Easy (5th grade)".to_string(),
        80..=89 => "Easy (6th grade)".to_string(),
        70..=79 => "Fairly Easy (7th grade)".to_string(),
        60..=69 => "Standard (8th-9th grade)".to_string(),
        50..=59 => "Fairly Difficult (10th-12th grade)".to_string(),
        30..=49 => "Difficult (College)".to_string(),
        _ => "Very Difficult (College graduate)".to_string(),
    };

    (score, grade)
}

/// Estimate syllable count (simplified)
fn count_syllables(text: &str) -> usize {
    text.split_whitespace()
        .map(|word| estimate_syllables(word))
        .sum()
}

/// Estimate syllables in a word (simplified heuristic)
fn estimate_syllables(word: &str) -> usize {
    let word = word.to_lowercase();
    let vowels = ['a', 'e', 'i', 'o', 'u', 'y'];
    
    let mut count = 0;
    let mut prev_was_vowel = false;
    
    for ch in word.chars() {
        if vowels.contains(&ch) {
            if !prev_was_vowel {
                count += 1;
            }
            prev_was_vowel = true;
        } else {
            prev_was_vowel = false;
        }
    }
    
    // Adjust for silent e
    if word.ends_with('e') && count > 1 {
        count -= 1;
    }
    
    count.max(1)
}

/// Analyze keyword density
fn analyze_keyword_density(text: &str) -> HashMap<String, KeywordStats> {
    let words: Vec<&str> = text.split_whitespace().collect();
    let total_words = words.len();
    let mut word_map: HashMap<String, Vec<usize>> = HashMap::new();
    
    // Track word positions
    let mut char_position = 0;
    for word in &words {
        let normalized = word.to_lowercase()
            .trim_matches(|c: char| !c.is_alphanumeric())
            .to_string();
        
        if normalized.len() > 3 { // Only track words longer than 3 chars
            word_map.entry(normalized).or_insert_with(Vec::new).push(char_position);
        }
        char_position += word.len() + 1;
    }
    
    // Calculate density and create stats
    let mut density_map = HashMap::new();
    for (word, positions) in word_map {
        let count = positions.len();
        let density = (count as f32 / total_words as f32) * 100.0;
        
        density_map.insert(word, KeywordStats {
            count,
            density,
            positions,
        });
    }
    
    density_map
}

/// Analyze heading structure
fn analyze_heading_structure(text: &str) -> HeadingStructure {
    let mut structure = HeadingStructure {
        h1_count: 0,
        h2_count: 0,
        h3_count: 0,
        has_proper_hierarchy: false,
        headings: Vec::new(),
    };
    
    // Simple markdown heading detection
    let mut char_pos = 0;
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("# ") && !trimmed.starts_with("## ") {
            structure.h1_count += 1;
            structure.headings.push(Heading {
                level: 1,
                text: trimmed[2..].to_string(),
                position: char_pos,
            });
        } else if trimmed.starts_with("## ") && !trimmed.starts_with("### ") {
            structure.h2_count += 1;
            structure.headings.push(Heading {
                level: 2,
                text: trimmed[3..].to_string(),
                position: char_pos,
            });
        } else if trimmed.starts_with("### ") {
            structure.h3_count += 1;
            structure.headings.push(Heading {
                level: 3,
                text: trimmed[4..].to_string(),
                position: char_pos,
            });
        }
        char_pos += line.len() + 1;
    }
    
    // Check hierarchy
    structure.has_proper_hierarchy = structure.h1_count == 1;
    
    structure
}

/// Generate meta description using AI
pub async fn generate_meta_description(content: &str, title: Option<&str>, llm_provider: &str) -> Result<String> {
    let title_part = title.map(|t| format!("Title: {}\n\n", t)).unwrap_or_default();
    
    let prompt = format!(
        "{}Write a compelling SEO meta description (150-160 characters) for this content:\n\n{}\n\n\
        Requirements:\n\
        - Exactly 150-160 characters\n\
        - Include main keywords naturally\n\
        - Engaging and click-worthy\n\
        - No quotation marks\n\
        Return only the meta description, nothing else.",
        title_part,
        content.chars().take(500).collect::<String>()
    );
    
    let description = llm::generate_text(&prompt, llm_provider, None)
        .await
        .context("Failed to generate meta description")?;
    
    // Trim to 160 characters
    Ok(description.trim().chars().take(160).collect())
}

/// Suggest keywords using AI
pub async fn suggest_keywords(content: &str, title: Option<&str>, llm_provider: &str) -> Result<Vec<String>> {
    let title_part = title.map(|t| format!("Title: {}\n\n", t)).unwrap_or_default();
    
    let prompt = format!(
        "{}Analyze this content and suggest 7-10 SEO keywords:\n\n{}\n\n\
        Requirements:\n\
        - Mix of short-tail and long-tail keywords\n\
        - Relevant to the content\n\
        - High search potential\n\
        - Return only keywords, comma-separated, no explanations",
        title_part,
        content.chars().take(1000).collect::<String>()
    );
    
    let response = llm::generate_text(&prompt, llm_provider, None)
        .await
        .context("Failed to suggest keywords")?;
    
    let keywords: Vec<String> = response
        .split(',')
        .map(|k| k.trim().to_string())
        .filter(|k| !k.is_empty())
        .take(10)
        .collect();
    
    Ok(keywords)
}

/// Generate Article schema markup (JSON-LD)
fn generate_article_schema(title: &str, analysis: &SeoAnalysis) -> String {
    format!(
        r#"{{
  "@context": "https://schema.org",
  "@type": "Article",
  "headline": "{}",
  "description": "{}",
  "wordCount": {},
  "keywords": "{}"
}}"#,
        title,
        analysis.meta_description,
        analysis.word_count,
        analysis.suggested_keywords.join(", ")
    )
}

/// Identify SEO issues
fn identify_seo_issues(analysis: &SeoAnalysis, title: Option<&str>) -> Vec<SeoIssue> {
    let mut issues = Vec::new();
    
    // Check title
    if title.is_none() {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Critical,
            title: "Missing Title".to_string(),
            description: "Content should have a clear title (H1)".to_string(),
            fix: Some("Add an H1 heading at the beginning".to_string()),
        });
    }
    
    // Check H1 count
    if analysis.heading_structure.h1_count == 0 {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Critical,
            title: "No H1 Heading".to_string(),
            description: "Every page should have exactly one H1 heading".to_string(),
            fix: Some("Add a single H1 heading".to_string()),
        });
    } else if analysis.heading_structure.h1_count > 1 {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Warning,
            title: "Multiple H1 Headings".to_string(),
            description: format!("Found {} H1 headings. Best practice is to have exactly one.", analysis.heading_structure.h1_count),
            fix: Some("Use only one H1, convert others to H2 or H3".to_string()),
        });
    }
    
    // Check meta description length
    let desc_len = analysis.meta_description.len();
    if desc_len < 120 {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Warning,
            title: "Short Meta Description".to_string(),
            description: format!("Meta description is {} characters. Recommended: 150-160", desc_len),
            fix: Some("Expand the meta description".to_string()),
        });
    } else if desc_len > 160 {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Info,
            title: "Long Meta Description".to_string(),
            description: format!("Meta description is {} characters. It may be truncated in search results.", desc_len),
            fix: Some("Shorten to 150-160 characters".to_string()),
        });
    }
    
    // Check word count
    if analysis.word_count < 300 {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Warning,
            title: "Low Word Count".to_string(),
            description: format!("Content has {} words. Recommended: 300+ for better SEO", analysis.word_count),
            fix: Some("Add more detailed content".to_string()),
        });
    }
    
    // Check readability
    if analysis.readability_score < 50.0 {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Info,
            title: "Difficult Readability".to_string(),
            description: format!("Readability score is {:.1}. Content may be too complex for general audience.", analysis.readability_score),
            fix: Some("Use shorter sentences and simpler words".to_string()),
        });
    }
    
    issues
}

/// Generate recommendations
fn generate_recommendations(analysis: &SeoAnalysis) -> Vec<String> {
    let mut recommendations = Vec::new();
    
    if analysis.heading_structure.h2_count == 0 {
        recommendations.push("Add H2 subheadings to break up content and improve readability".to_string());
    }
    
    if analysis.word_count > 500 && analysis.heading_structure.h2_count + analysis.heading_structure.h3_count < 3 {
        recommendations.push("Add more subheadings for better content structure".to_string());
    }
    
    if analysis.avg_sentence_length > 25.0 {
        recommendations.push("Consider shortening some sentences for better readability".to_string());
    }
    
    if analysis.suggested_keywords.len() < 5 {
        recommendations.push("Include more relevant keywords naturally in the content".to_string());
    }
    
    if analysis.schema_markup.is_none() {
        recommendations.push("Add Schema.org markup to enhance search engine understanding".to_string());
    }
    
    recommendations
}
