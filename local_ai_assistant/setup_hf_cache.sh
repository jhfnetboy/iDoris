#!/bin/bash

# Setup HuggingFace Cache on External Disk
# Moves cache to /Volumes/UltraDisk/Dev2/.cache/huggingface
# And sets environment variables

set -e

# External disk location
EXTERNAL_DISK="/Volumes/UltraDisk"
NEW_CACHE_DIR="$EXTERNAL_DISK/Dev2/.cache/huggingface"
OLD_CACHE_DIR="$HOME/.cache/huggingface"

echo "🔄 HuggingFace Cache Setup"
echo "=========================="
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

# Check if external disk is mounted
if [ ! -d "$EXTERNAL_DISK" ]; then
    print_error "External disk not mounted at $EXTERNAL_DISK"
    exit 1
fi

# Create new cache directory
print_status "Creating new cache directory..."
mkdir -p "$NEW_CACHE_DIR"
print_success "Created: $NEW_CACHE_DIR"

# Move existing cache if it exists
if [ -d "$OLD_CACHE_DIR" ]; then
    old_size=$(du -sh "$OLD_CACHE_DIR" 2>/dev/null | cut -f1)
    print_status "Moving existing cache ($old_size)..."

    # Use rsync to preserve permissions and resume if interrupted
    if [ -d "$OLD_CACHE_DIR/hub" ]; then
        print_status "Syncing models to new location..."
        rsync -avh --progress "$OLD_CACHE_DIR/" "$NEW_CACHE_DIR/" --exclude="*.lock"
        print_success "Cache moved successfully!"
    fi
else
    print_warning "No existing cache found at $OLD_CACHE_DIR"
fi

# Create environment file
print_status "Creating environment configuration..."

# For current session
cat > export_hf_cache.sh << 'EOF'
#!/bin/bash
# HuggingFace Cache Environment Variables
export HF_HOME="/Volumes/UltraDisk/Dev2/.cache/huggingface"
export HUGGINGFACE_HUB_CACHE="/Volumes/UltraDisk/Dev2/.cache/huggingface/hub"
export TRANSFORMERS_CACHE="/Volumes/UltraDisk/Dev2/.cache/huggingface/hub"
export HF_DATASETS_CACHE="/Volumes/UltraDisk/Dev2/.cache/huggingface/datasets"

# Rust kalosm typically uses ~/.cache, create symlink if needed
if [ ! -L "$HOME/.cache/huggingface" ]; then
    rm -rf "$HOME/.cache/huggingface" 2>/dev/null || true
    ln -sf "/Volumes/UltraDisk/Dev2/.cache/huggingface" "$HOME/.cache/huggingface"
fi

echo "✅ HuggingFace cache set to: /Volumes/UltraDisk/Dev2/.cache/huggingface"
EOF

chmod +x export_hf_cache.sh

# Add to shell profile
SHELL_PROFILE=""
if [ -n "$ZSH_VERSION" ] || [ "$SHELL" = "/bin/zsh" ]; then
    SHELL_PROFILE="$HOME/.zshrc"
elif [ -n "$BASH_VERSION" ] || [ "$SHELL" = "/bin/bash" ]; then
    SHELL_PROFILE="$HOME/.bash_profile"
fi

if [ -n "$SHELL_PROFILE" ]; then
    if ! grep -q "HF_HOME=/Volumes/UltraDisk/Dev2/.cache/huggingface" "$SHELL_PROFILE" 2>/dev/null; then
        print_status "Adding to $SHELL_PROFILE..."
        cat >> "$SHELL_PROFILE" << 'EOF'

# HuggingFace Cache on External Disk
export HF_HOME="/Volumes/UltraDisk/Dev2/.cache/huggingface"
export HUGGINGFACE_HUB_CACHE="/Volumes/UltraDisk/Dev2/.cache/huggingface/hub"
export TRANSFORMERS_CACHE="/Volumes/UltraDisk/Dev2/.cache/huggingface/hub"
export HF_DATASETS_CACHE="/Volumes/UltraDisk/Dev2/.cache/huggingface/datasets"
EOF
        print_success "Added to $SHELL_PROFILE"
    else
        print_warning "Already configured in $SHELL_PROFILE"
    fi
fi

# Apply to current session
export HF_HOME="/Volumes/UltraDisk/Dev2/.cache/huggingface"
export HUGGINGFACE_HUB_CACHE="/Volumes/UltraDisk/Dev2/.cache/huggingface/hub"
export TRANSFORMERS_CACHE="/Volumes/UltraDisk/Dev2/.cache/huggingface/hub"
export HF_DATASETS_CACHE="/Volumes/UltraDisk/Dev2/.cache/huggingface/datasets"

# Create symlink for compatibility
print_status "Creating symlink for compatibility..."
rm -rf "$HOME/.cache/huggingface" 2>/dev/null || true
ln -sf "/Volumes/UltraDisk/Dev2/.cache/huggingface" "$HOME/.cache/huggingface"

# Create cache index
cat > "$NEW_CACHE_DIR/cache_info.txt" << EOF
HuggingFace Cache Location: /Volumes/UltraDisk/Dev2/.cache/huggingface
Created: $(date)
Original Location: $OLD_CACHE_DIR
Moved: Yes

Cache Structure:
- /hub - Model files
- /datasets - Dataset files (will be created as needed)
- /accelerate - Accelerate cache (will be created as needed)

Total Size: $(du -sh . 2>/dev/null | cut -f1)
EOF

print_success "Cache setup complete!"
echo ""

print_status "New cache location: $NEW_CACHE_DIR"
new_size=$(du -sh "$NEW_CACHE_DIR" 2>/dev/null | cut -f1)
print_status "Cache size: $new_size"
echo ""

print_success "To apply changes to your shell:"
echo "  source ./export_hf_cache.sh"
echo "  # or restart your terminal"
echo ""

print_status "HuggingFace configuration:"
echo "  HF_HOME=$HF_HOME"
echo "  HUGGINGFACE_HUB_CACHE=$HUGGINGFACE_HUB_CACHE"
echo "  TRANSFORMERS_CACHE=$TRANSFORMERS_CACHE"
echo ""