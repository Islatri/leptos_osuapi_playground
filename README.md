<!-- markdownlint-disable MD033 MD041 MD045 MD026 -->
# 🎮 LeptosOsuapiPlayground

<div align="center">

<img src="https://github.com/Islatri/LeptosOsuapiPlayground/raw/main/public/logo.png" alt="LeptosOsuapiPlayground Logo" width="200"/>

<h3>高性能、结构优良、拓展性好的 Rust osu! API 客户端演示网站</h3>

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![Leptos](https://img.shields.io/badge/Leptos-最新版-blue.svg)](https://github.com/leptos-rs/leptos)
[![WASM](https://img.shields.io/badge/WASM-兼容-brightgreen.svg)](https://webassembly.org/)
[![osu!api](https://img.shields.io/badge/osu!api-v1%20%26%20v2-ff69b4.svg)](https://osu.ppy.sh/docs/index.html)

[🌐 体验演示](https://osuapi.osynic.com) | [📚 查看文档](https://docs.osuapi.osynic.com) | [🐛 报告问题](https://github.com/Islatri/LeptosOsuapiPlayground/issues)

</div>

## 📋 项目简介

LeptosOsuapiPlayground 是一个基于 Rust Leptos 框架开发的现代化 Web 应用，它展示了 OsynicOsuapi 库的功能和用法。通过 WebAssembly (WASM) 技术，该应用能够直接在浏览器中与 osu! API 进行交互，无需后端服务器处理请求。

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

访问 [https://osuapi.osynic.com](https://osuapi.osynic.com) 立即体验 LeptosOsuapiPlayground。

您需要自己的 osu! API 密钥才能使用演示功能。如果没有，可以在 [osu! 个人设置页](https://osu.ppy.sh/home/account/edit) 的"旧版本API"部分申请一个。

### 💻 本地运行

```bash
# 克隆仓库
git clone https://github.com/Islatri/LeptosOsuapiPlayground.git
cd LeptosOsuapiPlayground

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
│   └── app.rs              # 主应用组件
├── public/                 # 静态资源
├── i18n/                   # 翻译文件 (.ftl)
├── Cargo.toml              # 项目配置
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
- [🎮 OsynicOsuapi](https://github.com/Islatri/OsynicOsuapi) - Rust osu! API 客户端
- [🌐 Fluent](https://projectfluent.org/) - 国际化框架

## 🤝 贡献指南

我们欢迎各种形式的贡献！请按照以下步骤参与：

1. Fork 本仓库
2. 创建您的特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交您的更改 (`git commit -m 'Add some amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 开启一个 Pull Request

## 📜 许可证

本项目采用 MIT 许可证 - 详情请参阅 [LICENSE](LICENSE) 文件

## 🙏 致谢

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
