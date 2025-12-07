//! Main Application Component

use dioxus::prelude::*;
use crate::models::{Session, ChatMessage};
use super::{Sidebar, Chat};

/// Main application component
#[component]
pub fn App() -> Element {
    // Current active session
    let mut current_session: Signal<Option<Session>> = use_signal(|| None);

    // List of all sessions
    let mut sessions: Signal<Vec<Session>> = use_signal(Vec::new);

    // Messages for current session
    let mut messages: Signal<Vec<ChatMessage>> = use_signal(Vec::new);

    // Model initialization status
    let model_ready: Signal<bool> = use_signal(|| false);

    // Loading state
    let is_loading: Signal<bool> = use_signal(|| false);

    rsx! {
        div {
            class: "flex h-screen bg-gray-900 text-white",

            // Sidebar with session list
            Sidebar {
                sessions: sessions,
                current_session: current_session,
                on_new_session: move |_| {
                    let new_session = Session::default_title();
                    sessions.write().push(new_session.clone());
                    current_session.set(Some(new_session));
                    messages.write().clear();
                },
                on_select_session: move |session: Session| {
                    current_session.set(Some(session));
                    // TODO: Load messages for session
                    messages.write().clear();
                }
            }

            // Main chat area
            div {
                class: "flex-1 flex flex-col",

                // Header
                header {
                    class: "h-14 border-b border-gray-700 flex items-center px-4",
                    h1 {
                        class: "text-lg font-semibold",
                        if let Some(session) = current_session() {
                            "{session.title}"
                        } else {
                            "Local AI Assistant"
                        }
                    }

                    // Model status indicator
                    div {
                        class: "ml-auto flex items-center gap-2",
                        div {
                            class: if model_ready() { "w-2 h-2 rounded-full bg-green-500" } else { "w-2 h-2 rounded-full bg-yellow-500 animate-pulse" }
                        }
                        span {
                            class: "text-sm text-gray-400",
                            if model_ready() { "Ready" } else { "Loading..." }
                        }
                    }
                }

                // Chat component
                Chat {
                    messages: messages,
                    current_session: current_session,
                    is_loading: is_loading,
                    model_ready: model_ready
                }
            }
        }
    }
}
