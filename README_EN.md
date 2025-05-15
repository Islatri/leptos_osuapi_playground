<!-- markdownlint-disable MD033 MD041 MD045 MD026 -->
<p align="center" dir="auto">
    <img style="height:720px;width:581px" src="https://s2.loli.net/2025/05/15/qlgvPVuZhM5Gcod.png" alt="Logo escaped~"/>
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
    Beautiful, modern OsynicOsuapi demo website, developed with the Leptos framework
</p>

<hr />

[中文版本](README.md) | [English Version](README_EN.md)

# 🎮 LeptosOsuapiPlayground

## 📋 Project Introduction

LeptosOsuapiPlayground is a modern web application developed using the Rust Leptos framework, showcasing the features and usage of the OsynicOsuapi library. Through WebAssembly (WASM) technology, OsynicOsuapi can interact directly with the osu! API in the browser without requiring a backend server to process requests.

Obviously, due to CORS issues, directly using the V1 API in the browser without a proxy would encounter cross-origin problems (since WASM requests are sent from the browser frontend). Therefore, a proxy server [osynic-cors.deno.dev](https://osynic-cors.deno.dev) has been set up using [Deno](https://deno.dev) to work with the WASM client's `proxy_url` feature to enable proxied requests.

Currently, the website is deployed on [Deno](https://deno.dev) at [osynic-osuapi.deno.dev](https://osynic-osuapi.deno.dev/), supporting multiple languages including Chinese, Japanese, Korean, German, French, Russian, English, and more.

This project serves not only as a practical example of the OsynicOsuapi library but also as an excellent resource for learning Rust, Leptos, and WASM technologies.

## ✨ Features

- **🚀 High Performance**: Built on Rust, providing exceptional performance and memory safety
- **🛡️ Type Safety**: Leveraging Rust's strong type system to catch errors at compile time
- **🌐 WASM Compatible**: Runs directly in the browser without requiring a backend service
- **📊 Complete API Support**: Supports most endpoints from both osu! API v1 and v2
- **📱 Responsive Design**: Beautiful interface that adapts to various screen sizes
- **⚡ Live Demonstrations**: Provides real-time API request and response demonstrations

## 🚀 Quick Start

### 🌐 Online Experience

Visit [https://osynic-osuapi.deno.dev](https://osynic-osuapi.deno.dev) to immediately experience the LeptosOsuapiPlayground.

You'll need your own osu! API key to use the demo features. If you don't have one, you can apply for one in the "Legacy API" section of your [osu! personal settings page](https://osu.ppy.sh/home/account/edit).

### 💻 Local Setup

```bash
# Clone the repository
git clone https://github.com/Islatri/leptos_osuapi_playground.git
cd leptos_osuapi_playground

# Install dependencies (requires Rust and Cargo)
cargo install trunk
rustup target add wasm32-unknown-unknown

# Start the development server
trunk serve
```

Then visit `http://localhost:1420` in your browser.

## 📂 Project Structure

```bash
leptos_osuapi_playground/
├── src/                    # Source code directory
│   ├── components/         # UI components
│   ├── layouts/            # Layout components
│   ├── stores/             # State management
│   ├── i18n.rs             # Internationalization file
│   ├── main.rs             # Application entry
│   ├── index.css           # Global styles
│   └── app.rs              # Main application component
├── public/                 # Static resources
├── locales/                # Translation files (.ftl)
├── tailwind.config.js      # Tailwind CSS configuration
├── Cargo.toml              # Project configuration
├── Trunk.toml              # Trunk configuration
├── index.html              # HTML entry
└── README.md               # Project documentation
```

## 🔍 API Demo Features

LeptosOsuapiPlayground provides two main API query features:

### 🎵 Beatmap Queries

Enter a beatmap set ID to get detailed information about that beatmap set, including:

- Title and artist
- Difficulty names and star ratings
- Technical information such as BPM

### 👤 User Queries

Enter a username or user ID to get detailed information about that user, including:

- Username and user ID
- Country and ranking
- PP value and accuracy
- Play count and other statistics

## 🛠️ Local Development

### 📋 Prerequisites

- Rust toolchain (1.85+)
- trunk (for packaging WASM applications)
- wasm32-unknown-unknown target

### ⚙️ Environment Setup

```bash
rustup default stable
rustup target add wasm32-unknown-unknown
cargo install trunk
```

### 🔧 Development Mode

```bash
trunk serve --open
```

### 📦 Build Production Version

```bash
trunk build --release
```

## 💻 Technology Stack

- [🦀 Rust](https://www.rust-lang.org/) - Systems programming language
- [🔄 Leptos](https://github.com/leptos-rs/leptos) - Rust frontend framework
- [⚡ WebAssembly (WASM)](https://webassembly.org/) - Binary format that runs in browsers
- [🎮 OsynicOsuapi](https://github.com/osynicite/osynic_osuapi) - Rust osu! API client
- [🌐 LeptosFluent](https://github.com/mondeja/leptos-fluent) - Internationalization framework

## 🤝 Contribution Guidelines

We welcome contributions of all kinds! Please follow these steps to contribute:

1. Fork this repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details

## ❤️ Acknowledgements

- [osu!](https://osu.ppy.sh/) for providing the amazing game and API
- Rust and Leptos communities for their continued support
- All developers who have contributed to the project

## ⚠️ Disclaimer

LeptosOsuapiPlayground is an unofficial project not affiliated with osu! official. This project is for learning and demonstration purposes only.

---

<div align="center">
  <sub>Built with ❤️ by Osynicite</sub>
  <br>
  <sub>© 2025 Osynicite. OsynicOsuapi and LeptosOsuapiPlayground are unofficial projects, not affiliated with osu! official.</sub>
</div>
