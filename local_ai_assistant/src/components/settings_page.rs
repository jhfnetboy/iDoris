//! Settings Page Component - Full-page settings view

use dioxus::prelude::*;
use crate::models::{AppSettings, ResponseLanguage, Theme, FontSize, ModelInfo, ModelType};
use crate::server_functions::{
    list_context_files, add_context_document, delete_context_document, reload_context_database, ContextFile,
    is_image_model_ready, init_image_model,
    list_cached_models, download_model,
};


// Helper function to format size
fn format_size(size_mb: u64) -> String {
    if size_mb < 1024 {
        format!("{} MB", size_mb)
    } else {
        format!("{:.1} GB", size_mb as f64 / 1024.0)
    }
}

/// Settings page tabs
#[derive(Clone, PartialEq, Default)]
pub enum SettingsTab {
    #[default]
    Models,
    Appearance,
    Language,
    Context,
    Database,
    About,
}

/// Full-page settings component
#[component]
pub fn SettingsPage(
    settings: Signal<AppSettings>,
    on_close: EventHandler<()>,
) -> Element {
    let active_tab: Signal<SettingsTab> = use_signal(SettingsTab::default);

    rsx! {
        div {
            class: "fixed inset-0 bg-slate-900 z-50 flex flex-col",

            // Header
            div {
                class: "flex items-center justify-between px-6 py-4 border-b border-slate-700",
                div {
                    class: "flex items-center gap-3",
                    svg {
                        class: "w-6 h-6 text-slate-400",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        view_box: "0 0 24 24",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"
                        }
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M15 12a3 3 0 11-6 0 3 3 0 016 0z"
                        }
                    }
                    h1 {
                        class: "text-xl font-semibold text-white",
                        "Settings"
                    }
                }
                button {
                    class: "p-2 rounded-lg hover:bg-slate-700 transition-colors text-slate-400 hover:text-white",
                    onclick: move |_| on_close.call(()),
                    svg {
                        class: "w-6 h-6",
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

            // Main content
            div {
                class: "flex flex-1 overflow-hidden",

                // Sidebar navigation
                nav {
                    class: "w-56 bg-slate-800/50 border-r border-slate-700 p-4 space-y-1",

                    // Back button at the top
                    button {
                        class: "w-full flex items-center gap-3 px-3 py-2 mb-4 rounded-lg text-slate-300 hover:bg-slate-700 transition-colors border border-slate-600 hover:border-slate-500",
                        onclick: move |_| on_close.call(()),
                        svg {
                            class: "w-5 h-5",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            view_box: "0 0 24 24",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                d: "M10 19l-7-7m0 0l7-7m-7 7h18"
                            }
                        }
                        "Back"
                    }

                    { render_nav_item(active_tab.clone(), SettingsTab::Models, "Models", "M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z") }
                    { render_nav_item(active_tab.clone(), SettingsTab::Appearance, "Appearance", "M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zm0 0h12a2 2 0 002-2v-4a2 2 0 00-2-2h-2.343M11 7.343l1.657-1.657a2 2 0 012.828 0l2.829 2.829a2 2 0 010 2.828l-8.486 8.485M7 17h.01") }
                    { render_nav_item(active_tab.clone(), SettingsTab::Language, "Language", "M3 5h12M9 3v2m1.048 9.5A18.022 18.022 0 016.412 9m6.088 9h7M11 21l5-10 5 10M12.751 5C11.783 10.77 8.07 15.61 3 18.129") }
                    { render_nav_item(active_tab.clone(), SettingsTab::Context, "Context (RAG)", "M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z") }
                    { render_nav_item(active_tab.clone(), SettingsTab::Database, "Database", "M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4M4 7c0-2.21 3.582-4 8-4s8 1.79 8 4m0 5c0 2.21-3.582 4-8 4s-8-1.79-8-4") }
                    { render_nav_item(active_tab.clone(), SettingsTab::About, "About", "M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z") }
                }

                // Content area
                div {
                    class: "flex-1 overflow-y-auto p-6",
                    match active_tab() {
                        SettingsTab::Models => rsx! { ModelsSettings { settings: settings } },
                        SettingsTab::Appearance => rsx! { AppearanceSettings { settings: settings } },
                        SettingsTab::Language => rsx! { LanguageSettings { settings: settings } },
                        SettingsTab::Context => rsx! { ContextSettings {} },
                        SettingsTab::Database => rsx! { DatabaseSettings {} },
                        SettingsTab::About => rsx! { AboutSettings {} },
                    }
                }
            }
        }
    }
}

