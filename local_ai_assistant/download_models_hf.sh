#!/bin/bash

# Download Models directly to HuggingFace Cache
# Downloads models to ~/.cache/huggingface/hub so Kalosm can use them immediately

set -e

# HuggingFace cache directory
HF_CACHE_DIR="$HOME/.cache/huggingface/hub"

echo "🚀 HuggingFace Cache Model Downloader"
echo "====================================="
echo "📍 Target Directory: $HF_CACHE_DIR"
echo ""

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Create cache directory
mkdir -p "$HF_CACHE_DIR"
cd "$HF_CACHE_DIR"

print_status "Models will be cached in: $(pwd)"
echo ""

# Check for required tools
if ! command -v huggingface-cli &> /dev/null && ! command -v hf &> /dev/null; then
    print_status "Installing huggingface-hub..."
    pip3 install -q huggingface_hub[cli] || {
        print_error "Failed to install huggingface-hub"
        exit 1
    }
fi

# Function to download model to cache directory
download_to_cache() {
    local repo_id=$1
    local description=$2
    local cache_dir_name="models--$(echo $repo_id | tr '/' '--')"

    print_status "Downloading $description..."
    echo "  🔗 Repo: $repo_id"
    echo "  📁 Cache: $cache_dir_name"

    if [ -d "$cache_dir_name" ]; then
        print_warning "Model already cached at $cache_dir_name"
        return 0
    fi

    # Use hf cli (new) or huggingface-cli (old)
    if command -v hf &> /dev/null; then
        print_status "Using hf cli..."
        hf download "$repo_id" --include="*.safetensors,*.bin,*.json,*.txt,*.md" || {
            print_error "Failed to download $repo_id"
            return 1
        }
    else
        print_status "Using huggingface-cli..."
        huggingface-cli download "$repo_id" --include="*.safetensors,*.bin,*.json,*.txt,*.md" || {
            print_error "Failed to download $repo_id"
            return 1
        }
    fi

    print_success "✅ $description cached!"
    echo ""
}

# List of models to cache
print_status "📥 Caching Essential Models..."
echo "==============================="
echo ""

# 1. Language Models (LLMs)
print_status "=== LANGUAGE MODELS ==="

# Qwen 2.5 1.5B Instruct (Default)
download_to_cache "Qwen/Qwen2.5-1.5B-Instruct" \
    "Qwen 2.5 1.5B Instruct (Default LLM)"

# Qwen 2.5 3B Instruct
download_to_cache "Qwen/Qwen2.5-3B-Instruct" \
    "Qwen 2.5 3B Instruct"

# Qwen 2.5 7B Instruct
download_to_cache "Qwen/Qwen2.5-7B-Instruct" \
    "Qwen 2.5 7B Instruct"

# Llama 3.2 3B Instruct
download_to_cache "meta-llama/Llama-3.2-3B-Instruct" \
    "Llama 3.2 3B Instruct"

# 2. Image Generation Models
print_status "=== IMAGE GENERATION MODELS ==="

# FLUX.1 Schnell
download_to_cache "black-forest-labs/FLUX.1-schnell" \
    "FLUX.1 Schnell"

# FLUX.1 Dev
download_to_cache "black-forest-labs/FLUX.1-dev" \
    "FLUX.1 Dev"

# SDXL Turbo
download_to_cache "stabilityai/sdxl-turbo" \
    "SDXL Turbo"

# 3. Embedding Models
print_status "=== EMBEDDING MODELS ==="

# BGE models for RAG
download_to_cache "BAAI/bge-small-en-v1.5" \
    "BGE Small English v1.5"

download_to_cache "BAAI/bge-base-en-v1.5" \
    "BGE Base English v1.5"

download_to_cache "BAAI/bge-large-en-v1.5" \
    "BGE Large English v1.5"

# Chinese BGE (since you have it)
download_to_cache "BAAI/bge-large-zh-v1.5" \
    "BGE Large Chinese v1.5"

# 4. TTS/ASR Models (if needed)
print_status "=== AUDIO MODELS ==="

# Whisper models
download_to_cache "Systran/faster-whisper-large-v3" \
    "Whisper Large v3"

download_to_cache "Systran/faster-whisper-medium" \
    "Whisper Medium"

# 5. Sentence Transformers (for semantic search)
print_status "=== SEMANTIC SEARCH MODELS ==="

download_to_cache "sentence-transformers/all-MiniLM-L6-v2" \
    "MiniLM L6 v2"

# Create cache index
cat > "$HF_CACHE_DIR/../cached_models.txt" << EOF
# iDoris Cached Models Index
# Generated on $(date)

## Language Models
- models--Qwen--Qwen2.5-1.5B-Instruct
- models--Qwen--Qwen2.5-3B-Instruct
- models--Qwen--Qwen2.5-7B-Instruct
- models--meta-llama--Llama-3.2-3B-Instruct

## Image Generation
- models--black-forest-labs--FLUX.1-schnell
- models--black-forest-labs--FLUX.1-dev
- models--stabilityai--sdxl-turbo

## Embeddings
- models--BAAI--bge-small-en-v1.5
- models--BAAI--bge-base-en-v1.5
- models--BAAI--bge-large-en-v1.5
- models--BAAI--bge-large-zh-v1.5

## Audio/Speech
- models--Systran--faster-whisper-large-v3
- models--Systran--faster-whisper-medium

## Semantic Search
- models--sentence-transformers--all-MiniLM-L6-v2

Total cache size: $(du -sh . 2>/dev/null | cut -f1)
EOF

# Summary
echo ""
echo "============================"
print_success "✨ Models Cached! ✨"
echo "============================"
echo ""

total_size=$(du -sh . 2>/dev/null | cut -f1)
model_count=$(find . -maxdepth 1 -type d -name "models--*" | wc -l)

print_status "📁 Cache Location: $HF_CACHE_DIR"
print_status "📊 Total Cache Size: $total_size"
print_status "📦 Models Cached: $model_count"
echo ""

print_success "🚀 All models are now cached and ready for instant use!"
echo ""
print_status "The iDoris app will now start much faster since models are pre-cached."
print_status "Run ./run.sh to start the application."
echo ""

# Show current cache contents
echo "Current cache contents:"
ls -1 models--* 2>/dev/null | sed 's/^/  - /' || echo "  (No models cached yet)"
echo ""