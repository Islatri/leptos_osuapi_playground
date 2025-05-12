use leptos::*;
use leptos::prelude::{ClassAttribute,ElementChild};

use crate::components::{
    hero_section::HeroSection,
    api_section::ApiSection,
    api_demo::ApiDemo,
};

use crate::layouts::{
    header::Header,
    footer::Footer,
};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="min-h-screen flex flex-col">
            <Header />
            
            <main class="flex-grow">
                <HomePage />
            </main>
            
            <Footer />
        </div>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <HeroSection />
        <ApiSection />
        <ApiDemo />
        
        // 特性介绍部分
        <section class="py-16 bg-gray-100 dark:bg-gray-800">
            <div class="container mx-auto px-4">
                <h2 class="text-3xl font-bold text-center mb-12">为什么选择 osynic_osuapi?</h2>
                
                <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
                    // 特性卡片 1
                    <div class="card">
                        <div class="flex items-center mb-4">
                            <span class="text-osu-pink mr-3">
                                <i class="lucide-bolt text-2xl"></i>
                            </span>
                            <h3 class="text-xl font-semibold">高性能</h3>
                        </div>
                        <p class="text-gray-600 dark:text-gray-300">
                            "基于 Rust 构建，提供卓越的性能和内存安全性，适合高吞吐量应用。"
                        </p>
                    </div>
                    
                    // 特性卡片 2
                    <div class="card">
                        <div class="flex items-center mb-4">
                            <span class="text-osu-purple mr-3">
                                <i class="lucide-code text-2xl"></i>
                            </span>
                            <h3 class="text-xl font-semibold">类型安全</h3>
                        </div>
                        <p class="text-gray-600 dark:text-gray-300">
                            "利用 Rust 的强类型系统，在编译时捕获错误，提供完整的 API 模型。"
                        </p>
                    </div>
                    
                    // 特性卡片 3
                    <div class="card">
                        <div class="flex items-center mb-4">
                            <span class="text-osu-blue mr-3">
                                <i class="lucide-layers text-2xl"></i>
                            </span>
                            <h3 class="text-xl font-semibold">多平台</h3>
                        </div>
                        <p class="text-gray-600 dark:text-gray-300">
                            "支持 WASM、native 和服务器环境，让您能在任何地方使用相同的 API。"
                        </p>
                    </div>
                </div>
            </div>
        </section>
        
        // 使用指南部分
        <section class="py-16">
            <div class="container mx-auto px-4">
                <h2 class="text-3xl font-bold text-center mb-12">快速开始</h2>
                
                <div class="max-w-3xl mx-auto bg-gray-900 rounded-lg overflow-hidden">
                    <div class="flex items-center px-4 py-2 bg-gray-800">
                        <div class="flex space-x-2">
                            <div class="w-3 h-3 bg-red-500 rounded-full"></div>
                            <div class="w-3 h-3 bg-yellow-500 rounded-full"></div>
                            <div class="w-3 h-3 bg-green-500 rounded-full"></div>
                        </div>
                        <p class="ml-4 text-gray-400 text-sm">Cargo.toml</p>
                    </div>
                    
                    <pre class="p-4 text-sm overflow-x-auto text-gray-300">
                        <code>
{r#"[dependencies]
osynic-osuapi = "0.1.0"  # 替换为最新版本

# WASM 支持
wasm-bindgen = "0.2"
wasm-bindgen-futures = "0.4"
web-sys = { version = "0.3", features = ["console"] }"#}
                        </code>
                    </pre>
                </div>
                
                <div class="max-w-3xl mx-auto bg-gray-900 rounded-lg overflow-hidden mt-8">
                    <div class="flex items-center px-4 py-2 bg-gray-800">
                        <div class="flex space-x-2">
                            <div class="w-3 h-3 bg-red-500 rounded-full"></div>
                            <div class="w-3 h-3 bg-yellow-500 rounded-full"></div>
                            <div class="w-3 h-3 bg-green-500 rounded-full"></div>
                        </div>
                        <p class="ml-4 text-gray-400 text-sm">main.rs</p>
                    </div>
                    
                    <pre class="p-4 text-sm overflow-x-auto text-gray-300">
                        <code>
{r#"use wasm_bindgen_futures::spawn_local;
use osynic_osuapi::v1::client::gloo::client::OsynicOsuApiV1GlooClient;
use osynic_osuapi::v1::model::beatmap::GetBeatmapsParams;

// 在 WASM 环境中使用
fn main() {
    let client = OsynicOsuApiV1GlooClient::new("your_api_key".to_string());
    
    // 创建查询参数
    let params = GetBeatmapsParams::default()
        .sid("114514".to_string());
    
    // 在 WASM 中处理异步请求
    spawn_local(async move {
        match client.beatmap.get_beatmaps(params).await {
            Ok(beatmaps) => {
                // 处理返回的数据
                for beatmap in beatmaps {
                    web_sys::console::log_1(&format!("谱面: {}", beatmap.title).into());
                }
            },
            Err(e) => {
                web_sys::console::error_1(&format!("错误: {:?}", e).into());
            }
        }
    });
}"#}
                        </code>
                    </pre>
                </div>
                
                <div class="text-center mt-10">
                    <a href="#" class="btn-primary inline-flex items-center">
                        <i class="lucide-book mr-2"></i>
                        查看完整文档
                    </a>
                </div>
            </div>
        </section>
    }
}