fn render_nav_item(mut active_tab: Signal<SettingsTab>, tab: SettingsTab, label: &str, icon_path: &str) -> Element {
    let is_active = active_tab() == tab;
    let tab_clone = tab.clone();

    rsx! {
        button {
            class: if is_active {
                "w-full flex items-center gap-3 px-3 py-2 rounded-lg bg-blue-600 text-white"
            } else {
                "w-full flex items-center gap-3 px-3 py-2 rounded-lg text-slate-300 hover:bg-slate-700 transition-colors"
            },
            onclick: move |_| active_tab.set(tab_clone.clone()),
            svg {
                class: "w-5 h-5",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                view_box: "0 0 24 24",
                path {
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    d: "{icon_path}"
                }
            }
            "{label}"
        }
    }
}

/// Models settings section - Chat Model and Image Gen Model
#[component]
fn ModelsSettings(settings: Signal<AppSettings>) -> Element {
    let current = settings.read().clone();
    let mut image_model_ready: Signal<bool> = use_signal(|| false);
    let mut image_model_downloading: Signal<bool> = use_signal(|| false);
    let mut download_status: Signal<String> = use_signal(|| "Not downloaded".to_string());
    let mut download_progress: Signal<u8> = use_signal(|| 0);
    let mut voice_model_ready: Signal<bool> = use_signal(|| false);

    // LLM model states
    let mut models: Signal<Vec<ModelInfo>> = use_signal(|| Vec::new());
    let mut llm_downloading: Signal<bool> = use_signal(|| false);
    let mut llm_status: Signal<String> = use_signal(|| "Checking models...".to_string());

    // Check image model status on mount
    use_effect(move || {
        spawn(async move {
            match is_image_model_ready().await {
                Ok(ready) => {
                    image_model_ready.set(ready);
                    if ready {
                        download_status.set("Ready".to_string());
                    }
                }
                Err(_) => image_model_ready.set(false),
            }
        });
    });

    // Check voice model status on mount
    use_effect(move || {
        spawn(async move {
            match crate::server_functions::is_vibevoice_available().await {
                Ok(ready) => voice_model_ready.set(ready),
                Err(_) => voice_model_ready.set(false),
            }
        });
    });

    // Load LLM models on mount
    {
        let mut models = models.clone();
        let mut llm_status = llm_status.clone();
        use_effect(move || {
            spawn(async move {
                llm_status.set("Loading models...".to_string());
                match list_cached_models().await {
                    Ok(model_list) => {
                        let llm_models: Vec<ModelInfo> = model_list
                            .into_iter()
                            .filter(|m| matches!(m.model_type, ModelType::Language))
                            .collect();
                        models.set(llm_models);
                        llm_status.set("Models loaded".to_string());
                    }
                    Err(e) => {
                        llm_status.set(format!("Error: {}", e));
                    }
                }
            });
        });
    }

    rsx! {
        div {
            class: "max-w-2xl space-y-8",

            h2 {
                class: "text-lg font-semibold text-white mb-4",
                "Model Management"
            }

            // Chat Model Section
            div {
                class: "bg-slate-800 rounded-lg p-4 space-y-4",
                div {
                    class: "flex items-center gap-2 mb-3",
                    svg {
                        class: "w-5 h-5 text-blue-400",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        view_box: "0 0 24 24",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"
                        }
                    }
                    h3 {
                        class: "text-md font-medium text-white",
                        "Chat Model (LLM)"
                    }
                }

                p {
                    class: "text-xs text-slate-400 mb-3",
                    "Select the language model for generating chat responses"
                }

                select {
                    class: "w-full px-4 py-3 bg-slate-700 border border-slate-600 rounded-lg text-white focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500",
                    value: "{current.model_name}",
                    onchange: {
                        let mut settings = settings.clone();
                        move |e: Event<FormData>| {
                            let mut s = settings.read().clone();
                            s.model_name = e.value();
                            settings.set(s);
                        }
                    },
                    option { value: "Qwen 2.5 7B", "Qwen 2.5 7B (Recommended)" }
                    option { value: "Qwen 2.5 3B", "Qwen 2.5 3B (Faster)" }
                    option { value: "Llama 3.2 8B", "Llama 3.2 8B" }
                }

                div {
                    class: "mt-3 p-3 bg-slate-700/50 rounded-lg space-y-2",

                    // Show available LLM models
                    {
                        let current_models = models();
                    current_models.iter().map(|model| {
                        let model_id = model.id.clone();
                        let model_name = model.name.clone();
                        let model_cached = model.is_cached;
                        rsx! {
                            div {
                                class: "flex items-center justify-between p-2 bg-slate-600/50 rounded",
                                div {
                                    class: "flex items-center gap-2",
                                    div {
                                        class: if model_cached {
                                            "w-2 h-2 rounded-full bg-green-500"
                                        } else {
                                            "w-2 h-2 rounded-full bg-yellow-500"
                                        }
                                    }
                                    div {
                                        h4 {
                                            class: "text-sm font-medium text-white",
                                            "{model.name}"
                                        }
                                    p {
                                        class: "text-xs text-slate-400",
                                        "{model.description}"
                                    }
                                    if let Some(size) = model.size_mb {
                                        p {
                                            class: "text-xs text-slate-500",
                                            "Size: {format_size(size)}"
                                        }
                                    }
                                    }
                                    if !model_cached && !llm_downloading() {
                                    button {
                                        class: "px-3 py-1 text-xs bg-blue-600 hover:bg-blue-700 text-white rounded",
                                                onclick: move |_| {
                                            let mut models = models.clone();
                                            let mut llm_downloading = llm_downloading.clone();
                                            let mut llm_status = llm_status.clone();
                                            let model_id = model_id.clone();
                                            let model_name = model_name.clone();

                                            spawn(async move {
                                                llm_downloading.set(true);
                                                llm_status.set(format!("Downloading {}...", model_name));

                                                match download_model(model_id).await {
                                                Ok(_) => {
                                                    llm_status.set("Download complete".to_string());
                                                    // Refresh models
                                                    match list_cached_models().await {
                                                        Ok(model_list) => {
                                                            let llm_models: Vec<ModelInfo> = model_list
                                                                .into_iter()
                                                                .filter(|m| matches!(m.model_type, ModelType::Language))
                                                                .collect();
                                                            models.set(llm_models);
                                                        }
                                                        Err(e) => {
                                                            llm_status.set(format!("Error refreshing models: {}", e));
                                                        }
                                                    }
                                                }
                                                Err(e) => {
                                                    llm_status.set(format!("Download failed: {}", e));
                                                }
                                            }
                                            llm_downloading.set(false);
                                        });
                                    },
                                    "Download"
                                }
                            } else if model.is_cached {
                                span {
                                    class: "text-xs text-green-400",
                                    "Cached"
                                        }
                            }
                        }
                        }
                        }
                    }).collect::<Vec<_>>().into_iter()
                    }

                    // Show download status
                    if llm_downloading() {
                        div {
                            class: "flex items-center gap-2 text-sm text-blue-400",
                            div {
                                class: "w-2 h-2 rounded-full bg-blue-500 animate-pulse"
                            }
                            span { "{llm_status()}" }
                        }
                    } else {
                        div {
                            class: "flex items-center gap-2 text-sm",
                            div {
                                class: "w-2 h-2 rounded-full bg-green-500"
                            }
                            span { class: "text-slate-300", "Models ready for use" }
                        }
                    }

                    // Show overall status
                    div {
                        class: "text-xs text-slate-400",
                        "{llm_status()}"
                    }
                }
            }

            // Image Generation Model Section (MFLUX)
            div {
                class: "bg-slate-800 rounded-lg p-4 space-y-4",
                div {
                    class: "flex items-center gap-2 mb-3",
                    svg {
                        class: "w-5 h-5 text-purple-400",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        view_box: "0 0 24 24",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"
                        }
                    }
                    h3 {
                        class: "text-md font-medium text-white",
                        "Image Generation (MFLUX)"
                    }
                }

                p {
                    class: "text-xs text-slate-400 mb-3",
                    "FLUX models for high-quality image generation (Apple Silicon optimized)"
                }

                // Model info card
                div {
                    class: "p-3 bg-slate-700/50 rounded-lg space-y-2",
                    div {
                        class: "flex justify-between text-sm",
                        span { class: "text-slate-400", "Backend" }
                        span { class: "text-white", "MFLUX (MLX)" }
                    }
                    div {
                        class: "flex justify-between text-sm",
                        span { class: "text-slate-400", "Models" }
                        span { class: "text-white", "schnell / dev / z-image-turbo" }
                    }
                    div {
                        class: "flex justify-between text-sm",
                        span { class: "text-slate-400", "Status" }
                        span {
                            class: if image_model_ready() { "text-green-400" } else { "text-yellow-400" },
                            if image_model_ready() { "Ready" } else { "{download_status}" }
                        }
                    }
                }

                // Installation instructions
                if !image_model_ready() {
                    div {
                        class: "p-3 bg-yellow-900/30 border border-yellow-800 rounded-lg space-y-2",
                        p {
                            class: "text-sm text-yellow-200 font-medium",
                            "Installation Required"
                        }
                        p {
                            class: "text-xs text-yellow-300/80",
                            "Run in Terminal:"
                        }
                        code {
                            class: "block p-2 bg-slate-900 rounded text-purple-400 text-sm font-mono",
                            "uv tool install mflux"
                        }
                        p {
                            class: "text-xs text-yellow-300/70 mt-2",
                            "Models download automatically on first use (~10GB for schnell)"
                        }
                        p {
                            class: "text-xs text-green-300/80 mt-2",
                            "Tip: Use 'z-image-turbo' model - no HuggingFace login required!"
                        }
                    }

                    // Check status button
                    button {
                        class: "w-full px-4 py-3 bg-purple-600 hover:bg-purple-700 rounded-lg text-white font-medium transition-colors flex items-center justify-center gap-2",
                        onclick: move |_| {
                            image_model_downloading.set(true);
                            download_status.set("Checking...".to_string());
                            spawn(async move {
                                match init_image_model().await {
                                    Ok(_) => {
                                        image_model_ready.set(true);
                                        download_status.set("Ready".to_string());
                                    }
                                    Err(e) => {
                                        download_status.set(format!("Error: {}", e));
                                    }
                                }
                                image_model_downloading.set(false);
                            });
                        },
                        svg {
                            class: "w-5 h-5",
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
                        "Check MFLUX Status"
                    }
                } else {
                    div {
                        class: "flex items-center gap-2 p-3 bg-green-900/30 border border-green-800 rounded-lg",
                        svg {
                            class: "w-5 h-5 text-green-400",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            view_box: "0 0 24 24",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                d: "M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
                            }
                        }
                        span { class: "text-green-300 text-sm", "MFLUX installed and ready" }
                    }
                }
            }

            // Voice Model Section (VibeVoice)
            div {
                class: "bg-slate-800 rounded-lg p-4 space-y-4",
                div {
                    class: "flex items-center gap-2 mb-3",
                    svg {
                        class: "w-5 h-5 text-green-400",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        view_box: "0 0 24 24",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M19 11a7 7 0 01-7 7m0 0a7 7 0 01-7-7m7 7v4m0 0H8m4 0h4m-4-8a3 3 0 01-3-3V5a3 3 0 116 0v6a3 3 0 01-3 3z"
                        }
                    }
                    h3 {
                        class: "text-md font-medium text-white",
                        "Voice Model (VibeVoice)"
                    }
                }

                p {
                    class: "text-xs text-slate-400 mb-3",
                    "Microsoft VibeVoice-Realtime-0.5B for natural text-to-speech"
                }

                // Model info card
                div {
                    class: "p-3 bg-slate-700/50 rounded-lg space-y-2",
                    div {
                        class: "flex justify-between text-sm",
                        span { class: "text-slate-400", "Model" }
                        span { class: "text-white", "VibeVoice-Realtime-0.5B" }
                    }
                    div {
                        class: "flex justify-between text-sm",
                        span { class: "text-slate-400", "Size" }
                        span { class: "text-white", "~1GB" }
                    }
                    div {
                        class: "flex justify-between text-sm",
                        span { class: "text-slate-400", "Latency" }
                        span { class: "text-white", "~300ms" }
                    }
                }

                // Show status based on voice_model_ready
                if !voice_model_ready() {
                    // Installation instructions
                    div {
                        class: "p-3 bg-yellow-900/30 border border-yellow-800 rounded-lg space-y-2",
                        p {
                            class: "text-sm text-yellow-200 font-medium",
                            "Manual Download Required"
                        }
                        p {
                            class: "text-xs text-yellow-300/80",
                            "Run in Terminal:"
                        }
                        code {
                            class: "block p-2 bg-slate-900 rounded text-green-400 text-xs font-mono whitespace-pre-wrap break-all",
                            "mkdir -p ~/models && python3 -c \"from huggingface_hub import snapshot_download; snapshot_download('microsoft/VibeVoice-Realtime-0.5B', local_dir=__import__('os').path.expanduser('~/models/VibeVoice-Realtime-0.5B'))\""
                        }
                        p {
                            class: "text-xs text-yellow-300/70 mt-2",
                            "Or use System TTS (macOS built-in) as alternative"
                        }
                    }
                } else {
                    // Ready status
                    div {
                        class: "flex items-center gap-2 p-3 bg-green-900/30 border border-green-800 rounded-lg",
                        svg {
                            class: "w-5 h-5 text-green-400",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            view_box: "0 0 24 24",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                d: "M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
                            }
                        }
                        span { class: "text-green-300 text-sm", "VibeVoice model downloaded and ready" }
                    }
                }
            }

            // Info box
            div {
                class: "bg-blue-900/30 border border-blue-800 rounded-lg p-4",
                div {
                    class: "flex items-start gap-3",
                    svg {
                        class: "w-5 h-5 text-blue-400 mt-0.5",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        view_box: "0 0 24 24",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                        }
                    }
                    div {
                        class: "text-sm text-blue-200",
                        p { "Models are stored locally in the Hugging Face cache directory." }
                        p { class: "mt-1 text-blue-300/70", "Once downloaded, models persist across app restarts." }
                    }
                }
            }
        }
    }
}

