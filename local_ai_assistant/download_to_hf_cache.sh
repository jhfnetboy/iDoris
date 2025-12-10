#!/bin/bash

# Simple script to download models to HuggingFace cache directory
# Usage: ./download_to_hf_cache.sh

set -e

HF_CACHE_DIR="$HOME/.cache/huggingface/hub"

echo "📥 Downloading Models to HuggingFace Cache"
echo "=========================================="
echo ""

# Create cache directory if it doesn't exist
mkdir -p "$HF_CACHE_DIR"
cd "$HF_CACHE_DIR"

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

# Install huggingface-hub if needed
if ! command -v huggingface-cli &> /dev/null; then
    print_status "Installing huggingface-hub..."
    pip3 install -q huggingface_hub
fi

# Function to download a model to cache
download_model() {
    local repo_id=$1
    local name=$2
    local cache_name="models--$(echo $repo_id | tr '/' '--')"

    print_status "Downloading $name..."

    if [ -d "$cache_name" ]; then
        echo -e "${YELLOW}Already cached: $cache_name${NC}"
        return 0
    fi

    # Use huggingface-cli to download to cache
    huggingface-cli download "$repo_id" --resume-download
    print_success "✅ $name"
}

# Essential models for iDoris
print_status "Downloading essential models..."
echo ""

# Language Models
download_model "Qwen/Qwen2.5-1.5B-Instruct" "Qwen 2.5 1.5B (Default)"
download_model "Qwen/Qwen2.5-3B-Instruct" "Qwen 2.5 3B"
download_model "Qwen/Qwen2.5-7B-Instruct" "Qwen 2.5 7B"
download_model "meta-llama/Llama-3.2-3B-Instruct" "Llama 3.2 3B"

# Image Generation
download_model "black-forest-labs/FLUX.1-schnell" "FLUX.1 Schnell"

# Embeddings
download_model "BAAI/bge-small-en-v1.5" "BGE Small EN"
download_model "BAAI/bge-large-zh-v1.5" "BGE Large ZH (already have)"

echo ""
print_success "All models cached!"
echo ""

# Show total size
total_size=$(du -sh . 2>/dev/null | cut -f1)
echo "Total cache size: $total_size"
echo ""

# List cached models
echo "Cached models:"
ls -1 models--Qwen--* 2>/dev/null | sed 's/^/  /' | sed 's/models--/  /' | sed 's/--/ \//g'
ls -1 models--black-forest-labs--* 2>/dev/null | sed 's/^/  /' | sed 's/models--black-forest-labs--/  black-forest-labs\//'
ls -1 models--BAAI--* 2>/dev/null | sed 's/^/  /' | sed 's/models--BAAI--/  BAAI\//'
echo ""