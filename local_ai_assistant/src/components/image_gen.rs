//! Image Generation Component
//!
//! Phase 2.2: Image generation UI for creating images from text prompts.

use dioxus::prelude::*;
use crate::server_functions::{
    generate_image, generate_image_simple, is_image_generating, is_image_model_ready, ImageResult
};

#[component]
pub fn ImageGenPanel(show_image_gen: Signal<bool>) -> Element {
    if !show_image_gen() {
        return rsx! {};
    }

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

    // Check if model is ready on mount
    use_effect(move || {
        spawn(async move {
            match is_image_model_ready().await {
                Ok(ready) => model_ready.set(ready),
                Err(_) => model_ready.set(false),
            }
        });
    });

    rsx! {
        // Backdrop
        div {
            class: "fixed inset-0 bg-black/50 backdrop-blur-sm z-40",
            onclick: move |_| show_image_gen.set(false),
        }

        // Image Generation panel
        div {
            class: "fixed left-64 top-0 bottom-0 w-[600px] bg-slate-800 border-r border-slate-700 z-50 shadow-xl overflow-y-auto",

            // Header
            div {
                class: "flex items-center justify-between p-4 border-b border-slate-700",
                h2 {
                    class: "text-lg font-semibold text-white flex items-center gap-2",
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
                    "Image Generation"
                }
                button {
                    class: "p-1 rounded hover:bg-slate-700 transition-colors",
                    onclick: move |_| show_image_gen.set(false),
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

            // Content
            div {
                class: "p-4 space-y-4",

                // Model status warning
                if !model_ready() {
                    div {
                        class: "p-3 bg-amber-900/50 border border-amber-700 rounded-lg text-amber-200 text-sm",
                        "Image generation model is loading. This may take a few minutes on first use."
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
            }
        }
    }
}
