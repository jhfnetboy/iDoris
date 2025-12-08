//! Settings Panel Component

use dioxus::prelude::*;
use crate::models::{AppSettings, ResponseLanguage, Theme, FontSize, ModelInfo, ModelStatus};
use crate::server_functions::{
    list_context_files, add_context_document, delete_context_document, reload_context_database, ContextFile,
    list_available_models, get_current_model, switch_llm_model
};

#[component]
pub fn SettingsPanel(
    settings: Signal<AppSettings>,
    show_settings: Signal<bool>,
) -> Element {
    if !show_settings() {
        return rsx! {};
    }

    let current = settings.read().clone();

    rsx! {
        // Backdrop
        div {
            class: "fixed inset-0 bg-black/50 backdrop-blur-sm z-40",
            onclick: move |_| show_settings.set(false),
        }

        // Settings panel
        div {
            class: "fixed left-64 top-0 bottom-0 w-80 bg-slate-800 border-r border-slate-700 z-50 shadow-xl overflow-y-auto",

            // Header
            div {
                class: "flex items-center justify-between p-4 border-b border-slate-700",
                h2 {
                    class: "text-lg font-semibold text-white",
                    "Settings"
                }
                button {
                    class: "p-1 rounded hover:bg-slate-700 transition-colors",
                    onclick: move |_| show_settings.set(false),
                    svg {
                        class: "w-5 h-5 text-slate-400",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        view_box: "0 0 24 24",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M6 18L18 6M6 6l12 12"
                        }
                    }
                }
            }

            // Settings content
            div {
                class: "p-4 space-y-6",

                // Model Selection (Dynamic)
                ModelSelector { settings: settings }

                // Response Language
                div {
                    class: "space-y-2",
                    label {
                        class: "block text-sm font-medium text-slate-300",
                        "Response Language"
                    }
                    div {
                        class: "space-y-1",
                        { render_language_option(settings.clone(), ResponseLanguage::Chinese, "中文 (Chinese)", current.language == ResponseLanguage::Chinese) }
                        { render_language_option(settings.clone(), ResponseLanguage::English, "English", current.language == ResponseLanguage::English) }
                        { render_language_option(settings.clone(), ResponseLanguage::Spanish, "Español (Spanish)", current.language == ResponseLanguage::Spanish) }
                        { render_language_option(settings.clone(), ResponseLanguage::French, "Français (French)", current.language == ResponseLanguage::French) }
                        { render_language_option(settings.clone(), ResponseLanguage::German, "Deutsch (German)", current.language == ResponseLanguage::German) }
                        { render_language_option(settings.clone(), ResponseLanguage::Thai, "ไทย (Thai)", current.language == ResponseLanguage::Thai) }
                    }
                }

                // Theme
                div {
                    class: "space-y-2",
                    label {
                        class: "block text-sm font-medium text-slate-300",
                        "Theme"
                    }
                    div {
                        class: "grid grid-cols-2 gap-2",
                        { render_theme_option(settings.clone(), Theme::Dark, "Dark", current.theme == Theme::Dark) }
                        { render_theme_option(settings.clone(), Theme::Light, "Light", current.theme == Theme::Light) }
                        { render_theme_option(settings.clone(), Theme::Blue, "Blue", current.theme == Theme::Blue) }
                        { render_theme_option(settings.clone(), Theme::Purple, "Purple", current.theme == Theme::Purple) }
                    }
                }

                // Font Size
                div {
                    class: "space-y-2",
                    label {
                        class: "block text-sm font-medium text-slate-300",
                        "Response Font Size"
                    }
                    div {
                        class: "space-y-1",
                        { render_font_size_option(settings.clone(), FontSize::Small, current.font_size == FontSize::Small) }
                        { render_font_size_option(settings.clone(), FontSize::Medium, current.font_size == FontSize::Medium) }
                        { render_font_size_option(settings.clone(), FontSize::Large, current.font_size == FontSize::Large) }
                        { render_font_size_option(settings.clone(), FontSize::ExtraLarge, current.font_size == FontSize::ExtraLarge) }
                    }
                }

                // Divider
                div {
                    class: "border-t border-slate-600 my-4"
                }

                // Context Management (RAG)
                ContextManager {}
            }
        }
    }
}

