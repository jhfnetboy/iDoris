//! Chat Component
//!
//! Modern, clean chat interface for the AI assistant.

use dioxus::prelude::*;
use dioxus::html::input_data::keyboard_types::Key;
use crate::models::{ChatMessage, ChatRole, Session, AppSettings};
use crate::server_functions::{get_response, reset_chat, search_context, init_llm_model, init_embedding_model, init_db};
use super::Message;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[derive(Clone)]
struct ChatState {
    input_message: String,
    is_model_answering: bool,
    is_model_loading: bool,
    is_database_loading: bool,
    cancel_token: bool,
    use_context: bool,
}

#[component]
pub fn Chat(
    messages: Signal<Vec<ChatMessage>>,
    current_session: Signal<Option<Session>>,
    sessions: Signal<Vec<Session>>,
    is_loading: Signal<bool>,
    model_ready: Signal<bool>,
    settings: Signal<AppSettings>,
) -> Element {
    let mut state = use_signal(|| ChatState {
        input_message: String::new(),
        is_model_answering: false,
        is_model_loading: true,
        is_database_loading: true,
        cancel_token: false,
        use_context: false,
    });

    use_effect(move || {
        initialize_systems(state.clone(), model_ready.clone());
    });

    use_effect(move || {
        if !messages().is_empty() {
            scroll_to_bottom();
        }
    });

    let is_loading_state = state.read().is_model_loading || state.read().is_database_loading;

    rsx! {
        div {
            class: "flex-1 flex flex-col h-full bg-gradient-to-b from-slate-900 via-slate-800 to-slate-900",

            // Loading overlay
            if is_loading_state {
                { render_loading_screen() }
            }

            // Messages area - centered with max width
            div {
                id: "chat-container",
                class: "flex-1 overflow-y-auto",

                div {
                    class: "max-w-3xl mx-auto px-4 py-6",

                    if messages().is_empty() {
                        { render_empty_state() }
                    } else {
                        div {
                            class: "space-y-6",
                            for (index, msg) in messages().iter().enumerate() {
                                Message {
                                    key: "{msg.id}",
                                    messages: messages,
                                    index: index,
                                    settings: settings,
                                }
                            }
                        }
                    }
                }
            }

            // Input area - fixed at bottom
            { render_input_area(&state, &messages, &current_session, &sessions, &settings) }
        }
    }
}

fn render_empty_state() -> Element {
    rsx! {
        div {
            class: "h-full flex items-center justify-center min-h-[60vh]",
            div {
                class: "text-center space-y-6",

                // Logo/Icon
                div {
                    class: "w-20 h-20 mx-auto rounded-2xl bg-gradient-to-br from-blue-500 to-purple-600 flex items-center justify-center shadow-2xl shadow-blue-500/20",
                    svg {
                        class: "w-10 h-10 text-white",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "1.5",
                        view_box: "0 0 24 24",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M9.813 15.904L9 18.75l-.813-2.846a4.5 4.5 0 00-3.09-3.09L2.25 12l2.846-.813a4.5 4.5 0 003.09-3.09L9 5.25l.813 2.846a4.5 4.5 0 003.09 3.09L15.75 12l-2.846.813a4.5 4.5 0 00-3.09 3.09zM18.259 8.715L18 9.75l-.259-1.035a3.375 3.375 0 00-2.455-2.456L14.25 6l1.036-.259a3.375 3.375 0 002.455-2.456L18 2.25l.259 1.035a3.375 3.375 0 002.456 2.456L21.75 6l-1.035.259a3.375 3.375 0 00-2.456 2.456z"
                        }
                    }
                }

                // Title
                h1 {
                    class: "text-3xl font-semibold text-white",
                    "Local AI Assistant"
                }

                // Subtitle
                p {
                    class: "text-slate-400 text-lg max-w-md mx-auto",
                    "Your private AI running locally. Ask anything."
                }

                // Feature hints
                div {
                    class: "flex flex-wrap justify-center gap-3 mt-8",

                    div {
                        class: "px-4 py-2 rounded-full bg-slate-800/50 border border-slate-700/50 text-slate-300 text-sm",
                        "100% Private"
                    }
                    div {
                        class: "px-4 py-2 rounded-full bg-slate-800/50 border border-slate-700/50 text-slate-300 text-sm",
                        "No Internet Required"
                    }
                    div {
                        class: "px-4 py-2 rounded-full bg-slate-800/50 border border-slate-700/50 text-slate-300 text-sm",
                        "RAG Support"
                    }
                }
            }
        }
    }
}

