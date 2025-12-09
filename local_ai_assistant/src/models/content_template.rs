//! Content Template System
//!
//! Article templates for different platforms and content types.
//!
//! Phase 2.4: Content Workflow

use serde::{Deserialize, Serialize};

/// Target platform for content
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub enum Platform {
    #[default]
    Blog,
    WeChat,
    XiaoHongShu,
    Twitter,
    LinkedIn,
    Medium,
    Custom,
}

impl Platform {
    pub fn display_name(&self) -> &'static str {
        match self {
            Platform::Blog => "Blog Post",
            Platform::WeChat => "微信公众号",
            Platform::XiaoHongShu => "小红书",
            Platform::Twitter => "Twitter/X Thread",
            Platform::LinkedIn => "LinkedIn Article",
            Platform::Medium => "Medium Story",
            Platform::Custom => "Custom",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Platform::Blog => "📝",
            Platform::WeChat => "💬",
            Platform::XiaoHongShu => "📕",
            Platform::Twitter => "🐦",
            Platform::LinkedIn => "💼",
            Platform::Medium => "📰",
            Platform::Custom => "⚙️",
        }
    }
}

/// Writing style for content generation
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub enum WritingStyle {
    #[default]
    Professional,
    Casual,
    Academic,
    Conversational,
    Persuasive,
    Storytelling,
}

impl WritingStyle {
    pub fn display_name(&self) -> &'static str {
        match self {
            WritingStyle::Professional => "Professional",
            WritingStyle::Casual => "Casual",
            WritingStyle::Academic => "Academic",
            WritingStyle::Conversational => "Conversational",
            WritingStyle::Persuasive => "Persuasive",
            WritingStyle::Storytelling => "Storytelling",
        }
    }

    pub fn system_prompt(&self) -> &'static str {
        match self {
            WritingStyle::Professional => "Write in a professional, clear, and authoritative tone. Use industry-standard terminology and maintain objectivity.",
            WritingStyle::Casual => "Write in a friendly, relaxed tone. Use simple language and feel free to include personal anecdotes.",
            WritingStyle::Academic => "Write in a formal, scholarly tone. Cite sources, use precise terminology, and maintain objectivity.",
            WritingStyle::Conversational => "Write as if talking to a friend. Use 'you' and 'I', ask questions, and keep it engaging.",
            WritingStyle::Persuasive => "Write to convince the reader. Use strong arguments, emotional appeals, and clear calls to action.",
            WritingStyle::Storytelling => "Write in a narrative style. Use vivid descriptions, build tension, and engage emotions.",
        }
    }
}

/// A section within an article template
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TemplateSection {
    pub id: String,
    pub title: String,
    pub prompt: String,
    pub word_limit: Option<usize>,
    pub is_optional: bool,
}

impl TemplateSection {
    pub fn new(title: &str, prompt: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title: title.to_string(),
            prompt: prompt.to_string(),
            word_limit: None,
            is_optional: false,
        }
    }

    pub fn with_word_limit(mut self, limit: usize) -> Self {
        self.word_limit = Some(limit);
        self
    }

    pub fn optional(mut self) -> Self {
        self.is_optional = true;
        self
    }
}

/// Article template definition
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArticleTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub platform: Platform,
    pub style: WritingStyle,
    pub sections: Vec<TemplateSection>,
    pub is_builtin: bool,
}

impl ArticleTemplate {
    pub fn new(name: &str, platform: Platform) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            description: String::new(),
            platform,
            style: WritingStyle::default(),
            sections: Vec::new(),
            is_builtin: false,
        }
    }

    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }

    pub fn with_style(mut self, style: WritingStyle) -> Self {
        self.style = style;
        self
    }

    pub fn add_section(mut self, section: TemplateSection) -> Self {
        self.sections.push(section);
        self
    }

    pub fn builtin(mut self) -> Self {
        self.is_builtin = true;
        self
    }
}

