//! Image Generation Component
//!
//! Phase 2.2: Image generation UI for creating images from text prompts.

use dioxus::prelude::*;
use crate::server_functions::{
    generate_image, is_image_model_ready, get_image_gen_status, ImageResult, ImageGenStatus
};

/// Props for ImageGenPanel - embedded mode means it's part of the main content area
#[component]
pub fn ImageGenPanel(
    #[props(default = false)]
    embedded: bool,
) -> Element {
    let mut prompt: Signal<String> = use_signal(String::new);
    let mut negative_prompt: Signal<String> = use_signal(String::new);
    let mut width: Signal<u32> = use_signal(|| 512);
    let mut height: Signal<u32> = use_signal(|| 512);
    let mut steps: Signal<u32> = use_signal(|| 30);
    let mut show_advanced: Signal<bool> = use_signal(|| false);
    let mut is_generating: Signal<bool> = use_signal(|| false);
    let mut generated_image: Signal<Option<ImageResult>> = use_signal(|| None);
    let mut error_message: Signal<Option<String>> = use_signal(|| None);
    let mut model_ready: Signal<bool> = use_signal(|| false);
    let mut gen_status: Signal<String> = use_signal(|| String::new());
    let mut gen_progress: Signal<u8> = use_signal(|| 0);

    // Check if model is ready on mount
    use_effect(move || {
        spawn(async move {
            match is_image_model_ready().await {
                Ok(ready) => model_ready.set(ready),
                Err(_) => model_ready.set(false),
            }
        });
    });

    // Note: Status polling is now handled inside the generate button onclick handler
    // to avoid the use_effect dependency tracking issues that caused continuous polling

    // Use different container styles based on embedded mode
    let container_class = if embedded {
        "flex-1 flex flex-col overflow-hidden"
    } else {
        "fixed left-64 top-0 bottom-0 w-[600px] bg-slate-800 border-r border-slate-700 z-50 shadow-xl overflow-y-auto"
    };

    rsx! {
        // Image Generation panel - embedded in main content area
        div {
            class: "{container_class}",

            // Content area with scroll
            div {
                class: "flex-1 overflow-y-auto p-6",

                // Main content wrapper with max width for better readability
                div {
                    class: "max-w-2xl mx-auto space-y-6",

                // Model status info - show different message based on ready state
                if !is_generating() {
                    if model_ready() {
                        // Model is ready - show green success message
                        div {
                            class: "p-3 bg-green-900/50 border border-green-700 rounded-lg text-green-200 text-sm",
                            div { class: "flex items-center gap-2",
                                svg {
                                    class: "w-4 h-4",
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
                                "Model loaded and ready to generate images"
                            }
                        }
                    } else {
                        // Model not loaded - show download info
                        div {
                            class: "p-3 bg-blue-900/50 border border-blue-700 rounded-lg text-blue-200 text-sm",
                            div { class: "flex items-center gap-2",
                                svg {
                                    class: "w-4 h-4",
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
                                "Model (~2GB) downloads automatically when you click Generate"
                            }
                        }
                    }
                }

                // Prompt input
                div {
                    class: "space-y-2",
                    label {
                        class: "block text-sm font-medium text-slate-300",
                        "Prompt"
                    }
                    textarea {
                        class: "w-full px-3 py-2 bg-slate-700 border border-slate-600 rounded-lg text-white placeholder-slate-400 focus:outline-none focus:border-blue-500 resize-none",
                        rows: "3",
                        placeholder: "Describe the image you want to generate...\nExample: A serene mountain landscape at sunset with a calm lake",
                        value: "{prompt}",
                        oninput: move |e| prompt.set(e.value()),
                    }
                }

                // Advanced settings toggle
                button {
                    class: "flex items-center gap-2 text-sm text-slate-400 hover:text-white transition-colors",
                    onclick: move |_| show_advanced.set(!show_advanced()),
                    svg {
                        class: if show_advanced() { "w-4 h-4 transform rotate-90 transition-transform" } else { "w-4 h-4 transition-transform" },
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        view_box: "0 0 24 24",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M9 5l7 7-7 7"
                        }
                    }
                    "Advanced Settings"
                }

                // Advanced settings
                if show_advanced() {
                    div {
                        class: "space-y-4 p-4 bg-slate-700/50 rounded-lg",

                        // Negative prompt
                        div {
                            class: "space-y-2",
                            label {
                                class: "block text-sm font-medium text-slate-300",
                                "Negative Prompt (optional)"
                            }
                            textarea {
                                class: "w-full px-3 py-2 bg-slate-700 border border-slate-600 rounded-lg text-white placeholder-slate-400 focus:outline-none focus:border-blue-500 resize-none",
                                rows: "2",
                                placeholder: "What to avoid in the image...\nExample: blurry, low quality, distorted",
                                value: "{negative_prompt}",
                                oninput: move |e| negative_prompt.set(e.value()),
                            }
                        }

                        // Size options
                        div {
                            class: "grid grid-cols-2 gap-4",
                            div {
                                class: "space-y-2",
                                label {
                                    class: "block text-sm font-medium text-slate-300",
                                    "Width"
                                }
                                select {
                                    class: "w-full px-3 py-2 bg-slate-700 border border-slate-600 rounded-lg text-white focus:outline-none focus:border-blue-500",
                                    value: "{width}",
                                    onchange: move |e| {
                                        if let Ok(v) = e.value().parse::<u32>() {
                                            width.set(v);
                                        }
                                    },
                                    option { value: "256", "256px" }
                                    option { value: "384", "384px" }
                                    option { value: "512", selected: true, "512px" }
                                    option { value: "640", "640px" }
                                    option { value: "768", "768px" }
                                }
                            }
                            div {
                                class: "space-y-2",
                                label {
                                    class: "block text-sm font-medium text-slate-300",
                                    "Height"
                                }
                                select {
                                    class: "w-full px-3 py-2 bg-slate-700 border border-slate-600 rounded-lg text-white focus:outline-none focus:border-blue-500",
                                    value: "{height}",
                                    onchange: move |e| {
                                        if let Ok(v) = e.value().parse::<u32>() {
                                            height.set(v);
                                        }
                                    },
                                    option { value: "256", "256px" }
                                    option { value: "384", "384px" }
                                    option { value: "512", selected: true, "512px" }
                                    option { value: "640", "640px" }
                                    option { value: "768", "768px" }
                                }
                            }
                        }

                        // Steps slider
                        div {
                            class: "space-y-2",
                            label {
                                class: "block text-sm font-medium text-slate-300",
                                "Inference Steps: {steps}"
                            }
                            input {
                                r#type: "range",
                                class: "w-full",
                                min: "10",
                                max: "50",
                                value: "{steps}",
                                oninput: move |e| {
                                    if let Ok(v) = e.value().parse::<u32>() {
                                        steps.set(v);
                                    }
                                },
                            }
                            div {
                                class: "flex justify-between text-xs text-slate-500",
                                span { "10 (Fast)" }
                                span { "50 (Quality)" }
                            }
                        }
                    }
                }

                // Progress indicator (shown during generation)
                if is_generating() {
                    div {
                        class: "p-4 bg-slate-700/50 rounded-lg space-y-3",
                        // Status text
                        div {
                            class: "flex items-center justify-between text-sm",
                            span {
                                class: "text-slate-300",
                                if gen_status().is_empty() {
                                    "Starting..."
                                } else {
                                    "{gen_status()}"
                                }
                            }
                            span {
                                class: "text-purple-400 font-medium",
                                "{gen_progress()}%"
                            }
                        }
                        // Progress bar
                        div {
                            class: "w-full bg-slate-600 rounded-full h-2.5 overflow-hidden",
                            div {
                                class: "bg-purple-500 h-2.5 rounded-full transition-all duration-300",
                                style: "width: {gen_progress()}%",
                            }
                        }
                        // Animated dots
                        div {
                            class: "flex items-center justify-center gap-1 text-slate-400 text-xs",
                            span { class: "animate-pulse", "●" }
                            span { class: "animate-pulse delay-100", "●" }
                            span { class: "animate-pulse delay-200", "●" }
                        }
                    }
                }

                // Generate button
                button {
                    class: "w-full px-4 py-3 bg-purple-600 hover:bg-purple-700 disabled:bg-slate-600 disabled:cursor-not-allowed rounded-lg text-white font-medium transition-colors flex items-center justify-center gap-2",
                    disabled: is_generating() || prompt().trim().is_empty(),
                    onclick: move |_| {
                        let p = prompt().trim().to_string();
                        let neg = if negative_prompt().trim().is_empty() { None } else { Some(negative_prompt().trim().to_string()) };
                        let w = width();
                        let h = height();
                        let s = steps();

                        if !p.is_empty() {
                            is_generating.set(true);
                            error_message.set(None);
                            gen_status.set("Starting...".to_string());
                            gen_progress.set(0);

                            // Start status polling in a separate task
                            spawn(async move {
                                loop {
                                    #[cfg(target_arch = "wasm32")]
                                    {
                                        gloo_timers::future::TimeoutFuture::new(500).await;
                                    }
                                    #[cfg(not(target_arch = "wasm32"))]
                                    {
                                        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                                    }

                                    if !is_generating() {
                                        break;
                                    }

                                    match get_image_gen_status().await {
                                        Ok(status) => {
                                            gen_status.set(status.status);
                                            gen_progress.set(status.progress);
                                        }
                                        Err(_) => {}
                                    }
                                }
                            });

                            // Start the actual generation
                            spawn(async move {
                                match generate_image(p, neg, Some(w), Some(h), Some(s)).await {
                                    Ok(result) => {
                                        generated_image.set(Some(result));
                                    }
                                    Err(e) => {
                                        error_message.set(Some(format!("Generation failed: {}", e)));
                                    }
                                }
                                is_generating.set(false);
                            });
                        }
                    },
                    if is_generating() {
                        svg {
                            class: "w-5 h-5 animate-spin",
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
                        "Generating..."
                    } else {
                        svg {
                            class: "w-5 h-5",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            view_box: "0 0 24 24",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                d: "M13 10V3L4 14h7v7l9-11h-7z"
                            }
                        }
                        "Generate Image"
                    }
                }

                // Error message
                if let Some(err) = error_message() {
                    div {
                        class: "p-3 bg-red-900/50 border border-red-700 rounded-lg text-red-200 text-sm",
                        "{err}"
                        button {
                            class: "ml-2 text-red-400 hover:text-white",
                            onclick: move |_| error_message.set(None),
                            "×"
                        }
                    }
                }

                // Generated image display
                if let Some(img) = generated_image() {
                    div {
                        class: "space-y-3",
                        div {
                            class: "flex items-center justify-between",
                            h3 {
                                class: "text-sm font-medium text-slate-300",
                                "Generated Image ({img.width}×{img.height})"
                            }
                            div {
                                class: "flex gap-2",
                                // Download button
                                a {
                                    class: "px-3 py-1 bg-slate-700 hover:bg-slate-600 rounded text-sm text-white transition-colors",
                                    href: "{img.data_url}",
                                    download: "generated-image.png",
                                    "Download"
                                }
                                // Clear button
                                button {
                                    class: "px-3 py-1 bg-slate-700 hover:bg-slate-600 rounded text-sm text-slate-300 transition-colors",
                                    onclick: move |_| generated_image.set(None),
                                    "Clear"
                                }
                            }
                        }
                        div {
                            class: "border border-slate-600 rounded-lg overflow-hidden bg-slate-900",
                            img {
                                class: "w-full h-auto",
                                src: "{img.data_url}",
                                alt: "Generated image",
                            }
                        }
                    }
                }

                // Help text
                div {
                    class: "text-xs text-slate-500 p-3 bg-slate-800 rounded-lg border border-slate-700",
                    div { class: "font-medium mb-1", "Tips for better results:" }
                    ul {
                        class: "list-disc list-inside space-y-1",
                        li { "Be specific and descriptive in your prompt" }
                        li { "Include style keywords like 'photorealistic', 'oil painting', 'digital art'" }
                        li { "Use negative prompts to exclude unwanted elements" }
                        li { "Higher steps = better quality but slower generation" }
                    }
                }
                } // Close max-w-2xl div
            } // Close overflow-y-auto div
        } // Close container div
    } // Close rsx!
}
