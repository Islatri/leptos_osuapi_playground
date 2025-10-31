ciallo = 恰咯
demo = 演示

# Footer content
footer-description = 高性能, 结构优良, 拓展性好的 Rust osu! API 客户端 支持 WASM 和 native 环境.
footer-docs-heading = 文档
footer-docs-getting-started = 入门指南
footer-docs-api-reference = API 参考
footer-docs-examples = 示例
footer-resources-heading = 资源
footer-resources-api-v1 = osu! API V1 文档
footer-resources-api-v2 = osu! API V2 文档
footer-resources-leptos = Leptos 框架
footer-resources-issues = 问题反馈
footer-resources-changelog = 更新日志
footer-copyright = {$year} Osynicite. OsynicOsuapi 是非官方项目, 与 osu! 官方无关.
footer-terms = 使用条款
footer-privacy = 隐私政策


# Hero section
hero-badge = RUST OSU! API 客户端
hero-title-highlight = OsynicOsuapi
hero-title-subtext = 高性能, 结构优良, 拓展性好的 Rust osu! API 客户端
hero-description = 类型安全, 异步友好, WASM 兼容的 API 客户端, 让您的 Rust 项目与 osu! 无缝衔接.
hero-button-docs = 查看文档
hero-button-demo = 使用尝鲜
hero-feature-api = 支持 v1 和 v2 API
hero-feature-type-safe = 类型安全
hero-feature-wasm = WASM 兼容


# API Section
api-section-title = 完整的 API 覆盖
api-section-description = OsynicOsuapi 提供所有 osu! API 端点的完整类型安全访问, 包括 v1 和 v2 版本的大部分.

# V1 API
api-v1-title = API v1

# V1 API Cards
api-beatmaps-title = 谱面 API
api-beatmaps-description = 查询谱面信息, 包括详情, 评分和状态.支持按 ID, 图集 ID 或哈希查询.
api-users-title = 用户 API
api-users-description = 获取用户资料, 统计数据, 最佳成绩和最近游玩记录.支持用户名或 ID 查询.
api-scores-title = 分数 API
api-scores-description = 获取不同谱面和用户的分数信息, 支持按模式和筛选器查询.
api-replays-title = 回放 API
api-replays-description = 获取用户的回放数据.支持按 ID 查询.
api-multiplayer-title = 多人游戏 API
api-multiplayer-description = 获取多人游戏房间信息, 游戏记录和参与者数据.

# V2 API
api-v2-title = API v2

# V2 API Cards
api-oauth-title = OAuth API
api-oauth-description = 处理 OAuth 2.0 认证流程.
api-v2-beatmaps-title = 谱面 API
api-v2-beatmaps-description = 查询谱面信息, 详情和元数据.
api-beatmapsets-title = 谱面集 API
api-beatmapsets-description = 获取谱面集信息和其包含的谱面.
api-changelog-title = 更改日志 API
api-changelog-description = 获取游戏更新和变化记录.
api-chat-title = 聊天 API
api-chat-description = 获取聊天历史和发送消息.
api-comments-title = 评论 API
api-comments-description = 获取和发布评论内容.
api-events-title = 事件 API
api-events-description = 获取最新事件和动态.
api-forum-title = 论坛 API
api-forum-description = 获取论坛主题和帖子.
api-home-title = 主页 API
api-home-description = 获取个人主页数据.
api-matches-title = 比赛 API
api-matches-description = 获取官方比赛信息.
api-v2-multiplayer-title = 多人游戏 API
api-v2-multiplayer-description = 获取和管理多人游戏房间.
api-news-title = 新闻 API
api-news-description = 获取游戏最新新闻.
api-notifications-title = 通知 API
api-notifications-description = 获取个人通知信息.
api-rankings-title = 排名 API
api-rankings-description = 获取详细排名数据.
api-v2-scores-title = 分数 API
api-v2-scores-description = 获取详细分数信息.
api-v2-users-title = 用户 API
api-v2-users-description = 获取用户详细信息.
api-wiki-title = 维基 API
api-wiki-description = 获取维基页面内容.


# Features Section
features-section-title = 为什么选择 OsynicOsuapi?
features-section-description = 一个高性能、类型安全的 osu! API 库，为各种平台提供完整的开发体验
features-high-performance-title = 高性能
features-high-performance-description = 基于 Rust 构建，提供卓越的性能和内存安全性，适合高吞吐量应用。无论是服务器还是客户端应用，都能获得最佳性能体验。
features-type-safe-title = 类型安全
features-type-safe-description = 利用 Rust 的强类型系统，在编译时捕获错误，提供完整的 API 模型。基于 client、interface、model 三重模块划分，结构清晰易于理解。
features-multi-platform-title = 多平台
features-multi-platform-description = 支持 WASM、native 和服务器环境，让您能在任何地方使用相同的 API。为 V1 接口提供了 WebAssembly 支持，可直接从网页应用访问 OSU API。
features-api-support-title = 完整 API 支持
features-api-support-description = 支持 OSU! V1 的所有端点以及 V2 的大部分端点（除了文档未归类的接口）。无论您的项目需要哪个版本的 API，都能轻松集成。
features-flexible-extension-title = 灵活扩展
features-flexible-extension-description = client 部分聚合 interface 接口并支持多种 HTTP 客户端，便于拓展。模块化设计让您可以根据需求自定义实现，轻松适应不同的应用场景。
features-rich-examples-title = 丰富示例
features-rich-examples-description = 提供完整的示例支持。运行 `cargo run --example 示例名` 查看返回数据，通过示例快速学习和上手。从示例代码中学习是掌握本库的最佳方式。
features-cta-button = 查看OsynicOsuapi的crates.io页面