/// Appearance settings section
#[component]
fn AppearanceSettings(settings: Signal<AppSettings>) -> Element {
    let current = settings.read().clone();

    rsx! {
        div {
            class: "max-w-2xl space-y-6",

            h2 {
                class: "text-lg font-semibold text-white mb-4",
                "Appearance"
            }

            // Theme Selection
            div {
                class: "bg-slate-800 rounded-lg p-4 space-y-3",
                label {
                    class: "block text-sm font-medium text-slate-300 mb-2",
                    "Theme"
                }
                div {
                    class: "grid grid-cols-2 md:grid-cols-4 gap-3",
                    { render_theme_card(settings.clone(), Theme::Dark, "Dark", "bg-gray-900", current.theme == Theme::Dark) }
                    { render_theme_card(settings.clone(), Theme::Light, "Light", "bg-gray-100", current.theme == Theme::Light) }
                    { render_theme_card(settings.clone(), Theme::Blue, "Blue", "bg-slate-900", current.theme == Theme::Blue) }
                    { render_theme_card(settings.clone(), Theme::Purple, "Purple", "bg-purple-950", current.theme == Theme::Purple) }
                }
            }

            // Font Size
            div {
                class: "bg-slate-800 rounded-lg p-4 space-y-3",
                label {
                    class: "block text-sm font-medium text-slate-300 mb-2",
                    "Response Font Size"
                }
                div {
                    class: "space-y-2",
                    { render_font_option(settings.clone(), FontSize::Small, "Small", "text-sm", current.font_size == FontSize::Small) }
                    { render_font_option(settings.clone(), FontSize::Medium, "Medium", "text-base", current.font_size == FontSize::Medium) }
                    { render_font_option(settings.clone(), FontSize::Large, "Large", "text-lg", current.font_size == FontSize::Large) }
                    { render_font_option(settings.clone(), FontSize::ExtraLarge, "Extra Large", "text-xl", current.font_size == FontSize::ExtraLarge) }
                }
            }
        }
    }
}

