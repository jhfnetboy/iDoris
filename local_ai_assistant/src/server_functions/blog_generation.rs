//! Server functions for blog generation
//!
//! Dioxus server functions for AI-powered blog post creation.

use dioxus::prelude::*;
use crate::models::{BlogSection, BlogPost};

/// Generate a blog post outline from a topic
#[server]
pub async fn generate_blog_outline(topic: String) -> Result<Vec<BlogSection>, ServerFnError> {
    use crate::content::outline::generate_outline;
    
    if topic.trim().is_empty() {
        return Err(ServerFnError::new("Topic cannot be empty"));
    }
    
    generate_outline(&topic)
        .await
        .map_err(|e| ServerFnError::new(format!("Outline generation failed: {}", e)))
}

/// Generate SEO title suggestions for a topic
#[server]
pub async fn generate_seo_titles(topic: String, count: u8) -> Result<Vec<String>, ServerFnError> {
    use crate::core::llm::get_llm_response;
    
    let prompt = format!(
        "Generate {} catchy, SEO-optimized blog post titles for the topic: \"{}\"\n\n\
         Requirements:\n\
         - 50-60 characters each\n\
         - Include relevant keywords\n\
         - Engaging and click-worthy\n\
         - Return only the titles, one per line\n\
         - No numbering or bullets",
        count, topic
    );
    
    let response = get_llm_response(prompt, None)
        .await
        .map_err(|e| ServerFnError::new(e))?;
    
    let titles: Vec<String> = response
        .lines()
        .filter(|line| !line.trim().is_empty())
        .take(count as usize)
        .map(|line| line.trim().to_string())
        .collect();
    
    if titles.is_empty() {
        return Err(ServerFnError::new("No titles generated"));
    }
    
    Ok(titles)
}

/// Save a blog post to the content library
#[server]
pub async fn save_blog_post(post: BlogPost) -> Result<String, ServerFnError> {
    // TODO: Implement database storage in Week 3
    // For now, just return the ID
    println!("📝 Blog post saved: {} ({})", post.title, post.id);
    Ok(post.id)
}
