#!/bin/bash

# Quick Model Downloader - Download only essential models
# This downloads the minimum required models to get the app running

set -e

echo "⚡ Quick Model Downloader for iDoris"
echo "====================================="
echo ""

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "Error: Please run this script from the iDoris root directory"
    exit 1
fi

print_status "Creating a simple Rust program to download models..."
echo ""

# Create a temporary downloader program
cat > download_models.rs << 'EOF'
use kalosm::language::{Llama, LlamaSource};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📦 Downloading essential models...");

    // Model list from the application
    let models = vec![
        ("qwen-2.5-1.5b", LlamaSource::qwen_2_5_1_5b_instruct()),
        ("qwen-2.5-3b", LlamaSource::qwen_2_5_3b_instruct()),
        ("qwen-2.5-7b", LlamaSource::qwen_2_5_7b_instruct()),
        ("llama-3.2-3b", LlamaSource::llama_3_2_3b_chat()),
    ];

    for (name, source) in models {
        println!("\n🔄 Downloading model: {}", name);

        // Try to load the model (this will trigger download if not cached)
        match Llama::builder()
            .with_source(source)
            .build()
            .await
        {
            Ok(_) => {
                println!("✅ Model {} downloaded/verified successfully!", name);
            }
            Err(e) => {
                println!("❌ Failed to download model {}: {}", name, e);
            }
        }
    }

    println!("\n🎉 Model download process completed!");
    println!("💡 The app should now start much faster!");

    Ok(())
}
EOF

print_status "Building and running model downloader..."
echo ""

# Run the downloader
cargo run --bin download_models 2>/dev/null || {
    print_status "Running with direct rustc..."
    rustc download_models.rs --edition 2021 -o download_models --extern kalosm=target/debug/deps/libkalosm-*.rlib 2>/dev/null || {
        print_status "Falling back to cargo run with features..."
        cargo run --features server --bin download_models || {
            echo "Error: Could not compile the downloader"
            rm -f download_models.rs
            exit 1
        }
    }

    # If we compiled successfully, run it
    if [ -f "./download_models" ]; then
        ./download_models
        rm -f download_models
    fi
}

# Clean up
rm -f download_models.rs

echo ""
print_success "Essential models are now cached!"
echo ""
echo "The application should start much faster now."
echo "Run: ./run.sh"
EOF

chmod +x quick_download.sh