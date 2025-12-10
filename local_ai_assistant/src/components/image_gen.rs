//! Image Generation Component
//!
//! Phase 2.2: Image generation UI for creating images from text prompts.

use dioxus::prelude::*;
use crate::server_functions::{
    generate_image, is_image_model_ready, get_image_gen_status, ImageResult
};

/// Props for ImageGenPanel - embedded mode means it's part of the main content area
#[component]
pub fn ImageGenPanel(
    #[props(default = false)]
    embedded: bool,
    on_open_settings: Option<EventHandler<()>>,
) -> Element {
    let mut prompt: Signal<String> = use_signal(|| "a small yellow dog is playing at the grass ground".to_string());
    let mut negative_prompt: Signal<String> = use_signal(String::new);
    let mut width: Signal<u32> = use_signal(|| 512);
    let mut height: Signal<u32> = use_signal(|| 512);
    let mut steps: Signal<u32> = use_signal(|| 4);  // Schnell default
    let mut show_advanced: Signal<bool> = use_signal(|| false);
    let mut is_generating: Signal<bool> = use_signal(|| false);
    let mut generated_image: Signal<Option<ImageResult>> = use_signal(|| None);
    let mut error_message: Signal<Option<String>> = use_signal(|| None);
    let mut generation_time_ms: Signal<Option<u64>> = use_signal(|| None);
    let mut start_time: Signal<Option<f64>> = use_signal(|| None);
    let mut model_ready: Signal<bool> = use_signal(|| false);
    let mut gen_status: Signal<String> = use_signal(|| String::new());
    let mut gen_progress: Signal<u8> = use_signal(|| 0);
    let mut selected_model: Signal<String> = use_signal(|| "schnell".to_string());  // schnell is free and reliable
    let mut quantize: Signal<u8> = use_signal(|| 4);

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
                        // Model not loaded - show download prompt with link to settings
                        div {
                            class: "p-4 bg-yellow-900/50 border border-yellow-700 rounded-lg text-yellow-200",
                            div { class: "flex items-start gap-3",
                                svg {
                                    class: "w-5 h-5 mt-0.5 flex-shrink-0",
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
                                    class: "flex-1",
                                    p { class: "font-medium mb-1", "Image model not downloaded" }
                                    p { class: "text-sm text-yellow-300/80 mb-3", "You need to download the image generation model (~2GB) before generating images." }
                                    if on_open_settings.is_some() {
                                        button {
                                            class: "px-4 py-2 bg-purple-600 hover:bg-purple-700 rounded-lg text-white text-sm font-medium transition-colors flex items-center gap-2",
                                            onclick: move |_| {
                                                if let Some(ref handler) = on_open_settings {
                                                    handler.call(());
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
                                                    d: "M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"
                                                }
                                                path {
                                                    stroke_linecap: "round",
                                                    stroke_linejoin: "round",
                                                    d: "M15 12a3 3 0 11-6 0 3 3 0 016 0z"
                                                }
                                            }
                                            "Go to Settings > Models"
                                        }
                                    }
                                }
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

                // Model selection - always visible
                div {
                    class: "space-y-2 p-3 bg-slate-700/50 rounded-lg",
                    label {
                        class: "block text-sm font-medium text-slate-300",
                        "MFLUX Model"
                    }
                    select {
                        class: "w-full px-3 py-2 bg-slate-700 border border-slate-600 rounded-lg text-white focus:outline-none focus:border-blue-500",
                        value: "{selected_model}",
                        onchange: move |e| {
                            let model = e.value();
                            selected_model.set(model.clone());
                            // Update default steps based on model
                            let default_steps = match model.as_str() {
                                "dev" => 20,
                                "z-image-turbo" => 9,
                                _ => 4, // schnell
                            };
                            steps.set(default_steps);
                        },
                        option { value: "schnell", "FLUX.1 Schnell (4 steps, fast)" }
                        option { value: "dev", "FLUX.1 Dev (20 steps, quality)" }
                        option { value: "z-image-turbo", "Z-Image Turbo (9 steps)" }
                    }
                    p {
                        class: "text-xs text-amber-400 mt-1",
                        "⚠️ All FLUX models require HF login: hf auth login"
                    }
                }

                // Quick resolution presets
                div {
                    class: "space-y-2 p-3 bg-slate-700/50 rounded-lg",
                    label {
                        class: "block text-sm font-medium text-slate-300",
                        "Quick Presets"
                    }
                    div {
                        class: "flex flex-wrap gap-2",
                        // Fast preset
                        button {
                            class: if width() == 512 && height() == 512 && quantize() == 4 {
                                "px-3 py-1.5 text-sm rounded-lg bg-green-600 text-white font-medium"
                            } else {
                                "px-3 py-1.5 text-sm rounded-lg bg-slate-600 text-slate-300 hover:bg-slate-500"
                            },
                            onclick: move |_| {
                                width.set(512);
                                height.set(512);
                                quantize.set(4);
                            },
                            "Fast (512x512)"
                        }
                        // Balanced preset
                        button {
                            class: if width() == 768 && height() == 768 && quantize() == 4 {
                                "px-3 py-1.5 text-sm rounded-lg bg-blue-600 text-white font-medium"
                            } else {
                                "px-3 py-1.5 text-sm rounded-lg bg-slate-600 text-slate-300 hover:bg-slate-500"
                            },
                            onclick: move |_| {
                                width.set(768);
                                height.set(768);
                                quantize.set(4);
                            },
                            "Balanced (768x768)"
                        }
                        // Quality preset
                        button {
                            class: if width() == 1024 && height() == 1024 {
                                "px-3 py-1.5 text-sm rounded-lg bg-purple-600 text-white font-medium"
                            } else {
                                "px-3 py-1.5 text-sm rounded-lg bg-slate-600 text-slate-300 hover:bg-slate-500"
                            },
                            onclick: move |_| {
                                width.set(1024);
                                height.set(1024);
                                quantize.set(8);
                            },
                            "Quality (1024x1024)"
                        }
                    }
                    p {
                        class: "text-xs text-slate-500 mt-1",
                        "Current: {width()}x{height()}, {quantize()}-bit"
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
                    "More Settings"
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
                                    option { value: "512", "512px" }
                                    option { value: "768", "768px" }
                                    option { value: "1024", "1024px" }
                                    option { value: "1280", "1280px" }
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
                                    option { value: "512", "512px" }
                                    option { value: "768", "768px" }
                                    option { value: "1024", "1024px" }
                                    option { value: "1280", "1280px" }
                                }
                            }
                        }

                        // Steps and Quantization
                        div {
                            class: "grid grid-cols-2 gap-4",
                            // Steps slider
                            div {
                                class: "space-y-2",
                                label {
                                    class: "block text-sm font-medium text-slate-300",
                                    "Steps: {steps}"
                                }
                                input {
                                    r#type: "range",
                                    class: "w-full",
                                    min: "1",
                                    max: "30",
                                    value: "{steps}",
                                    oninput: move |e| {
                                        if let Ok(v) = e.value().parse::<u32>() {
                                            steps.set(v);
                                        }
                                    },
                                }
                            }
                            // Quantization
                            div {
                                class: "space-y-2",
                                label {
                                    class: "block text-sm font-medium text-slate-300",
                                    "Quantization"
                                }
                                select {
                                    class: "w-full px-3 py-2 bg-slate-700 border border-slate-600 rounded-lg text-white focus:outline-none focus:border-blue-500",
                                    value: "{quantize}",
                                    onchange: move |e| {
                                        if let Ok(v) = e.value().parse::<u8>() {
                                            quantize.set(v);
                                        }
                                    },
                                    option { value: "4", "4-bit (Fastest, Recommended)" }
                                    option { value: "8", "8-bit (Better Quality)" }
                                }
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
                        let model = selected_model();
                        let quant = quantize();

                        if !p.is_empty() {
                            is_generating.set(true);
                            error_message.set(None);
                            gen_status.set("Starting...".to_string());
                            gen_progress.set(0);
                            generation_time_ms.set(None);

                            // Record start time using js_sys for WASM
                            #[cfg(target_arch = "wasm32")]
                            {
                                start_time.set(Some(js_sys::Date::now()));
                            }
                            #[cfg(not(target_arch = "wasm32"))]
                            {
                                start_time.set(Some(std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .unwrap()
                                    .as_millis() as f64));
                            }

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
                                match generate_image(p, neg, Some(w), Some(h), Some(s), Some(model), Some(quant)).await {
                                    Ok(result) => {
                                        generated_image.set(Some(result));
                                        // Calculate generation time
                                        if let Some(start) = start_time() {
                                            #[cfg(target_arch = "wasm32")]
                                            {
                                                let elapsed = (js_sys::Date::now() - start) as u64;
                                                generation_time_ms.set(Some(elapsed));
                                            }
                                            #[cfg(not(target_arch = "wasm32"))]
                                            {
                                                let now = std::time::SystemTime::now()
                                                    .duration_since(std::time::UNIX_EPOCH)
                                                    .unwrap()
                                                    .as_millis() as f64;
                                                let elapsed = (now - start) as u64;
                                                generation_time_ms.set(Some(elapsed));
                                            }
                                        }
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
                            div {
                                class: "flex items-center gap-3",
                                h3 {
                                    class: "text-sm font-medium text-slate-300",
                                    "Generated Image ({img.width}×{img.height})"
                                }
                                // Show generation time
                                if let Some(time_ms) = generation_time_ms() {
                                    span {
                                        class: "text-xs text-green-400 bg-green-900/30 px-2 py-0.5 rounded",
                                        {
                                            if time_ms >= 60000 {
                                                format!("⏱ {}m {:.1}s", time_ms / 60000, (time_ms % 60000) as f64 / 1000.0)
                                            } else {
                                                format!("⏱ {:.1}s", time_ms as f64 / 1000.0)
                                            }
                                        }
                                    }
                                }
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
