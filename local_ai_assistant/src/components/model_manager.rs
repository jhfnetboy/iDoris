//! Model Manager Component
//!
//! UI component for managing HuggingFace models

use crate::models::{ModelInfo, CacheInfo, ModelType};
use crate::server_functions::{
    list_cached_models, download_model, delete_model, get_cache_info, init_hf_cache
};
use dioxus::prelude::*;

#[component]
pub fn ModelManagerPage() -> Element {
    let mut models = use_signal(Vec::<ModelInfo>::new);
    let mut cache_info = use_signal(|| CacheInfo {
        path: std::path::PathBuf::new(),
        total_size_mb: 0,
        model_count: 0,
    });
    let mut loading = use_signal(|| false);
    let mut error_msg = use_signal(|| String::new());
    let mut success_msg = use_signal(|| String::new());

    // Load models on mount
    use_effect({
        let models = models.clone();
        let cache_info = cache_info.clone();
        let loading = loading.clone();
        let error_msg = error_msg.clone();

        move || {
            spawn(async move {
                load_models(models, cache_info, loading, error_msg).await;
            });
        }
    });

    rsx! {
        style { {include_str!("../styles/model_manager.css")} }

        div { class: "model-manager-container",
            h2 { "Model Manager" }

            if error_msg().len() > 0 {
                div { class: "error-message",
                    "{error_msg}"
                    button {
                        onclick: move |_| error_msg.set(String::new()),
                        "×"
                    }
                }
            }

            if success_msg().len() > 0 {
                div { class: "success-message",
                    "{success_msg}"
                    button {
                        onclick: move |_| success_msg.set(String::new()),
                        "×"
                    }
                }
            }

            div { class: "cache-info",
                h3 { "Cache Information" }
                div { class: "cache-stats",
                    div { class: "stat",
                        span { "Location: " }
                        code { "{cache_info().path.display()}" }
                    }
                    div { class: "stat",
                        span { "Total Size: " }
                        span { "{format_size_mb(cache_info().total_size_mb)}" }
                    }
                    div { class: "stat",
                        span { "Models Cached: " }
                        span { "{cache_info().model_count}" }
                    }
                }
            }

            div { class: "models-section",
                h3 { "Available Models" }

                if loading() {
                    div { class: "loading", "Loading models..." }
                } else {
                    div { class: "models-grid",
                    {
                        let models_to_show = models();
                        models_to_show.iter().map(|model| {
                            rsx! {
                                ModelCard {
                                    model: model.clone()
                                }
                            }
                        }).collect::<Vec<_>>().into_iter()
                    }
                    }
                }
            }

            div { class: "actions",
                button {
                    class: "btn btn-secondary",
                    onclick: move |_| {
                        let models = models.clone();
                        let cache_info = cache_info.clone();
                        let loading = loading.clone();
                        let error_msg = error_msg.clone();

                        spawn(async move {
                            load_models(models, cache_info, loading, error_msg).await;
                        });
                    },
                    "Refresh"
                }
                button {
                    class: "btn btn-primary",
                    onclick: move |_| {
                        let mut error_msg = error_msg.clone();
                        let mut success_msg = success_msg.clone();

                        spawn(async move {
                            match init_hf_cache().await {
                                Ok(msg) => success_msg.set(msg),
                                Err(e) => error_msg.set(e.to_string()),
                            }
                        });
                    },
                    "Initialize Cache"
                }
            }
        }
    }
}

#[component]
fn ModelCard(
    model: ModelInfo,
) -> Element {
    rsx! {
        div { class: if model.is_cached { "model-card cached" } else { "model-card" },
            div { class: "model-header",
                h4 { "{model.name}" }
                ModelTypeBadge { model_type: model.model_type }
            }

            div { class: "model-info",
                p { "{model.description}" }

                if let Some(size) = model.size_mb {
                    p { class: "size",
                        "Size: {format_size_mb(size)}"
                    }
                }
            }

            div { class: "model-id",
                code { "{model.id}" }
            }

            div { class: "model-status",
                if model.is_cached {
                    span { class: "status cached", "✓ Cached" }
                } else {
                    span { class: "status not-cached", "Not cached" }
                }
            }
        }
    }
}

#[component]
fn ModelTypeBadge(model_type: ModelType) -> Element {
    let (class, label) = match model_type {
        ModelType::Language => ("badge badge-primary", "LLM"),
        ModelType::ImageGeneration => ("badge badge-success", "Image"),
        ModelType::Embedding => ("badge badge-info", "Embedding"),
        ModelType::Audio => ("badge badge-warning", "Audio"),
        ModelType::Multimodal => ("badge badge-secondary", "Multimodal"),
    };

    rsx! {
        span { class: "{class}", "{label}" }
    }
}

// Helper functions
async fn load_models(
    mut models: Signal<Vec<ModelInfo>>,
    mut cache_info: Signal<CacheInfo>,
    mut loading: Signal<bool>,
    mut error_msg: Signal<String>,
) {
    loading.set(true);
    error_msg.set(String::new());

    match list_cached_models().await {
        Ok(m) => models.set(m),
        Err(e) => error_msg.set(e.to_string()),
    }

    match get_cache_info().await {
        Ok(info) => cache_info.set(info),
        Err(e) => error_msg.set(e.to_string()),
    }

    loading.set(false);
}

async fn download_model_action(
    model_id: String,
    mut models: Signal<Vec<ModelInfo>>,
    mut cache_info: Signal<CacheInfo>,
    mut loading: Signal<bool>,
    mut error_msg: Signal<String>,
    mut success_msg: Signal<String>,
) {
    loading.set(true);
    error_msg.set(String::new());
    success_msg.set(String::new());

    match download_model(model_id.clone()).await {
        Ok(msg) => {
            success_msg.set(msg);
            // Refresh models list
            load_models(models, cache_info, loading, error_msg).await;
        }
        Err(e) => error_msg.set(e.to_string()),
    }

    loading.set(false);
}

async fn delete_model_action(
    model_id: String,
    mut models: Signal<Vec<ModelInfo>>,
    mut cache_info: Signal<CacheInfo>,
    mut loading: Signal<bool>,
    mut error_msg: Signal<String>,
    mut success_msg: Signal<String>,
) {
    loading.set(true);
    error_msg.set(String::new());
    success_msg.set(String::new());

    match delete_model(model_id.clone()).await {
        Ok(msg) => {
            success_msg.set(msg);
            // Refresh models list
            load_models(models, cache_info, loading, error_msg).await;
        }
        Err(e) => error_msg.set(e.to_string()),
    }

    loading.set(false);
}

fn format_size_mb(size_mb: u64) -> String {
    if size_mb < 1024 {
        format!("{} MB", size_mb)
    } else {
        format!("{:.2} GB", size_mb as f64 / 1024.0)
    }
}