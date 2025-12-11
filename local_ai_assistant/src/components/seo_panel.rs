use dioxus::prelude::*;
use crate::core::seo::{SeoAnalysis, SeoIssue, SeoIssueSeverity, analyze_content};

#[component]
pub fn SeoAnalyzerPanel() -> Element {
    let mut content = use_signal(|| String::new());
    let mut title = use_signal(|| String::new());
    let mut analysis = use_signal(|| None::<SeoAnalysis>);
    let mut is_analyzing = use_signal(|| false);
    let mut error_message = use_signal(|| None::<String>);

    let analyze = move |_| {
        let content_val = content().clone();
        let title_val = title().clone();
        
        if content_val.trim().is_empty() {
            error_message.set(Some("Please enter content to analyze".to_string()));
            return;
        }

        is_analyzing.set(true);
        error_message.set(None);
        
        spawn(async move {
            let title_opt = if title_val.trim().is_empty() {
                None
            } else {
                Some(title_val.as_str())
            };

            match analyze_content(&content_val, title_opt, "openai").await {
                Ok(result) => {
                    analysis.set(Some(result));
                    is_analyzing.set(false);
                }
                Err(e) => {
                    error_message.set(Some(format!("Analysis failed: {}", e)));
                    is_analyzing.set(false);
                }
            }
        });
    };

    rsx! {
        div {
            class: "seo-analyzer-panel",
            
            // Header
            div {
                class: "panel-header",
                h1 { "SEO Content Analyzer" }
                p { "Optimize your content for search engines" }
            }

            // Input Section
            div {
                class: "input-section",
                
                div {
                    class: "input-group",
                    label { r#for: "title-input", "Title (optional):" }
                    input {
                        id: "title-input",
                        r#type: "text",
                        class: "title-input",
                        placeholder: "Enter article title...",
                        value: "{title}",
                        oninput: move |evt| title.set(evt.value().clone()),
                        disabled: is_analyzing(),
                    }
                }

                div {
                    class: "input-group",
                    label { r#for: "content-input", "Content:" }
                    textarea {
                        id: "content-input",
                        class: "content-input",
                        placeholder: "Paste your content here for SEO analysis...",
                        rows: "10",
                        value: "{content}",
                        oninput: move |evt| content.set(evt.value().clone()),
                        disabled: is_analyzing(),
                    }
                }

                button {
                    class: "btn-analyze",
                    disabled: is_analyzing(),
                    onclick: analyze,
                    if is_analyzing() {
                        "Analyzing..."
                    } else {
                        "Analyze SEO"
                    }
                }
            }

            // Error Message
            if let Some(ref err) = error_message() {
                div {
                    class: "error-message",
                    "⚠️ {err}"
                }
            }

            // Analysis Results
            if let Some(ref seo) = analysis() {
                div {
                    class: "analysis-results",
                    
                    // Quick Stats
                    div {
                        class: "stats-section",
                        h2 { "Quick Stats" }
                        div {
                            class: "stats-grid",
                            
                            div {
                                class: "stat-card",
                                div { class: "stat-label", "Word Count" }
                                div { class: "stat-value", "{seo.word_count}" }
                            }
                            
                            div {
                                class: "stat-card",
                                div { class: "stat-label", "Sentences" }
                                div { class: "stat-value", "{seo.sentence_count}" }
                            }
                            
                            div {
                                class: "stat-card",
                                div { class: "stat-label", "Avg Sentence Length" }
                                div { class: "stat-value", "{seo.avg_sentence_length:.1} words" }
                            }
                            
                            div {
                                class: "stat-card readability",
                                div { class: "stat-label", "Readability Score" }
                                div { class: "stat-value", "{seo.readability_score:.1}" }
                                div { class: "stat-grade", "{seo.readability_grade}" }
                            }
                        }
                    }

                    // SEO Issues
                    if !seo.issues.is_empty() {
                        div {
                            class: "issues-section",
                            h2 { "SEO Issues ({seo.issues.len()})" }
                            
                            for issue in &seo.issues {
                                div {
                                    class: match issue.severity {
                                        SeoIssueSeverity::Critical => "issue-card critical",
                                        SeoIssueSeverity::Warning => "issue-card warning",
                                        SeoIssueSeverity::Info => "issue-card info",
                                    },
                                    
                                    div {
                                        class: "issue-header",
                                        span {
                                            class: "issue-severity",
                                            match issue.severity {
                                                SeoIssueSeverity::Critical => "🔴 Critical",
                                                SeoIssueSeverity::Warning => "🟡 Warning",
                                                SeoIssueSeverity::Info => "🔵 Info",
                                            }
                                        }
                                        span { class: "issue-title", "{issue.title}" }
                                    }
                                    
                                    p { class: "issue-description", "{issue.description}" }
                                    
                                    if let Some(ref fix) = issue.fix {
                                        div {
                                            class: "issue-fix",
                                            strong { "Fix: " }
                                            span { "{fix}" }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Meta Data
                    div {
                        class: "meta-section",
                        h2 { "Meta Information" }
                        
                        div {
                            class: "meta-item",
                            strong { "Meta Description:" }
                            p { class: "meta-description", "{seo.meta_description}" }
                            span {
                                class: if seo.meta_description.len() >= 120 && seo.meta_description.len() <= 160 {
                                    "char-count good"
                                } else {
                                    "char-count warning"
                                },
                                "{seo.meta_description.len()} characters"
                            }
                        }
                    }

                    // Keywords
                    div {
                        class: "keywords-section",
                        h2 { "Suggested Keywords" }
                        
                        div {
                            class: "keywords-list",
                            for keyword in &seo.suggested_keywords {
                                span { class: "keyword-tag", "{keyword}" }
                            }
                        }
                    }

                    // Keyword Density
                    div {
                        class: "density-section",
                        h2 { "Top Keyword Density" }
                        
                        div {
                            class: "density-list",
                            for (keyword, stats) in seo.keyword_density.iter().take(10) {
                                div {
                                    class: "density-item",
                                    span { class: "density-keyword", "{keyword}" }
                                    span { class: "density-stats", "{stats.count} times ({stats.density:.2}%)" }
                                }
                            }
                        }
                    }

                    // Heading Structure
                    div {
                        class: "headings-section",
                        h2 { "Heading Structure" }
                        
                        div {
                            class: "heading-stats",
                            p { "H1: {seo.heading_structure.h1_count}" }
                            p { "H2: {seo.heading_structure.h2_count}" }
                            p { "H3: {seo.heading_structure.h3_count}" }
                            p {
                                class: if seo.heading_structure.has_proper_hierarchy {
                                    "hierarchy-good"
                                } else {
                                    "hierarchy-warning"
                                },
                                if seo.heading_structure.has_proper_hierarchy {
                                    "✓ Proper hierarchy"
                                } else {
                                    "⚠ Hierarchy issues detected"
                                }
                            }
                        }
                        
                        if !seo.heading_structure.headings.is_empty() {
                            div {
                                class: "headings-list",
                                for heading in &seo.heading_structure.headings {
                                    div {
                                        class: "heading-item h{heading.level}",
                                        span { class: "heading-level", "H{heading.level}" }
                                        span { class: "heading-text", "{heading.text}" }
                                    }
                                }
                            }
                        }
                    }

                    // Schema Markup
                    if let Some(ref schema) = seo.schema_markup {
                        div {
                            class: "schema-section",
                            h2 { "Schema.org Markup (JSON-LD)" }
                            
                            div {
                                class: "schema-actions",
                                button {
                                    class: "btn-copy",
                                    onclick: move |_| {
                                        // TODO: Copy to clipboard
                                        log::info!("Copy schema to clipboard");
                                    },
                                    "📋 Copy to Clipboard"
                                }
                            }
                            
                            pre {
                                class: "schema-code",
                                code { "{schema}" }
                            }
                        }
                    }

                    // Recommendations
                    if !seo.recommendations.is_empty() {
                        div {
                            class: "recommendations-section",
                            h2 { "Recommendations" }
                            
                            ul {
                                class: "recommendations-list",
                                for rec in &seo.recommendations {
                                    li { "💡 {rec}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Styles for SEO analyzer panel
pub fn seo_analyzer_styles() -> &'static str {
    r#"
.seo-analyzer-panel {
    max-width: 1200px;
    margin: 0 auto;
    padding: 2rem;
}

.panel-header {
    text-align: center;
    margin-bottom: 2rem;
}

.panel-header h1 {
    font-size: 2.5rem;
    margin-bottom: 0.5rem;
}

.input-section {
    background: white;
    padding: 2rem;
    border-radius: 8px;
    box-shadow: 0 2px 8px rgba(0,0,0,0.1);
    margin-bottom: 2rem;
}

.input-group {
    margin-bottom: 1.5rem;
}

.input-group label {
    display: block;
    font-weight: 600;
    margin-bottom: 0.5rem;
}

.title-input,
.content-input {
    width: 100%;
    padding: 0.75rem;
    font-size: 1rem;
    border: 1px solid #ddd;
    border-radius: 4px;
}

.content-input {
    font-family: monospace;
    resize: vertical;
}

.btn-analyze {
    width: 100%;
    padding: 1rem;
    font-size: 1.1rem;
    font-weight: 600;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
}

.btn-analyze:hover:not(:disabled) {
    transform: translateY(-2px);
}

.btn-analyze:disabled {
    opacity: 0.6;
}

.error-message {
    background: #fee;
    color: #c33;
    padding: 1rem;
    border-radius: 4px;
    margin-bottom: 1rem;
}

.analysis-results > div {
    background: white;
    padding: 2rem;
    border-radius: 8px;
    box-shadow: 0 2px 8px rgba(0,0,0,0.1);
    margin-bottom: 2rem;
}

.stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1rem;
}

.stat-card {
    background: #f9f9f9;
    padding: 1.5rem;
    border-radius: 8px;
    text-align: center;
}

.stat-label {
    font-size: 0.9rem;
    color: #666;
    margin-bottom: 0.5rem;
}

.stat-value {
    font-size: 2rem;
    font-weight: bold;
    color: #333;
}

.stat-grade {
    font-size: 0.85rem;
    color: #888;
    margin-top: 0.25rem;
}

.issue-card {
    padding: 1rem;
    border-left: 4px solid;
    margin-bottom: 1rem;
    border-radius: 4px;
}

.issue-card.critical {
    background: #fee;
    border-color: #c33;
}

.issue-card.warning {
    background: #ffc;
    border-color: #fc3;
}

.issue-card.info {
    background: #eff;
    border-color: #09c;
}

.issue-header {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-bottom: 0.5rem;
}

.issue-severity {
    font-weight: bold;
    font-size: 0.85rem;
}

.issue-title {
    font-weight: 600;
}

.issue-description {
    margin: 0.5rem 0;
}

.issue-fix {
    font-size: 0.9rem;
    color: #666;
}

.meta-description {
    background: #f5f5f5;
    padding: 1rem;
    border-radius: 4px;
    margin: 0.5rem 0;
}

.char-count {
    font-size: 0.85rem;
    font-weight: 600;
}

.char-count.good {
    color: #0a0;
}

.char-count.warning {
    color: #fa0;
}

.keywords-list {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
}

.keyword-tag {
    background: #667eea;
    color: white;
    padding: 0.5rem 1rem;
    border-radius: 20px;
    font-size: 0.9rem;
}

.density-list {
    max-height: 400px;
    overflow-y: auto;
}

.density-item {
    display: flex;
    justify-content: space-between;
    padding: 0.5rem 0;
    border-bottom: 1px solid #eee;
}

.heading-stats {
    margin-bottom: 1rem;
}

.hierarchy-good {
    color: #0a0;
}

.hierarchy-warning {
    color: #fa0;
}

.headings-list {
    margin-top: 1rem;
}

.heading-item {
    padding: 0.5rem;
    margin: 0.25rem 0;
    display: flex;
    gap: 1rem;
}

.heading-item.h1 {
    font-size: 1.2rem;
    font-weight: bold;
}

.heading-item.h2 {
    margin-left: 1rem;
    font-size: 1.1rem;
}

.heading-item.h3 {
    margin-left: 2rem;
    font-size: 1rem;
}

.schema-code {
    background: #f5f5f5;
    padding: 1rem;
    border-radius: 4px;
    overflow-x: auto;
}

.recommendations-list {
    list-style: none;
    padding: 0;
}

.recommendations-list li {
    padding: 0.75rem;
    background: #f9f9f9;
    margin-bottom: 0.5rem;
    border-radius: 4px;
}
"#
}
