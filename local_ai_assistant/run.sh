#!/bin/bash

# Local AI Assistant Launcher
# Usage: ./run.sh [build|run|dev]

set -e

APP_NAME="local_ai_assistant"
PORT=${PORT:-8080}
BUILD_DIR="target/dx/$APP_NAME/release/web"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Kill existing processes
kill_existing() {
    print_status "Killing existing Rust processes..."
    # Kill all Rust-related processes
    ps aux | grep -E "(dx serve|$APP_NAME|rustc)" | grep -v grep | awk '{print $2}' | xargs kill -9 2>/dev/null || true
    pkill -f "$APP_NAME" 2>/dev/null || true
    pkill -f "dx serve" 2>/dev/null || true

    print_status "Checking for process on port $PORT..."
    lsof -ti:$PORT | xargs kill -9 2>/dev/null || true
    sleep 1
}

# Clean build artifacts
clean_build() {
    print_status "Cleaning build artifacts..."
    rm -rf target/ 2>/dev/null || true
    print_status "Clean completed!"
}

# Build the project
build() {
    clean_build
    print_status "Building $APP_NAME in release mode..."
    dx build --platform web --release
    print_status "Build completed!"
}

# Run the server
run_server() {
    if [ ! -f "$BUILD_DIR/$APP_NAME" ]; then
        print_error "Binary not found. Run './run.sh build' first."
        exit 1
    fi

    kill_existing

    print_status "Starting $APP_NAME on port $PORT..."
    echo ""
    echo "======================================"
    echo "  Local AI Assistant"
    echo "  URL: http://127.0.0.1:$PORT"
    echo "  Press Ctrl+C to stop"
    echo "======================================"
    echo ""

    cd "$BUILD_DIR"
    PORT=$PORT ./$APP_NAME
}

# Development mode with dx serve
dev_server() {
    kill_existing

    print_status "Starting development server..."
    dx serve --platform web --release
}

# Show usage
usage() {
    echo "Local AI Assistant Launcher"
    echo ""
    echo "Usage: ./run.sh [command]"
    echo ""
    echo "Commands:"
    echo "  build    Clean and build the project in release mode"
    echo "  run      Run the built server (default)"
    echo "  dev      Start development server with hot reload"
    echo "  clean    Clean build artifacts"
    echo "  kill     Kill all running instances"
    echo "  help     Show this help message"
    echo ""
    echo "Environment variables:"
    echo "  PORT     Server port (default: 8080)"
    echo ""
    echo "Examples:"
    echo "  ./run.sh build        # Build the project"
    echo "  ./run.sh run          # Run on default port 8080"
    echo "  PORT=8282 ./run.sh    # Run on port 8282"
}

# Main
case "${1:-run}" in
    build)
        build
        ;;
    run)
        run_server
        ;;
    dev)
        dev_server
        ;;
    clean)
        clean_build
        ;;
    kill)
        kill_existing
        print_status "All $APP_NAME processes killed."
        ;;
    help|--help|-h)
        usage
        ;;
    *)
        print_error "Unknown command: $1"
        usage
        exit 1
        ;;
esac
