//! Chat Component

use dioxus::prelude::*;
use crate::models::{ChatMessage, ChatRole, Session};
use super::Message;

#[component]
pub fn Chat(
    messages: Signal<Vec<ChatMessage>>,
    current_session: Signal<Option<Session>>,
    is_loading: Signal<bool>,
    model_ready: Signal<bool>,
) -> Element {
    let mut input_text = use_signal(String::new);

    let send_message = move |_| {
        let text = input_text().trim().to_string();
        if text.is_empty() || !model_ready() {
            return;
        }

        if let Some(session) = current_session() {
            // Add user message
            let user_msg = ChatMessage::user(session.id, text.clone());
            messages.write().push(user_msg);

            // Clear input
            input_text.set(String::new());

            // Add placeholder for assistant response
            let assistant_msg = ChatMessage::assistant(session.id, String::new());
            messages.write().push(assistant_msg);

            is_loading.set(true);

            // TODO: Call server function to get response
            // spawn(async move {
            //     match get_response(session.id.to_string(), text).await {
            //         Ok(stream) => { ... }
            //         Err(e) => { ... }
            //     }
            // });
        }
    };

    rsx! {
        div {
            class: "flex-1 flex flex-col",

            // Messages area
            div {
                class: "flex-1 overflow-y-auto p-4 space-y-4",

                if messages().is_empty() {
                    // Empty state
                    div {
                        class: "h-full flex items-center justify-center",
                        div {
                            class: "text-center text-gray-500",
                            div {
                                class: "text-6xl mb-4",
                                "🤖"
                            }
                            h2 {
                                class: "text-xl font-semibold mb-2",
                                "Welcome to Local AI Assistant"
                            }
                            p {
                                class: "text-sm",
                                "Start a conversation by typing a message below."
                            }
                        }
                    }
                } else {
                    for msg in messages() {
                        Message {
                            key: "{msg.id}",
                            message: msg
                        }
                    }
                }
            }

            // Input area
            div {
                class: "border-t border-gray-700 p-4",
                form {
                    class: "flex gap-2",
                    onsubmit: send_message,

                    input {
                        class: "flex-1 bg-gray-800 border border-gray-600 rounded-lg px-4 py-2 focus:outline-none focus:border-blue-500",
                        r#type: "text",
                        placeholder: if model_ready() { "Type your message..." } else { "Waiting for model to load..." },
                        disabled: !model_ready(),
                        value: "{input_text}",
                        oninput: move |e| input_text.set(e.value())
                    }

                    button {
                        class: if model_ready() && !input_text().is_empty() {
                            "px-4 py-2 bg-blue-600 hover:bg-blue-700 rounded-lg transition-colors"
                        } else {
                            "px-4 py-2 bg-gray-600 rounded-lg cursor-not-allowed"
                        },
                        r#type: "submit",
                        disabled: !model_ready() || input_text().is_empty(),

                        if is_loading() {
                            // Loading spinner
                            svg {
                                class: "w-5 h-5 animate-spin",
                                fill: "none",
                                view_box: "0 0 24 24",
                                circle {
                                    class: "opacity-25",
                                    cx: "12",
                                    cy: "12",
                                    r: "10",
                                    stroke: "currentColor",
                                    stroke_width: "4"
                                }
                                path {
                                    class: "opacity-75",
                                    fill: "currentColor",
                                    d: "M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"
                                }
                            }
                        } else {
                            // Send icon
                            svg {
                                class: "w-5 h-5",
                                fill: "none",
                                stroke: "currentColor",
                                view_box: "0 0 24 24",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    d: "M12 19l9 2-9-18-9 18 9-2zm0 0v-8"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