/// Get all built-in templates
pub fn get_builtin_templates() -> Vec<ArticleTemplate> {
    vec![
        // Tech Blog Template
        ArticleTemplate::new("Tech Blog Post", Platform::Blog)
            .with_description("A standard technical blog post with introduction, main content, and conclusion")
            .with_style(WritingStyle::Professional)
            .add_section(
                TemplateSection::new(
                    "Introduction",
                    "Write an engaging introduction that hooks the reader and explains what problem this article solves."
                ).with_word_limit(150)
            )
            .add_section(
                TemplateSection::new(
                    "Background",
                    "Provide necessary background context for readers who may be unfamiliar with the topic."
                ).with_word_limit(200).optional()
            )
            .add_section(
                TemplateSection::new(
                    "Main Content",
                    "Explain the main concepts, techniques, or solutions in detail. Use code examples where appropriate."
                ).with_word_limit(800)
            )
            .add_section(
                TemplateSection::new(
                    "Best Practices",
                    "Share practical tips and best practices related to the topic."
                ).with_word_limit(300).optional()
            )
            .add_section(
                TemplateSection::new(
                    "Conclusion",
                    "Summarize key takeaways and provide next steps or resources for further learning."
                ).with_word_limit(150)
            )
            .builtin(),

        // 小红书种草文 Template
        ArticleTemplate::new("小红书种草", Platform::XiaoHongShu)
            .with_description("小红书风格的种草推荐文案，适合产品推荐")
            .with_style(WritingStyle::Conversational)
            .add_section(
                TemplateSection::new(
                    "吸睛标题",
                    "写一个吸引眼球的标题，可以用emoji，要有冲击力和悬念感"
                ).with_word_limit(30)
            )
            .add_section(
                TemplateSection::new(
                    "痛点共鸣",
                    "描述目标用户的痛点或困扰，让读者产生共鸣"
                ).with_word_limit(100)
            )
            .add_section(
                TemplateSection::new(
                    "产品介绍",
                    "介绍推荐的产品/方法，强调独特卖点和使用体验"
                ).with_word_limit(200)
            )
            .add_section(
                TemplateSection::new(
                    "使用效果",
                    "分享使用效果和个人体验，要真实可信"
                ).with_word_limit(150)
            )
            .add_section(
                TemplateSection::new(
                    "行动号召",
                    "引导读者行动，可以是收藏、关注、购买链接等"
                ).with_word_limit(50)
            )
            .builtin(),

        // Twitter Thread Template
        ArticleTemplate::new("Twitter Thread", Platform::Twitter)
            .with_description("A viral Twitter thread format with hook, content, and CTA")
            .with_style(WritingStyle::Conversational)
            .add_section(
                TemplateSection::new(
                    "Hook Tweet",
                    "Write a compelling hook that makes people want to read the thread. Use curiosity, controversy, or value promise."
                ).with_word_limit(280)
            )
            .add_section(
                TemplateSection::new(
                    "Context Tweet",
                    "Provide context or background for the main content."
                ).with_word_limit(280)
            )
            .add_section(
                TemplateSection::new(
                    "Main Points (5 tweets)",
                    "Break down the main content into 5 digestible tweets. Each should stand alone but flow together."
                ).with_word_limit(1400)
            )
            .add_section(
                TemplateSection::new(
                    "Summary Tweet",
                    "Summarize the key takeaways in one tweet."
                ).with_word_limit(280)
            )
            .add_section(
                TemplateSection::new(
                    "CTA Tweet",
                    "End with a call to action - follow, retweet, or link to more content."
                ).with_word_limit(280)
            )
            .builtin(),

        // 微信公众号 Template
        ArticleTemplate::new("微信公众号文章", Platform::WeChat)
            .with_description("适合微信公众号的长文格式")
            .with_style(WritingStyle::Storytelling)
            .add_section(
                TemplateSection::new(
                    "标题",
                    "写一个吸引人的标题，控制在20字以内"
                ).with_word_limit(20)
            )
            .add_section(
                TemplateSection::new(
                    "引子",
                    "用一个故事、问题或现象引入话题，吸引读者继续阅读"
                ).with_word_limit(200)
            )
            .add_section(
                TemplateSection::new(
                    "正文",
                    "展开主要内容，可以分小节，每节有小标题"
                ).with_word_limit(1500)
            )
            .add_section(
                TemplateSection::new(
                    "金句/观点",
                    "提炼一两句可以被转发的金句或核心观点"
                ).with_word_limit(100)
            )
            .add_section(
                TemplateSection::new(
                    "结尾互动",
                    "以问题或互动结尾，引导读者留言或转发"
                ).with_word_limit(100)
            )
            .builtin(),

        // LinkedIn Article Template
        ArticleTemplate::new("LinkedIn Article", Platform::LinkedIn)
            .with_description("Professional article format for LinkedIn")
            .with_style(WritingStyle::Professional)
            .add_section(
                TemplateSection::new(
                    "Headline",
                    "Write a professional headline that promises value to the reader."
                ).with_word_limit(100)
            )
            .add_section(
                TemplateSection::new(
                    "Personal Hook",
                    "Start with a personal story or observation that relates to your professional experience."
                ).with_word_limit(150)
            )
            .add_section(
                TemplateSection::new(
                    "Key Insights",
                    "Share 3-5 key insights or lessons learned, each with a brief explanation."
                ).with_word_limit(600)
            )
            .add_section(
                TemplateSection::new(
                    "Actionable Advice",
                    "Provide concrete, actionable advice readers can apply immediately."
                ).with_word_limit(200)
            )
            .add_section(
                TemplateSection::new(
                    "Call to Discussion",
                    "End with a question or call for others to share their experiences."
                ).with_word_limit(100)
            )
            .builtin(),
    ]
}

