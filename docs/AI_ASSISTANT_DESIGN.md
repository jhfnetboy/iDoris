# Local AI Assistant - 架构设计文档

## 1. 项目概述

| 属性 | 值 |
|------|-----|
| **项目名称** | local_ai_assistant |
| **目标** | 跨平台本地 AI 助手 |
| **平台支持** | Web / Desktop (macOS, Windows, Linux) / Mobile (iOS, Android) |
| **架构参考** | rusty_bot (Dioxus 0.7 + Kalosm) |

---

## 2. 核心特性

### 2.1 功能需求

| 功能 | 优先级 | 说明 |
|------|--------|------|
| 本地 LLM 推理 | P0 | 支持多种模型（Qwen、Llama、Mistral 等） |
| RAG 知识库 | P0 | 文档检索增强生成 |
| 流式响应 | P0 | 实时 token 输出 |
| 对话历史 | P1 | 持久化保存对话 |
| 多会话管理 | P1 | 支持多个独立对话 |
| 跨平台 UI | P0 | Web + Desktop + Mobile |
| 模型切换 | P2 | 运行时切换不同模型 |
| 插件系统 | P3 | 支持工具调用和扩展 |

### 2.2 非功能需求

| 需求 | 目标 |
|------|------|
| 首次响应延迟 | < 2s (模型加载后) |
| 内存占用 | < 8GB (7B 模型) |
| 离线运行 | 完全本地，无需网络 |
| 隐私保护 | 数据不出设备 |

---

## 3. 技术栈选型

### 3.1 核心依赖

| 组件 | 技术方案 | 版本 | 选型理由 |
|------|----------|------|----------|
| **UI 框架** | Dioxus | 0.7.2 | 全栈 Rust，跨平台一致性 |
| **LLM 推理** | llama.cpp (via llama-cpp-rs) | latest | 更广泛的模型支持，GGUF 格式 |
| **向量数据库** | LanceDB | 0.4+ | 纯 Rust，嵌入式，无服务器 |
| **Embedding** | fastembed-rs | latest | 轻量级，多模型支持 |
| **异步运行时** | Tokio | 1.45+ | 标准异步运行时 |
| **持久化** | SQLite (rusqlite) | 0.31+ | 轻量级关系数据库 |
| **配置管理** | config-rs | 0.14+ | 多格式配置支持 |

### 3.2 对比 rusty_bot 的改进

| 方面 | rusty_bot | local_ai_assistant | 改进原因 |
|------|-----------|---------------------|----------|
| LLM | Kalosm (git) | llama-cpp-rs | 稳定版本，更多模型支持 |
| 向量库 | SurrealDB | LanceDB | 纯 Rust，更简单集成 |
| Embedding | BERT | fastembed | 更轻量，多模型选择 |
| 持久化 | 无 | SQLite | 对话历史保存 |
| 配置 | 硬编码 | config-rs | 外部化配置 |

### 3.3 Feature Flags

```toml
[features]
default = []
web = ["dioxus/web"]
desktop = ["dioxus/desktop"]
mobile = ["dioxus/mobile"]
server = ["dioxus/server", "dep:llama-cpp-rs", "dep:lancedb", "dep:fastembed"]
cuda = ["llama-cpp-rs/cuda"]     # NVIDIA GPU 加速
metal = ["llama-cpp-rs/metal"]   # Apple Silicon 加速
```

---

## 4. 系统架构

### 4.1 整体架构

