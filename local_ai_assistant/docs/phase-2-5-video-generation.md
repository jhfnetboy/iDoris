# Phase 2.5: 外部 API 视频生成

## 概述

Phase 2.5 实现了基于外部 API 的视频生成功能，集成了国内外主流的视频生成服务提供商，为用户提供高性价比的视频创作能力。

## 主要特性

### 1. 多服务商支持
- **国内厂商（推荐）**:
  - 字节跳动：即梦V1/V2、豆包视频
  - 阿里巴巴：通义万象、阿里VGen
  - 百度：文心视频、飞桨视频
  - 腾讯：混元视频

- **国际厂商**:
  - OpenRouter：Pika 2.0、Gen-2、Stable Video Diffusion
  - Together.ai：Stable Video、OpenSora
  - Replicate：Zeroscope、Stable Video Turbo

### 2. 成本优势
- **国内厂商**：$0.005-0.020/秒（约0.035-0.15 RMB/秒）
- **国际厂商**：$0.01-0.06/秒
- **默认推荐**：即梦V2（性价比最高）

### 3. 灵活的配置选项
- 视频尺寸：256x256 到 2048x2048
- 时长：2-30秒
- 质量：标准(480p)、高清(720p)、超清(1080p+)
- 帧率：8-60 FPS
- 随机种子支持

## 实现文件

### 核心模块
- `src/core/video_gen.rs` - 视频生成核心逻辑
- `src/server_functions/server_video_gen.rs` - 服务器端API函数
- `src/components/video_gen.rs` - UI组件

### 集成修改
- `src/core/mod.rs` - 添加video_gen模块导出
- `src/components/mod.rs` - 添加VideoGenPanel导出
- `src/server_functions/mod.rs` - 添加server_video_gen模块
- `src/components/app.rs` - 集成ActivePanel::VideoGen
- `src/components/sidebar.rs` - 添加视频生成按钮
- `Cargo.toml` - 添加lazy_static、tokio-util依赖

## 使用方法

### 1. 配置API密钥
```bash
# 复制环境变量模板
cp .env.example .env

# 编辑 .env 文件，添加至少一个服务商的API密钥
# 推荐先配置 BYTEDANCE_API_KEY
```

### 2. 启动应用
```bash
cargo run --features server
```

### 3. 使用视频生成
1. 点击侧边栏的"视频生成"按钮
2. 输入视频描述
3. 选择服务商和模型
4. 调整参数（可选）
5. 点击"生成视频"
6. 等待生成完成并下载

## API集成示例

### 字节跳动（即梦）
```rust
let request = VideoRequest::new("一只可爱的小猫在花园里玩耍")
    .with_provider(VideoProvider::ByteDance)
    .with_model(VideoModel::JimengV2)
    .with_config(VideoConfig {
        duration_seconds: 5,
        quality: VideoQuality::HD,
        ..Default::default()
    });
```

### OpenRouter
```rust
let request = VideoRequest::new("A cute cat playing in a garden")
    .with_provider(VideoProvider::OpenRouter)
    .with_model(VideoModel::Pika2)
    .with_config(VideoConfig {
        duration_seconds: 10,
        quality: VideoQuality::Premium,
        ..Default::default()
    });
```

## 技术架构

### 1. 统一接口
- `VideoGenerator` 结构体管理所有提供商
- `VideoRequest` 统一请求格式
- `VideoResponse` 统一响应格式

### 2. 异步处理
- 使用 tokio 异步HTTP请求
- 支持长时间任务的超时处理
- 全局单例模式管理API客户端

### 3. 成本估算
- 实时计算预估成本
- 根据模型、质量、时长动态调整
- 透明化费用显示

## 后续优化方向

### 短期（v0.2.0）
- [ ] 添加任务队列支持
- [ ] 实现视频生成进度追踪
- [ ] 批量生成功能
- [ ] 视频风格预设

### 中期（v0.3.0）
- [ ] 视频编辑功能
- [ ] 模板系统
- [ ] 历史记录管理
- [ ] 视频预览优化

### 长期（v1.0.0）
- [ ] 本地视频生成集成
- [ ] 视频后处理效果
- [ ] 视频拼接和剪辑
- [ ] 社交媒体直接发布

## 成本对比（5秒720p视频）

| 服务商 | 模型 | 成本(美元) | 成本(人民币) | 推荐场景 |
|-------|------|-----------|------------|----------|
| 即梦V2 | JimengV2 | $0.05 | ~0.35 | 中文内容，性价比首选 |
| 通义万象 | WanX | $0.07 | ~0.50 | 企业级应用 |
| 文心视频 | Ernie | $0.075 | ~0.53 | 稳定性要求高 |
| Pika 2.0 | Pika2 | $0.15 | ~1.05 | 英文内容，高质量 |
| Gen-2 | Gen2 | $0.20 | ~1.40 | 专业级视频 |

## 安全考虑

1. **API密钥管理**：使用环境变量存储，不硬编码
2. **请求限制**：实现超时和重试机制
3. **成本控制**：实时显示预估成本，防止意外消耗
4. **内容过滤**：依赖服务商的内容审核机制

## 故障排除

### 常见问题
1. **API密钥未配置**：检查 .env 文件是否正确设置
2. **网络连接失败**：检查防火墙和网络设置
3. **生成超时**：降低视频长度或分辨率
4. **余额不足**：检查服务商账户余额

### 调试技巧
1. 查看浏览器开发者工具的网络请求
2. 检查服务器端日志输出
3. 使用服务商的API文档验证请求格式

---

*Phase 2.5 已完成 ✅*
*下一阶段：Phase 3 - 企业级功能增强*