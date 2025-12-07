//! Local AI Assistant - Main Entry Point
//!
//! A cross-platform local AI assistant with RAG support.

use dioxus::prelude::*;

mod components;
mod models;

#[cfg(feature = "server")]
mod core;
#[cfg(feature = "server")]
mod services;
#[cfg(feature = "server")]
mod storage;
#[cfg(feature = "server")]
mod config;

mod server_functions;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        components::App {}
    }
}