```
┌─────────────────────────────────────────────────────────────┐
│                     Dioxus UI Layer                         │
│         Web (WASM) │ Desktop (Native) │ Mobile              │
└─────────────────────────┬───────────────────────────────────┘
                          │ Server Functions
┌─────────────────────────▼───────────────────────────────────┐
│                    Application Layer                         │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────────────┐ │
│  │ Chat Service │ │ RAG Service  │ │ Session Management   │ │
│  └──────────────┘ └──────────────┘ └──────────────────────┘ │
└─────────────────────────┬───────────────────────────────────┘
                          │
┌─────────────────────────▼───────────────────────────────────┐
│                    Core Services                             │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────────────┐ │
│  │ LLM Engine   │ │ Embedding    │ │ Vector Store         │ │
│  │ (llama.cpp)  │ │ (fastembed)  │ │ (LanceDB)            │ │
│  └──────────────┘ └──────────────┘ └──────────────────────┘ │
└─────────────────────────┬───────────────────────────────────┘
                          │
┌─────────────────────────▼───────────────────────────────────┐
│                    Storage Layer                             │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────────────┐ │
│  │ SQLite       │ │ Model Files  │ │ Knowledge Base       │ │
│  │ (sessions)   │ │ (GGUF)       │ │ (documents)          │ │
│  └──────────────┘ └──────────────┘ └──────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

### 4.2 目录结构

```
local_ai_assistant/
├── Cargo.toml
├── Dioxus.toml
├── config/
│   ├── default.toml          # 默认配置
│   └── models.toml           # 模型配置
├── assets/
│   ├── tailwind.css
│   └── favicon.ico
├── src/
│   ├── main.rs               # 入口点
│   ├── lib.rs                # 库导出
│   │
│   ├── config/               # 配置管理
│   │   ├── mod.rs
│   │   └── settings.rs
│   │
│   ├── components/           # UI 组件
│   │   ├── mod.rs
│   │   ├── app.rs            # 主应用组件
│   │   ├── sidebar.rs        # 会话列表
│   │   ├── chat.rs           # 聊天界面
│   │   ├── message.rs        # 消息渲染
│   │   └── settings.rs       # 设置界面
│   │
│   ├── models/               # 数据模型
│   │   ├── mod.rs
│   │   ├── chat.rs           # 聊天消息
│   │   ├── session.rs        # 会话
│   │   └── document.rs       # 文档
│   │
│   ├── services/             # 业务服务
│   │   ├── mod.rs
│   │   ├── chat_service.rs   # 聊天服务
│   │   ├── rag_service.rs    # RAG 服务
│   │   └── session_service.rs # 会话管理
│   │
│   ├── core/                 # 核心引擎
│   │   ├── mod.rs
│   │   ├── llm.rs            # LLM 推理
│   │   ├── embedding.rs      # 文本嵌入
│   │   └── vector_store.rs   # 向量存储
│   │
│   ├── storage/              # 持久化
│   │   ├── mod.rs
│   │   ├── database.rs       # SQLite 操作
│   │   └── migrations.rs     # 数据库迁移
│   │
│   └── server_functions/     # Server Functions
│       ├── mod.rs
│       ├── chat.rs           # 聊天 API
│       ├── session.rs        # 会话 API
│       └── rag.rs            # RAG API
│
├── context/                  # 知识库文档
│   └── *.md
│
└── models/                   # 模型存储 (gitignore)
    └── *.gguf
```

---

## 5. 核心模块设计

### 5.1 LLM 引擎

```rust
// src/core/llm.rs

pub struct LlmEngine {
    model: llama_cpp::LlamaModel,
    config: LlmConfig,
}

pub struct LlmConfig {
    pub model_path: PathBuf,
    pub context_size: u32,      // 默认 4096
    pub temperature: f32,        // 默认 0.7
    pub top_p: f32,              // 默认 0.9
    pub max_tokens: u32,         // 默认 2048
    pub gpu_layers: u32,         // GPU 卸载层数
}

impl LlmEngine {
    pub async fn generate_stream(&self, prompt: &str)
        -> impl Stream<Item = String>;

    pub async fn generate(&self, prompt: &str) -> String;
}
```

### 5.2 RAG 服务

```rust
// src/services/rag_service.rs

pub struct RagService {
    embedding: EmbeddingEngine,
    vector_store: LanceDb,
}

impl RagService {
    /// 索引文档
    pub async fn index_documents(&self, dir: &Path) -> Result<usize>;

    /// 检索相关上下文
    pub async fn retrieve(&self, query: &str, top_k: usize)
        -> Result<Vec<Document>>;

    /// 增强提示词
    pub async fn augment_prompt(&self, query: &str) -> Result<String>;
}
```

### 5.3 会话管理

```rust
// src/services/session_service.rs

pub struct SessionService {
    db: SqlitePool,
}

impl SessionService {
    pub async fn create_session(&self) -> Result<Session>;
    pub async fn list_sessions(&self) -> Result<Vec<Session>>;
    pub async fn get_session(&self, id: Uuid) -> Result<Session>;
    pub async fn delete_session(&self, id: Uuid) -> Result<()>;
    pub async fn add_message(&self, session_id: Uuid, msg: ChatMessage) -> Result<()>;
    pub async fn get_messages(&self, session_id: Uuid) -> Result<Vec<ChatMessage>>;
}
```

### 5.4 Server Functions

```rust
// src/server_functions/chat.rs

#[server]
pub async fn init_models() -> Result<(), ServerFnError>;