fn render_loading_screen() -> Element {
    rsx! {
        div {
            class: "absolute inset-0 bg-slate-900/95 backdrop-blur-sm flex flex-col items-center justify-center z-50",

            // Animated loading indicator with glow effect
            div {
                class: "relative mb-8",
                // Glow background
                div {
                    class: "absolute inset-0 w-20 h-20 bg-blue-500/20 rounded-full blur-xl animate-pulse"
                }
                // Spinning ring
                div {
                    class: "w-20 h-20 rounded-full border-4 border-slate-700 border-t-blue-500 animate-spin"
                }
                // Center icon
                div {
                    class: "absolute inset-0 flex items-center justify-center",
                    svg {
                        class: "w-8 h-8 text-blue-400",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "1.5",
                        view_box: "0 0 24 24",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M9.813 15.904L9 18.75l-.813-2.846a4.5 4.5 0 00-3.09-3.09L2.25 12l2.846-.813a4.5 4.5 0 003.09-3.09L9 5.25l.813 2.846a4.5 4.5 0 003.09 3.09L15.75 12l-2.846.813a4.5 4.5 0 00-3.09 3.09z"
                        }
                    }
                }
            }

            h2 {
                class: "text-xl font-semibold text-white mb-2",
                "Initializing AI Model"
            }

            p {
                class: "text-slate-400 text-center max-w-sm mb-6",
                "Loading Qwen 2.5 7B model. First run downloads ~10GB."
            }

            // Progress info box
            div {
                class: "bg-slate-800/50 border border-slate-700/50 rounded-xl p-4 max-w-md",

                div {
                    class: "flex items-start gap-3",

                    // Info icon
                    svg {
                        class: "w-5 h-5 text-blue-400 mt-0.5 flex-shrink-0",
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
                            class: "text-slate-300 text-sm font-medium mb-1",
                            "Download Progress"
                        }
                        p {
                            class: "text-slate-500 text-xs",
                            "Check your terminal for real-time download progress."
                        }
                        p {
                            class: "text-slate-500 text-xs mt-1",
                            "Model is cached after first download."
                        }
                    }
                }
            }

            // Status dots
            div {
                class: "flex items-center gap-2 mt-6",
                div { class: "w-2 h-2 rounded-full bg-blue-500 animate-pulse" }
                div { class: "w-2 h-2 rounded-full bg-blue-500 animate-pulse", style: "animation-delay: 0.2s;" }
                div { class: "w-2 h-2 rounded-full bg-blue-500 animate-pulse", style: "animation-delay: 0.4s;" }
            }
        }
    }
}

