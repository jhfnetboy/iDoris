use dioxus::prelude::*;
use crate::models::{ContentPackage, GenerationProgress, GenerationStage, ExportFormat};
use crate::core::content_generator::{ContentGenerator, export_content_package};

#[component]
pub fn ContentPackageGenerator() -> Element {
    let mut topic = use_signal(|| String::new());
    let mut package = use_signal(|| None::<ContentPackage>);
    let mut is_generating = use_signal(|| false);
    let mut progress = use_signal(|| None::<GenerationProgress>);
    let mut error_message = use_signal(|| None::<String>);

    let generate_package = move |_| {
        let topic_val = topic().clone();
        if topic_val.trim().is_empty() {
            error_message.set(Some("Please enter a topic".to_string()));
            return;
        }

        is_generating.set(true);
        error_message.set(None);
        
        spawn(async move {
            // Create generator with default providers
            let generator = ContentGenerator::new(
                "openai".to_string(),
                "local".to_string(),
                "volcengine".to_string(),
            );

            // Progress callback
            let progress_signal = progress.clone();
            let callback = Box::new(move |p: GenerationProgress| {
                progress_signal.set(Some(p));
            });

            match generator.generate_from_topic(&topic_val, Some(callback)).await {
                Ok(pkg) => {
                    package.set(Some(pkg));
                    is_generating.set(false);
                }
                Err(e) => {
                    error_message.set(Some(format!("Generation failed: {}", e)));
                    is_generating.set(false);
                }
            }
        });
    };

    let export_package = move |format: ExportFormat| {
        if let Some(ref pkg) = package() {
            let pkg_clone = pkg.clone();
            spawn(async move {
                match export_content_package(&pkg_clone, format).await {
                    Ok(content) => {
                        log::info!("Exported content package successfully");
                        // TODO: Download or save the content
                        // For now, just log it
                        log::info!("Content length: {} bytes", content.len());
                    }
                    Err(e) => {
                        log::error!("Export failed: {}", e);
                    }
                }
            });
        }
    };

    let regenerate_article = move |_| {
        // TODO: Implement regenerating just the article
        log::info!("Regenerating article...");
    };

    let regenerate_images = move |_| {
        // TODO: Implement regenerating just images
        log::info!("Regenerating images...");
    };

    let regenerate_videos = move |_| {
        // TODO: Implement regenerating just videos
        log::info!("Regenerating videos...");
    };

    rsx! {
        div {
            class: "content-package-generator",
            
            // Header
            div {
                class: "generator-header",
                h1 { "Content Package Generator" }
                p { "Generate complete multi-modal content packages with one click" }
            }

            // Topic Input
            div {
                class: "topic-input-section",
                label {
                    r#for: "topic-input",
                    "Enter your content topic:"
                }
                input {
                    id: "topic-input",
                    r#type: "text",
                    class: "topic-input",
                    placeholder: "e.g., 'The Future of AI in Healthcare'",
                    value: "{topic}",
                    oninput: move |evt| topic.set(evt.value().clone()),
                    disabled: is_generating(),
                }
                button {
                    class: "btn-generate",
                    disabled: is_generating(),
                    onclick: generate_package,
                    if is_generating() {
                        "Generating..."
                    } else {
                        "Generate Content Package"
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

            // Progress Bar
            if let Some(ref prog) = progress() {
                div {
                    class: "progress-section",
                    h3 { "Generation Progress" }
                    div {
                        class: "progress-bar-container",
                        div {
                            class: "progress-bar",
                            style: "width: {prog.progress_percent}%",
                        }
                    }
                    p {
                        class: "progress-status",
                        "Stage: {prog.stage:?} - {prog.current_task}"
                    }
                    p {
                        class: "progress-percent",
                        "{prog.progress_percent}% complete"
                    }
                }
            }

            // Package Preview
            if let Some(ref pkg) = package() {
                div {
                    class: "package-preview",
                    
                    // Article Preview
                    div {
                        class: "article-section",
                        div {
                            class: "section-header",
                            h2 { "Article" }
                            button {
                                class: "btn-regenerate",
                                onclick: regenerate_article,
                                "🔄 Regenerate"
                            }
                        }
                        
                        h3 { "{pkg.article.title}" }
                        if let Some(ref subtitle) = pkg.article.subtitle {
                            p { class: "subtitle", "{subtitle}" }
                        }
                        p {
                            class: "meta",
                            "📖 {pkg.article.word_count} words • ⏱️ {pkg.article.reading_time_minutes} min read"
                        }
                        
                        div {
                            class: "article-sections",
                            for section in &pkg.article.sections {
                                div {
                                    class: "article-section-item",
                                    h4 { "{section.heading}" }
                                    p { "{section.content}" }
                                }
                            }
                        }
                    }

                    // Images Preview
                    {
                        let image_count = pkg.section_images.len() + if pkg.header_image.is_some() { 1 } else { 0 };
                        rsx! {
                            div {
                                class: "images-section",
                                div {
                                    class: "section-header",
                                    h2 { "Images ({image_count})" }
                                    button {
                                        class: "btn-regenerate",
                                        onclick: regenerate_images,
                                        "🔄 Regenerate"
                                    }
                                }

                        
                        div {
                            class: "images-grid",
                            if let Some(ref header) = pkg.header_image {
                                div {
                                    class: "image-item header-image",
                                    if let Some(ref url) = header.url {
                                        img {
                                            src: "{url}",
                                            alt: "{header.alt_text}",
                                        }
                                    }
                                    p { class: "image-label", "Header Image" }
                                }
                            }
                            
                            for (idx, img) in pkg.section_images.iter().enumerate() {
                                div {
                                    class: "image-item",
                                    if let Some(ref url) = img.url {
                                        img {
                                            src: "{url}",
                                            alt: "{img.alt_text}",
                                        }
                                    }
                                    p { class: "image-label", "Section Image {idx + 1}" }
                                }
                            }
                        }
                            }
                        }
                    }


                    // Videos Preview
                    div {
                        class: "videos-section",
                        div {
                            class: "section-header",
                            h2 { "Social Media Clips ({pkg.social_clips.len()})" }
                            button {
                                class: "btn-regenerate",
                                onclick: regenerate_videos,
                                "🔄 Regenerate"
                            }
                        }
                        
                        div {
                            class: "videos-grid",
                            for clip in &pkg.social_clips {
                                div {
                                    class: "video-item",
                                    if let Some(ref url) = clip.url {
                                        video {
                                            src: "{url}",
                                            controls: true,
                                        }
                                    }
                                    p { class: "video-platform", "{clip.platform:?}" }
                                    if let Some(ref caption) = clip.caption {
                                        p { class: "video-caption", "{caption}" }
                                    }
                                }
                            }
                        }
                    }

                    // SEO Metadata
                    div {
                        class: "seo-section",
                        h2 { "SEO Metadata" }
                        
                        div {
                            class: "seo-item",
                            strong { "Meta Title: " }
                            span { "{pkg.seo_metadata.meta_title}" }
                        }
                        
                        div {
                            class: "seo-item",
                            strong { "Meta Description: " }
                            span { "{pkg.seo_metadata.meta_description}" }
                        }
                        
                        div {
                            class: "seo-item",
                            strong { "Keywords: " }
                            span { "{pkg.seo_metadata.keywords.join(\", \")}" }
                        }
                        
                        if let Some(ref focus) = pkg.seo_metadata.focus_keyword {
                            div {
                                class: "seo-item",
                                strong { "Focus Keyword: " }
                                span { "{focus}" }
                            }
                        }
                        
                        if let Some(ref schema) = pkg.seo_metadata.schema_markup {
                            details {
                                summary { "Schema Markup (JSON-LD)" }
                                pre { "{schema}" }
                            }
                        }
                    }

                    // Export Options
                    div {
                        class: "export-section",
                        h2 { "Export Content Package" }
                        div {
                            class: "export-buttons",
                            button {
                                class: "btn-export",
                                onclick: move |_| export_package(ExportFormat::Markdown),
                                "📄 Export as Markdown"
                            }
                            button {
                                class: "btn-export",
                                onclick: move |_| export_package(ExportFormat::Html),
                                "🌐 Export as HTML"
                            }
                            button {
                                class: "btn-export",
                                onclick: move |_| export_package(ExportFormat::Json),
                                "📦 Export as JSON"
                            }
                            button {
                                class: "btn-export",
                                onclick: move |_| export_package(ExportFormat::WordPress),
                                "📝 Export to WordPress"
                            }
                            button {
                                class: "btn-export",
                                onclick: move |_| export_package(ExportFormat::Medium),
                                "✍️ Export to Medium"
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Styles for the content package generator
pub fn content_package_styles() -> &'static str {
    r#"
.content-package-generator {
    max-width: 1200px;
    margin: 0 auto;
    padding: 2rem;
}

.generator-header {
    text-align: center;
    margin-bottom: 2rem;
}

.generator-header h1 {
    font-size: 2.5rem;
    margin-bottom: 0.5rem;
}

.generator-header p {
    color: #666;
    font-size: 1.1rem;
}

.topic-input-section {
    background: white;
    padding: 2rem;
    border-radius: 8px;
    box-shadow: 0 2px 8px rgba(0,0,0,0.1);
    margin-bottom: 2rem;
}

.topic-input-section label {
    display: block;
    font-weight: 600;
    margin-bottom: 0.5rem;
}

.topic-input {
    width: 100%;
    padding: 0.75rem;
    font-size: 1rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    margin-bottom: 1rem;
}

.btn-generate {
    width: 100%;
    padding: 1rem;
    font-size: 1.1rem;
    font-weight: 600;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    transition: transform 0.2s;
}

.btn-generate:hover:not(:disabled) {
    transform: translateY(-2px);
}

.btn-generate:disabled {
    opacity: 0.6;
    cursor: not-allowed;
}

.error-message {
    background: #fee;
    color: #c33;
    padding: 1rem;
    border-radius: 4px;
    margin-bottom: 1rem;
}

.progress-section {
    background: white;
    padding: 2rem;
    border-radius: 8px;
    box-shadow: 0 2px 8px rgba(0,0,0,0.1);
    margin-bottom: 2rem;
}

.progress-bar-container {
    width: 100%;
    height: 30px;
    background: #f0f0f0;
    border-radius: 15px;
    overflow: hidden;
    margin: 1rem 0;
}

.progress-bar {
    height: 100%;
    background: linear-gradient(90deg, #667eea 0%, #764ba2 100%);
    transition: width 0.3s ease;
}

.progress-status {
    font-weight: 600;
    margin-top: 0.5rem;
}

.package-preview {
    margin-top: 2rem;
}

.article-section,
.images-section,
.videos-section,
.seo-section,
.export-section {
    background: white;
    padding: 2rem;
    border-radius: 8px;
    box-shadow: 0 2px 8px rgba(0,0,0,0.1);
    margin-bottom: 2rem;
}

.section-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;
}

.btn-regenerate {
    padding: 0.5rem 1rem;
    background: #f0f0f0;
    border: none;
    border-radius: 4px;
    cursor: pointer;
}

.btn-regenerate:hover {
    background: #e0e0e0;
}

.article-sections {
    margin-top: 1rem;
}

.article-section-item {
    margin-bottom: 2rem;
    padding-bottom: 2rem;
    border-bottom: 1px solid #eee;
}

.article-section-item:last-child {
    border-bottom: none;
}

.images-grid,
.videos-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
    gap: 1rem;
    margin-top: 1rem;
}

.image-item img,
.video-item video {
    width: 100%;
    border-radius: 8px;
}

.image-label,
.video-platform,
.video-caption {
    margin-top: 0.5rem;
    font-size: 0.9rem;
    color: #666;
}

.seo-item {
    margin-bottom: 1rem;
    padding: 0.5rem;
    background: #f9f9f9;
    border-radius: 4px;
}

.export-buttons {
    display: flex;
    gap: 1rem;
    flex-wrap: wrap;
}

.btn-export {
    padding: 0.75rem 1.5rem;
    background: #667eea;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    transition: background 0.2s;
}

.btn-export:hover {
    background: #5568d3;
}
"#
}
