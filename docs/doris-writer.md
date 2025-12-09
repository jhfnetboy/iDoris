# DorisCreator
An AI robot and tool for my wife to create sth.
这是一份为您定制的 **CreatorDoris 产品需求文档 (PRD) 与架构设计书**。

这份设计旨在打造一个**“本地优先、AI 驱动、全链路闭环”**的超级创作者工作台。核心理念是：**把重复的交给机器，把灵魂留给创作者。**

---

# CreatorDoris：全链路智能创作者操作系统 (PRD)

## 1. 产品概述 (Product Overview)

* **产品名称**：CreatorDoris (多丽丝·创作者)
* **产品定位**：面向专业内容创作者的 AI Agent 工作流平台。集信息捕获、RAG 辅助写作、智能视觉布局、RPA 自动化发布于一体。
* **核心价值**：
    1.  **输入自动化**：解决“写什么”的信息焦虑。
    2.  **写作智能化**：解决“怎么写”的效率问题，拒绝 AI 味。
    3.  **排版自动化**：解决“配图难”的审美门槛。
    4.  **分发自动化**：解决“多平台”的繁琐操作，通过本地 Chrome 模拟保障账号安全。

---

## 2. 核心工作流设计 (Core Workflow)

用户在 CreatorDoris 中的旅程分为四个阶段：**捕获 (Capture) -> 铸造 (Craft) -> 视觉 (Visual) -> 启航 (Ship)**。

### 阶段一：Doris Radar (全网雷达与知识库)
* **自动订阅**：
    * 内置 RSS 阅读器（支持公众号转 RSS）。
    * Twitter List 监控 / User Timeline 抓取。
    * Substack 邮件解析。
* **智能清洗**：
    * 系统自动抓取正文，去除广告，转为 Markdown 或 PDF 存储。
    * **Auto-Tagging**：LLM 自动提取关键词、摘要、情感倾向，存入 **Vector DB (向量数据库)**。
* **用户操作**：
    * 在“信息流”中浏览，点击“收藏/归档”将有价值内容加入 **RAG 库**。

### 阶段二：Doris Writer (灵感工坊)
* **RAG 检索与链接**：
    * 用户输入模糊意图（例如：“写一篇关于 AI 降本增效的文章”）。
    * 系统通过 RAG 检索库中相关材料，展示在左侧栏，并提供**源文件回溯链接**。
* **多源融合写作**：
    * 用户勾选 3-5 篇参考素材 + 输入核心 Idea/大纲。
    * **点击“开始创作”** -> 调用 LLM（GPT-4/Claude 3.5）。
* **风格化与去 AI 化**：
    * **风格预设**：选择“小红书种草风”、“公众号深度风”、“推特短很多风”。
    * **去 AI 化引擎**：通过 Prompt Chain（提示词链）和少样本学习（Few-Shot），模仿用户上传的历史文章风格，增加口语词、情绪词，去除“首先、其次、综上所述”等 AI 特征。
    * **渠道适配**：自动调整字数（小红书 < 800字）、Emoji 密度、段落间距。

### 阶段三：Doris Studio (视觉布局)
* **Markdown 渲染**：
    * 即时预览最终排版效果（支持微信公众号 CSS 注入）。
* **AI 绘图引擎**：
    * 基于文章内容自动提取 Prompt。
    * **风格选择**：勾选“扁平插画”、“美漫漫画”、“极简线条 Sketch”、“高逼真摄影”、“海报 Banner”。
    * **多图生成**：一次生成 4 张备选图。
* **图文合成 (Canvas)**：
    * 自动将文章金句/标题叠加在图片上（智能反色、自动排版），生成封面图。
    * 插图自动插入 Markdown 对应位置。

### 阶段四：Doris Publisher (自动化发布)
* **本地 Chrome 接管**：
    * 不使用官方 API（避免高昂费用和权限限制），采用 **RPA (Robotic Process Automation)** 技术。
    * 调用本地安装的 Chrome 浏览器，指定 `--user-data-dir` (用户配置文件)，**直接复用用户已登录的 Cookie 状态**，无需重复扫码。
* **全自动填表**：
    * 自动打开公众号后台/小红书网页版。
    * 自动上传封面图、正文、填写标题、选择标签/合集。
* **人工确认 (Human-in-the-loop)**：
    * RPA 完成草稿录入后，弹窗通知用户。
    * 用户在自动打开的浏览器窗口中点击最终的“发布”按钮（确保安全感）。

---

## 3. 功能模块详细规格 (Detailed Specs)

### 3.1 数据层 (The Memory)
* **技术栈**：SQLite (元数据) + ChromaDB/Milvus (向量数据) + 本地文件系统 (MD/PDF)。
* **RAG 实现**：
    * **Embeddings**：支持调用 OpenAI Embedding API 或本地运行 BGE-M3 模型（保护隐私）。
    * **召回策略**：关键词匹配 + 语义相似度混合检索。

