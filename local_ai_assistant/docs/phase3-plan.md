# iDoris Phase 3: Production Excellence & Advanced Features

## 目标

将 iDoris 从功能原型打造为生产级的本地 AI 内容创作平台，提升稳定性、用户体验和高级功能。

---

## Phase 3 路线图

### 优先级顺序
1. **Phase 3.1: Technical Debt Cleanup** (4-5 天) ✅ Current
2. **Phase 3.2: Advanced Content Features** (3-4 周)
3. **Phase 3.3: Production Readiness** (2-3 周)
4. **Phase 3.4: Mobile & Desktop Apps** (6-8 周)

---

## Phase 3.1: Technical Debt Cleanup ⬅️ 当前阶段

**目标**: 清理代码库，提升代码质量和可维护性

### 任务清单

#### 1. 代码清理 (2天)
- [ ] 移除所有 `todo!()` 存根实现
- [ ] 清理未使用的函数和变量
  - `is_model_switching_supported()` - llm.rs
  - `is_initialized()` - embedding.rs
  - `init_model()`, `embed_batch()` - embedding.rs
  - `speak_text()` - tts.rs
  - Model manager 未使用方法
  - ContentSource 未使用方法
- [ ] 修复所有编译器警告
- [ ] 删除备份文件 (`settings_page.rs.backup`)
- [ ] 清理调试日志文件 (`build_errors.log`, `check_output*.txt`)

#### 2. 错误处理改进 (1天)
- [ ] 统一错误类型定义
  ```rust
  // src/core/error.rs - 新增
  pub enum iDorisError {
      ApiError(String),
      ModelError(String),
      ConfigError(String),
      IoError(std::io::Error),
  }
  ```
- [ ] 所有 `unwrap()` 替换为适当的错误处理
- [ ] 添加用户友好的错误消息

#### 3. 配置管理优化 (1天)
- [ ] 验证 `.env` 文件格式
- [ ] 添加配置验证函数
  ```rust
  pub fn validate_config() -> Result<(), ConfigError>;
  ```
- [ ] 为缺失的 API 密钥提供清晰的错误提示

#### 4. 性能优化 (1天)
- [ ] 使用 `cargo build --release --timings` 分析编译时间
- [ ] 优化模型加载时间
- [ ] 添加异步任务的进度跟踪
- [ ] 实现模型缓存预热

#### 5. 测试基础设施 (选择性)
- [ ] 添加关键模块的单元测试
  - `video_gen::sign_volc_request` - 签名算法测试
  - `content_source::parse_rss` - RSS 解析测试
- [ ] 集成测试设置（可选）

**预期产出**:
- ✅ 零编译警告
- ✅ 所有 `todo!()` 已替换
- ✅ 统一的错误处理
- ✅ 更快的构建和运行时性能

---

## Phase 3.2: Advanced Content Features (C)

**目标**: 打造完整的内容创作套件

### 3.2.1 多模态内容生成 (1周)

#### 内容包工作流
**文件**:
- `src/components/content_package.rs` - 新增
- `src/core/content_generator.rs` - 新增
- `src/models/content_package.rs` - 新增

**功能**:
```rust
pub struct ContentPackage {
    pub topic: String,
    pub article: Article,           // 主文章
    pub header_image: ImageAsset,   // 头图
    pub social_clips: Vec<VideoClip>, // 社交媒体短视频
    pub seo_metadata: SeoMetadata,
}

impl ContentPackage {
    pub async fn generate_from_topic(topic: &str) -> Result<Self>;
    pub async fn export_all(&self, format: ExportFormat) -> Result<()>;
}
```

**任务**:
- [ ] 实现"一键生成"流程
  1. 用户输入主题
  2. AI 生成文章大纲
  3. 生成配图（基于标题和各节）
  4. 生成 1-3 个短视频片段（用于社交媒体）
  5. 生成 SEO 元数据
- [ ] 添加进度追踪 UI
- [ ] 支持预览和单独重新生成

### 3.2.2 SEO 优化工具 (1周)

**文件**:
- `src/core/seo.rs` - 新增
- `src/components/seo_panel.rs` - 新增

**功能**:
```rust
pub struct SeoAnalysis {
    pub keyword_density: HashMap<String, f32>,
    pub readability_score: f32,
    pub meta_description: String,
    pub suggested_keywords: Vec<String>,
    pub schema_markup: String, // JSON-LD
}

pub async fn analyze_content(content: &str) -> SeoAnalysis;
pub async fn generate_meta_description(content: &str) -> String;
pub async fn suggest_keywords(topic: &str) -> Vec<String>;
```

