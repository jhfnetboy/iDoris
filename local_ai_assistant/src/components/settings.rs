//! Settings Panel Component

use dioxus::prelude::*;
use crate::models::{AppSettings, ResponseLanguage, Theme, FontSize};

#[component]
pub fn SettingsPanel(
    settings: Signal<AppSettings>,
    show_settings: Signal<bool>,
) -> Element {
    if !show_settings() {
        return rsx! {};
    }

    let current = settings.read().clone();

    rsx! {
        // Backdrop
        div {
            class: "fixed inset-0 bg-black/50 backdrop-blur-sm z-40",
            onclick: move |_| show_settings.set(false),
        }

        // Settings panel
        div {
            class: "fixed left-64 top-0 bottom-0 w-80 bg-slate-800 border-r border-slate-700 z-50 shadow-xl overflow-y-auto",

            // Header
            div {
                class: "flex items-center justify-between p-4 border-b border-slate-700",
                h2 {
                    class: "text-lg font-semibold text-white",
                    "Settings"
                }
                button {
                    class: "p-1 rounded hover:bg-slate-700 transition-colors",
                    onclick: move |_| show_settings.set(false),
                    svg {
                        class: "w-5 h-5 text-slate-400",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        view_box: "0 0 24 24",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M6 18L18 6M6 6l12 12"
                        }
                    }
                }
            }

            // Settings content
            div {
                class: "p-4 space-y-6",

                // Model Selection
                div {
                    class: "space-y-2",
                    label {
                        class: "block text-sm font-medium text-slate-300",
                        "AI Model"
                    }
                    select {
                        class: "w-full px-3 py-2 bg-slate-700 border border-slate-600 rounded-lg text-white focus:outline-none focus:border-blue-500",
                        value: "{current.model_name}",
                        onchange: {
                            let mut settings = settings.clone();
                            move |e: Event<FormData>| {
                                let mut s = settings.read().clone();
                                s.model_name = e.value();
                                settings.set(s);
                            }
                        },
                        option { value: "Qwen 2.5 7B", "Qwen 2.5 7B" }
                        option { value: "Qwen 2.5 3B", "Qwen 2.5 3B" }
                        option { value: "Llama 3.2 8B", "Llama 3.2 8B" }
                    }
                }

                // Response Language
                div {
                    class: "space-y-2",
                    label {
                        class: "block text-sm font-medium text-slate-300",
                        "Response Language"
                    }
                    div {
                        class: "space-y-1",
                        { render_language_option(settings.clone(), ResponseLanguage::Chinese, "中文 (Chinese)", current.language == ResponseLanguage::Chinese) }
                        { render_language_option(settings.clone(), ResponseLanguage::English, "English", current.language == ResponseLanguage::English) }
                        { render_language_option(settings.clone(), ResponseLanguage::Thai, "ไทย (Thai)", current.language == ResponseLanguage::Thai) }
                    }
                }

                // Theme
                div {
                    class: "space-y-2",
                    label {
                        class: "block text-sm font-medium text-slate-300",
                        "Theme"
                    }
                    div {
                        class: "grid grid-cols-2 gap-2",
                        { render_theme_option(settings.clone(), Theme::Dark, "Dark", current.theme == Theme::Dark) }
                        { render_theme_option(settings.clone(), Theme::Light, "Light", current.theme == Theme::Light) }
                        { render_theme_option(settings.clone(), Theme::Blue, "Blue", current.theme == Theme::Blue) }
                        { render_theme_option(settings.clone(), Theme::Purple, "Purple", current.theme == Theme::Purple) }
                    }
                }

                // Font Size
                div {
                    class: "space-y-2",
                    label {
                        class: "block text-sm font-medium text-slate-300",
                        "Response Font Size"
                    }
                    div {
                        class: "space-y-1",
                        { render_font_size_option(settings.clone(), FontSize::Small, current.font_size == FontSize::Small) }
                        { render_font_size_option(settings.clone(), FontSize::Medium, current.font_size == FontSize::Medium) }
                        { render_font_size_option(settings.clone(), FontSize::Large, current.font_size == FontSize::Large) }
                        { render_font_size_option(settings.clone(), FontSize::ExtraLarge, current.font_size == FontSize::ExtraLarge) }
                    }
                }
            }
        }
    }
}

fn render_language_option(mut settings: Signal<AppSettings>, lang: ResponseLanguage, label: &str, is_selected: bool) -> Element {
    let lang_clone = lang.clone();
    rsx! {
        button {
            class: if is_selected {
                "w-full px-3 py-2 text-left rounded-lg bg-blue-600 text-white"
            } else {
                "w-full px-3 py-2 text-left rounded-lg bg-slate-700 text-slate-300 hover:bg-slate-600 transition-colors"
            },
            onclick: move |_| {
                let mut s = settings.read().clone();
                s.language = lang_clone.clone();
                settings.set(s);
            },
            "{label}"
        }
    }
}

fn render_theme_option(mut settings: Signal<AppSettings>, theme: Theme, label: &str, is_selected: bool) -> Element {
    let theme_clone = theme.clone();
    let preview_class = match theme {
        Theme::Dark => "bg-gray-900",
        Theme::Light => "bg-gray-100",
        Theme::Blue => "bg-slate-900",
        Theme::Purple => "bg-purple-950",
    };

    rsx! {
        button {
            class: if is_selected {
                "flex items-center gap-2 px-3 py-2 rounded-lg bg-blue-600 text-white"
            } else {
                "flex items-center gap-2 px-3 py-2 rounded-lg bg-slate-700 text-slate-300 hover:bg-slate-600 transition-colors"
            },
            onclick: move |_| {
                let mut s = settings.read().clone();
                s.theme = theme_clone.clone();
                settings.set(s);
            },
            div {
                class: "w-4 h-4 rounded border border-slate-500 {preview_class}"
            }
            "{label}"
        }
    }
}

fn render_font_size_option(mut settings: Signal<AppSettings>, size: FontSize, is_selected: bool) -> Element {
    let size_clone = size.clone();
    let (label, sample_class) = match size {
        FontSize::Small => ("Small", "text-sm"),
        FontSize::Medium => ("Medium", "text-base"),
        FontSize::Large => ("Large", "text-lg"),
        FontSize::ExtraLarge => ("Extra Large", "text-xl"),
    };

    rsx! {
        button {
            class: if is_selected {
                "w-full px-3 py-2 text-left rounded-lg bg-blue-600 text-white flex items-center justify-between"
            } else {
                "w-full px-3 py-2 text-left rounded-lg bg-slate-700 text-slate-300 hover:bg-slate-600 transition-colors flex items-center justify-between"
            },
            onclick: move |_| {
                let mut s = settings.read().clone();
                s.font_size = size_clone.clone();
                settings.set(s);
            },
            span { "{label}" }
            span {
                class: sample_class,
                "Aa"
            }
        }
    }
}