### 3.2 写作层 (The Brain)
* **模型路由**：支持用户配置 Key（OpenAI, Anthropic, Gemini, DeepSeek）。
* **Prompt 市场**：内置多套经过打磨的 System Prompt。
    * *例：[小红书风] 你是一个资深博主，多用Emoji，第一句话就要吸引眼球，正文分点叙述，结尾要有互动引导。*

### 3.3 视觉层 (The Eye)
* **绘图接口**：
    * 云端：Midjourney (需代理), DALL-E 3。
    * 本地：支持连接本地 Stable Diffusion WebUI / ComfyUI API。
* **配文功能**：基于 Canvas API 或 Python Pillow 库，支持自定义字体库。

### 3.4 发布层 (The Hand)
* **核心技术**：Playwright 或 Puppeteer (连接现有 Chrome 实例)。
* **渠道配置**：
    * **公众号**：支持排版样式保留。
    * **小红书**：支持多图轮播上传，Tag 自动生成。
    * **自定义流 (高级版)**：允许用户通过录制器（Recorder）录制动作，生成新的发布脚本。

---

## 4. UI/UX 界面设计概念

### 主界面：Dashboard
* 左侧：侧边栏（雷达、写作、画室、发布、设置）。
* 中间：任务看板（待选材、创作中、待发布、已归档）。

### 创作界面：三栏式布局
1.  **左栏 (Context)**：素材区。RAG 搜索结果，勾选的推文/RSS 卡片，源文件预览。
2.  **中栏 (Editor)**：所见即所得 (WYSIWYG) Markdown 编辑器。
    * *AI 悬浮窗*：选中文本 -> 润色/扩写/改写风格。
3.  **右栏 (Config)**：参数区。
    * 目标平台选择（WeChat/XHS）。
    * 风格滑块（正式 <-> 幽默）。
    * 配图生成面板（点击生成图片，拖拽入正文）。

---

## 5. 软件架构 (Software Architecture)

为了实现“本地操作”，建议采用 **Electron** 桌面应用架构。

* **Frontend**: React + TailwindCSS (响应式 UI).
* **Backend (In-App)**:
    * Node.js (主进程控制).
    * Python 子进程 (用于处理重型任务：RAG 向量化、Playwright 脚本执行、图片合成处理)。
* **Database**: 本地 SQLite 文件。
* **Browser Control**: 通过 WebSocket 连接本地 Chrome 调试端口 (Remote Debugging Port)。

---

## 6. 路线图 (Roadmap)

### Phase 1: MVP (核心闭环)
* 完成 RSS/Twitter 采集入库。
* 基础 RAG + GPT-4 写作。
* DALL-E 3 配图。
* 公众号、小红书 RPA 发布脚本。

### Phase 2: 风格化与扩展
* **本地模型支持**：支持 Ollama (本地 LLM) 和 SD (本地绘图)。
* **AI 风格克隆**：上传 10 篇历史文章，训练 LoRA 或生成 Few-Shot Prompt。
* **图片文字编辑器**：更强大的海报设计功能。

### Phase 3: 生态开放
* **自定义工作流编辑器**：节点式编排（类似 ComfyUI 的逻辑编排采集和发布流程）。
* **插件中心**：社区贡献新的平台发布脚本（如知乎、即使、WordPress）。

---

## 7. 典型用户故事 (User Story)

> **用户**：Alex，科技自媒体人。
>
> 1.  **早上 10:00**：打开 CreatorDoris。系统已自动抓取昨晚 AI 领域的 50 条推文和 3 篇 Substack 深度文。
> 2.  **筛选**：Alex 搜索“Agent”，Doris 筛选出 5 条高相关内容。Alex 勾选它们，点击“创建草稿”。
> 3.  **写作**：Alex 输入 Idea：“分析 Agent 对普通人的影响”。Doris 结合素材生成初稿。Alex 觉得太生硬，点击“去 AI 化 - 毒舌风格”，文章变得犀利。
> 4.  **配图**：Alex 选择“赛博朋克漫画风格”，生成 3 张配图。选中一张，添加文字标题“Agent 时代：普通人的末日？”，生成封面。
> 5.  **发布**：Alex 选择“发布到公众号和小红书”。Doris 自动唤起 Chrome，上传图文。
> 6.  **完成**：Alex 喝了口咖啡，点击浏览器里的“群发”按钮。全程耗时 15 分钟。

这个设计方案是否符合您心中的 CreatorDoris？如果需要，我们可以深入讨论“本地 Chrome 接管”的具体技术实现细节。

