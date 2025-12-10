#!/bin/bash

# Download Models to Specific Directory
# Downloads all HuggingFace models to /Volumes/UltraDisk/Dev2/crypto-projects/iDoris/local_ai_assistant/models

set -e

# Target directory
MODELS_DIR="/Volumes/UltraDisk/Dev2/crypto-projects/iDoris/local_ai_assistant/models"

echo "🚀 HuggingFace Model Downloader"
echo "================================"
echo "📍 Target Directory: $MODELS_DIR"
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

# Create models directory
mkdir -p "$MODELS_DIR"
cd "$MODELS_DIR"

print_status "Models will be downloaded to: $(pwd)"
echo ""

# Check for required tools
if ! command -v git &> /dev/null; then
    print_error "git is required but not installed"
    exit 1
fi

if ! command -v git-lfs &> /dev/null; then
    print_warning "git-lfs not found. Installing..."
    if command -v brew &> /dev/null; then
        brew install git-lfs
        git lfs install
    elif command -v apt &> /dev/null; then
        sudo apt-get update
        sudo apt-get install -y git-lfs
        git lfs install
    else
        print_error "Please install git-lfs manually: https://git-lfs.github.com/"
        exit 1
    fi
fi

# Function to download model using huggingface-hub or git
download_model() {
    local model_dir=$1
    local repo_id=$2
    local description=$3

    print_status "Downloading $description..."
    echo "  📁 Directory: $model_dir"
    echo "  🔗 Repo: $repo_id"

    if [ -d "$model_dir" ]; then
        print_warning "Directory $model_dir already exists. Skipping..."
        return 0
    fi

    # Try using huggingface-cli if available
    if command -v huggingface-cli &> /dev/null; then
        print_status "Using huggingface-cli..."
        huggingface-cli download "$repo_id" --local-dir "$model_dir" --local-dir-use-symlinks False || {
            print_warning "huggingface-cli failed, trying git clone..."
            git clone "https://huggingface.co/$repo_id" "$model_dir" || {
                print_error "Failed to download $repo_id"
                return 1
            }
        }
    else
        # Use git clone
        print_status "Using git clone..."
        git clone "https://huggingface.co/$repo_id" "$model_dir" || {
            print_error "Failed to download $repo_id"
            return 1
        }
    fi

    print_success "✅ $description downloaded!"
    echo ""
}

# Install huggingface-hub if not available
if ! command -v huggingface-cli &> /dev/null; then
    print_status "Installing huggingface-hub..."
    pip3 install -q huggingface_hub[cli] || {
        print_warning "Could not install huggingface-hub, will use git instead"
    }
fi

echo "📥 Starting Model Downloads..."
echo "=============================="
echo ""

# 1. Language Models (LLMs)
print_status "=== LANGUAGE MODELS ==="

# Qwen 2.5 1.5B Instruct (Primary Model)
download_model "qwen2.5-1.5b-instruct" \
    "Qwen/Qwen2.5-1.5B-Instruct" \
    "Qwen 2.5 1.5B Instruct (Main LLM)"

# Qwen 2.5 3B Instruct
download_model "qwen2.5-3b-instruct" \
    "Qwen/Qwen2.5-3B-Instruct" \
    "Qwen 2.5 3B Instruct (Medium LLM)"

# Qwen 2.5 7B Instruct
download_model "qwen2.5-7b-instruct" \
    "Qwen/Qwen2.5-7B-Instruct" \
    "Qwen 2.5 7B Instruct (Large LLM)"

# Llama 3.2 3B Instruct
download_model "llama3.2-3b-instruct" \
    "meta-llama/Llama-3.2-3B-Instruct" \
    "Llama 3.2 3B Instruct (Alternative LLM)"

# 2. Image Generation Models
print_status "=== IMAGE GENERATION MODELS ==="

# FLUX.1 Schnell
download_model "flux.1-schnell" \
    "black-forest-labs/FLUX.1-schnell" \
    "FLUX.1 Schnell (Fast Image Generation)"