# Quick Start Section
quick-start-title = 快速开始
quick-start-description = 简单几步配置, 快速接入 osu! API
quick-start-reqwest-client = Reqwest 客户端
quick-start-env-var = 环境变量
quick-start-config-file = 配置文件
quick-start-example-code = 示例代码
quick-start-wasm-client = WASM 客户端
quick-start-view-examples = 查看示例代码


# API Demo Section
api-demo-title-1 = osu! API V1 (WASM) 体验使用
api-demo-description-1 = 输入您的 osu! API 密钥, 尝试在浏览器中使用 OsynicOsuapi.
api-demo-description-2 = 如果您没有 API 密钥, 直接去自己的
api-demo-account-settings = osu! 个人设置页
api-demo-api-section = 翻到下面的 旧版本API 申请一个即可.
api-demo-label-api-key = API 密钥
api-demo-input-placeholder = 输入您的 osu! API 密钥
api-demo-key-security = 密钥只在您的浏览器中使用, 不会传输到其他地方.


# API Demo Additional Elements
api-demo-beatmap-query = 谱面查询
api-demo-user-query = 用户查询
api-demo-input-beatmap-label = 谱面集 ID
api-demo-input-beatmap-placeholder = 输入谱面集 ID
api-demo-input-user-label = 用户名或用户 ID
api-demo-input-user-placeholder = 输入用户名或用户 ID
api-demo-loading = 加载中
api-demo-search = 查询
api-demo-search-result = 查询结果
api-demo-left-formatted = 左侧: 格式化结果
api-demo-right-raw = 右侧: 原始 JSON 数据
api-demo-formatted-results = 格式化结果
api-demo-raw-json = API 原始 JSON
api-demo-description-wasm = 这个演示使用 OsynicOsuapi 通过 WASM 直接在您的浏览器中运行, 无需后端服务器处理.
api-demo-view-github = 查看 GitHub 项目
api-demo-current-date = 当前日期和时间 (UTC - YYYY-MM-DD HH:MM:SS 格式):
api-demo-current-user = 当前用户的登录名:

# API Demo Code Logic
api-demo-result-placeholder = // 结果将显示在这里
api-demo-raw-json-placeholder = // API 原始 JSON 将显示在这里
api-demo-enter-api-key = 请输入 API 密钥
api-demo-loading-text = 正在加载...
api-demo-no-beatmap-found = 未找到谱面
api-demo-beatmap-number = --- 谱面 #{$number} ---
api-demo-title = 标题: {$title}
api-demo-artist = 艺术家: {$artist}
api-demo-version = 难度名: {$version}
api-demo-bpm = BPM: {$bpm}
api-demo-stars = 星级: {$stars}
api-demo-error = 错误: {$error}
api-demo-no-user-found = 未找到用户
api-demo-username = 用户名: {$username}
api-demo-user-id = 用户 ID: {$id}
api-demo-country = 国家: {$country}
api-demo-pp = PP: {$pp}
api-demo-accuracy = 准确度: {$accuracy}%
api-demo-global-rank = 全球排名: #{$rank}
api-demo-country-rank = 国家排名: #{$country_rank}
api-demo-playcount = 游戏次数: {$count}

# 用户最近成绩演示翻译
api-demo-no-recent-found = 未找到该用户的最近游玩记录
api-demo-mode-label = 游戏模式
api-demo-mode-osu = osu! (标准模式)
api-demo-mode-taiko = 太鼓模式
api-demo-mode-ctb = 接水果模式
api-demo-mode-mania = osu!mania (下落式)
api-demo-limit-label = 结果数量限制 (1-50)
api-demo-limit-placeholder = 输入结果数量（默认：10）
api-demo-search-recent = 搜索最近成绩

# 最近游玩结果显示
api-demo-recent-play = 第 {$index} 个成绩
api-demo-beatmap-id = 谱面 ID：{$id}
api-demo-score = 得分：{$score}
api-demo-combo = 最大连击：{$combo}x
api-demo-accuracy-hits = 命中：{$c300}×300 / {$c100}×100 / {$c50}×50 / {$miss}×Miss
api-demo-rank = 评级：{$rank}
api-demo-mods = 模组：{$mods}
api-demo-date = 日期：{$date}
api-demo-perfect = 全连击：{$perfect}
api-demo-user-recent-query = 用户最近成绩查询

# 用户最佳成绩查询
api-demo-best-play = 用户最佳成绩
api-demo-no-best-found = 未找到该用户的最佳成绩
api-demo-user-best-query = 用户最佳成绩查询

# Scores Demo
api-demo-scores-query = 分数查询
api-demo-no-scores-found = 未找到此谱面的分数记录
api-demo-score-number = 分数 #{$number}
api-demo-replay = 回放: {$replay}
api-demo-input-user-label-optional = 用户名（可选）
api-demo-input-user-placeholder-optional = 留空以获取所有分数

api-docs-label = osu! API 文档