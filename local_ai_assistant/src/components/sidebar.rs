//! Sidebar Component for Session Management

use dioxus::prelude::*;
use crate::models::Session;

#[component]
pub fn Sidebar(
    sessions: Signal<Vec<Session>>,
    current_session: Signal<Option<Session>>,
    on_new_session: EventHandler<()>,
    on_select_session: EventHandler<Session>,
    on_toggle_settings: EventHandler<()>,
    sidebar_collapsed: Signal<bool>,
) -> Element {
    if sidebar_collapsed() {
        return rsx! {};
    }

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
                                    class: "truncate font-medium text-slate-100",
                                    "{session.title}"
                                }
                                div {
                                    class: "text-xs text-slate-400 mt-1",
                                    {session.created_at.format("%m/%d %H:%M").to_string()}
                                }
                            }
                        }
                    }
                }
            }

            // Footer with settings button
            div {
                class: "p-4 border-t border-gray-700 space-y-3",

                // Settings button
                button {
                    class: "w-full py-2 px-4 bg-slate-700 hover:bg-slate-600 rounded-lg flex items-center gap-3 transition-colors",
                    onclick: move |_| on_toggle_settings.call(()),
                    svg {
                        class: "w-5 h-5 text-slate-300",
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
                    span {
                        class: "text-slate-300",
                        "Settings"
                    }
                }

                div {
                    class: "text-xs text-gray-500 text-center",
                    "Local AI Assistant v0.1.0"
                }
            }
        }
    }
}
