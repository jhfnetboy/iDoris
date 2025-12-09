这是一份深度调研后的技术架构方案。为了满足**高性能、本地化、隐私安全**的需求，我们将核心架构建立在 **Rust** 之上，同时在 AI 和自动化发布环节采用“胶水策略”，灵活集成 Python 生态中无可替代的优秀工具（如 ComfyUI 或 Playwright 的特定脚本）。

这个架构被称为 **"Rust-Core, Polyglot-Edge"（Rust 为核，多语言为翼）**。

-----

# CreatorDoris 技术架构方案 (Rust Edition)

## 1\. 核心框架 (App Shell & OS Interaction)

**目标**：构建跨平台桌面应用（Win/Mac/Linux），体积小，内存占用低，且能深度调用本地系统资源。

  * **技术选型**：**Tauri v2**
  * **Repo URL**：[https://github.com/tauri-apps/tauri](https://github.com/tauri-apps/tauri)
  * **实施细节**：
      * **架构**：Rust 作为后端（主进程），负责数据库读写、文件系统操作、RAG 运算和系统调用；前端使用 React/Vue 构建 UI。
      * **优势**：相比 Electron，Tauri 的安装包非常小（\<20MB），且后端逻辑直接用 Rust 编写，安全性极高。
      * **本地通信**：使用 Tauri 的 IPC (Inter-Process Communication) 机制连接前端 UI 和 Rust 核心。

-----

## 2\. 阶段一：Doris Radar (采集与清洗)

**目标**：无头浏览抓取、RSS 解析、HTML 转 Markdown。

### 2.1 RSS 订阅与解析

  * **技术选型**：**feed-rs**
  * **Repo URL**：[https://github.com/feed-rs/feed-rs](https://github.com/feed-rs/feed-rs)
  * **实施细节**：
      * Rust 原生的 Atom/RSS 解析库，速度极快。
      * 配合 `reqwest` (Rust 的 HTTP 客户端) 定时拉取 XML 内容。

### 2.2 网页清洗与正文提取 (HTML -\> Markdown)

  * **技术选型**：**readability.rs** + **html2text**
  * **Repo URL**：
      * [https://github.com/j5/readability](https://www.google.com/search?q=https://github.com/j5/readability) (Mozilla Readability 的 Rust 移植)
      * [https://github.com/krdln/html2text](https://www.google.com/search?q=https://github.com/krdln/html2text)
  * **实施细节**：
      * 当抓取到网页后，先通过 `readability` 去除广告、侧边栏，提取主要内容。
      * 然后使用 `html2text` 将清洗后的 HTML 转换为干净的 Markdown 格式存入数据库。

### 2.3 动态网页抓取 (Twitter/X)

  * **技术选型**：**chromiumoxide** (基于 CDP 协议)
  * **Repo URL**：[https://github.com/mattsse/chromiumoxide](https://github.com/mattsse/chromiumoxide)
  * **实施细节**：
      * 这是一个纯 Rust 的 Chrome DevTools Protocol (CDP) 高级 API。
      * **无 API 抓取**：不使用 Twitter API，而是启动一个无头浏览器实例，模拟滚动，拦截网络请求或直接解析 DOM 获取推文。这种方式最适合本地工具，因为它可以使用用户的本地 Cookie 绕过登录墙。

-----

## 3\. 阶段二：Doris RAG (数据层与记忆)

**目标**：本地向量存储、全文检索、隐私保护。

### 3.1 嵌入式向量数据库 (The Memory)

  * **技术选型**：**LanceDB** (Rust SDK)
  * **Repo URL**：[https://github.com/lancedb/lancedb](https://github.com/lancedb/lancedb)
  * **实施细节**：
      * **为什么选它**：LanceDB 是为 AI 时代设计的，核心用 Rust 编写，支持直接嵌入应用内运行（Serverless），不需要用户在本地安装 Docker 或配置 Milvus 服务。
      * 它支持存储原始数据（Text）和向量数据（Vectors），读写速度极快。

### 3.2 本地 Embedding 模型 (向量化)

  * **技术选型**：**fastembed-rs**
  * **Repo URL**：[https://github.com/Anush008/fastembed-rs](https://github.com/Anush008/fastembed-rs)
  * **实施细节**：
      * 这是一个纯 Rust 实现的 Embedding 库，底层使用 ONNX Runtime。
      * **关键点**：它不需要 Python 环境！可以直接在 Rust 中运行 `BGE-M3` 或 `All-MiniLM-L6-v2` 等轻量级模型，将采集到的文本转化为向量存入 LanceDB。

-----

## 4\. 阶段三：Doris Writer (写作与 LLM)

**目标**：本地/云端 LLM 调度、Prompt 管理。

### 4.1 LLM 推理客户端

  * **技术选型**：**llm-chain** 或 **ollama-rs**
  * **Repo URL**：
      * [https://github.com/sobelio/llm-chain](https://github.com/sobelio/llm-chain) (通用链式调用)
      * [https://github.com/pepperoni21/ollama-rs](https://github.com/pepperoni21/ollama-rs) (本地 Ollama 绑定)
  * **实施细节**：
      * **本地优先**：检测用户本地是否安装了 Ollama，优先调用本地模型（如 Llama 3, Qwen 2）。
      * **云端备选**：通过 `reqwest` 调用 OpenAI/Claude 格式的 API。
      * **Prompt 模板**：使用 **Tera** ([https://github.com/Keats/tera](https://github.com/Keats/tera)) 模板引擎来管理不同风格的 Prompt（如“小红书风”、“深度长文”），将 RAG 检索到的 Context 动态填充进 Prompt。

-----

## 5\. 阶段四：Doris Studio (视觉与布局)

**目标**：AI 绘图、自动排版、海报生成。

### 5.1 AI 绘图连接器 (ComfyUI)

  * **方案**：不直接在 Rust 里重写 Stable Diffusion，而是集成 **ComfyUI**。
  * **Repo URL**：[https://github.com/comfyanonymous/ComfyUI](https://github.com/comfyanonymous/ComfyUI)
  * **实施细节**：
      * CreatorDoris 内置一个轻量级的 Python 环境（或引导用户安装），后台静默启动 ComfyUI 的 API 模式。
      * Rust 端通过 HTTP 请求向 ComfyUI 发送 Workflow JSON（包含提示词、Seed），接收生成的 Base64 图片。
      * 这也是目前最强大的 AI 绘图工作流方案，支持各类 LoRA 和 ControlNet。

### 5.2 矢量绘图与海报合成 (SVG to Image)

  * **技术选型**：**resvg** + **usvg**
  * **Repo URL**：[https://github.com/RazrFalcon/resvg](https://github.com/RazrFalcon/resvg)
  * **实施细节**：
      * **核心思路**：不要用像素绘图库去画海报，很难排版。
      * **方案**：使用 React 前端设计海报模板（HTML/CSS/SVG），然后将其导出为 SVG 字符串。
      * 使用 Rust 的 `resvg` 库，在后端极速将 SVG 渲染为高分辨率 PNG/JPG。这允许你轻松实现“图中加字”、“自动换行”、“复杂布局”。

-----

## 6\. 阶段五：Doris Publisher (自动化发布)

**目标**：接管本地 Chrome，复用 Cookie，RPA 操作。

### 6.1 浏览器自动化 (Chrome DevTools Protocol)

  * **技术选型**：**headless\_chrome** (Rust)
  * **Repo URL**：[https://github.com/rust-headless-chrome/rust-headless-chrome](https://github.com/rust-headless-chrome/rust-headless-chrome)
  * **实施细节**：
      * 这是发布环节的核心。
      * **关键技术点（接管模式）**：
        1.  Rust 调用系统命令启动用户的 Chrome 浏览器，带上参数 `--remote-debugging-port=9222` 和 `--user-data-dir="用户数据路径"`。
        2.  `headless_chrome` 连接到 `localhost:9222`。
        3.  此时，脚本直接拥有了用户的登录状态（Cookie/Local Storage）。
      * **操作逻辑**：通过 CSS Selector 定位输入框，模拟键盘输入（`type_text`），模拟点击发布按钮。

-----

## 7\. 综合数据流架构图

```mermaid
graph TD
    User[用户] --> Frontend[Tauri Frontend (React)]
    
    subgraph Rust Core [Tauri Backend Process]
        Manager[工作流管理器]
        RSS[Feed-rs 解析器]
        Scraper[Chromiumoxide 爬虫]
        DB[(LanceDB 向量库)]
        Layout[Resvg 渲染器]
        Publisher[Headless_Chrome RPA]
    end
    
    subgraph External/Local Services
        Ollama[Ollama (Local LLM)]
        ComfyUI[ComfyUI (Local SD)]
        Chrome[用户本地 Chrome 浏览器]
    end

    Frontend -- IPC Commands --> Manager
    
    Manager -- 调度 --> RSS
    Manager -- 调度 --> Scraper
    RSS & Scraper -- 清洗存入 --> DB
    
    Manager -- 查询 Context --> DB
    Manager -- Prompt + Context --> Ollama
    Ollama -- 生成文本 --> Manager
    
    Manager -- 绘图请求 --> ComfyUI
    ComfyUI -- 图片 --> Layout
    Manager -- 文本 --> Layout
    Layout -- 合成海报 --> Frontend
    
    Frontend -- 确认发布 --> Manager
    Manager -- 控制指令 (CDP) --> Publisher
    Publisher -- WebSocket --> Chrome
    Chrome -- 上传发布 --> Internet[小红书/公众号]
```

## 8\. 开发路线建议 (MVP Step-by-Step)

1.  **基础建设**：使用 `Tauri v2` 初始化项目，配置 `rusqlite` 建立基础的文章元数据表。
2.  **输入端**：引入 `feed-rs`，实现最简单的 RSS 阅读器功能，将数据存入 SQLite。
3.  **RAG 端**：引入 `lancedb` 和 `fastembed-rs`。当 RSS 更新时，自动 Embed 摘要并存入 LanceDB。实现一个简单的“语义搜索”Demo。
4.  **写作端**：集成 `ollama-rs`。让用户选中一条 RSS，点击“改写”，调用本地 Llama3 生成 Markdown。
5.  **发布端 (难点)**：先写一个独立的 Rust 脚本，使用 `headless_chrome` 尝试连接本地开启了调试端口的 Chrome，并完成一次“在 Google 搜索框自动打字”的操作，验证 RPA 可行性。

这个技术栈组合充分利用了 Rust 的**系统级控制能力**（RPA、文件、高性能数据库）和 Python 的**AI 生态**（ComfyUI），是构建高性能 CreatorDoris 的最佳实践。