**任务**:
- [ ] 关键词密度分析
- [ ] 可读性评分（Flesch-Kincaid）
- [ ] 自动生成 meta description
- [ ] Schema.org markup 生成（Article, VideoObject）
- [ ] H1/H2/H3 结构检查

### 3.2.3 发布集成 (1-2周)

**文件**:
- `src/core/publishers/mod.rs` - 新增
- `src/core/publishers/wordpress.rs` - 新增
- `src/core/publishers/medium.rs` - 新增
- `src/core/publishers/social.rs` - 新增

**支持平台**:
1. **WordPress**
   - XML-RPC API 集成
   - 自动上传图片到 Media Library
   - 发布或保存为草稿
   
2. **Medium**
   - REST API 集成
   - Markdown 格式支持
   
3. **社交媒体** (可选)
   - Twitter API (现 X)
   - LinkedIn API
   - 微信公众号 (需研究 API)

**任务**:
- [ ] 实现 WordPress XML-RPC 客户端
- [ ] Medium API 集成
- [ ] 发布历史记录
- [ ] 发布状态追踪

### 3.2.4 协作功能 (可选)

**文件**:
- `src/core/collaboration.rs` - 新增
- `src/models/comment.rs` - 新增

**功能**:
- [ ] 内容评论系统
- [ ] 简单版本控制（保存历史版本）
- [ ] 导出/导入内容（JSON 格式）

**预期产出**:
- ✅ 完整的内容创作到发布工作流
- ✅ SEO 优化建议
- ✅ 主流平台发布支持

---

## Phase 3.3: Production Readiness (A)

**目标**: 提升用户体验和系统稳定性

### 3.3.1 配置管理 UI (1周)

**文件**:
- `src/components/api_config.rs` - 新增
- `src/core/config_manager.rs` - 新增

**功能**:
- [ ] API 密钥管理界面
  - 添加/编辑/删除 API 密钥
  - 密钥可见性切换（显示/隐藏）
  - 测试连接按钮
- [ ] 提供商健康检查
  ```rust
  pub async fn check_provider_health(provider: VideoProvider) -> HealthStatus;
  ```
- [ ] 使用量追踪
  - 每个 API 的调用次数
  - 费用统计（基于预估）

### 3.3.2 内容历史和图库 (1周)

**文件**:
- `src/components/gallery.rs` - 新增
- `src/storage/content_history.rs` - 新增
- `src/models/content_asset.rs` - 新增

**功能**:
```rust
pub struct ContentAsset {
    pub id: String,
    pub asset_type: AssetType, // Image, Video, Audio
    pub url: String,
    pub local_path: Option<String>,
    pub prompt: String,
    pub metadata: AssetMetadata,
    pub created_at: DateTime<Utc>,
}

// 图库操作
pub async fn save_asset(asset: ContentAsset) -> Result<()>;
pub async fn list_assets(filter: AssetFilter) -> Result<Vec<ContentAsset>>;
pub async fn delete_asset(id: &str) -> Result<()>;
```

**UI 功能**:
- [ ] 网格视图展示所有生成的内容
- [ ] 按类型筛选（图片/视频/音频）
- [ ] 按时间排序
- [ ] 搜索功能（基于 prompt）
- [ ] 一键重新生成
- [ ] 导出/下载

### 3.3.3 后台任务管理 (1周)

**文件**:
- `src/core/task_queue.rs` - 新增
- `src/components/task_manager.rs` - 新增

**功能**:
```rust
pub struct TaskQueue {
    pending: Vec<BackgroundTask>,
    running: Vec<BackgroundTask>,
    completed: Vec<BackgroundTask>,
}

pub enum BackgroundTask {
    ImageGeneration { prompt: String, config: ImageConfig },
    VideoGeneration { prompt: String, config: VideoConfig },
    ContentGeneration { topic: String },
}

impl TaskQueue {
    pub async fn enqueue(&mut self, task: BackgroundTask) -> TaskId;
    pub async fn cancel(&mut self, id: TaskId) -> Result<()>;
    pub fn get_status(&self, id: TaskId) -> TaskStatus;
}
```

**任务**:
- [ ] 任务队列实现（最多 3 个并行任务）
- [ ] 任务状态 UI（进行中、完成、失败）
- [ ] 通知系统（任务完成时提示）

### 3.3.4 提供商回退机制 (3天)

**文件**:
- 修改 `src/core/image_gen.rs`
- 修改 `src/core/video_gen.rs`

**功能**:
```rust
pub struct ProviderFallback {
    primary: Provider,
    fallbacks: Vec<Provider>,
}

impl ProviderFallback {
    pub async fn generate_with_retry<T>(
        &self,
        request: T,
    ) -> Result<Response> {
        // 先尝试 primary
        // 失败则依次尝试 fallback
    }
}
```

