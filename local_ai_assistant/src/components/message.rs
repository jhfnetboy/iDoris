//! Message Component
//!
//! Renders individual chat messages with Markdown support and modern styling.

use comrak::{markdown_to_html_with_plugins, ExtensionOptions, Plugins, RenderOptions, RenderPlugins};
use comrak::plugins::syntect::SyntectAdapterBuilder;
use crate::models::{ChatMessage, ChatRole, AppSettings};
use dioxus::prelude::*;

/// Message component for rendering individual chat messages
/// Uses index-based access to maintain reactivity with the parent's Signal<Vec<ChatMessage>>
#[component]
pub fn Message(messages: Signal<Vec<ChatMessage>>, index: usize, settings: Signal<AppSettings>) -> Element {
    // Read the message reactively by accessing the signal
    let is_assistant = use_memo(move || {
        messages.read().get(index).map(|m| m.role == ChatRole::Assistant).unwrap_or(false)
    });

    let is_empty = use_memo(move || {
        messages.read().get(index).map(|m| m.role == ChatRole::Assistant && m.content.is_empty()).unwrap_or(false)
    });

    // Process markdown content to HTML with syntax highlighting
    let content = use_memo(move || {
        let msgs = messages.read();
        let Some(message) = msgs.get(index) else {
            return String::new();
        };
        let msg_content = &message.content;

        if msg_content.is_empty() {
            return String::new();
        }

        // Configure syntax highlighter with dark theme
        let syntec_adapter = SyntectAdapterBuilder::new()
            .theme("base16-ocean.dark")
            .build();

        // Set up Comrak plugins for rendering with syntax highlighting
        let plugins = Plugins::builder()
            .render(
                RenderPlugins::builder()
                    .codefence_syntax_highlighter(&syntec_adapter)
                    .build()
            ).build();

        // Configure markdown extension options
        let extension_options = ExtensionOptions::builder()
            .strikethrough(true)
            .tagfilter(true)
            .autolink(true)
            .table(true)
            .build();

        // Configure HTML rendering options
        let render_options = RenderOptions::builder()
            .hardbreaks(true)
            .github_pre_lang(true)
            .build();

        let options = comrak::Options {
            extension: extension_options,
            render: render_options,
            ..Default::default()
        };

        markdown_to_html_with_plugins(msg_content, &options, &plugins)
    });

    rsx! {
        div {
            class: "flex w-full mb-4",
            class: if *is_assistant.read() { "justify-start" } else { "justify-end" },

            div {
                class: "flex items-start gap-3 max-w-[85%]",
                class: if !*is_assistant.read() { "flex-row-reverse" },

                // Avatar
                div {
                    class: "flex-shrink-0 w-8 h-8 rounded-full flex items-center justify-center text-white text-sm font-medium",
                    class: if *is_assistant.read() { "bg-gradient-to-br from-emerald-500 to-teal-600" } else { "bg-gradient-to-br from-blue-500 to-indigo-600" },
                    if *is_assistant.read() { "AI" } else { "U" }
                }

                // Message bubble
                div {
                    class: "px-4 py-3 rounded-2xl",
                    class: if *is_assistant.read() {
                        "bg-slate-700/50 text-slate-100 rounded-tl-sm"
                    } else {
                        "bg-gradient-to-br from-blue-500 to-indigo-600 text-white rounded-tr-sm"
                    },

                    if *is_empty.read() {
                        // Typing indicator for empty assistant messages
                        div {
                            class: "flex items-center gap-1.5 py-1 px-2",
                            div {
                                class: "w-2 h-2 rounded-full bg-slate-400 animate-bounce",
                                style: "animation-delay: 0ms;"
                            }
                            div {
                                class: "w-2 h-2 rounded-full bg-slate-400 animate-bounce",
                                style: "animation-delay: 150ms;"
                            }
                            div {
                                class: "w-2 h-2 rounded-full bg-slate-400 animate-bounce",
                                style: "animation-delay: 300ms;"
                            }
                        }
                    } else {
                        // Render the processed HTML content with dynamic font size
                        {
                            let font_style = settings.read().font_size.font_style();
                            rsx! {
                                div {
                                    class: "prose prose-invert max-w-none",
                                    class: "[&_pre]:bg-slate-800/80 [&_pre]:rounded-lg [&_pre]:p-3 [&_pre]:my-2 [&_pre]:overflow-x-auto",
                                    class: "[&_code]:bg-slate-800/60 [&_code]:px-1.5 [&_code]:py-0.5 [&_code]:rounded [&_code]:text-emerald-400 [&_code]:text-sm",
                                    class: "[&_pre_code]:bg-transparent [&_pre_code]:p-0",
                                    class: "[&_p]:my-1.5 [&_ul]:my-1.5 [&_ol]:my-1.5",
                                    class: "[&_a]:text-blue-400 [&_a:hover]:text-blue-300",
                                    class: "[&_strong]:text-white [&_em]:text-slate-300",
                                    style: "{font_style}",
                                    dangerous_inner_html: content
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
