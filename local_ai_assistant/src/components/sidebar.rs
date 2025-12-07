//! Sidebar Component for Session Management

use dioxus::prelude::*;
use crate::models::Session;

#[component]
pub fn Sidebar(
    sessions: Signal<Vec<Session>>,
    current_session: Signal<Option<Session>>,
    on_new_session: EventHandler<()>,
    on_select_session: EventHandler<Session>,
) -> Element {
    rsx! {
        aside {
            class: "w-64 bg-gray-800 border-r border-gray-700 flex flex-col",

            // New chat button
            div {
                class: "p-4",
                button {
                    class: "w-full py-2 px-4 bg-blue-600 hover:bg-blue-700 rounded-lg flex items-center justify-center gap-2 transition-colors",
                    onclick: move |_| on_new_session.call(()),
                    svg {
                        class: "w-5 h-5",
                        fill: "none",
                        stroke: "currentColor",
                        view_box: "0 0 24 24",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            d: "M12 4v16m8-8H4"
                        }
                    }
                    span { "New Chat" }
                }
            }

            // Session list
            div {
                class: "flex-1 overflow-y-auto px-2",
                for session in sessions() {
                    {
                        let is_active = current_session().map(|s| s.id == session.id).unwrap_or(false);
                        let session_clone = session.clone();
                        rsx! {
                            button {
                                key: "{session.id}",
                                class: if is_active {
                                    "w-full text-left p-3 rounded-lg mb-1 bg-gray-700"
                                } else {
                                    "w-full text-left p-3 rounded-lg mb-1 hover:bg-gray-700 transition-colors"
                                },
                                onclick: move |_| on_select_session.call(session_clone.clone()),
                                div {
                                    class: "truncate font-medium",
                                    "{session.title}"
                                }
                                div {
                                    class: "text-xs text-gray-400 mt-1",
                                    {session.created_at.format("%m/%d %H:%M").to_string()}
                                }
                            }
                        }
                    }
                }
            }

            // Footer
            div {
                class: "p-4 border-t border-gray-700",
                div {
                    class: "text-xs text-gray-500 text-center",
                    "Local AI Assistant v0.1.0"
                }
            }
        }
    }
}
