#!/bin/bash

# Model Downloader for iDoris Local AI Assistant
# Downloads all required models from HuggingFace

set -e

echo "🚀 iDoris Model Downloader"
echo "=========================="
echo ""

# Create models directory
mkdir -p models
cd models

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
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

# Function to download a model
download_model() {
    local model_name=$1
    local model_url=$2
    local description=$3

    print_status "Downloading $description..."
    echo "  Model: $model_name"
    echo "  URL: $model_url"

    # Create directory for model
    mkdir -p "$model_name"

    # Download using git lfs or wget
    if command -v git &> /dev/null && command -v git-lfs &> /dev/null; then
        print_status "Using git lfs to download..."
        git lfs clone "$model_url" "$model_name" 2>/dev/null || {
            print_warning "git lfs clone failed, trying git clone..."
            git clone "$model_url" "$model_name" 2>/dev/null || {
                print_error "Failed to clone $model_name"
                return 1
            }
        }
    elif command -v wget &> /dev/null; then
        print_status "Using wget to download..."
        wget -r -np -nH --cut-dirs=1 -R "index.html*" "$model_url" -P "$model_name" || {
            print_error "Failed to download $model_name"
            return 1
        }
    elif command -v curl &> /dev/null; then
        print_status "Using curl to download..."
        curl -L "$model_url" -o "$model_name/model.bin" || {
            print_error "Failed to download $model_name"
            return 1
        }
    else
        print_error "Please install git-lfs, wget or curl"
        return 1
    fi

    print_success "$description downloaded successfully!"
    echo ""
}

# Check if git-lfs is installed
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
        print_warning "Please install git-lfs manually: https://git-lfs.github.com/"
    fi
fi

# List of models to download
print_status "Preparing to download models..."
echo ""

# 1. Qwen 2.5 models (LLM)
print_status "=== Language Models ==="

# Qwen 2.5 1.5B (default)
download_model "qwen-2.5-1.5b-instruct" \
    "https://huggingface.co/Qwen/Qwen2.5-1.5B-Instruct" \
    "Qwen 2.5 1.5B Instruct (Primary LLM)"

# Qwen 2.5 3B
download_model "qwen-2.5-3b-instruct" \
    "https://huggingface.co/Qwen/Qwen2.5-3B-Instruct" \
    "Qwen 2.5 3B Instruct (Medium LLM)"

# Qwen 2.5 7B
download_model "qwen-2.5-7b-instruct" \
    "https://huggingface.co/Qwen/Qwen2.5-7B-Instruct" \
    "Qwen 2.5 7B Instruct (Large LLM)"

# Llama 3.2 3B
download_model "llama-3.2-3b-instruct" \
    "https://huggingface.co/meta-llama/Llama-3.2-3B-Instruct" \
    "Llama 3.2 3B Instruct (Alternative LLM)"

# 2. Image Generation Models
print_status "=== Image Generation Models ==="

# FLUX.1-schnell (for MFLUX)
download_model "flux.1-schnell" \
    "https://huggingface.co/black-forest-labs/FLUX.1-schnell" \
    "FLUX.1 Schnell (Image Generation)"

# SDXL Turbo
download_model "sdxl-turbo" \
    "https://huggingface.co/stabilityai/sdxl-turbo" \
    "SDXL Turbo (Fast Image Generation)"

# 3. TTS Models
print_status "=== Text-to-Speech Models ==="

# Coqui TTS models will be downloaded automatically by Kalosm
print_warning "TTS models are downloaded automatically by Kalosm when first used"

# 4. Embedding Models
print_status "=== Embedding Models ==="

# BGE Small
download_model "bge-small-en-v1.5" \
    "https://huggingface.co/BAAI/bge-small-en-v1.5" \
    "BGE Small v1.5 (Text Embeddings)"

# BGE Base
download_model "bge-base-en-v1.5" \
    "https://huggingface.co/BAAI/bge-base-en-v1.5" \
    "BGE Base v1.5 (Text Embeddings)"

# 5. Multimodal Models (Future)
print_status "=== Multimodal Models (Optional) ==="

# LLaVA 1.5 7B
download_model "llava-1.5-7b" \
    "https://huggingface.co/liuhaotian/llava-v1.5-7b" \
    "LLaVA 1.5 7B (Vision-Language)"

print_status "=== Model Download Complete ==="
echo ""

# Calculate total size
total_size=$(du -sh . 2>/dev/null | cut -f1)
print_success "All models downloaded!"
print_status "Total size: $total_size"
echo ""

# Create model registry
cat > model_registry.txt << EOF
# iDoris Model Registry
# This file tracks downloaded models

[language_models]
qwen-2.5-1.5b-instruct = Qwen/Qwen2.5-1.5B-Instruct
qwen-2.5-3b-instruct = Qwen/Qwen2.5-3B-Instruct
qwen-2.5-7b-instruct = Qwen/Qwen2.5-7B-Instruct
llama-3.2-3b-instruct = meta-llama/Llama-3.2-3B-Instruct

[image_models]
flux.1-schnell = black-forest-labs/FLUX.1-schnell
sdxl-turbo = stabilityai/sdxl-turbo

[embedding_models]
bge-small-en-v1.5 = BAAI/bge-small-en-v1.5
bge-base-en-v1.5 = BAAI/bge-base-en-v1.5

[multimodal_models]
llava-1.5-7b = liuhaotian/llava-v1.5-7b
EOF

print_success "Model registry created: model_registry.txt"
echo ""

print_status "Model cache location: $(pwd)"
print_status "Models are ready for use with iDoris!"
echo ""

print_warning "Note: Some models may require acceptance of terms on HuggingFace"
print_warning "Visit the model pages to accept terms if downloads fail"
echo ""

print_status "Next steps:"
echo "1. Restart the iDoris application"
echo "2. The app should now load much faster"
echo "3. Check the Settings page to switch between models"
echo ""