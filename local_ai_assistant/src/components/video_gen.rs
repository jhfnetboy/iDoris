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

    // 生成视频
    let handle_generate = move |_| {
        if is_generating() {
            return;
        }

        let current_form = form.read().clone();
        if current_form.prompt.is_empty() {
            error_msg.set(Some("请输入视频描述".to_string()));
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
                    error_msg.set(Some(format!("视频生成失败: {}", e)));
                }
            }
        });
    };

    rsx! {
        div { class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50",
            div { class: "bg-white rounded-lg p-6 w-full max-w-4xl max-h-[90vh] overflow-y-auto",
                // Header
                div { class: "flex justify-between items-center mb-6",
                    h2 { class: "text-2xl font-bold text-gray-800", "视频生成" }
                    button {
                        onclick: move |_| props.on_close.call(()),
                        class: "text-gray-500 hover:text-gray-700 text-2xl",
                        "×"
                    }
                }

                // 错误提示
                if let Some(error) = error_msg() {
                    div { class: "bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4",
                        {error}
                    }
                }

                // 主表单
                div { class: "grid grid-cols-1 lg:grid-cols-2 gap-6",
                    // 左侧：基本设置
                    div { class: "space-y-4",
                        div {
                            label { class: "block text-sm font-medium text-gray-700 mb-1", "视频描述" }
                            textarea {
                                value: form.read().prompt.clone(),
                                oninput: move |e| {
                                    let mut f = form.write();
                                    f.prompt = e.value();
                                    estimate_cost(());
                                },
                                placeholder: "描述你想要生成的视频内容，例如：一只可爱的小猫在花园里玩耍",
                                class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                                rows: 3
                            }
                        }

                        div {
                            label { class: "block text-sm font-medium text-gray-700 mb-1", "负面描述 (可选)" }
                            textarea {
                                value: form.read().negative_prompt.clone().unwrap_or_default(),
                                oninput: move |e| {
                                    form.write().negative_prompt = if e.value().is_empty() { None } else { Some(e.value()) };
                                },
                                placeholder: "不希望出现在视频中的内容",
                                class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                                rows: 2
                            }
                        }

                        // 服务商和模型选择
                        div { class: "grid grid-cols-2 gap-4",
                            div {
                                label { class: "block text-sm font-medium text-gray-700 mb-1", "服务商" }
                                select {
                                    value: format!("{:?}", form.read().provider),
                                    onchange: move |e| {
                                        if let Ok(provider) = serde_json::from_str::<VideoProvider>(&format!("\"{}\"", e.value())) {
                                            let provider_clone = provider.clone();
                                            form.write().provider = provider;
                                            // 更新默认模型
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
                                label { class: "block text-sm font-medium text-gray-700 mb-1", "模型" }
                                select {
                                    value: format!("{:?}", form.read().model),
                                    onchange: move |e| {
                                        if let Ok(model) = serde_json::from_str::<VideoModel>(&format!("\"{}\"", e.value())) {
                                            form.write().model = model;
                                            estimate_cost(());
                                        }
                                    },
                                    class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                                    // 显示当前服务商的可用模型
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

                    // 右侧：高级设置
                    div { class: "space-y-4",
                        button {
                            onclick: move |_| {
                            let current = *show_advanced.read();
                            show_advanced.set(!current);
                        },
                            class: "text-blue-600 hover:text-blue-800 text-sm font-medium",
                            if *show_advanced.read() {
                                "隐藏高级设置"
                            } else {
                                "显示高级设置"
                            }
                        }

                        if *show_advanced.read() {
                            div { class: "space-y-4 border-t pt-4",
                                // 尺寸设置
                                div { class: "grid grid-cols-2 gap-4",
                                    div {
                                        label { class: "block text-sm font-medium text-gray-700 mb-1", "宽度" }
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
                                        label { class: "block text-sm font-medium text-gray-700 mb-1", "高度" }
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

                                // 时长和质量
                                div { class: "grid grid-cols-2 gap-4",
                                    div {
                                        label { class: "block text-sm font-medium text-gray-700 mb-1", "时长 (秒)" }
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
                                        label { class: "block text-sm font-medium text-gray-700 mb-1", "质量" }
                                        select {
                                            value: format!("{:?}", form.read().quality),
                                            onchange: move |e| {
                                                if let Ok(quality) = serde_json::from_str::<VideoQuality>(&format!("\"{}\"", e.value())) {
                                                    form.write().quality = quality;
                                                    estimate_cost(());
                                                }
                                            },
                                            class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                                            option { value: "Standard", "标准 (480p)" }
                                            option { value: "HD", "高清 (720p)" }
                                            option { value: "Premium", "超清 (1080p+)" }
                                        }
                                    }
                                }

                                // 帧率和种子
                                div { class: "grid grid-cols-2 gap-4",
                                    div {
                                        label { class: "block text-sm font-medium text-gray-700 mb-1", "帧率 (FPS)" }
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
                                        label { class: "block text-sm font-medium text-gray-700 mb-1", "随机种子 (可选)" }
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

                        // 成本估算
                        div { class: "bg-blue-50 border border-blue-200 rounded-lg p-4",
                            div { class: "flex justify-between items-center",
                                span { class: "text-sm font-medium text-gray-700", "预估成本" }
                                span { class: "text-lg font-bold text-blue-600", "预估中..." }
                            }
                            p { class: "text-xs text-gray-600 mt-1", "基于参数计算成本" }
                        }
                    }
                }

                // 生成按钮
                div { class: "mt-6 flex justify-center",
                    button {
                        onclick: handle_generate,
                        disabled: is_generating(),
                        class: "px-8 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:bg-gray-400 disabled:cursor-not-allowed transition-colors font-medium",
                        if is_generating() {
                            "生成中..."
                        } else {
                            "生成视频"
                        }
                    }
                }

                // 生成结果
                if let Some(result) = generation_result.read().clone() {
                    div { class: "mt-6 border-t pt-6",
                        h3 { class: "text-lg font-semibold mb-4", "生成结果" }
                        div { class: "bg-gray-50 rounded-lg p-4",
                            div { class: "grid grid-cols-1 md:grid-cols-2 gap-4 mb-4",
                                div {
                                    span { class: "text-sm text-gray-600", "时长: " }
                                    span { class: "font-medium", "已完成" }
                                }
                                div {
                                    span { class: "text-sm text-gray-600", "实际成本: " }
                                    span { class: "font-medium", "已计算" }
                                }
                                div {
                                    span { class: "text-sm text-gray-600", "状态: " }
                                    span { class: "font-medium text-green-600", "已完成" }
                                }
                                div {
                                    span { class: "text-sm text-gray-600", "任务ID: " }
                                    span { class: "font-mono text-xs", "任务ID" }
                                }
                            }

                            // 视频预览
                            div { class: "space-y-2",
                                video {
                                    controls: true,
                                    width: "100%",
                                    max_width: "640",
                                    class: "rounded-lg shadow-md",
                                    source { src: result.video_url.clone(), r#type: "video/mp4" }
                                    "您的浏览器不支持视频播放"
                                }

                                // 下载按钮
                                a {
                                    href: result.video_url.clone(),
                                    download: "generated_video.mp4",
                                    class: "inline-flex items-center px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 transition-colors",
                                    "下载视频"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}