# FLUX.1 Dev (if needed)
download_model "flux.1-dev" \
    "black-forest-labs/FLUX.1-dev" \
    "FLUX.1 Dev (High Quality Image Generation)"

# Stable Diffusion XL Turbo
download_model "sdxl-turbo" \
    "stabilityai/sdxl-turbo" \
    "SDXL Turbo (Fast Image Generation)"

# 3. Embedding Models
print_status "=== EMBEDDING MODELS ==="

# BGE Small English v1.5
download_model "bge-small-en-v1.5" \
    "BAAI/bge-small-en-v1.5" \
    "BGE Small v1.5 (Text Embeddings)"

# BGE Base English v1.5
download_model "bge-base-en-v1.5" \
    "BAAI/bge-base-en-v1.5" \
    "BGE Base v1.5 (Text Embeddings)"

# BGE Large English v1.5
download_model "bge-large-en-v1.5" \
    "BAAI/bge-large-en-v1.5" \
    "BGE Large v1.5 (Text Embeddings)"

# 4. Multimodal Models
print_status "=== MULTIMODAL MODELS ==="

# LLaVA 1.5 7B
download_model "llava-v1.5-7b" \
    "liuhaotian/llava-v1.5-7b" \
    "LLaVA 1.5 7B (Vision-Language)"

# 5. Audio Models (TTS)
print_status "=== AUDIO MODELS ==="

# Coqui TTS models (these will be downloaded by Kalosm automatically)
print_warning "TTS models are managed by Kalosm and will download on first use"

# Create model index
cat > "$MODELS_DIR/model_index.md" << EOF
# iDoris Model Index

This directory contains pre-downloaded models for iDoris Local AI Assistant.

## Directory Structure

### Language Models (LLMs)
- \`qwen2.5-1.5b-instruct\` - Qwen 2.5 1.5B Instruct (Default Model)
- \`qwen2.5-3b-instruct\` - Qwen 2.5 3B Instruct
- \`qwen2.5-7b-instruct\` - Qwen 2.5 7B Instruct
- \`llama3.2-3b-instruct\` - Llama 3.2 3B Instruct

### Image Generation Models
- \`flux.1-schnell\` - FLUX.1 Schnell (Fast Image Generation)
- \`flux.1-dev\` - FLUX.1 Dev (High Quality)
- \`sdxl-turbo\` - SDXL Turbo

### Embedding Models
- \`bge-small-en-v1.5\` - BGE Small v1.5
- \`bge-base-en-v1.5\` - BGE Base v1.5
- \`bge-large-en-v1.5\` - BGE Large v1.5

### Multimodal Models
- \`llava-v1.5-7b\` - LLaVA 1.5 7B (Vision-Language)

## Usage

The application will automatically use these pre-downloaded models, resulting in faster startup times.

## Total Disk Space Used

\`$(du -sh . 2>/dev/null | cut -f1)\`

## Last Updated

$(date)
EOF

# Summary
echo ""
echo "============================"
print_success "✨ Download Complete! ✨"
echo "============================"
echo ""

total_size=$(du -sh . 2>/dev/null | cut -f1)
model_count=$(find . -maxdepth 1 -type d | wc -l)
print_status "📁 Download Location: $MODELS_DIR"
print_status "📊 Total Size: $total_size"
print_status "📦 Models Downloaded: $((model_count - 1))"
echo ""

print_status "🚀 The iDoris application should now start much faster!"
print_status "   All models are pre-cached and ready to use."
echo ""

print_warning "💡 Note: Some models may require:"
print_warning "   - Acceptance of terms on HuggingFace website"
print_warning "   - Additional dependencies for certain models"
echo ""

# Create a symlink for Kalosm to find models (optional)
print_status "Creating symlink for Kalosm compatibility..."
ln -sf "$MODELS_DIR" "$HOME/.cache/kalosm/models" 2>/dev/null || {
    print_warning "Could not create symlink. Models will still be used by the app."
}

echo ""
print_success "Setup complete! Run ./run.sh to start the application."
EOF

chmod +x download_to_models.sh