/// Context Manager Component for RAG documents
#[component]
fn ContextManager() -> Element {
    let mut context_files: Signal<Vec<ContextFile>> = use_signal(Vec::new);
    let mut show_add_form: Signal<bool> = use_signal(|| false);
    let mut new_title: Signal<String> = use_signal(String::new);
    let mut new_content: Signal<String> = use_signal(String::new);
    let mut status_message: Signal<Option<String>> = use_signal(|| None);
    let mut is_loading: Signal<bool> = use_signal(|| false);

    // Load context files on mount
    use_effect(move || {
        spawn(async move {
            match list_context_files().await {
                Ok(files) => context_files.set(files),
                Err(e) => println!("Error loading context files: {:?}", e),
            }
        });
    });

    // Reload files function
    let reload_files = move || {
        spawn(async move {
            match list_context_files().await {
                Ok(files) => context_files.set(files),
                Err(e) => println!("Error reloading context files: {:?}", e),
            }
        });
    };

    rsx! {
        div {
            class: "space-y-3",

            // Header
            div {
                class: "flex items-center justify-between",
                label {
                    class: "block text-sm font-medium text-slate-300",
                    "Context Documents (RAG)"
                }
                button {
                    class: "text-xs px-2 py-1 bg-blue-600 hover:bg-blue-700 rounded text-white transition-colors",
                    onclick: move |_| show_add_form.set(!show_add_form()),
                    if show_add_form() { "Cancel" } else { "+ Add" }
                }
            }

            // Add form
            if show_add_form() {
                div {
                    class: "space-y-2 p-3 bg-slate-700 rounded-lg",
                    input {
                        class: "w-full px-3 py-2 bg-slate-600 border border-slate-500 rounded text-white text-sm placeholder-slate-400 focus:outline-none focus:border-blue-500",
                        r#type: "text",
                        placeholder: "Document title...",
                        value: "{new_title}",
                        oninput: move |e| new_title.set(e.value()),
                    }
                    textarea {
                        class: "w-full px-3 py-2 bg-slate-600 border border-slate-500 rounded text-white text-sm placeholder-slate-400 focus:outline-none focus:border-blue-500 resize-none",
                        rows: "6",
                        placeholder: "Paste your context content here...\n\nThis can be documentation, knowledge base articles, or any text you want the AI to reference.",
                        value: "{new_content}",
                        oninput: move |e| new_content.set(e.value()),
                    }
                    div {
                        class: "flex gap-2",
                        button {
                            class: "flex-1 px-3 py-2 bg-green-600 hover:bg-green-700 rounded text-white text-sm transition-colors disabled:opacity-50",
                            disabled: is_loading() || new_title().trim().is_empty() || new_content().trim().is_empty(),
                            onclick: move |_| {
                                let title = new_title().trim().to_string();
                                let content = new_content().trim().to_string();
                                if !title.is_empty() && !content.is_empty() {
                                    is_loading.set(true);
                                    spawn(async move {
                                        match add_context_document(title, content).await {
                                            Ok(_) => {
                                                status_message.set(Some("Document added! Reload database to use it.".to_string()));
                                                new_title.set(String::new());
                                                new_content.set(String::new());
                                                show_add_form.set(false);
                                                // Reload file list
                                                if let Ok(files) = list_context_files().await {
                                                    context_files.set(files);
                                                }
                                            }
                                            Err(e) => {
                                                status_message.set(Some(format!("Error: {}", e)));
                                            }
                                        }
                                        is_loading.set(false);
                                    });
                                }
                            },
                            if is_loading() { "Saving..." } else { "Save Document" }
                        }
                    }
                }
            }

            // Status message
            if let Some(msg) = status_message() {
                div {
                    class: "text-xs p-2 bg-slate-700 rounded text-slate-300",
                    "{msg}"
                    button {
                        class: "ml-2 text-slate-400 hover:text-white",
                        onclick: move |_| status_message.set(None),
                        "×"
                    }
                }
            }

            // File list
            div {
                class: "space-y-1 max-h-48 overflow-y-auto",
                if context_files().is_empty() {
                    div {
                        class: "text-sm text-slate-400 italic py-2",
                        "No context documents. Add some to enable RAG."
                    }
                } else {
                    for file in context_files() {
                        div {
                            key: "{file.name}",
                            class: "flex items-center justify-between p-2 bg-slate-700 rounded text-sm",
                            div {
                                class: "flex-1 min-w-0",
                                div {
                                    class: "text-white truncate",
                                    "{file.name}"
                                }
                                div {
                                    class: "text-xs text-slate-400 truncate",
                                    "{file.preview}"
                                }
                            }
                            button {
                                class: "ml-2 p-1 text-red-400 hover:text-red-300 hover:bg-slate-600 rounded transition-colors",
                                onclick: {
                                    let filename = file.name.clone();
                                    move |_| {
                                        let filename = filename.clone();
                                        spawn(async move {
                                            if let Ok(_) = delete_context_document(filename).await {
                                                if let Ok(files) = list_context_files().await {
                                                    context_files.set(files);
                                                }
                                                status_message.set(Some("Document deleted. Reload database to apply.".to_string()));
                                            }
                                        });
                                    }
                                },
                                svg {
                                    class: "w-4 h-4",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    view_box: "0 0 24 24",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        d: "M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Reload database button
            button {
                class: "w-full px-3 py-2 bg-slate-700 hover:bg-slate-600 rounded text-sm text-slate-300 transition-colors flex items-center justify-center gap-2",
                disabled: is_loading(),
                onclick: move |_| {
                    is_loading.set(true);
                    status_message.set(Some("Reloading context database...".to_string()));
                    spawn(async move {
                        match reload_context_database().await {
                            Ok(msg) => {
                                status_message.set(Some(msg));
                            }
                            Err(e) => {
                                status_message.set(Some(format!("Reload failed: {}", e)));
                            }
                        }
                        is_loading.set(false);
                    });
                },
                svg {
                    class: if is_loading() { "w-4 h-4 animate-spin" } else { "w-4 h-4" },
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    view_box: "0 0 24 24",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        d: "M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
                    }
                }
                "Reload Context Database"
            }

            // Help text
            div {
                class: "text-xs text-slate-500",
                "Add documents to give AI context. Enable 'Use Context' in chat to use RAG."
            }
        }
    }
}

fn render_language_option(mut settings: Signal<AppSettings>, lang: ResponseLanguage, label: &str, is_selected: bool) -> Element {
    let lang_clone = lang.clone();
    rsx! {
        button {
            class: if is_selected {
                "w-full px-3 py-2 text-left rounded-lg bg-blue-600 text-white"
            } else {
                "w-full px-3 py-2 text-left rounded-lg bg-slate-700 text-slate-300 hover:bg-slate-600 transition-colors"
            },
            onclick: move |_| {
                let mut s = settings.read().clone();
                s.language = lang_clone.clone();
                settings.set(s);
            },
            "{label}"
        }
    }
}

fn render_theme_option(mut settings: Signal<AppSettings>, theme: Theme, label: &str, is_selected: bool) -> Element {
    let theme_clone = theme.clone();
    let preview_class = match theme {
        Theme::Dark => "bg-gray-900",
        Theme::Light => "bg-gray-100",
        Theme::Blue => "bg-slate-900",
        Theme::Purple => "bg-purple-950",
    };

    rsx! {
        button {
            class: if is_selected {
                "flex items-center gap-2 px-3 py-2 rounded-lg bg-blue-600 text-white"
            } else {
                "flex items-center gap-2 px-3 py-2 rounded-lg bg-slate-700 text-slate-300 hover:bg-slate-600 transition-colors"
            },
            onclick: move |_| {
                let mut s = settings.read().clone();
                s.theme = theme_clone.clone();
                settings.set(s);
            },
            div {
                class: "w-4 h-4 rounded border border-slate-500 {preview_class}"
            }
            "{label}"
        }
    }
}

fn render_font_size_option(mut settings: Signal<AppSettings>, size: FontSize, is_selected: bool) -> Element {
    let size_clone = size.clone();
    let (label, sample_class) = match size {
        FontSize::Small => ("Small", "text-sm"),
        FontSize::Medium => ("Medium", "text-base"),
        FontSize::Large => ("Large", "text-lg"),
        FontSize::ExtraLarge => ("Extra Large", "text-xl"),
    };

    rsx! {
        button {
            class: if is_selected {
                "w-full px-3 py-2 text-left rounded-lg bg-blue-600 text-white flex items-center justify-between"
            } else {
                "w-full px-3 py-2 text-left rounded-lg bg-slate-700 text-slate-300 hover:bg-slate-600 transition-colors flex items-center justify-between"
            },
            onclick: move |_| {
                let mut s = settings.read().clone();
                s.font_size = size_clone.clone();
                settings.set(s);
            },
            span { "{label}" }
            span {
                class: sample_class,
                "Aa"
            }
        }
    }
}

/// Model Selector Component for LLM model selection (Phase 2.1)
#[component]
fn ModelSelector(settings: Signal<AppSettings>) -> Element {
    let mut available_models: Signal<Vec<ModelInfo>> = use_signal(Vec::new);
    let mut current_model: Signal<Option<ModelInfo>> = use_signal(|| None);
    let mut status_message: Signal<Option<String>> = use_signal(|| None);
    let mut is_loading: Signal<bool> = use_signal(|| false);

    // Load available models and current model on mount
    use_effect(move || {
        spawn(async move {
            // Load available models
            match list_available_models().await {
                Ok(models) => available_models.set(models),
                Err(e) => println!("Error loading available models: {:?}", e),
            }

            // Load current model
            match get_current_model().await {
                Ok(model) => current_model.set(Some(model)),
                Err(e) => println!("Error loading current model: {:?}", e),
            }
        });
    });

    rsx! {
        div {
            class: "space-y-3",

            // Header
            label {
                class: "block text-sm font-medium text-slate-300",
                "AI Model"
            }

            // Current model info
            if let Some(model) = current_model() {
                div {
                    class: "p-3 bg-slate-700 rounded-lg",
                    div {
                        class: "flex items-center justify-between",
                        div {
                            class: "text-white font-medium",
                            "{model.name}"
                        }
                        div {
                            class: "text-xs px-2 py-1 rounded bg-green-600 text-white",
                            "Active"
                        }
                    }
                    div {
                        class: "text-xs text-slate-400 mt-1",
                        "Size: {model.size} | Memory: {model.memory_required}"
                    }
                    div {
                        class: "text-xs text-slate-500 mt-1",
                        "{model.description}"
                    }
                }
            }

            // Status message
            if let Some(msg) = status_message() {
                div {
                    class: "text-xs p-2 bg-amber-900/50 border border-amber-700 rounded text-amber-200",
                    "{msg}"
                    button {
                        class: "ml-2 text-amber-400 hover:text-white",
                        onclick: move |_| status_message.set(None),
                        "×"
                    }
                }
            }

            // Available models list
            div {
                class: "space-y-2",
                label {
                    class: "block text-xs text-slate-400",
                    "Available Models"
                }
                for model in available_models() {
                    {
                        let model_id = model.id.clone();
                        let model_name = model.name.clone();
                        let is_current = current_model().map(|m| m.id == model.id).unwrap_or(false);

                        rsx! {
                            button {
                                key: "{model.id}",
                                class: if is_current {
                                    "w-full p-3 text-left rounded-lg bg-blue-600/30 border border-blue-500 text-white"
                                } else {
                                    "w-full p-3 text-left rounded-lg bg-slate-700 hover:bg-slate-600 border border-slate-600 text-slate-300 transition-colors"
                                },
                                disabled: is_loading() || is_current,
                                onclick: move |_| {
                                    let model_id = model_id.clone();
                                    let model_name = model_name.clone();
                                    is_loading.set(true);
                                    status_message.set(Some(format!("Switching to {}...", model_name)));
                                    spawn(async move {
                                        match switch_llm_model(model_id.clone()).await {
                                            Ok(_) => {
                                                // Refresh current model
                                                if let Ok(model) = get_current_model().await {
                                                    current_model.set(Some(model));
                                                    status_message.set(Some(format!("{} is now active", model_name)));
                                                }
                                            }
                                            Err(e) => {
                                                status_message.set(Some(format!("Cannot switch: {}", e)));
                                            }
                                        }
                                        is_loading.set(false);
                                    });
                                },
                                div {
                                    class: "flex items-center justify-between",
                                    div {
                                        span {
                                            class: "font-medium",
                                            "{model.name}"
                                        }
                                        if is_current {
                                            span {
                                                class: "ml-2 text-xs text-blue-300",
                                                "(current)"
                                            }
                                        }
                                    }
                                    span {
                                        class: "text-xs text-slate-400",
                                        "{model.size}"
                                    }
                                }
                                div {
                                    class: "text-xs text-slate-500 mt-1",
                                    "{model.description}"
                                }
                                div {
                                    class: "text-xs text-slate-600 mt-1",
                                    "Memory: {model.memory_required}"
                                }
                            }
                        }
                    }
                }
            }

            // Note about model switching
            div {
                class: "text-xs text-slate-500 p-2 bg-slate-800 rounded border border-slate-700",
                "⚠ Model switching requires app restart. Select your preferred model before initializing."
            }
        }
    }
}
