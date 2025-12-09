//! Content Editor Panel Component
//!
//! A three-column editor for content creation workflow.
//!
//! Phase 2.4: Content Workflow

use dioxus::prelude::*;

use crate::models::content_template::{
    ArticleTemplate, EditorContent, EditorSection, Platform,
    WritingStyle, get_builtin_templates,
};
use crate::server_functions::{
    fetch_rss_entries, extract_article_content, generate_outline, expand_section,
};

/// Content Editor Panel component
#[component]
pub fn ContentEditorPanel(
    on_open_settings: EventHandler<()>,
) -> Element {
    // State
    let mut templates = use_signal(|| get_builtin_templates());
    let mut selected_template: Signal<Option<ArticleTemplate>> = use_signal(|| None);
    let mut editor_content = use_signal(EditorContent::new);
    let mut is_generating = use_signal(|| false);
    let mut error_message: Signal<Option<String>> = use_signal(|| None);
    let mut rss_url = use_signal(|| String::new());
    let mut rss_entries: Signal<Vec<(String, String, String)>> = use_signal(|| Vec::new()); // (title, url, summary)
    let mut article_url = use_signal(|| String::new());
    let mut active_section: Signal<Option<usize>> = use_signal(|| None);
    let mut show_preview = use_signal(|| false);

    // Handle template selection
    let handle_select_template = move |template: ArticleTemplate| {
        let content = EditorContent::from_template(&template);
        editor_content.set(content);
        selected_template.set(Some(template));
    };

    // Handle RSS fetch
    let handle_fetch_rss = move |_| {
        let url = rss_url.read().clone();
        if url.trim().is_empty() {
            error_message.set(Some("Please enter an RSS URL".to_string()));
            return;
        }

        is_generating.set(true);
        error_message.set(None);

        spawn(async move {
            match fetch_rss_entries(url).await {
                Ok(entries) => {
                    rss_entries.set(entries);
                    is_generating.set(false);
                }
                Err(e) => {
                    error_message.set(Some(format!("Failed to fetch RSS: {:?}", e)));
                    is_generating.set(false);
                }
            }
        });
    };

    // Handle article extraction
    let handle_extract_article = move |_| {
        let url = article_url.read().clone();
        if url.trim().is_empty() {
            error_message.set(Some("Please enter an article URL".to_string()));
            return;
        }

        is_generating.set(true);
        error_message.set(None);

        spawn(async move {
            match extract_article_content(url).await {
                Ok((title, content)) => {
                    let mut ec = editor_content.read().clone();
                    ec.title = title;
                    if let Some(section) = ec.sections.first_mut() {
                        section.content = content;
                    }
                    editor_content.set(ec);
                    is_generating.set(false);
                }
                Err(e) => {
                    error_message.set(Some(format!("Failed to extract article: {:?}", e)));
                    is_generating.set(false);
                }
            }
        });
    };

    // Handle outline generation
    let handle_generate_outline = move |_| {
        let title = editor_content.read().title.clone();
        let template_name = selected_template.read()
            .as_ref()
            .map(|t| t.name.clone())
            .unwrap_or_else(|| "Blog Post".to_string());

        if title.trim().is_empty() {
            error_message.set(Some("Please enter a title first".to_string()));
            return;
        }

        is_generating.set(true);
        error_message.set(None);

        spawn(async move {
            match generate_outline(title, template_name).await {
                Ok(sections) => {
                    let mut ec = editor_content.read().clone();
                    ec.sections = sections.into_iter().map(|(title, prompt)| {
                        let mut s = EditorSection::new(&title);
                        s.content = prompt;
                        s
                    }).collect();
                    editor_content.set(ec);
                    is_generating.set(false);
                }
                Err(e) => {
                    error_message.set(Some(format!("Failed to generate outline: {:?}", e)));
                    is_generating.set(false);
                }
            }
        });
    };

    // Handle section expansion
    let handle_expand_section = move |index: usize| {
        let ec = editor_content.read().clone();
        if let Some(section) = ec.sections.get(index) {
            let section_title = section.title.clone();
            let context = ec.title.clone();

            is_generating.set(true);
            active_section.set(Some(index));

            spawn(async move {
                match expand_section(section_title, context).await {
                    Ok(content) => {
                        let mut ec = editor_content.read().clone();
                        if let Some(section) = ec.sections.get_mut(index) {
                            section.content = content;
                            section.is_generated = true;
                        }
                        editor_content.set(ec);
                        is_generating.set(false);
                        active_section.set(None);
                    }
                    Err(e) => {
                        error_message.set(Some(format!("Failed to expand section: {:?}", e)));
                        is_generating.set(false);
                        active_section.set(None);
                    }
                }
            });
        }
    };

    // Handle export
    let handle_export_markdown = move |_| {
        let md = editor_content.read().to_markdown();
        // In a real implementation, this would trigger a download
        web_sys::console::log_1(&format!("Markdown:\n{}", md).into());
    };

    rsx! {
        div {
            class: "flex-1 flex flex-col h-full overflow-hidden",

            // Header
            div {
                class: "flex items-center justify-between px-6 py-4 border-b border-slate-700",
                div {
                    class: "flex items-center gap-3",
                    h2 {
                        class: "text-xl font-bold text-white",
                        "Content Editor"
                    }
                    span {
                        class: "px-2 py-1 text-xs bg-orange-600 text-white rounded",
                        "Phase 2.4"
                    }
                }
                div {
                    class: "flex items-center gap-2",
                    // Preview toggle
                    button {
                        class: if show_preview() {
                            "px-3 py-1.5 text-sm bg-blue-600 text-white rounded"
                        } else {
                            "px-3 py-1.5 text-sm bg-slate-700 text-slate-300 rounded hover:bg-slate-600"
                        },
                        onclick: move |_| show_preview.set(!show_preview()),
                        "Preview"
                    }
                    // Export button
                    button {
                        class: "px-3 py-1.5 text-sm bg-green-600 text-white rounded hover:bg-green-700",
                        onclick: handle_export_markdown,
                        "Export MD"
                    }
                }
            }

            // Main content area - three columns
            div {
                class: "flex-1 flex overflow-hidden",

                // Left column - Sources
                div {
                    class: "w-64 flex-shrink-0 border-r border-slate-700 overflow-y-auto",

                    // Templates section
                    div {
                        class: "p-4 border-b border-slate-700",
                        h3 {
                            class: "text-sm font-semibold text-slate-300 mb-3",
                            "Templates"
                        }
                        div {
                            class: "space-y-1",
                            for template in templates.read().iter() {
                                button {
                                    key: "{template.id}",
                                    class: if selected_template.read().as_ref().map(|t| t.id == template.id).unwrap_or(false) {
                                        "w-full text-left px-3 py-2 rounded bg-orange-600 text-white text-sm"
                                    } else {
                                        "w-full text-left px-3 py-2 rounded hover:bg-slate-700 text-slate-300 text-sm"
                                    },
                                    onclick: {
                                        let t = template.clone();
                                        move |_| handle_select_template(t.clone())
                                    },
                                    div {
                                        class: "flex items-center gap-2",
                                        span { "{template.platform.icon()}" }
                                        span { "{template.name}" }
                                    }
                                }
                            }
                        }
                    }

                    // RSS Import section
                    div {
                        class: "p-4 border-b border-slate-700",
                        h3 {
                            class: "text-sm font-semibold text-slate-300 mb-3",
                            "RSS Import"
                        }
                        div {
                            class: "space-y-2",
                            input {
                                class: "w-full px-3 py-2 bg-slate-700 border border-slate-600 rounded text-white text-sm placeholder-slate-400",
                                placeholder: "RSS Feed URL",
                                value: "{rss_url}",
                                oninput: move |e| rss_url.set(e.value()),
                            }
                            button {
                                class: "w-full px-3 py-2 bg-blue-600 text-white text-sm rounded hover:bg-blue-700",
                                disabled: is_generating(),
                                onclick: handle_fetch_rss,
                                if is_generating() { "Fetching..." } else { "Fetch RSS" }
                            }
                        }
                        // RSS entries list
                        if !rss_entries.read().is_empty() {
                            div {
                                class: "mt-3 space-y-1 max-h-40 overflow-y-auto",
                                for (title, url, _summary) in rss_entries.read().iter() {
                                    button {
                                        class: "w-full text-left px-2 py-1.5 text-xs text-slate-300 hover:bg-slate-700 rounded truncate",
                                        onclick: {
                                            let url = url.clone();
                                            move |_| article_url.set(url.clone())
                                        },
                                        "{title}"
                                    }
                                }
                            }
                        }
                    }

                    // URL Import section
                    div {
                        class: "p-4",
                        h3 {
                            class: "text-sm font-semibold text-slate-300 mb-3",
                            "Article URL"
                        }
                        div {
                            class: "space-y-2",
                            input {
                                class: "w-full px-3 py-2 bg-slate-700 border border-slate-600 rounded text-white text-sm placeholder-slate-400",
                                placeholder: "https://...",
                                value: "{article_url}",
                                oninput: move |e| article_url.set(e.value()),
                            }
                            button {
                                class: "w-full px-3 py-2 bg-purple-600 text-white text-sm rounded hover:bg-purple-700",
                                disabled: is_generating(),
                                onclick: handle_extract_article,
                                if is_generating() { "Extracting..." } else { "Extract Article" }
                            }
                        }
                    }
                }

                // Middle column - Editor
                div {
                    class: "flex-1 flex flex-col overflow-hidden",

                    // Title input
                    div {
                        class: "p-4 border-b border-slate-700",
                        input {
                            class: "w-full px-4 py-3 bg-slate-700 border border-slate-600 rounded-lg text-white text-lg font-semibold placeholder-slate-400",
                            placeholder: "Article Title",
                            value: "{editor_content.read().title}",
                            oninput: move |e| {
                                let mut ec = editor_content.read().clone();
                                ec.title = e.value();
                                editor_content.set(ec);
                            },
                        }

                        // Generate outline button
                        div {
                            class: "mt-3 flex gap-2",
                            button {
                                class: "px-4 py-2 bg-orange-600 text-white text-sm rounded hover:bg-orange-700",
                                disabled: is_generating(),
                                onclick: handle_generate_outline,
                                if is_generating() { "Generating..." } else { "Generate Outline" }
                            }
                        }
                    }

                    // Sections editor
                    div {
                        class: "flex-1 overflow-y-auto p-4 space-y-4",

                        if editor_content.read().sections.is_empty() {
                            div {
                                class: "text-center py-12 text-slate-400",
                                p { "Select a template to start" }
                                p {
                                    class: "text-sm mt-2",
                                    "Or enter a title and click 'Generate Outline'"
                                }
                            }
                        }

                        for (index, section) in editor_content.read().sections.iter().enumerate() {
                            div {
                                key: "{section.id}",
                                class: "bg-slate-800 rounded-lg border border-slate-700",

                                // Section header
                                div {
                                    class: "flex items-center justify-between px-4 py-3 border-b border-slate-700",
                                    h4 {
                                        class: "font-medium text-white",
                                        "{section.title}"
                                    }
                                    div {
                                        class: "flex items-center gap-2",
                                        if active_section() == Some(index) {
                                            div {
                                                class: "w-4 h-4 border-2 border-orange-400 border-t-transparent rounded-full animate-spin"
                                            }
                                        }
                                        button {
                                            class: "px-3 py-1 text-xs bg-orange-600 text-white rounded hover:bg-orange-700",
                                            disabled: is_generating(),
                                            onclick: move |_| handle_expand_section(index),
                                            "Expand"
                                        }
                                    }
                                }

                                // Section content
                                div {
                                    class: "p-4",
                                    textarea {
                                        class: "w-full min-h-[150px] px-3 py-2 bg-slate-700 border border-slate-600 rounded text-white text-sm placeholder-slate-400 resize-y",
                                        placeholder: "Section content...",
                                        value: "{section.content}",
                                        oninput: {
                                            move |e| {
                                                let mut ec = editor_content.read().clone();
                                                if let Some(s) = ec.sections.get_mut(index) {
                                                    s.content = e.value();
                                                }
                                                editor_content.set(ec);
                                            }
                                        },
                                    }
                                }
                            }
                        }
                    }
                }

                // Right column - Preview (conditional)
                if show_preview() {
                    div {
                        class: "w-96 flex-shrink-0 border-l border-slate-700 overflow-y-auto p-4",
                        h3 {
                            class: "text-sm font-semibold text-slate-300 mb-4",
                            "Preview"
                        }
                        div {
                            class: "prose prose-invert prose-sm max-w-none",
                            dangerous_inner_html: "{editor_content.read().to_html()}"
                        }

                        // Word count
                        div {
                            class: "mt-4 pt-4 border-t border-slate-700 text-sm text-slate-400",
                            "Word count: {editor_content.read().word_count()}"
                        }
                    }
                }
            }

            // Error message
            if let Some(err) = error_message() {
                div {
                    class: "px-6 py-3 bg-red-900/50 border-t border-red-700 text-red-300 text-sm",
                    "{err}"
                }
            }
        }
    }
}
