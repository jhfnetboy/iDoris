use dioxus::prelude::*;
use crate::server_functions::{
    VideoGenForm, VideoResponse, VideoProviderInfo,
    get_available_video_providers, estimate_video_cost, generate_video
};
use crate::models::{VideoProvider, VideoModel, VideoConfig, VideoQuality};

#[derive(Clone, PartialEq, Props)]
pub struct VideoGenPanelProps {
    pub on_close: EventHandler<()>,
}

#[component]
pub fn VideoGenPanel(props: VideoGenPanelProps) -> Element {
    let mut form = use_signal(|| VideoGenForm::default());
    let mut is_generating = use_signal(|| false);
    let mut generation_result = use_signal::<Option<VideoResponse>>(|| None);
    let mut error_msg = use_signal::<Option<String>>(|| None);
    let mut estimated_cost = use_signal(|| 0.0f64);
    let mut providers = use_signal(|| Vec::<VideoProviderInfo>::new());
    let mut show_advanced = use_signal(|| false);

    // 加载可用的视频生成服务
    use_effect(move || {
        spawn(async move {
            match get_available_video_providers().await {
                Ok(p) => {
                    providers.set(p);
                }
                Err(e) => {
                    web_sys::console::error_1(&format!("Failed to load providers: {:?}", e).into());
                }
            }
        });
    });

    // 实时估算成本
    let estimate_cost = move |_| {
        spawn(async move {
            let current_form = form.read().clone();
            match estimate_video_cost(current_form).await {
                Ok(cost) => {
                    estimated_cost.set(cost);
                }
                Err(e) => {
                    web_sys::console::error_1(&format!("Failed to estimate cost: {:?}", e).into());
                }
            }
        });
    };

    // Generate    // Generate video
    let handle_generate = move |_| {
        if is_generating() {
            return;
        }

        let current_form = form.read().clone();
        if current_form.prompt.is_empty() {
            error_msg.set(Some("Please enter a video description".to_string()));
            return;
        }

        is_generating.set(true);
        error_msg.set(None);
        generation_result.set(None);

        spawn(async move {
            match generate_video(current_form).await {
                Ok(response) => {
                    is_generating.set(false);
                    generation_result.set(Some(response));
                }
                Err(e) => {
                    is_generating.set(false);
                    error_msg.set(Some(format!("Video generation failed: {}", e)));
                }
            }
        });
    };

    rsx! {
        // Changed from fixed overlay to full-height flex container for sidebar usage
        div { class: "h-full flex flex-col bg-white text-gray-900 overflow-y-auto",
            div { class: "p-6 w-full max-w-4xl mx-auto",
                // Header
                div { class: "flex justify-between items-center mb-6",
                    h2 { class: "text-2xl font-bold text-gray-800", "Video Generation" }
                    // Sidebar close button usually handled by parent calling props.on_close, 
                    // but we keep a close button if the user wants to explicitly close this panel.
                    // Or if it's a main panel, maybe we don't need a close button? 
                    // The user said "use right side page panel(like other menu click)", 
                    // implying it's a main view. Navigation handles switching.
                    // We'll keep it optional or remove if it feels redundant with the sidebar navigation.
                    // Let's keep it for now but maybe style it differently or remove if redundant.
                    // Actually, if it's a "page panel", standard design often doesn't have a close 'x'.
                    // But to be safe and allow closing back to chat:
                }

                // Error Message
                if let Some(error) = error_msg() {
                    div { class: "bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4",
                        {error}
                    }
                }

                // Main Form
                div { class: "grid grid-cols-1 gap-6", // Single column for sidebar width constraints usually, but 4xl allows 2 cols
                    // Basic Settings
                    div { class: "space-y-4",
                        div {
                            label { class: "block text-sm font-medium text-gray-700 mb-1", "Prompt" }
                            textarea {
                                value: form.read().prompt.clone(),
                                oninput: move |e| {
                                    let mut f = form.write();
                                    f.prompt = e.value();
                                    estimate_cost(());
                                },
                                // Updated placeholder to match requested default
                                placeholder: "Describe the video you want to generate, e.g., a lovely white cat is playing in the garden",
                                class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                                rows: 3
                            }
                        }

                        div {
                            label { class: "block text-sm font-medium text-gray-700 mb-1", "Negative Prompt (Optional)" }
                            textarea {
                                value: form.read().negative_prompt.clone().unwrap_or_default(),
                                oninput: move |e| {
                                    form.write().negative_prompt = if e.value().is_empty() { None } else { Some(e.value()) };
                                },
                                placeholder: "Content you don't want in the video",
                                class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                                rows: 2
                            }
                        }

                        // Provider and Model Selection
                        div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                            div {
                                label { class: "block text-sm font-medium text-gray-700 mb-1", "Provider" }
                                select {
                                    value: format!("{:?}", form.read().provider),
                                    onchange: move |e| {
                                        if let Ok(provider) = serde_json::from_str::<VideoProvider>(&format!("\"{}\"", e.value())) {
                                            let provider_clone = provider.clone();
                                            form.write().provider = provider;
                                            // Update default model
                                            let providers = providers.read();
                                            if let Some(p) = providers.iter().find(|p| p.provider == provider_clone) {
                                                if let Some((_, model)) = p.models.first() {
                                                    form.write().model = model.clone();
                                                }
                                            }
                                        }
                                    },
                                    class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                                    for provider in providers.read().iter() {
                                        option { value: format!("{:?}", provider.provider), {provider.name.clone()} }
                                    }
                                }
                            }

                            div {
                                label { class: "block text-sm font-medium text-gray-700 mb-1", "Model" }
                                select {
                                    value: format!("{:?}", form.read().model),
                                    onchange: move |e| {
                                        if let Ok(model) = serde_json::from_str::<VideoModel>(&format!("\"{}\"", e.value())) {
                                            form.write().model = model;
                                            estimate_cost(());
                                        }
                                    },
                                    class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                                    // Show available models for current provider
                                    for provider in providers.read().iter() {
                                        if provider.provider == form.read().provider {
                                            for (name, model) in provider.models.iter() {
                                                option { value: format!("{:?}", model), {name.clone()} }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Advanced Settings
                    div { class: "space-y-4",
                        button {
                            onclick: move |_| {
                            let current = *show_advanced.read();
                            show_advanced.set(!current);
                        },
                            class: "text-blue-600 hover:text-blue-800 text-sm font-medium",
                            if *show_advanced.read() {
                                "Hide Advanced Settings"
                            } else {
                                "Show Advanced Settings"
                            }
                        }

                        if *show_advanced.read() {
                            div { class: "space-y-4 border-t pt-4",
                                // Dimensions
                                div { class: "grid grid-cols-2 gap-4",
                                    div {
                                        label { class: "block text-sm font-medium text-gray-700 mb-1", "Width" }
                                        input {
                                            r#type: "number",
                                            value: form.read().width.to_string(),
                                            oninput: move |e| {
                                                if let Ok(width) = e.value().parse::<u32>() {
                                                    form.write().width = width;
                                                    estimate_cost(());
                                                }
                                            },
                                            min: 256,
                                            max: 2048,
                                            step: 64,
                                            class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                                        }
                                    }

                                    div {
                                        label { class: "block text-sm font-medium text-gray-700 mb-1", "Height" }
                                        input {
                                            r#type: "number",
                                            value: form.read().height.to_string(),
                                            oninput: move |e| {
                                                if let Ok(height) = e.value().parse::<u32>() {
                                                    form.write().height = height;
                                                    estimate_cost(());
                                                }
                                            },
                                            min: 256,
                                            max: 2048,
                                            step: 64,
                                            class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                                        }
                                    }
                                }

                                // Duration and Quality
                                div { class: "grid grid-cols-2 gap-4",
                                    div {
                                        label { class: "block text-sm font-medium text-gray-700 mb-1", "Duration (s)" }
                                        input {
                                            r#type: "number",
                                            value: form.read().duration_seconds.to_string(),
                                            oninput: move |e| {
                                                if let Ok(duration) = e.value().parse::<u32>() {
                                                    form.write().duration_seconds = duration.clamp(2, 30);
                                                    estimate_cost(());
                                                }
                                            },
                                            min: 2,
                                            max: 30,
                                            class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                                        }
                                    }

                                    div {
                                        label { class: "block text-sm font-medium text-gray-700 mb-1", "Quality" }
                                        select {
                                            value: format!("{:?}", form.read().quality),
                                            onchange: move |e| {
                                                if let Ok(quality) = serde_json::from_str::<VideoQuality>(&format!("\"{}\"", e.value())) {
                                                    form.write().quality = quality;
                                                    estimate_cost(());
                                                }
                                            },
                                            class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                                            option { value: "Standard", "Standard (480p)" }
                                            option { value: "HD", "HD (720p)" }
                                            option { value: "Premium", "Premium (1080p+)" }
                                        }
                                    }
                                }

                                // FPS and Seed
                                div { class: "grid grid-cols-2 gap-4",
                                    div {
                                        label { class: "block text-sm font-medium text-gray-700 mb-1", "FPS" }
                                        input {
                                            r#type: "number",
                                            value: form.read().fps.to_string(),
                                            oninput: move |e| {
                                                if let Ok(fps) = e.value().parse::<u8>() {
                                                    form.write().fps = fps.clamp(8, 60);
                                                }
                                            },
                                            min: 8,
                                            max: 60,
                                            class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                                        }
                                    }

                                    div {
                                        label { class: "block text-sm font-medium text-gray-700 mb-1", "Seed (Optional)" }
                                        input {
                                            r#type: "number",
                                            value: form.read().seed.map(|s| s.to_string()).unwrap_or_default(),
                                            oninput: move |e| {
                                                form.write().seed = if e.value().is_empty() { None } else { e.value().parse::<u32>().ok() };
                                            },
                                            min: 0,
                                            max: 999999999,
                                            class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                                        }
                                    }
                                }
                            }
                        }

                        // Cost Estimation
                        div { class: "bg-blue-50 border border-blue-200 rounded-lg p-4",
                            div { class: "flex justify-between items-center",
                                span { class: "text-sm font-medium text-gray-700", "Estimated Cost" }
                                span { class: "text-lg font-bold text-blue-600", "Calculating..." }
                            }
                            p { class: "text-xs text-gray-600 mt-1", "Based on current settings" }
                        }
                    }
                }

                // Generate Button
                div { class: "mt-6 flex justify-center",
                    button {
                        onclick: handle_generate,
                        disabled: is_generating(),
                        class: "px-8 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:bg-gray-400 disabled:cursor-not-allowed transition-colors font-medium",
                        if is_generating() {
                            "Generating..."
                        } else {
                            "Generate Video"
                        }
                    }
                }

                // Results
                if let Some(result) = generation_result.read().clone() {
                    div { class: "mt-6 border-t pt-6",
                        h3 { class: "text-lg font-semibold mb-4 text-gray-900", "Generation Result" }
                        div { class: "bg-gray-50 rounded-lg p-4",
                            div { class: "grid grid-cols-1 md:grid-cols-2 gap-4 mb-4",
                                div {
                                    span { class: "text-sm text-gray-600", "Duration: " }
                                    span { class: "font-medium text-gray-900", "{result.duration_seconds}s" }
                                }
                                div {
                                    span { class: "text-sm text-gray-600", "Cost: " }
                                    span { class: "font-medium text-gray-900", "${result.cost_estimate:.4}" }
                                }
                                div {
                                    span { class: "text-sm text-gray-600", "Status: " }
                                    span { 
                                        class: "font-medium text-green-600", 
                                        {format!("{:?}", result.status)}
                                    }
                                }
                                div {
                                    span { class: "text-sm text-gray-600", "Task ID: " }
                                    span { 
                                        class: "font-mono text-xs text-gray-900", 
                                        title: "{result.generation_id}",
                                        {result.generation_id.chars().take(16).collect::<String>()}
                                        "..."
                                    }
                                }
                            }

                            // Video URL Info
                            div { class: "mb-4 p-3 bg-blue-50 border border-blue-200 rounded",
                                p { class: "text-xs font-medium text-blue-900 mb-1", "Video URL:" }
                                p { 
                                    class: "text-xs text-blue-700 break-all font-mono",
                                    {result.video_url.clone()}
                                }
                            }

                            // Video Preview
                            div { class: "space-y-2",
                                video {
                                    controls: true,
                                    autoplay: false,
                                    width: "100%",
                                    max_width: "640",
                                    class: "rounded-lg shadow-md bg-black",
                                    crossorigin: "anonymous",
                                    source { 
                                        src: result.video_url.clone(), 
                                        r#type: "video/mp4" 
                                    }
                                    "Your browser does not support the video tag. Please use the download button below."
                                }

                                p { 
                                    class: "text-xs text-gray-500 italic",
                                    "Note: If video doesn't play due to CORS/403, use the download button or open URL directly."
                                }

                                // Download Button  
                                div { class: "flex gap-2",
                                    a {
                                        href: result.video_url.clone(),
                                        target: "_blank",
                                        class: "inline-flex items-center px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 transition-colors",
                                        "Open Video"
                                    }
                                    button {
                                        onclick: move |_| {
                                            // Copy URL to clipboard
                                            let url = result.video_url.clone();
                                            let _ = eval(&format!("navigator.clipboard.writeText('{}')", url));
                                        },
                                        class: "inline-flex items-center px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors",
                                        "Copy URL"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}