**任务**:
- [ ] 实现自动重试逻辑
- [ ] 配置回退顺序（例如：Local → Replicate → Together）
- [ ] 记录失败原因

### 3.3.5 成本追踪 (3天)

**文件**:
- `src/storage/usage_tracker.rs` - 新增
- `src/components/usage_dashboard.rs` - 新增

**功能**:
- [ ] 记录每次 API 调用的成本
- [ ] 按提供商分组统计
- [ ] 每日/每周/每月报告
- [ ] 成本预警（超过设定阈值）

**预期产出**:
- ✅ 无需编辑 `.env`，所有配置通过 UI 管理
- ✅ 完整的生成历史和图库
- ✅ 后台任务不阻塞 UI
- ✅ 成本透明化

---

## Phase 3.4: Mobile & Desktop Apps (D)

**目标**: 扩展到原生应用

### 3.4.1 Desktop App (Tauri + Dioxus) (3-4周)

**新文件**:
- `src-tauri/` - Tauri 配置
- `src/desktop/` - 桌面特定代码

**功能**:
- [ ] 原生菜单栏集成
- [ ] 系统托盘图标
  - 快速访问常用功能
  - 后台生成通知
- [ ] 离线模式
  - 本地模型优先
  - 无网络时仍可使用基础功能
- [ ] 文件系统集成
  - 拖放导入图片/视频
  - 直接保存到用户选择目录
- [ ] 自动更新

**技术选型**:
- **Tauri** (推荐) - 小巧、快速、跨平台
- 或 **Dioxus Desktop** - 纯 Rust，但生态不如 Tauri 成熟

### 3.4.2 Mobile App (Dioxus Mobile) (4-5周)

**新文件**:
- `mobile/` - 移动端代码
- `mobile/ios/` - iOS 配置
- `mobile/android/` - Android 配置

**功能**:
- [ ] 响应式 UI（适配小屏幕）
- [ ] 相机集成
  - 拍照后直接用于图片编辑/分析
- [ ] 语音录制
  - 用于 TTS 输入
  - 或语音转文字（Whisper）
- [ ] 离线模式
  - 本地小模型（Qwen 1.5B）
- [ ] 推送通知
  - 生成完成提醒

**挑战**:
- Dioxus Mobile 仍在早期，可能需要自定义构建
- 模型大小限制（移动端内存受限）

**预期产出**:
- ✅ macOS/Windows/Linux 桌面应用
- ✅ iOS 和 Android 应用（如可行）

---

## 技术栈更新

### Phase 3 新增依赖

```toml
[dependencies]
# Phase 3.1 - Error Handling
thiserror = "2.0"
anyhow = "1.0" # 已有

# Phase 3.2 - Publishing
reqwest = { version = "0.12", features = ["json", "multipart"] } # 已有
xmlrpc = "0.15"  # WordPress
url = "2.5"

# Phase 3.3 - Task Queue
tokio = { version = "1.45", features = ["sync", "time"] } # 已有
notify-rust = "4.11"  # 系统通知

# Phase 3.4 - Desktop
[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
tauri = { version = "2.0" }
```

---

## 实施时间表

| 阶段 | 时长 | 开始日期 |
|------|------|---------|
| Phase 3.1: Tech Debt | 4-5 天 | 2025-12-10 |
| Phase 3.2: Advanced Content | 3-4 周 | 2025-12-16 |
| Phase 3.3: Production Readiness | 2-3 周 | 2026-01-13 |
| Phase 3.4: Mobile & Desktop | 6-8 周 | 2026-02-03 |

**总计**: 约 3-4 个月

---

## 风险与缓解

| 风险 | 缓解措施 |
|------|---------|
| Dioxus Mobile 生态不成熟 | 使用 Tauri Mobile (RC) 或 Flutter 作为备选 |
| API 成本超预算 | 实现成本预警和自动停止 |
| 移动端模型性能差 | 仅提供轻量级功能，复杂任务使用云端 |
| WordPress API 权限问题 | 提供详细文档和故障排查指南 |

---

## 成功指标

- **Phase 3.1**: 零编译警告，构建时间 < 200 秒
- **Phase 3.2**: 用户可一键生成完整内容包并发布
- **Phase 3.3**: 用户完全不需要接触 `.env` 文件
- **Phase 3.4**: 桌面应用可独立分发，移动应用通过 TestFlight/Beta 测试

---

## 参考资源

- [Tauri 文档](https://tauri.app/)
- [Dioxus Mobile](https://dioxuslabs.com/learn/0.6/guides/mobile)
- [WordPress XML-RPC API](https://codex.wordpress.org/XML-RPC_WordPress_API)
- [Medium API](https://github.com/Medium/medium-api-docs)
