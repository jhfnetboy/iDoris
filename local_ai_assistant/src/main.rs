//! Local AI Assistant - Main Entry Point
//!
//! A cross-platform local AI assistant with RAG support.
//! Based on rusty_bot architecture with session management.

use dioxus::prelude::*;

mod components;
mod models;

#[cfg(feature = "server")]
mod core;

#[cfg(feature = "server")]
mod storage;

mod server_functions;

/// Static resources used by the application
/// Favicon that will appear in the browser tab
const FAVICON: Asset = asset!("/assets/favicon.ico");

/// Main function that launches the Dioxus application
fn main() {
    #[cfg(feature = "server")]
    {
        println!("Server starting...");
        // Load .env file if it exists
        if let Err(e) = dotenv::dotenv() {
            println!("Note: .env file not found or could not be loaded: {}", e);
        } else {
            println!(".env loaded");
        }
    }
    dioxus::launch(App);
}

/// Root component of the application.
///
/// This component defines the basic structure of the HTML document,
/// including:
/// - Links to resources such as favicon and CSS styles
/// - Page body with dark background
/// - The main App component that handles the interface
#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Title { "iDoris | Your Local AI Assistant" }
        // Use Tailwind CDN for complete class support
        script { src: "https://cdn.tailwindcss.com" }
        // Also set title via script for better compatibility
        script {
            "document.title = 'iDoris | Your Local AI Assistant';"
        }
        body {
            class: "bg-slate-900 text-white",
            components::App {}
        }
    }
}