fn render_input_area(
    state: &Signal<ChatState>,
    messages: &Signal<Vec<ChatMessage>>,
    current_session: &Signal<Option<Session>>,
    sessions: &Signal<Vec<Session>>,
    settings: &Signal<AppSettings>,
) -> Element {
    let current_state = state.read();
    let is_disabled = current_state.is_model_answering ||
                      current_state.is_model_loading ||
                      current_state.is_database_loading;

    let placeholder = if current_state.is_model_loading || current_state.is_database_loading {
        "Initializing..."
    } else if current_state.is_model_answering {
        "AI is thinking..."
    } else {
        "Type your message..."
    };

    let is_answering = current_state.is_model_answering;
    let is_loading = current_state.is_model_loading || current_state.is_database_loading;
    let is_empty = current_state.input_message.trim().is_empty();
    let can_send = !is_loading && !is_empty;

    rsx! {
        div {
            class: "border-t border-slate-700/50 bg-slate-900/80 backdrop-blur-lg",

            div {
                class: "max-w-3xl mx-auto p-4",

                // RAG Toggle
                div {
                    class: "flex items-center justify-between mb-3",

                    label {
                        class: "flex items-center gap-3 cursor-pointer group",

                        div {
                            class: "relative",
                            input {
                                disabled: is_disabled,
                                r#type: "checkbox",
                                class: "sr-only peer",
                                checked: "{current_state.use_context}",
                                onchange: {
                                    let mut state = state.clone();
                                    move |e| {
                                        let mut new_state = state.read().clone();
                                        new_state.use_context = e.value().parse::<bool>().unwrap_or(false);
                                        state.set(new_state);
                                    }
                                },
                            }
                            div {
                                class: "w-9 h-5 bg-slate-700 rounded-full peer peer-checked:bg-blue-600 transition-colors"
                            }
                            div {
                                class: "absolute left-0.5 top-0.5 w-4 h-4 bg-white rounded-full transition-transform peer-checked:translate-x-4"
                            }
                        }

                        span {
                            class: "text-sm text-slate-400 group-hover:text-slate-300 transition-colors",
                            "Use Context (RAG)"
                        }
                    }

                    // Reset button
                    button {
                        class: if is_loading || is_answering {
                            "text-slate-600 cursor-not-allowed text-sm"
                        } else {
                            "text-slate-400 hover:text-red-400 transition-colors text-sm"
                        },
                        disabled: is_loading || is_answering,
                        onclick: {
                            let mut messages = messages.clone();
                            move |_| {
                                spawn(async move {
                                    if let Err(e) = reset_chat().await {
                                        println!("Error resetting chat: {:?}", e);
                                    }
                                    messages.set(Vec::new());
                                });
                            }
                        },
                        "Clear Chat"
                    }
                }

                // Input container
                div {
                    class: "relative flex items-end gap-3",

                    // Textarea
                    div {
                        class: "flex-1 relative",
                        textarea {
                            id: "message-input",
                            rows: "1",
                            class: "w-full px-4 py-3 bg-slate-800 border border-slate-700 rounded-2xl text-white placeholder-slate-500 resize-none focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500 transition-all min-h-[48px] max-h-[200px]",
                            placeholder: placeholder,
                            value: "{current_state.input_message}",
                            disabled: is_disabled,
                            oninput: {
                                let mut state = state.clone();
                                move |event| {
                                    let mut new_state = state.read().clone();
                                    new_state.input_message = event.value();
                                    state.set(new_state);
                                }
                            },
                            onkeydown: {
                                let state = state.clone();
                                let messages = messages.clone();
                                let session = current_session.clone();
                                let sessions = sessions.clone();
                                let settings = settings.clone();
                                move |event| {
                                    if event.key() == Key::Enter && !event.modifiers().shift() {
                                        event.prevent_default();
                                        let current = state.read().clone();
                                        if !current.input_message.trim().is_empty() {
                                            spawn(handle_message_send(state.clone(), messages.clone(), session.clone(), sessions.clone(), settings.clone()));
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Send button
                    button {
                        class: if is_answering {
                            "w-12 h-12 rounded-xl bg-red-600 hover:bg-red-700 flex items-center justify-center transition-all shadow-lg shadow-red-600/20"
                        } else if can_send {
                            "w-12 h-12 rounded-xl bg-blue-600 hover:bg-blue-700 flex items-center justify-center transition-all shadow-lg shadow-blue-600/20"
                        } else {
                            "w-12 h-12 rounded-xl bg-slate-700 flex items-center justify-center cursor-not-allowed"
                        },
                        disabled: !can_send && !is_answering,
                        onclick: {
                            let state = state.clone();
                            let messages = messages.clone();
                            let session = current_session.clone();
                            let sessions = sessions.clone();
                            let settings = settings.clone();
                            move |_| {
                                spawn(handle_message_send(state.clone(), messages.clone(), session.clone(), sessions.clone(), settings.clone()));
                            }
                        },

                        if is_answering {
                            // Stop icon
                            svg {
                                class: "w-5 h-5 text-white",
                                fill: "currentColor",
                                view_box: "0 0 24 24",
                                rect {
                                    x: "6",
                                    y: "6",
                                    width: "12",
                                    height: "12",
                                    rx: "2"
                                }
                            }
                        } else {
                            // Send icon
                            svg {
                                class: "w-5 h-5 text-white",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                view_box: "0 0 24 24",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    d: "M6 12L3.269 3.126A59.768 59.768 0 0121.485 12 59.77 59.77 0 013.27 20.876L5.999 12zm0 0h7.5"
                                }
                            }
                        }
                    }
                }

                // Hint text
                p {
                    class: "text-xs text-slate-500 mt-2 text-center",
                    "Press Enter to send, Shift+Enter for new line"
                }
            }
        }
    }
}

fn initialize_systems(state: Signal<ChatState>, model_ready: Signal<bool>) {
    initialize_language_model(state.clone(), model_ready.clone());
    initialize_database(state.clone());
    initialize_embedding_model();
}

fn initialize_language_model(mut state: Signal<ChatState>, mut model_ready: Signal<bool>) {
    spawn(async move {
        match init_llm_model().await {
            Ok(_) => {
                let mut current_state = state.read().clone();
                current_state.is_model_loading = false;
                state.set(current_state);
                model_ready.set(true);
            }
            Err(e) => {
                let mut current_state = state.read().clone();
                current_state.is_model_loading = false;
                state.set(current_state);
                println!("Error initializing model: {}", e);
            }
        }
    });
}

fn initialize_database(mut state: Signal<ChatState>) {
    spawn(async move {
        match init_db().await {
            Ok(_) => {
                let mut current_state = state.read().clone();
                current_state.is_database_loading = false;
                state.set(current_state);
            }
            Err(e) => {
                let mut current_state = state.read().clone();
                current_state.is_database_loading = false;
                state.set(current_state);
                println!("Error initializing database: {}", e);
            }
        }
    });
}

fn initialize_embedding_model() {
    spawn(async move {
        if let Err(e) = init_embedding_model().await {
            println!("Error initializing embeddings: {}", e);
        }
    });
}

async fn handle_message_send(
    mut state: Signal<ChatState>,
    mut messages: Signal<Vec<ChatMessage>>,
    mut current_session: Signal<Option<Session>>,
    mut sessions: Signal<Vec<Session>>,
    settings: Signal<AppSettings>,
) {
    let current_state = state.read().clone();
    let session = current_session();

    if current_state.is_model_answering {
        let mut new_state = current_state.clone();
        new_state.cancel_token = true;
        new_state.is_model_answering = false;
        state.set(new_state);
        return;
    }

    if current_state.is_model_loading || current_state.is_database_loading {
        return;
    }

    if current_state.input_message.trim().is_empty() {
        return;
    }

    // Auto-create session if none exists and add to sidebar history
    let session = match session {
        Some(s) => s,
        None => {
            // Generate session title from first message (truncate if too long)
            let first_msg = current_state.input_message.trim();
            let title = if first_msg.len() > 30 {
                format!("{}...", &first_msg[..27])
            } else {
                first_msg.to_string()
            };
            let new_session = Session::new(title);
            // Add to sessions list so it appears in sidebar
            sessions.write().insert(0, new_session.clone());
            // Set as current session
            current_session.set(Some(new_session.clone()));
            new_session
        }
    };

    let mut new_state = current_state.clone();
    new_state.cancel_token = false;
    new_state.is_model_answering = true;
    let user_message = current_state.input_message.trim().to_string();
    messages.write().push(ChatMessage::user(session.id, user_message.clone()));
    messages.write().push(ChatMessage::assistant(session.id, String::new()));
    new_state.input_message = String::new();
    state.set(new_state);

    // Get language instruction from settings
    let language_instruction = {
        let settings_guard = settings.read();
        settings_guard.language.prompt_instruction().to_string()
    };

    process_response(state.clone(), messages.clone(), user_message, language_instruction);
}

fn process_response(mut state: Signal<ChatState>, mut messages: Signal<Vec<ChatMessage>>, user_message: String, language_instruction: String) {
    spawn(async move {
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&"[WASM] process_response started".into());

        let use_context_enabled = state.read().use_context;

        // Build a more substantial prompt for short messages to ensure model responds
        // Qwen 2.5 1.5B sometimes returns empty for very short greetings
        let enhanced_message = if user_message.trim().len() < 10 {
            format!("User says: '{}'. Please respond appropriately.", user_message)
        } else {
            user_message.clone()
        };

        // Prepend language instruction to the message
        let mut final_message = format!("{} {}", language_instruction, enhanced_message);

        // Get relevant context when enabled
        if use_context_enabled {
            match search_context(user_message.clone()).await {
                Ok(context) => {
                    let context_string = format!("\n\n[Potentially useful context:\n{}]", context);
                    final_message.push_str(&context_string);
                },
                Err(e) => {
                    #[cfg(target_arch = "wasm32")]
                    web_sys::console::log_1(&format!("[WASM] Error searching context: {:?}", e).into());
                }
            }
        }

        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&format!("[WASM] Calling get_response with: {}", final_message).into());

        // Get and process response stream
        match get_response(final_message).await {
            Ok(mut stream) => {
                #[cfg(target_arch = "wasm32")]
                web_sys::console::log_1(&"[WASM] Got stream, starting to consume".into());

                let mut chunk_count = 0;
                while let Some(result) = stream.next().await {
                    chunk_count += 1;
                    match result {
                        Ok(chunk) => {
                            #[cfg(target_arch = "wasm32")]
                            web_sys::console::log_1(&format!("[WASM] Chunk {}: '{}'", chunk_count, chunk).into());

                            // Check if response was canceled
                            if state.read().cancel_token {
                                break;
                            }

                            // Clone, modify, set - same pattern as rusty_bot
                            let mut current_messages = messages.read().clone();
                            if let Some(last_message) = current_messages.last_mut() {
                                last_message.content.push_str(&chunk);
                                messages.set(current_messages);
                            }
                        },
                        Err(e) => {
                            #[cfg(target_arch = "wasm32")]
                            web_sys::console::log_1(&format!("[WASM] Error in chunk {}: {:?}", chunk_count, e).into());
                        }
                    }
                }

                #[cfg(target_arch = "wasm32")]
                web_sys::console::log_1(&format!("[WASM] Stream finished. Total chunks: {}", chunk_count).into());
            },
            Err(e) => {
                #[cfg(target_arch = "wasm32")]
                web_sys::console::log_1(&format!("[WASM] Error getting response: {:?}", e).into());
            }
        }

        // Finalize response state
        let mut current_state = state.read().clone();
        current_state.is_model_answering = false;
        state.set(current_state);

        // Refocus the input after response is complete
        #[cfg(target_arch = "wasm32")]
        focus_input();

        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&"[WASM] process_response finished".into());
    });
}

#[cfg(target_arch = "wasm32")]
fn scroll_to_bottom() {
    let window = web_sys::window().expect("no window");
    let document = window.document().expect("no document");
    if let Some(element) = document.get_element_by_id("chat-container") {
        let div = element.dyn_into::<web_sys::HtmlElement>().unwrap();
        div.set_scroll_top(div.scroll_height());
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn scroll_to_bottom() {}

#[cfg(target_arch = "wasm32")]
fn focus_input() {
    let window = web_sys::window().expect("no window");
    let document = window.document().expect("no document");
    if let Some(element) = document.get_element_by_id("message-input") {
        if let Ok(input) = element.dyn_into::<web_sys::HtmlElement>() {
            let _ = input.focus();
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn focus_input() {}