fn render_theme_card(mut settings: Signal<AppSettings>, theme: Theme, label: &str, preview_class: &str, is_selected: bool) -> Element {
    let theme_clone = theme.clone();

    rsx! {
        button {
            class: if is_selected {
                "flex flex-col items-center gap-2 p-3 rounded-lg border-2 border-blue-500 bg-slate-700"
            } else {
                "flex flex-col items-center gap-2 p-3 rounded-lg border border-slate-600 hover:border-slate-500 transition-colors"
            },
            onclick: move |_| {
                let mut s = settings.read().clone();
                s.theme = theme_clone.clone();
                settings.set(s);
            },
            div {
                class: "w-full h-12 rounded {preview_class} border border-slate-500"
            }
            span {
                class: "text-sm text-slate-300",
                "{label}"
            }
        }
    }
}

fn render_font_option(mut settings: Signal<AppSettings>, size: FontSize, label: &str, sample_class: &str, is_selected: bool) -> Element {
    let size_clone = size.clone();

    rsx! {
        button {
            class: if is_selected {
                "w-full flex items-center justify-between px-4 py-3 rounded-lg bg-blue-600 text-white"
            } else {
                "w-full flex items-center justify-between px-4 py-3 rounded-lg bg-slate-700 text-slate-300 hover:bg-slate-600 transition-colors"
            },
            onclick: move |_| {
                let mut s = settings.read().clone();
                s.font_size = size_clone.clone();
                settings.set(s);
            },
            span { "{label}" }
            span {
                class: sample_class,
                "Sample Aa"
            }
        }
    }
}

