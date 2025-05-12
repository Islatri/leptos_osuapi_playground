use leptos::*;
use leptos::prelude::{ClassAttribute,ElementChild};

#[component]
pub fn HeroSection() -> impl IntoView {
    view! {
        <section class="relative bg-gray-900 text-white overflow-hidden">
            // 背景图片，暗化处理
            <div class="absolute inset-0 z-0">
                <div class="absolute inset-0 bg-gradient-to-r from-pink-600/70 to-purple-900/70 mix-blend-multiply"></div>
                <img 
                    src="/public/irena.jpg" 
                    alt="osu! background" 
                    class="w-full h-full object-cover opacity-30"
                />
            </div>
            
            <div class="container mx-auto px-4 py-20 md:py-32 relative z-10">
                <div class="max-w-3xl">
                    <h1 class="text-4xl md:text-6xl font-bold mb-6">
                        <span class="text-pink-400">osynic_osuapi</span><br />
                        <span class="text-transparent bg-clip-text bg-gradient-to-r from-pink-400 to-purple-400">
                            高性能 Rust osu! API 客户端
                        </span>
                    </h1>
                    
                    <p class="text-xl md:text-2xl mb-8 text-gray-200">
                        类型安全, 异步友好, WASM 兼容的 API 客户端,
                        让您的 Rust 项目与 osu! 无缝衔接.
                    </p>
                    
                    <div class="flex flex-col sm:flex-row space-y-4 sm:space-y-0 sm:space-x-4">
                        <a href="#" class="btn-primary">
                            <i class="lucide-book mr-2"></i>
                            查看文档
                        </a>
                        <a href="#demo" class="btn-secondary">
                            <i class="lucide-play mr-2"></i>
                            尝试演示
                        </a>
                    </div>
                    
                    <div class="mt-12 text-sm text-gray-300 flex items-center">
                        <div class="flex items-center mr-6">
                            <i class="lucide-package text-pink-400 mr-2"></i>
                            <span>支持 v1 和 v2 API</span>
                        </div>
                        <div class="flex items-center mr-6">
                            <i class="lucide-verified text-pink-400 mr-2"></i>
                            <span>类型安全</span>
                        </div>
                        <div class="flex items-center">
                            <i class="lucide-globe text-pink-400 mr-2"></i>
                            <span>WASM 兼容</span>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}