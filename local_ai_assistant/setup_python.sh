#!/bin/bash

# Setup Python dependencies for Local AI Assistant (TTS)
set -e

# Detect python command
if command -v python3 &> /dev/null; then
    PYTHON_CMD="python3"
    PIP_CMD="pip3"
elif command -v python &> /dev/null; then
    PYTHON_CMD="python"
    PIP_CMD="pip"
else
    echo "Error: Python not found. Please install Python 3.10 or later."
    exit 1
fi

echo "Using $PYTHON_CMD ($($PYTHON_CMD --version))"

# Check if pip is available
if ! command -v $PIP_CMD &> /dev/null; then
    echo "Error: pip not found. Please install pip."
    exit 1
fi

echo "Installing dependencies..."
# Install core dependencies for TTS (VibeVoice)
$PIP_CMD install --upgrade pip
$PIP_CMD install torch torchaudio numpy huggingface_hub scipy librosa --verbose

echo "Python dependencies installed successfully!"
