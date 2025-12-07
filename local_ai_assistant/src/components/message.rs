//! Message Component
//!
//! Renders individual chat messages with Markdown support.

use comrak::{markdown_to_html_with_plugins, ExtensionOptions, Plugins, RenderOptions, RenderPlugins};
use comrak::plugins::syntect::SyntectAdapterBuilder;
use crate::models::{ChatMessage, ChatRole};
use dioxus::prelude::*;

/// Message component for rendering individual chat messages
#[component]
pub fn Message(message: ChatMessage) -> Element {
    let is_assistant = message.role == ChatRole::Assistant;
    let is_empty = message.content.is_empty();

    // Process markdown content to HTML with syntax highlighting
    let content = use_memo(move || {
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

    let message_class = "max-w-[70rem] p-4 mb-2 break-words";
    let user_class = "self-end bg-blue-500 rounded-tl-lg rounded-tr-lg rounded-bl-lg text-white";
    let assistant_class = "self-start max-w-full text-gray-200";

    rsx! {
        div {
            class: "{message_class}",
            class: if is_assistant { "{assistant_class}" } else { "{user_class}" },
            class: if is_assistant && is_empty { "text-gray-400" },

            if is_assistant && is_empty {
                // Loading animation for empty assistant messages
                div {
                    class: "flex flex-col items-center justify-center min-h-[20px] w-full",
                    div {
                        class: "flex flex-row gap-1 justify-center items-center",
                        div {
                            class: "w-2 h-2 rounded-full bg-gray-100 animate-bounce [animation-delay:.7s]"
                        }
                        div {
                            class: "w-2 h-2 rounded-full bg-gray-100 animate-bounce [animation-delay:.3s]"
                        }
                        div {
                            class: "w-2 h-2 rounded-full bg-gray-100 animate-bounce [animation-delay:.7s]"
                        }
                    }
                }
            } else {
                // Render the processed HTML content
                div {
                    dangerous_inner_html: content
                }
            }
        }
    }
}