#[get("/api/chat?session_id&prompt")]
pub async fn chat_stream(session_id: String, prompt: String)
    -> Result<TextStream>;

#[server]
pub async fn get_sessions() -> Result<Vec<Session>, ServerFnError>;

#[server]
pub async fn create_session() -> Result<Session, ServerFnError>;

#[server]
pub async fn delete_session(id: String) -> Result<(), ServerFnError>;
```

---

## 6. 数据模型

### 6.1 SQLite Schema

```sql
-- 会话表
CREATE TABLE sessions (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 消息表
CREATE TABLE messages (
    id TEXT PRIMARY KEY,
    session_id TEXT NOT NULL,
    role TEXT NOT NULL,  -- 'user' | 'assistant' | 'system'
    content TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE
);

-- 索引
CREATE INDEX idx_messages_session ON messages(session_id);
```

### 6.2 Rust 数据结构

```rust
// src/models/session.rs
#[derive(Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// src/models/chat.rs
#[derive(Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: Uuid,
    pub session_id: Uuid,
    pub role: ChatRole,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum ChatRole {
    User,
    Assistant,
    System,
}
```

---

## 7. 配置管理

### 7.1 默认配置 (config/default.toml)

```toml
[server]
host = "127.0.0.1"
port = 8080

[llm]
model_path = "./models/qwen2.5-7b-instruct-q4_k_m.gguf"
context_size = 4096
temperature = 0.7
top_p = 0.9
max_tokens = 2048
gpu_layers = 0  # 0 = CPU only

[embedding]
model_name = "BAAI/bge-small-en-v1.5"

[rag]
knowledge_dir = "./context"
chunk_size = 512
chunk_overlap = 50
top_k = 5

[database]
path = "./data/assistant.db"
```

---

## 8. 构建与部署

### 8.1 开发命令

```bash
# Web 开发
dx serve --platform web

# Desktop 开发 (macOS with Metal)
dx serve --platform desktop --features metal

# 生产构建
dx build --platform web --release
dx bundle --platform desktop --release
```

### 8.2 模型下载

```bash
# 推荐模型 (量化版本)
# Qwen 2.5 7B Instruct Q4_K_M (~4.4GB)
huggingface-cli download Qwen/Qwen2.5-7B-Instruct-GGUF qwen2.5-7b-instruct-q4_k_m.gguf

# 或使用更小的模型
# Qwen 2.5 3B Instruct Q4_K_M (~2.2GB)
huggingface-cli download Qwen/Qwen2.5-3B-Instruct-GGUF qwen2.5-3b-instruct-q4_k_m.gguf
```

---

## 9. 开发路线图

### Phase 1: MVP (1-2 weeks)

- [ ] 项目初始化
- [ ] LLM 推理集成 (llama-cpp-rs)
- [ ] 基础聊天 UI
- [ ] 流式响应

### Phase 2: 核心功能 (2-3 weeks)

- [ ] RAG 集成 (LanceDB + fastembed)
- [ ] 会话管理
- [ ] 对话历史持久化
- [ ] Desktop 支持

### Phase 3: 增强功能 (2-3 weeks)

- [ ] 模型切换
- [ ] GPU 加速 (CUDA/Metal)
- [ ] Mobile 支持
- [ ] 设置界面

### Phase 4: 优化与扩展 (ongoing)

- [ ] 性能优化
- [ ] 插件系统
- [ ] 多模态支持

---

## 10. 风险与缓解

| 风险 | 严重程度 | 缓解措施 |
|------|----------|----------|
| llama-cpp-rs 编译复杂 | 中 | 提供详细编译文档，使用预编译二进制 |
| 内存占用过高 | 高 | 支持量化模型，提供模型选择指南 |
| 跨平台兼容性 | 中 | 完善 CI/CD，多平台测试 |
| GPU 驱动问题 | 中 | 默认 CPU 模式，GPU 可选 |

---

## 11. 确认项

请确认以下设计决策：

1. **LLM 引擎**: llama-cpp-rs (vs Kalosm)
2. **向量数据库**: LanceDB (vs SurrealDB)
3. **Embedding**: fastembed-rs (vs Kalosm BERT)
4. **模型格式**: GGUF (量化模型)
5. **默认模型**: Qwen 2.5 7B Instruct Q4_K_M
6. **持久化**: SQLite (vs 其他)
7. **配置格式**: TOML

如确认无误，我将开始创建项目结构。

---

*设计文档版本: 1.0*
*创建时间: 2025-12-07*
