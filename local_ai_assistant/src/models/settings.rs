//! Application Settings Model

use serde::{Deserialize, Serialize};

/// Response language options
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ResponseLanguage {
    #[default]
    Chinese,
    English,
    Thai,
    Spanish,
    French,
    German,
}

impl ResponseLanguage {
    pub fn as_str(&self) -> &'static str {
        match self {
            ResponseLanguage::Chinese => "Chinese",
            ResponseLanguage::English => "English",
            ResponseLanguage::Thai => "Thai",
            ResponseLanguage::Spanish => "Spanish",
            ResponseLanguage::French => "French",
            ResponseLanguage::German => "German",
        }
    }

    pub fn prompt_instruction(&self) -> &'static str {
        match self {
            ResponseLanguage::Chinese => "请用中文回答。",
            ResponseLanguage::English => "Please respond in English.",
            ResponseLanguage::Thai => "กรุณาตอบเป็นภาษาไทย",
            ResponseLanguage::Spanish => "Por favor, responde en español.",
            ResponseLanguage::French => "Veuillez répondre en français.",
            ResponseLanguage::German => "Bitte antworte auf Deutsch.",
        }
    }
}

/// Theme options
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Theme {
    #[default]
    Dark,
    Light,
    Blue,
    Purple,
}

impl Theme {
    pub fn as_str(&self) -> &'static str {
        match self {
            Theme::Dark => "Dark",
            Theme::Light => "Light",
            Theme::Blue => "Blue",
            Theme::Purple => "Purple",
        }
    }

    pub fn bg_class(&self) -> &'static str {
        match self {
            Theme::Dark => "bg-gray-900",
            Theme::Light => "bg-gray-100",
            Theme::Blue => "bg-slate-900",
            Theme::Purple => "bg-purple-950",
        }
    }

    pub fn sidebar_bg(&self) -> &'static str {
        match self {
            Theme::Dark => "bg-gray-800",
            Theme::Light => "bg-white",
            Theme::Blue => "bg-slate-800",
            Theme::Purple => "bg-purple-900",
        }
    }

    pub fn text_class(&self) -> &'static str {
        match self {
            Theme::Light => "text-gray-900",
            _ => "text-white",
        }
    }
}

/// Font size options
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum FontSize {
    Small,
    #[default]
    Medium,
    Large,
    ExtraLarge,
}

impl FontSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            FontSize::Small => "Small",
            FontSize::Medium => "Medium",
            FontSize::Large => "Large",
            FontSize::ExtraLarge => "Extra Large",
        }
    }

    pub fn prose_class(&self) -> &'static str {
        match self {
            FontSize::Small => "prose-sm",
            FontSize::Medium => "prose-base",
            FontSize::Large => "prose-lg",
            FontSize::ExtraLarge => "prose-xl",
        }
    }

    /// Returns inline CSS style for font-size (fallback when Tailwind classes are purged)
    pub fn font_style(&self) -> &'static str {
        match self {
            FontSize::Small => "font-size: 0.875rem; line-height: 1.25rem;",
            FontSize::Medium => "font-size: 1rem; line-height: 1.5rem;",
            FontSize::Large => "font-size: 1.125rem; line-height: 1.75rem;",
            FontSize::ExtraLarge => "font-size: 1.25rem; line-height: 1.75rem;",
        }
    }
}

/// Application settings
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppSettings {
    pub language: ResponseLanguage,
    pub theme: Theme,
    pub font_size: FontSize,
    pub model_name: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            language: ResponseLanguage::Chinese,
            theme: Theme::Dark,
            font_size: FontSize::Medium,
            model_name: "Qwen 2.5 7B".to_string(),
        }
    }
}
