//! Main Application Component

use dioxus::prelude::*;
use crate::models::{Session, ChatMessage, AppSettings};
use crate::server_functions::get_session_messages;
use super::{Sidebar, Chat, SettingsPage};

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

    // Settings state
    let settings: Signal<AppSettings> = use_signal(AppSettings::default);
    let mut show_settings: Signal<bool> = use_signal(|| false);

    // Sidebar collapsed state
    let mut sidebar_collapsed: Signal<bool> = use_signal(|| false);

    // Note: Sessions are loaded after SQLite initialization in Chat component

    // Get theme classes from settings
    let theme = settings.read().theme.clone();
    let bg_class = theme.bg_class();
    let text_class = theme.text_class();

    rsx! {
        div {
            class: "flex h-screen {bg_class} {text_class}",

            // Sidebar toggle button (visible when collapsed)
            if sidebar_collapsed() {
                button {
                    class: "fixed top-3 left-3 z-30 p-2 rounded-lg bg-slate-700 hover:bg-slate-600 transition-colors",
                    onclick: move |_| sidebar_collapsed.set(false),
                    svg {
                        class: "w-5 h-5 text-white",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        view_box: "0 0 24 24",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M4 6h16M4 12h16M4 18h16"
                        }
                    }
                }
            }

            // Sidebar with session list
            Sidebar {
                sessions: sessions,
                current_session: current_session,
                on_new_session: move |_| {
                    let new_session = Session::default_title();
                    sessions.write().insert(0, new_session.clone());
                    current_session.set(Some(new_session));
                    messages.write().clear();
                },
                on_select_session: move |session: Session| {
                    let session_id = session.id.to_string();
                    current_session.set(Some(session));
                    // Load messages for selected session
                    spawn(async move {
                        match get_session_messages(session_id).await {
                            Ok(loaded_messages) => {
                                messages.set(loaded_messages);
                            }
                            Err(e) => {
                                println!("Error loading messages: {:?}", e);
                                messages.set(Vec::new());
                            }
                        }
                    });
                },
                on_toggle_settings: move |_| {
                    show_settings.set(!show_settings());
                },
                sidebar_collapsed: sidebar_collapsed,
            }

            // Settings page (full-page overlay)
            if show_settings() {
                SettingsPage {
                    settings: settings,
                    on_close: move |_| show_settings.set(false),
                }
            }

            // Main chat area
            div {
                class: "flex-1 flex flex-col",

                // Header
                header {
                    class: "h-14 border-b border-gray-700 flex items-center px-4",

                    // Sidebar collapse button
                    if !sidebar_collapsed() {
                        button {
                            class: "p-2 mr-3 rounded-lg hover:bg-slate-700 transition-colors",
                            onclick: move |_| sidebar_collapsed.set(true),
                            svg {
                                class: "w-5 h-5 text-slate-400",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                view_box: "0 0 24 24",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    d: "M11 19l-7-7 7-7m8 14l-7-7 7-7"
                                }
                            }
                        }
                    }

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
                    sessions: sessions,
                    is_loading: is_loading,
                    model_ready: model_ready,
                    settings: settings,
                }
            }
        }
    }
}
