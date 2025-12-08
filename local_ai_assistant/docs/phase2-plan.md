# iDoris Phase 2: Content Creator Engine

## 目标

将 iDoris 打造成本地内容创作后端引擎，支持文章、图片、视频内容生成。

---

## 硬件配置要求

| 配置 | RAM | 适用场景 |
|------|-----|---------|
| **基础版** | 16GB | 小模型、基础图片生成 |
| **标准版** | 32GB | 中等模型、高质量图片 |
| **专业版** | 64GB+ | 大模型、视频处理 |

---

## 模型选型

### 1. 文本生成 (LLM)

#### 16GB 配置
| 模型 | 量化 | VRAM | 用途 |
|------|------|------|------|
| **Qwen 2.5 7B** (当前) | Q4_K_M | 10-12GB | 通用对话、文章 |
| Llama 3.2 3B | Q4 | 8-10GB | 快速响应 |
| Phi-3 3.8B | Q4 | 8-10GB | 轻量推理 |

#### 32GB+ 配置
| 模型 | 量化 | VRAM | 用途 |
|------|------|------|------|
| **Qwen 2.5 14B** | Q4_K_M | 20-24GB | 高质量文章 |
| Llama 3.1 8B | Q6_K | 20-24GB | 平衡质量 |
| Mixtral 8x7B | Q4 | 24-28GB | MoE 多任务 |

### 2. 图片生成

| 模型 | VRAM | 分辨率 | 用途 |
|------|------|--------|------|
| **SD 3.5 Turbo** | 8-12GB | 1024x1024 | 快速生成 |
| SDXL | 16-24GB | 1024x1024 | 高质量 |
| FLUX.1 schnell | 12-16GB | 可变 | 平衡 |
| FLUX.1 dev | 24-32GB | 可变 | 最高质量 |
| **z-image** | 可变 | 可变 | 备选方案 |

### 3. 视频理解 (Phase 2.5)

| 方案 | 实现 | 用途 |
|------|------|------|
| 帧提取 + LLaVA | ffmpeg + Vision LLM | 视频摘要 |
| Whisper | 音频转文字 | 字幕生成 |

---

## 技术架构

### Rust 集成方案

```
优先级:
1. Kalosm (当前) - 保持兼容
2. Candle - HuggingFace 官方 Rust ML
3. llama-cpp-rs - GGUF 模型支持
4. MLX (FFI) - Apple Silicon 优化
```

### 模块设计

```
src/core/
├── llm.rs           # 现有 - 文本生成
├── embedding.rs     # 现有 - 向量嵌入
├── image_gen.rs     # 新增 - 图片生成
├── video_proc.rs    # 新增 - 视频处理
└── model_manager.rs # 新增 - 多模型管理
```

### 模型管理器设计

```rust
// 支持动态加载/卸载模型以节省内存
pub struct ModelManager {
    active_llm: Option<LlmModel>,
    active_image: Option<ImageModel>,
    config: ModelConfig,
}

impl ModelManager {
    pub async fn load_model(&mut self, model_type: ModelType) -> Result<()>;
    pub async fn unload_model(&mut self, model_type: ModelType) -> Result<()>;
    pub fn get_available_models(&self) -> Vec<ModelInfo>;
}
```

---

## 实施路线

### Phase 2.1: 多模型 LLM 支持 ⬅️ 当前
**修改文件**:
- `src/core/llm.rs` - 添加模型切换逻辑
- `src/models/mod.rs` - 添加 ModelInfo 结构
- `src/components/settings.rs` - 模型选择 UI 已有，需连接后端
- `src/server_functions/chat.rs` - 添加模型切换 API

**任务**:
- [x] 添加模型选择器 UI (已有)
- [ ] 定义 ModelInfo 数据结构
- [ ] 实现模型切换 server function
- [ ] 连接 UI 和后端
- [ ] 支持 Qwen 1.5B/7B 切换

### Phase 2.2: 图片生成
**修改文件**:
- `src/core/image_gen.rs` - 新增
- `src/server_functions/image.rs` - 新增 API
- `src/components/image_gen.rs` - 生成 UI

**任务**:
- [ ] 集成 Stable Diffusion (diffusers-rs 或 Candle)
- [ ] 添加图片生成 UI
- [ ] 实现提示词优化
- [ ] 支持 z-image 作为备选

### Phase 2.3: 内容创作工作流
**修改文件**:
- `src/components/content_editor.rs` - 新增
- `src/models/content.rs` - 内容数据结构

**任务**:
- [ ] 文章生成模板
- [ ] 图文配合功能
- [ ] 导出格式支持 (Markdown, HTML)

### Phase 2.5: 视频理解 (可选)
**修改文件**:
- `src/core/video_proc.rs` - 新增

**任务**:
- [ ] 视频帧提取
- [ ] 视频摘要生成
- [ ] 字幕生成 (Whisper)

---

## API 设计

### 新增 Server Functions

```rust
// 模型信息
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub size: String,
    pub status: ModelStatus,
}

// 获取可用模型
#[server]
pub async fn list_available_models() -> Result<Vec<ModelInfo>, ServerFnError>;

// 切换模型
#[server]
pub async fn switch_llm_model(model_id: String) -> Result<(), ServerFnError>;

// 获取当前模型
#[server]
pub async fn get_current_model() -> Result<ModelInfo, ServerFnError>;
```

---

## 风险与缓解

| 风险 | 缓解措施 |
|------|---------|
| 内存不足 | 模型动态加载/卸载 |
| 生成速度慢 | 量化模型 + Metal 加速 |
| 图片生成依赖不成熟 | 备选: z-image 或 Python subprocess |

---

## 业界参考

| 框架 | 特点 | 链接 |
|------|------|------|
| **Candle** | HuggingFace 官方 Rust ML | [GitHub](https://github.com/huggingface/candle) |
| **Kalosm** | 当前使用，简单易用 | [Docs](https://docs.rs/kalosm) |
| **llama-cpp-rs** | GGUF 模型支持 | [GitHub](https://github.com/utilityai/llama-cpp-rs) |
| **diffusers-rs** | Rust SD 实现 | [GitHub](https://github.com/pykeio/diffusers) |
