<!-- markdownlint-disable MD033 MD041 MD045 MD026 -->
<p align="center" dir="auto">
    <img style="height:240px;width:240px" src="https://s2.loli.net/2025/05/15/Ww1hovEL4PmKdD6.png" alt="Logo逃走啦~"/>
</p>

<h1 align="center" tabindex="-1" class="heading-element" dir="auto">LeptosOsuapiPlayground</h1>

<p align="center">
  <a href="https://www.rust-lang.org/" target="_blank"><img src="https://img.shields.io/badge/Rust-1.85%2B-blue"/></a>
  <a href="https://crates.io/crates/osynic_osuapi" target="_blank"><img src="https://img.shields.io/crates/v/osynic_osuapi"/></a>
  <a href="https://docs.rs/osynic_osuapi" target="_blank"><img src="https://img.shields.io/docsrs/osynic_osuapi/0.1.0"/></a>
  <a href="https://osynic-osuapi.deno.dev" target="_blank"><img src="https://img.shields.io/badge/Deno-white?logo=deno&logoColor=black"/></a>
  <a href="https://github.com/islatri/leptos_osuapi_playground" target="_blank"><img src="https://img.shields.io/badge/License-MIT-green.svg"/></a>
  <a href="https://discord.gg/DRnZSES3BC" target="_blank"><img src="https://img.shields.io/badge/chat-discord-7289da.svg"/></a>
  <a href="https://github.com/osynicite" target="_blank"><img src="https://img.shields.io/badge/buy%20me-a%20coffee-orange.svg?style=flat-square"/></a>

</p>

<p align="center">
    美观、现代化的 OsynicOsuapi 演示网站, 基于Leptos框架开发
</p>

<hr />

[中文版本](README.md) | [English Version](README_EN.md)

# 🎮 LeptosOsuapiPlayground

## 📋 项目简介

LeptosOsuapiPlayground 是一个基于 Rust Leptos 框架开发的现代化 Web 应用，它展示了 OsynicOsuapi 库的功能和用法。通过 WebAssembly (WASM) 技术，OsynicOsuapi 能够直接在浏览器中与 osu! API 进行交互，无需后端服务器处理请求。

很显然，由于CORS的问题，不代理直接在浏览器中使用V1的API会遇到跨域问题（毕竟WASM部分是浏览器前端发的请求嘛），所以用[Deno](https://deno.dev)来搭建了一个中转服务器[osynic-cors.deno.dev](https://osynic-cors.deno.dev)，配合WASM客户端的`proxy_url`来实现代理请求；

目前网站通过[Deno](https://deno.dev)部署在[osynic-osuapi.deno.dev](https://osynic-osuapi.deno.dev/)，支持中日韩德法俄英等多种语言；

这个项目不仅是 OsynicOsuapi 库的一个实际应用示例，也是学习 Rust、Leptos 和 WASM 技术的绝佳资源。

## ✨ 特性

- **🚀 高性能**：基于 Rust 构建，提供卓越的性能和内存安全性
- **🛡️ 类型安全**：利用 Rust 的强类型系统，在编译时捕获错误
- **🌐 WASM 兼容**：直接在浏览器中运行，无需后端服务
- **📊 完整 API 支持**：支持 osu! API v1 和 v2 的大多数端点
- **📱 响应式设计**：适配各种屏幕尺寸的美观界面
- **⚡ 实时演示**：提供实时 API 请求和响应展示

## 🚀 快速开始

### 🌐 在线体验

访问 [https://osynic-osuapi.deno.dev](https://osynic-osuapi.deno.dev) 立即体验 LeptosOsuapiPlayground。

您需要自己的 osu! API 密钥才能使用演示功能。如果没有，可以在 [osu! 个人设置页](https://osu.ppy.sh/home/account/edit) 的"旧版本API"部分申请一个。

### 💻 本地运行

```bash
# 克隆仓库
git clone https://github.com/Islatri/leptos_osuapi_playground.git
cd leptos_osuapi_playground

# 安装依赖（需要 Rust 和 Cargo）
cargo install trunk
rustup target add wasm32-unknown-unknown

# 启动开发服务器
trunk serve
```

然后在浏览器中访问 `http://localhost:1420` 即可。

## 📂 项目结构

```bash
LeptosOsuapiPlayground/
├── src/                    # 源代码目录
│   ├── components/         # UI 组件
│   ├── layouts/            # 布局组件
│   ├── stores/             # 状态管理
│   ├── i18n.rs             # 国际化文件
│   ├── main.rs             # 应用入口
│   ├── index.css           # 全局样式
│   └── app.rs              # 主应用组件
├── public/                 # 静态资源
├── locales/                # 翻译文件 (.ftl)
├── tailwind.config.js      # Tailwind CSS 配置
├── Cargo.toml              # 项目配置
├── Trunk.toml              # Trunk 配置
├── index.html              # HTML 入口
└── README.md               # 项目说明
```

## 🔍 API 演示功能

LeptosOsuapiPlayground 提供了两种主要的 API 查询功能：

### 🎵 谱面查询

输入谱面集 ID，获取该谱面集的详细信息，包括：

- 标题和艺术家
- 难度名称和星级
- BPM 等技术信息

### 👤 用户查询

输入用户名或用户 ID，获取该用户的详细信息，包括：

- 用户名和用户 ID
- 国家和排名
- PP 值和准确度
- 游戏次数等统计数据

## 🛠️ 本地开发

### 📋 前提条件

- Rust 工具链 (1.70+)
- trunk (用于打包 WASM 应用)
- wasm32-unknown-unknown 目标

### ⚙️ 环境设置

```bash
rustup default stable
rustup target add wasm32-unknown-unknown
cargo install trunk
```

### 🔧 开发模式

```bash
trunk serve --open
```

### 📦 构建生产版本

```bash
trunk build --release
```

## 💻 技术栈

- [🦀 Rust](https://www.rust-lang.org/) - 系统编程语言
- [🔄 Leptos](https://github.com/leptos-rs/leptos) - Rust 前端框架
- [⚡ WebAssembly (WASM)](https://webassembly.org/) - 浏览器中运行的二进制格式
- [🎮 OsynicOsuapi](https://github.com/osynicite/osynic_osuapi) - Rust osu! API 客户端
- [🌐 LeptosFluent](https://github.com/mondeja/leptos-fluent) - 国际化框架

## 🤝 贡献指南

我们欢迎各种形式的贡献！请按照以下步骤参与：

1. Fork 本仓库
2. 创建您的特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交您的更改 (`git commit -m 'Add some amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 开启一个 Pull Request

## 📜 许可证

本项目采用 MIT 许可证 - 详情请参阅 [LICENSE](LICENSE) 文件

## ❤️ 致谢

- [osu!](https://osu.ppy.sh/) 提供精彩的游戏和 API
- Rust 和 Leptos 社区的持续支持
- 所有为项目做出贡献的开发者

## ⚠️ 免责声明

LeptosOsuapiPlayground 是非官方项目，与 osu! 官方无关。本项目仅用于学习和展示目的。

---

<div align="center">
  <sub>由 Osynicite 用 ❤️ 构建</sub>
  <br>
  <sub>© 2025 Osynicite. OsynicOsuapi 和 LeptosOsuapiPlayground 是非官方项目, 与 osu! 官方无关.</sub>
</div>
