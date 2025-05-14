use leptos::*;
use leptos::prelude::{ClassAttribute, ElementChild};
use lucide_leptos::{Layers, CodeXml, Sailboat, Globe, Server, BookOpen};

#[component]
pub fn FeaturesSection() -> impl IntoView {
    view! {
        // 特性介绍部分
        <section class="py-20 bg-gradient-to-b from-gray-100 to-white dark:from-gray-800 dark:to-gray-900">
            <div class="container mx-auto px-4">
                <h2 class="text-4xl font-bold text-center mb-4 text-gray-800 dark:text-gray-200">为什么选择 OsynicOsuapi?</h2>
                <p class="text-center text-gray-600 dark:text-gray-300 mb-16 max-w-3xl mx-auto">
                    "一个高性能、类型安全的 osu! API 库，为各种平台提供完整的开发体验"
                </p>
                
                <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
                    // 特性卡片 1
                    <div class="card bg-white dark:bg-gray-700 rounded-xl shadow-lg hover:shadow-xl transition-shadow p-6 border-t-4 border-pink-500">
                        <div class="flex items-center mb-4">
                            <span class="text-pink-500 mr-3 text-2xl bg-pink-50 dark:bg-pink-900/30 p-3 rounded-lg">
                                <Sailboat />
                            </span>
                            <h3 class="text-xl font-semibold text-gray-800 dark:text-gray-200">高性能</h3>
                        </div>
                        <p class="text-gray-600 dark:text-gray-300">
                            "基于 Rust 构建，提供卓越的性能和内存安全性，适合高吞吐量应用。无论是服务器还是客户端应用，都能获得最佳性能体验。"
                        </p>
                    </div>
                    
                    // 特性卡片 2
                    <div class="card bg-white dark:bg-gray-700 rounded-xl shadow-lg hover:shadow-xl transition-shadow p-6 border-t-4 border-purple-600">
                        <div class="flex items-center mb-4">
                            <span class="text-purple-600 mr-3 text-2xl bg-purple-50 dark:bg-purple-900/30 p-3 rounded-lg">
                                <CodeXml />
                            </span>
                            <h3 class="text-xl font-semibold text-gray-800 dark:text-gray-200">类型安全</h3>
                        </div>
                        <p class="text-gray-600 dark:text-gray-300">
                            "利用 Rust 的强类型系统，在编译时捕获错误，提供完整的 API 模型。基于 client、interface、model 三重模块划分，结构清晰易于理解。"
                        </p>
                    </div>
                    
                    // 特性卡片 3
                    <div class="card bg-white dark:bg-gray-700 rounded-xl shadow-lg hover:shadow-xl transition-shadow p-6 border-t-4 border-blue-500">
                        <div class="flex items-center mb-4">
                            <span class="text-blue-500 mr-3 text-2xl bg-blue-50 dark:bg-blue-900/30 p-3 rounded-lg">
                                <Layers />
                            </span>
                            <h3 class="text-xl font-semibold text-gray-800 dark:text-gray-200">多平台</h3>
                        </div>
                        <p class="text-gray-600 dark:text-gray-300">
                            "支持 WASM、native 和服务器环境，让您能在任何地方使用相同的 API。为 V1 接口提供了 WebAssembly 支持，可直接从网页应用访问 OSU API。"
                        </p>
                    </div>

                    // 特性卡片 4 - API 支持
                    <div class="card bg-white dark:bg-gray-700 rounded-xl shadow-lg hover:shadow-xl transition-shadow p-6 border-t-4 border-green-500">
                        <div class="flex items-center mb-4">
                            <span class="text-green-500 mr-3 text-2xl bg-green-50 dark:bg-green-900/30 p-3 rounded-lg">
                                <Globe />
                            </span>
                            <h3 class="text-xl font-semibold text-gray-800 dark:text-gray-200">完整 API 支持</h3>
                        </div>
                        <p class="text-gray-600 dark:text-gray-300">
                            "支持 OSU! V1 的所有端点以及 V2 的大部分端点（除了文档未归类的接口）。无论您的项目需要哪个版本的 API，都能轻松集成。"
                        </p>
                    </div>

                    // 特性卡片 5 - 项目结构
                    <div class="card bg-white dark:bg-gray-700 rounded-xl shadow-lg hover:shadow-xl transition-shadow p-6 border-t-4 border-amber-500">
                        <div class="flex items-center mb-4">
                            <span class="text-amber-500 mr-3 text-2xl bg-amber-50 dark:bg-amber-900/30 p-3 rounded-lg">
                                <Server />
                            </span>
                            <h3 class="text-xl font-semibold text-gray-800 dark:text-gray-200">灵活扩展</h3>
                        </div>
                        <p class="text-gray-600 dark:text-gray-300">
                            "client 部分聚合 interface 接口并支持多种 HTTP 客户端，便于拓展。模块化设计让您可以根据需求自定义实现，轻松适应不同的应用场景。"
                        </p>
                    </div>

                    // 特性卡片 6 - 学习示例
                    <div class="card bg-white dark:bg-gray-700 rounded-xl shadow-lg hover:shadow-xl transition-shadow p-6 border-t-4 border-yellow-500">
                        <div class="flex items-center mb-4">
                            <span class="text-yellow-500 mr-3 text-2xl bg-yellow-50 dark:bg-yellow-900/30 p-3 rounded-lg">
                                <BookOpen />
                            </span>
                            <h3 class="text-xl font-semibold text-gray-800 dark:text-gray-200">丰富示例</h3>
                        </div>
                        <p class="text-gray-600 dark:text-gray-300">
                            "提供完整的示例支持。运行 `cargo run --example 示例名` 查看返回数据，通过示例快速学习和上手。从示例代码中学习是掌握本库的最佳方式。"
                        </p>
                    </div>
                </div>

                <div class="mt-16 text-center">
                    <a 
                        href="https://crates.io/crates/osynic_osuapi" 
                        class="inline-block px-8 py-3 bg-pink-500 hover:bg-pink-600 text-white font-semibold rounded-lg shadow-md hover:shadow-lg transition-all transform hover:-translate-y-1"
                    >
                        "查看OsynicOsuapi的crates.io页面"
                    </a>
                </div>
            </div>
        </section>
        
    }
}