/// Language settings section
#[component]
fn LanguageSettings(settings: Signal<AppSettings>) -> Element {
    let current = settings.read().clone();

    rsx! {
        div {
            class: "max-w-2xl space-y-6",

            h2 {
                class: "text-lg font-semibold text-white mb-4",
                "Language Settings"
            }

            div {
                class: "bg-slate-800 rounded-lg p-4 space-y-3",
                label {
                    class: "block text-sm font-medium text-slate-300 mb-2",
                    "Response Language"
                }
                p {
                    class: "text-xs text-slate-500 mb-3",
                    "The AI will respond in this language"
                }
                div {
                    class: "grid grid-cols-1 md:grid-cols-2 gap-2",
                    { render_lang_option(settings.clone(), ResponseLanguage::Chinese, "中文", "Chinese", current.language == ResponseLanguage::Chinese) }
                    { render_lang_option(settings.clone(), ResponseLanguage::English, "English", "English", current.language == ResponseLanguage::English) }
                    { render_lang_option(settings.clone(), ResponseLanguage::Spanish, "Español", "Spanish", current.language == ResponseLanguage::Spanish) }
                    { render_lang_option(settings.clone(), ResponseLanguage::French, "Français", "French", current.language == ResponseLanguage::French) }
                    { render_lang_option(settings.clone(), ResponseLanguage::German, "Deutsch", "German", current.language == ResponseLanguage::German) }
                    { render_lang_option(settings.clone(), ResponseLanguage::Thai, "ไทย", "Thai", current.language == ResponseLanguage::Thai) }
                }
            }
        }
    }
}

