use leptos::*;
use leptos::prelude::{ClassAttribute, ElementChild};
use lucide_leptos::{BookOpen,  Play, Package, ShieldCheck, Globe};

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
                <div class="flex flex-col lg:flex-row lg:items-center lg:justify-between">
                    <div class="max-w-3xl">
                        <h1 class="text-4xl md:text-6xl font-bold mb-6">
                            <span class="text-pink-400 text-6xl md:text-8xl inline-block mb-8 animate-pulse-subtle">OsynicOsuapi</span><br />
                            <span class="text-transparent bg-clip-text bg-gradient-to-r from-pink-400 to-purple-400">
                                高性能, 结构优良, 拓展性好的 Rust osu! API 客户端
                            </span>
                        </h1>
                        
                        <p class="text-xl md:text-2xl mb-8 text-gray-200 leading-relaxed">
                            类型安全, 异步友好, WASM 兼容的 API 客户端,
                            让您的 Rust 项目与 osu! 无缝衔接.
                        </p>
                        
                        <div class="flex flex-col sm:flex-row space-y-4 sm:space-y-0 sm:space-x-4">
                            <a href="#" class="btn-primary flex group transition-all duration-300 transform hover:scale-105">
                                <div class="mr-2 group-hover:animate-bounce-subtle">
                                    <BookOpen size=24 />
                                </div>
                                查看文档
                            </a>
                            <a href="#demo" class="btn-secondary flex group transition-all duration-300 transform hover:scale-105">
                                <div class="mr-2 group-hover:animate-bounce-subtle">
                                    <Play size=24 />
                                </div>
                                尝试演示
                            </a>
                        </div>
                        
                        <div class="mt-12 grid grid-cols-1 md:grid-cols-3 gap-4 text-sm text-gray-300">
                            <div class="flex items-center p-3 bg-gray-800/50 rounded-lg backdrop-blur-sm hover:bg-gray-700/50 transition-all">
                                <div class="text-pink-400 mr-2">
                                    <Package size=24 color="#f472b6" />
                                </div>
                                <span>支持 v1 和 v2 API</span>
                            </div>
                            <div class="flex items-center p-3 bg-gray-800/50 rounded-lg backdrop-blur-sm hover:bg-gray-700/50 transition-all">
                                <div class="text-pink-400 mr-2">
                                    <ShieldCheck size=24 color="#f472b6" />
                                </div>
                                <span>类型安全</span>
                            </div>
                            <div class="flex items-center p-3 bg-gray-800/50 rounded-lg backdrop-blur-sm hover:bg-gray-700/50 transition-all">
                                <div class="text-pink-400 mr-2">
                                    <Globe size=24 color="#f472b6" />
                                </div>
                                <span>WASM 兼容</span>
                            </div>
                        </div>
                    </div>
                    
                    // 修改: 使SVG在移动端也能显示
                    <div class="mt-16 lg:mt-0 flex justify-center lg:block transform hover:scale-105 transition-transform duration-500">
                        <div class="relative w-48 h-48 md:w-64 md:h-64 lg:w-80 lg:h-80 animate-float">
                            // 添加一点装饰性的光效
                            <div class="absolute -inset-4 bg-gradient-to-r from-pink-500/30 to-purple-500/30 rounded-full blur-xl"></div>
                            
                            // 这里放置您的osu SVG public/Osu!_Logo_2016.svg
                            <img 
                                src="/public/Osu!_Logo_2016.svg" 
                                alt="osu! logo" 
                                class="w-full h-full drop-shadow-[0_0_15px_rgba(236,72,153,0.5)]"
                            />
                        </div>
                    </div>
                </div>
            </div>
            
        </section>
    }
}