/// Editor content state
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct EditorContent {
    pub title: String,
    pub sections: Vec<EditorSection>,
    pub template_id: Option<String>,
    pub platform: Platform,
    pub style: WritingStyle,
}

/// A section in the editor
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EditorSection {
    pub id: String,
    pub title: String,
    pub content: String,
    pub is_generated: bool,
    pub is_expanded: bool,
}

impl EditorSection {
    pub fn new(title: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title: title.to_string(),
            content: String::new(),
            is_generated: false,
            is_expanded: true,
        }
    }

    pub fn with_content(mut self, content: &str) -> Self {
        self.content = content.to_string();
        self
    }
}

impl EditorContent {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_template(template: &ArticleTemplate) -> Self {
        let sections = template.sections.iter().map(|s| {
            EditorSection::new(&s.title)
        }).collect();

        Self {
            title: String::new(),
            sections,
            template_id: Some(template.id.clone()),
            platform: template.platform.clone(),
            style: template.style.clone(),
        }
    }

    pub fn to_markdown(&self) -> String {
        let mut md = format!("# {}\n\n", self.title);

        for section in &self.sections {
            md.push_str(&format!("## {}\n\n", section.title));
            md.push_str(&section.content);
            md.push_str("\n\n");
        }

        md
    }

    pub fn to_html(&self) -> String {
        let md = self.to_markdown();
        comrak::markdown_to_html(&md, &comrak::Options::default())
    }

    pub fn word_count(&self) -> usize {
        self.sections.iter()
            .map(|s| s.content.split_whitespace().count())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builtin_templates() {
        let templates = get_builtin_templates();
        assert!(!templates.is_empty());
        assert!(templates.iter().all(|t| t.is_builtin));
    }

    #[test]
    fn test_editor_content_from_template() {
        let templates = get_builtin_templates();
        let template = &templates[0];

        let content = EditorContent::from_template(template);
        assert_eq!(content.sections.len(), template.sections.len());
    }

    #[test]
    fn test_to_markdown() {
        let mut content = EditorContent::new();
        content.title = "Test Article".to_string();
        content.sections.push(
            EditorSection::new("Introduction").with_content("Hello world!")
        );

        let md = content.to_markdown();
        assert!(md.contains("# Test Article"));
        assert!(md.contains("## Introduction"));
        assert!(md.contains("Hello world!"));
    }
}