fn render_lang_option(mut settings: Signal<AppSettings>, lang: ResponseLanguage, native: &str, english: &str, is_selected: bool) -> Element {
    let lang_clone = lang.clone();

    rsx! {
        button {
            class: if is_selected {
                "flex items-center gap-3 px-4 py-3 rounded-lg bg-blue-600 text-white"
            } else {
                "flex items-center gap-3 px-4 py-3 rounded-lg bg-slate-700 text-slate-300 hover:bg-slate-600 transition-colors"
            },
            onclick: move |_| {
                let mut s = settings.read().clone();
                s.language = lang_clone.clone();
                settings.set(s);
            },
            span { class: "font-medium", "{native}" }
            span { class: "text-sm opacity-70", "({english})" }
        }
    }
}

/// Context (RAG) settings section
#[component]
fn ContextSettings() -> Element {
    let mut context_files: Signal<Vec<ContextFile>> = use_signal(Vec::new);
    let mut show_add_form: Signal<bool> = use_signal(|| false);
    let mut new_title: Signal<String> = use_signal(String::new);
    let mut new_content: Signal<String> = use_signal(String::new);
    let mut status_message: Signal<Option<(String, bool)>> = use_signal(|| None); // (message, is_error)
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

    rsx! {
        div {
            class: "max-w-3xl space-y-6",

            h2 {
                class: "text-lg font-semibold text-white mb-4",
                "Context Documents (RAG)"
            }

            // Info box
            div {
                class: "bg-blue-900/30 border border-blue-800 rounded-lg p-4",
                div {
                    class: "flex items-start gap-3",
                    svg {
                        class: "w-5 h-5 text-blue-400 mt-0.5",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        view_box: "0 0 24 24",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                        }
                    }
                    div {
                        class: "text-sm text-blue-200",
                        p { "Add documents here to provide context for the AI." }
                        p { class: "mt-1 text-blue-300/70", "Enable 'Use Context' in chat to use RAG (Retrieval-Augmented Generation)." }
                    }
                }
            }

            // Status message
            if let Some((msg, is_error)) = status_message() {
                div {
                    class: if is_error {
                        "bg-red-900/50 border border-red-800 rounded-lg p-3 flex items-center justify-between"
                    } else {
                        "bg-green-900/50 border border-green-800 rounded-lg p-3 flex items-center justify-between"
                    },
                    span {
                        class: if is_error { "text-sm text-red-200" } else { "text-sm text-green-200" },
                        "{msg}"
                    }
                    button {
                        class: "text-slate-400 hover:text-white",
                        onclick: move |_| status_message.set(None),
                        "×"
                    }
                }
            }

            // Add document section
            div {
                class: "bg-slate-800 rounded-lg p-4",
                div {
                    class: "flex items-center justify-between mb-4",
                    h3 {
                        class: "text-sm font-medium text-slate-300",
                        "Documents"
                    }
                    button {
                        class: if show_add_form() {
                            "px-3 py-1.5 bg-slate-600 hover:bg-slate-500 rounded-lg text-sm text-white transition-colors"
                        } else {
                            "px-3 py-1.5 bg-blue-600 hover:bg-blue-700 rounded-lg text-sm text-white transition-colors"
                        },
                        onclick: move |_| show_add_form.set(!show_add_form()),
                        if show_add_form() { "Cancel" } else { "+ Add Document" }
                    }
                }

                // Add form
                if show_add_form() {
                    div {
                        class: "space-y-3 mb-4 p-4 bg-slate-700/50 rounded-lg",
                        input {
                            class: "w-full px-4 py-2 bg-slate-600 border border-slate-500 rounded-lg text-white placeholder-slate-400 focus:outline-none focus:border-blue-500",
                            r#type: "text",
                            placeholder: "Document title (e.g., API Documentation)",
                            value: "{new_title}",
                            oninput: move |e| new_title.set(e.value()),
                        }
                        textarea {
                            class: "w-full px-4 py-2 bg-slate-600 border border-slate-500 rounded-lg text-white placeholder-slate-400 focus:outline-none focus:border-blue-500 resize-none",
                            rows: "8",
                            placeholder: "Paste your document content here...\n\nThis can be:\n- Technical documentation\n- Knowledge base articles\n- Code references\n- Any text you want the AI to reference",
                            value: "{new_content}",
                            oninput: move |e| new_content.set(e.value()),
                        }
                        button {
                            class: "w-full px-4 py-2 bg-green-600 hover:bg-green-700 rounded-lg text-white font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed",
                            disabled: is_loading() || new_title().trim().is_empty() || new_content().trim().is_empty(),
                            onclick: move |_| {
                                let title = new_title().trim().to_string();
                                let content = new_content().trim().to_string();
                                if !title.is_empty() && !content.is_empty() {
                                    is_loading.set(true);
                                    spawn(async move {
                                        match add_context_document(title, content).await {
                                            Ok(_) => {
                                                status_message.set(Some(("Document added successfully! Click 'Reload Database' to index it.".to_string(), false)));
                                                new_title.set(String::new());
                                                new_content.set(String::new());
                                                show_add_form.set(false);
                                                if let Ok(files) = list_context_files().await {
                                                    context_files.set(files);
                                                }
                                            }
                                            Err(e) => {
                                                status_message.set(Some((format!("Error: {}", e), true)));
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

                // Document list
                div {
                    class: "space-y-2",
                    if context_files().is_empty() {
                        div {
                            class: "text-center py-8 text-slate-500",
                            svg {
                                class: "w-12 h-12 mx-auto mb-3 opacity-50",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "1.5",
                                view_box: "0 0 24 24",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    d: "M19.5 14.25v-2.625a3.375 3.375 0 00-3.375-3.375h-1.5A1.125 1.125 0 0113.5 7.125v-1.5a3.375 3.375 0 00-3.375-3.375H8.25m2.25 0H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 00-9-9z"
                                }
                            }
                            p { "No context documents yet" }
                            p { class: "text-sm mt-1", "Add documents to enable RAG" }
                        }
                    } else {
                        for file in context_files() {
                            div {
                                key: "{file.name}",
                                class: "flex items-center justify-between p-3 bg-slate-700 rounded-lg",
                                div {
                                    class: "flex-1 min-w-0",
                                    div {
                                        class: "flex items-center gap-2",
                                        svg {
                                            class: "w-4 h-4 text-slate-400",
                                            fill: "none",
                                            stroke: "currentColor",
                                            stroke_width: "2",
                                            view_box: "0 0 24 24",
                                            path {
                                                stroke_linecap: "round",
                                                stroke_linejoin: "round",
                                                d: "M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
                                            }
                                        }
                                        span {
                                            class: "text-white font-medium truncate",
                                            "{file.name}"
                                        }
                                    }
                                    p {
                                        class: "text-xs text-slate-400 mt-1 truncate",
                                        "{file.preview}"
                                    }
                                }
                                button {
                                    class: "ml-3 p-2 text-red-400 hover:text-red-300 hover:bg-red-900/30 rounded-lg transition-colors",
                                    onclick: {
                                        let filename = file.name.clone();
                                        move |_| {
                                            let filename = filename.clone();
                                            spawn(async move {
                                                if let Ok(_) = delete_context_document(filename).await {
                                                    if let Ok(files) = list_context_files().await {
                                                        context_files.set(files);
                                                    }
                                                    status_message.set(Some(("Document deleted. Click 'Reload Database' to apply.".to_string(), false)));
                                                }
                                            });
                                        }
                                    },
                                    svg {
                                        class: "w-5 h-5",
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
            }

            // Reload database button
            div {
                class: "bg-slate-800 rounded-lg p-4",
                button {
                    class: "w-full px-4 py-3 bg-slate-700 hover:bg-slate-600 rounded-lg text-white font-medium transition-colors flex items-center justify-center gap-2 disabled:opacity-50",
                    disabled: is_loading(),
                    onclick: move |_| {
                        is_loading.set(true);
                        status_message.set(Some(("Reloading context database...".to_string(), false)));
                        spawn(async move {
                            match reload_context_database().await {
                                Ok(msg) => {
                                    status_message.set(Some((msg, false)));
                                }
                                Err(e) => {
                                    status_message.set(Some((format!("Reload failed: {}", e), true)));
                                }
                            }
                            is_loading.set(false);
                        });
                    },
                    svg {
                        class: if is_loading() { "w-5 h-5 animate-spin" } else { "w-5 h-5" },
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
                p {
                    class: "text-xs text-slate-500 mt-2 text-center",
                    "Re-index all documents after adding or removing"
                }
            }
        }
    }
}

/// Database settings section
#[component]
fn DatabaseSettings() -> Element {
    rsx! {
        div {
            class: "max-w-2xl space-y-6",

            h2 {
                class: "text-lg font-semibold text-white mb-4",
                "Database Settings"
            }

            // Vector Store Info
            div {
                class: "bg-slate-800 rounded-lg p-4 space-y-3",
                h3 {
                    class: "text-sm font-medium text-slate-300 mb-3",
                    "Vector Store (RAG)"
                }
                div {
                    class: "space-y-2 text-sm",
                    div {
                        class: "flex justify-between py-2 border-b border-slate-700",
                        span { class: "text-slate-400", "Engine" }
                        span { class: "text-white", "SurrealDB + SemanticChunker" }
                    }
                    div {
                        class: "flex justify-between py-2 border-b border-slate-700",
                        span { class: "text-slate-400", "Embedding Model" }
                        span { class: "text-white", "BERT (kalosm)" }
                    }
                    div {
                        class: "flex justify-between py-2 border-b border-slate-700",
                        span { class: "text-slate-400", "Location" }
                        span { class: "text-white font-mono text-xs", "./db/" }
                    }
                    div {
                        class: "flex justify-between py-2",
                        span { class: "text-slate-400", "Context Folder" }
                        span { class: "text-white font-mono text-xs", "./context/" }
                    }
                }
            }

            // Session Storage Info
            div {
                class: "bg-slate-800 rounded-lg p-4 space-y-3",
                h3 {
                    class: "text-sm font-medium text-slate-300 mb-3",
                    "Session Storage"
                }
                div {
                    class: "space-y-2 text-sm",
                    div {
                        class: "flex justify-between py-2 border-b border-slate-700",
                        span { class: "text-slate-400", "Engine" }
                        span { class: "text-white", "SQLite" }
                    }
                    div {
                        class: "flex justify-between py-2",
                        span { class: "text-slate-400", "Location" }
                        span { class: "text-white font-mono text-xs", "./data/assistant.db" }
                    }
                }
            }

            // Warning
            div {
                class: "bg-yellow-900/30 border border-yellow-800 rounded-lg p-4",
                div {
                    class: "flex items-start gap-3",
                    svg {
                        class: "w-5 h-5 text-yellow-400 mt-0.5",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        view_box: "0 0 24 24",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
                        }
                    }
                    div {
                        class: "text-sm text-yellow-200",
                        p { "Database files are stored locally in the project directory." }
                        p { class: "mt-1 text-yellow-300/70", "Do not delete the ./data/ or ./db/ folders to preserve your chat history and context." }
                    }
                }
            }
        }
    }
}

/// About section
#[component]
fn AboutSettings() -> Element {
    rsx! {
        div {
            class: "max-w-2xl space-y-6",

            h2 {
                class: "text-lg font-semibold text-white mb-4",
                "About"
            }

            div {
                class: "bg-slate-800 rounded-lg p-6 text-center",
                h3 {
                    class: "text-2xl font-bold text-white mb-2",
                    "Local AI Assistant"
                }
                p {
                    class: "text-slate-400 mb-4",
                    "A privacy-focused AI assistant that runs entirely on your machine"
                }
                div {
                    class: "inline-block px-3 py-1 bg-blue-600/30 text-blue-300 rounded-full text-sm",
                    "v0.1.0-phase1"
                }
            }

            div {
                class: "bg-slate-800 rounded-lg p-4 space-y-3",
                h3 {
                    class: "text-sm font-medium text-slate-300 mb-3",
                    "Technology Stack"
                }
                div {
                    class: "grid grid-cols-2 gap-3 text-sm",
                    div {
                        class: "p-3 bg-slate-700 rounded-lg",
                        div { class: "text-slate-400 text-xs", "Framework" }
                        div { class: "text-white", "Dioxus 0.7.2" }
                    }
                    div {
                        class: "p-3 bg-slate-700 rounded-lg",
                        div { class: "text-slate-400 text-xs", "LLM Engine" }
                        div { class: "text-white", "Kalosm + Qwen" }
                    }
                    div {
                        class: "p-3 bg-slate-700 rounded-lg",
                        div { class: "text-slate-400 text-xs", "Vector Store" }
                        div { class: "text-white", "SurrealDB" }
                    }
                    div {
                        class: "p-3 bg-slate-700 rounded-lg",
                        div { class: "text-slate-400 text-xs", "Session Store" }
                        div { class: "text-white", "SQLite" }
                    }
                }
            }

            div {
                class: "bg-slate-800 rounded-lg p-4",
                h3 {
                    class: "text-sm font-medium text-slate-300 mb-3",
                    "Features"
                }
                ul {
                    class: "space-y-2 text-sm text-slate-400",
                    li { class: "flex items-center gap-2",
                        span { class: "text-green-400", "✓" }
                        "100% local - no data leaves your machine"
                    }
                    li { class: "flex items-center gap-2",
                        span { class: "text-green-400", "✓" }
                        "RAG support for custom knowledge bases"
                    }
                    li { class: "flex items-center gap-2",
                        span { class: "text-green-400", "✓" }
                        "Session persistence across restarts"
                    }
                    li { class: "flex items-center gap-2",
                        span { class: "text-green-400", "✓" }
                        "Multiple language support"
                    }
                    li { class: "flex items-center gap-2",
                        span { class: "text-green-400", "✓" }
                        "Streaming responses"
                    }
                }
            }
        }
    }
}
