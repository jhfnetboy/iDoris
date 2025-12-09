//! Sidebar Component for Session Management

use dioxus::prelude::*;
use crate::models::Session;
use super::ActivePanel;

#[component]
pub fn Sidebar(
    sessions: Signal<Vec<Session>>,
    current_session: Signal<Option<Session>>,
    active_panel: Signal<ActivePanel>,
    on_new_session: EventHandler<()>,
    on_select_session: EventHandler<Session>,
    on_toggle_settings: EventHandler<()>,
    on_select_panel: EventHandler<ActivePanel>,
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

            // Panel selector menu
            div {
                class: "p-3 border-t border-gray-700",
                div {
                    class: "text-xs text-slate-500 uppercase font-semibold mb-2 px-1",
                    "Panels"
                }

                // Chat panel button
                button {
                    class: if matches!(active_panel(), ActivePanel::Chat) {
                        "w-full py-2 px-3 bg-blue-600 rounded-lg flex items-center gap-3 transition-colors mb-2"
                    } else {
                        "w-full py-2 px-3 hover:bg-slate-700 rounded-lg flex items-center gap-3 transition-colors mb-2"
                    },
                    onclick: move |_| on_select_panel.call(ActivePanel::Chat),
                    svg {
                        class: "w-5 h-5",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        view_box: "0 0 24 24",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"
                        }
                    }
                    span { "Chat" }
                }

                // Image Generation panel button
                button {
                    class: if matches!(active_panel(), ActivePanel::ImageGen) {
                        "w-full py-2 px-3 bg-purple-600 rounded-lg flex items-center gap-3 transition-colors mb-2"
                    } else {
                        "w-full py-2 px-3 hover:bg-slate-700 rounded-lg flex items-center gap-3 transition-colors mb-2"
                    },
                    onclick: move |_| on_select_panel.call(ActivePanel::ImageGen),
                    svg {
                        class: "w-5 h-5",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        view_box: "0 0 24 24",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"
                        }
                    }
                    span { "Image Gen" }
                }

                // TTS panel button
                button {
                    class: if matches!(active_panel(), ActivePanel::Tts) {
                        "w-full py-2 px-3 bg-green-600 rounded-lg flex items-center gap-3 transition-colors mb-2"
                    } else {
                        "w-full py-2 px-3 hover:bg-slate-700 rounded-lg flex items-center gap-3 transition-colors mb-2"
                    },
                    onclick: move |_| on_select_panel.call(ActivePanel::Tts),
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
                    span { "Text to Speech" }
                }

                // Content Editor panel button
                button {
                    class: if matches!(active_panel(), ActivePanel::ContentEditor) {
                        "w-full py-2 px-3 bg-orange-600 rounded-lg flex items-center gap-3 transition-colors mb-2"
                    } else {
                        "w-full py-2 px-3 hover:bg-slate-700 rounded-lg flex items-center gap-3 transition-colors mb-2"
                    },
                    onclick: move |_| on_select_panel.call(ActivePanel::ContentEditor),
                    svg {
                        class: "w-5 h-5",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        view_box: "0 0 24 24",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"
                        }
                    }
                    span { "Content Editor" }
                }

                // Future panels (commented out for now)
                // Video Gen
                // button {
                //     class: "w-full py-2 px-3 hover:bg-slate-700 rounded-lg flex items-center gap-3 transition-colors opacity-50 cursor-not-allowed mb-2",
                //     disabled: true,
                //     svg { ... }
                //     span { "Video Gen" }
                //     span { class: "text-xs text-slate-500 ml-auto", "Soon" }
                // }
            }

            // Footer with settings button
            div {
                class: "p-3 border-t border-gray-700 space-y-2",

                // Settings button
                button {
                    class: "w-full py-2 px-3 hover:bg-slate-700 rounded-lg flex items-center gap-3 transition-colors",
                    onclick: move |_| on_toggle_settings.call(()),
                    svg {
                        class: "w-5 h-5 text-slate-400",
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
                        class: "text-slate-400",
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
