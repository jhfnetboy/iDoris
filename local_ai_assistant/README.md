# iDoris

```
    ██╗██████╗  ██████╗ ██████╗ ██╗███████╗
    ██║██╔══██╗██╔═══██╗██╔══██╗██║██╔════╝
    ██║██║  ██║██║   ██║██████╔╝██║███████╗
    ██║██║  ██║██║   ██║██╔══██╗██║╚════██║
    ██║██████╔╝╚██████╔╝██║  ██║██║███████║
    ╚═╝╚═════╝  ╚═════╝ ╚═╝  ╚═╝╚═╝╚══════╝
         Your Private AI Assistant
```

> **100% Local. 100% Private. Zero Cloud.**

iDoris is a fully local AI assistant that runs entirely on your personal computer. No data leaves your machine - ever.

## Features

### Phase 1 (Current)
- **Local LLM Chat** - Powered by Qwen 2.5 7B, runs on your hardware
- **RAG Knowledge Base** - Add your own documents for context-aware responses
- **Session Persistence** - Chat history saved locally in SQLite
- **Multi-language Support** - English, Chinese, Spanish, French, German
- **Beautiful Web UI** - Modern dark theme with Dioxus + Tailwind

### Phase 2 (Planned)
- **Advanced Search** - Hybrid semantic + keyword search
- **Content Generation** - Articles, summaries, and more
- **Image Generation** - Local Stable Diffusion integration
- **Video Understanding** - Process and analyze local videos

### Future Vision
- **Voice Interface** - Local speech-to-text and text-to-speech
- **Plugin System** - Extend with custom capabilities
- **Multi-model Support** - Switch between different local models

## Tech Stack

| Component | Technology |
|-----------|------------|
| Frontend | Dioxus 0.7 + Tailwind CSS |
| LLM | Kalosm + Qwen 2.5 7B |
| Embeddings | BERT (768D vectors) |
| Vector Store | SurrealDB |
| Database | SQLite |
| Language | Rust |

## Quick Start

### Prerequisites
- Rust 1.75+
- ~10GB disk space (for model download)
- 16GB+ RAM recommended

### Installation

```bash
# Clone the repository
git clone https://github.com/jhfnetboy/AI-test.git
cd AI-test/local_ai_assistant

# Build and run
cargo install dioxus-cli
dx serve --platform web --release
```

Open http://127.0.0.1:8080 in your browser.

**First run**: The model (~10GB) will download automatically. Check terminal for progress.

## Usage

### Basic Chat
Just type and press Enter. The AI responds using the local Qwen model.

### RAG (Knowledge Base)
1. Click the **Settings** icon (gear)
2. Add documents to the **Context Manager**
3. Enable **"Use Context (RAG)"** toggle
4. Ask questions about your documents

### Supported Document Formats
- Markdown (.md)
- Text files (.txt)
- More formats coming in Phase 2

## Architecture

```
iDoris/
├── src/
│   ├── components/     # UI components (Dioxus)
│   ├── core/           # LLM, embedding, vector store
│   ├── models/         # Data structures
│   ├── server_functions/  # API endpoints
│   └── storage/        # SQLite persistence
├── context/            # RAG documents folder
├── data/               # SQLite database
└── docs/               # Documentation
```

## Privacy

- **No cloud services** - Everything runs locally
- **No telemetry** - Zero data collection
- **No API keys** - Models are downloaded and run locally
- **Your data stays yours** - Documents never leave your machine

## Performance

| Hardware | Response Time |
|----------|---------------|
| M1 Mac (16GB) | ~2-3s first token |
| Intel i7 + 32GB | ~3-5s first token |
| GPU (CUDA) | Coming soon |

## Contributing

Contributions welcome! See [docs/RAG_SOLUTIONS.md](docs/RAG_SOLUTIONS.md) for RAG enhancement roadmap.

## License

MIT License - See LICENSE file for details.

---

<p align="center">
  <b>iDoris</b> - Intelligence at your fingertips, privacy in your hands.
</p>
