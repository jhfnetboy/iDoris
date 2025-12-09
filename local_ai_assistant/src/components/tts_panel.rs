//! Text-to-Speech Panel Component
//!
//! UI for testing TTS functionality with different engines.

use dioxus::prelude::*;

use crate::server_functions::generate_tts;

/// TTS Panel component for testing text-to-speech
#[component]
pub fn TtsPanel(
    on_open_settings: EventHandler<()>,
) -> Element {
    let mut input_text = use_signal(|| "Hello, welcome to the Local AI Assistant!".to_string());
    let mut is_generating = use_signal(|| false);
    let mut error_message: Signal<Option<String>> = use_signal(|| None);
    let mut audio_url: Signal<Option<String>> = use_signal(|| None);
    let mut selected_engine = use_signal(|| "system".to_string());
    let mut speed = use_signal(|| 1.0f32);

    // Handle TTS generation
    let handle_generate = move |_| {
        let text = input_text.read().clone();
        let engine = selected_engine.read().clone();
        let spd = *speed.read();

        if text.trim().is_empty() {
            error_message.set(Some("Please enter some text".to_string()));
            return;
        }

        is_generating.set(true);
        error_message.set(None);
        audio_url.set(None);

        spawn(async move {
            match generate_tts(text, engine, spd).await {
                Ok(url) => {
                    audio_url.set(Some(url));
                    is_generating.set(false);
                }
                Err(e) => {
                    error_message.set(Some(format!("Generation failed: {:?}", e)));
                    is_generating.set(false);
                }
            }
        });
    };

    rsx! {
        div {
            class: "flex-1 flex flex-col p-6 overflow-y-auto",

            // Title and description
            div {
                class: "mb-6",
                h2 {
                    class: "text-2xl font-bold text-white mb-2",
                    "Text to Speech"
                }
                p {
                    class: "text-slate-400",
                    "Convert text to speech using various TTS engines. VibeVoice requires model download."
                }
            }

            // Engine selection
            div {
                class: "mb-4",
                label {
                    class: "block text-sm font-medium text-slate-300 mb-2",
                    "TTS Engine"
                }
                div {
                    class: "flex gap-3",

                    // System TTS
                    button {
                        class: if selected_engine() == "system" {
                            "px-4 py-2 rounded-lg bg-green-600 text-white"
                        } else {
                            "px-4 py-2 rounded-lg bg-slate-700 text-slate-300 hover:bg-slate-600"
                        },
                        onclick: move |_| selected_engine.set("system".to_string()),
                        "System TTS"
                    }

                    // VibeVoice
                    button {
                        class: if selected_engine() == "vibevoice" {
                            "px-4 py-2 rounded-lg bg-purple-600 text-white"
                        } else {
                            "px-4 py-2 rounded-lg bg-slate-700 text-slate-300 hover:bg-slate-600"
                        },
                        onclick: move |_| selected_engine.set("vibevoice".to_string()),
                        "VibeVoice"
                    }
                }

                // Engine info
                p {
                    class: "mt-2 text-xs text-slate-500",
                    match selected_engine().as_str() {
                        "system" => "macOS built-in TTS (always available)",
                        "vibevoice" => "Microsoft VibeVoice-Realtime-0.5B (~300ms latency)",
                        _ => ""
                    }
                }
            }

            // Speed control
            div {
                class: "mb-4",
                label {
                    class: "block text-sm font-medium text-slate-300 mb-2",
                    "Speed: {speed:.1}x"
                }
                input {
                    r#type: "range",
                    class: "w-full h-2 bg-slate-700 rounded-lg appearance-none cursor-pointer",
                    min: "0.5",
                    max: "2.0",
                    step: "0.1",
                    value: "{speed}",
                    oninput: move |e| {
                        if let Ok(val) = e.value().parse::<f32>() {
                            speed.set(val);
                        }
                    }
                }
            }

            // Text input
            div {
                class: "mb-4",
                label {
                    class: "block text-sm font-medium text-slate-300 mb-2",
                    "Text to speak"
                }
                textarea {
                    class: "w-full h-32 px-4 py-3 bg-slate-700 border border-slate-600 rounded-lg text-white placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-blue-500 resize-none",
                    placeholder: "Enter text to convert to speech...",
                    value: "{input_text}",
                    oninput: move |e| input_text.set(e.value()),
                }
                p {
                    class: "mt-1 text-xs text-slate-500",
                    "{input_text.read().len()} characters"
                }
            }

            // Generate button
            button {
                class: if is_generating() {
                    "w-full py-3 px-6 bg-slate-600 text-slate-400 rounded-lg cursor-not-allowed"
                } else {
                    "w-full py-3 px-6 bg-green-600 hover:bg-green-700 text-white rounded-lg transition-colors font-medium"
                },
                disabled: is_generating(),
                onclick: handle_generate,
                if is_generating() {
                    div {
                        class: "flex items-center justify-center gap-2",
                        div {
                            class: "w-5 h-5 border-2 border-slate-400 border-t-transparent rounded-full animate-spin"
                        }
                        span { "Generating..." }
                    }
                } else {
                    div {
                        class: "flex items-center justify-center gap-2",
                        svg {
                            class: "w-5 h-5",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            view_box: "0 0 24 24",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                d: "M15.536 8.464a5 5 0 010 7.072m2.828-9.9a9 9 0 010 12.728M5.586 15H4a1 1 0 01-1-1v-4a1 1 0 011-1h1.586l4.707-4.707C10.923 3.663 12 4.109 12 5v14c0 .891-1.077 1.337-1.707.707L5.586 15z"
                            }
                        }
                        span { "Generate Speech" }
                    }
                }
            }

            // Error message
            if let Some(err) = error_message() {
                div {
                    class: "mt-4 p-3 bg-red-900/50 border border-red-700 rounded-lg text-red-300 text-sm",
                    "{err}"
                }
            }

            // Audio player
            if let Some(url) = audio_url() {
                div {
                    class: "mt-6 p-4 bg-slate-700/50 rounded-lg",
                    h3 {
                        class: "text-sm font-medium text-slate-300 mb-3",
                        "Generated Audio"
                    }
                    audio {
                        class: "w-full",
                        controls: true,
                        autoplay: true,
                        src: "{url}"
                    }
                }
            }

            // VibeVoice model info
            if selected_engine() == "vibevoice" {
                div {
                    class: "mt-6 p-4 bg-purple-900/30 border border-purple-700/50 rounded-lg",
                    div {
                        class: "flex items-start gap-3",
                        svg {
                            class: "w-5 h-5 text-purple-400 mt-0.5 flex-shrink-0",
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
                            p {
                                class: "text-purple-300 text-sm",
                                "VibeVoice model needs to be downloaded first."
                            }
                            p {
                                class: "text-purple-400 text-xs mt-1",
                                "Check ~/models/VibeVoice-Realtime-0.5B folder."
                            }
                            button {
                                class: "mt-2 text-xs text-purple-300 underline hover:text-purple-200",
                                onclick: move |_| on_open_settings.call(()),
                                "Open Settings to manage models"
                            }
                        }
                    }
                }
            }

            // Tips section
            div {
                class: "mt-6 p-4 bg-slate-800/50 rounded-lg",
                h3 {
                    class: "text-sm font-medium text-slate-300 mb-2",
                    "Tips"
                }
                ul {
                    class: "text-xs text-slate-400 space-y-1",
                    li { "• System TTS uses macOS built-in voices (always available)" }
                    li { "• VibeVoice provides more natural speech but requires model download" }
                    li { "• Adjust speed for faster or slower playback" }
                    li { "• Shorter text generates faster" }
                }
            }
        }
    }
}
