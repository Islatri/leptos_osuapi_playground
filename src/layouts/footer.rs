use leptos::*;
use leptos::prelude::{ClassAttribute,ElementChild};

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="bg-gray-800 text-white py-12">
            <div class="container mx-auto px-4">
                <div class="grid grid-cols-1 md:grid-cols-4 gap-8">
                    // 项目信息
                    <div class="col-span-1 md:col-span-2">
                        <div class="flex items-center mb-4">
                            <img src="/public/osynic.png" alt="osynic Logo" class="h-8 w-8 mr-2" />
                            <span class="font-bold text-xl text-pink-400">osynic_osuapi</span>
                        </div>
                        <p class="text-gray-400 mb-4">
                            高性能, 类型安全的 Rust 实现 osu! API 客户端,
                            支持 WASM 和 native 环境.
                        </p>
                        <div class="flex space-x-4">
                            <a href="https://github.com/osynicite/osynic_osuapi" class="text-gray-400 hover:text-white transition">
                                <i class="lucide-github text-xl"></i>
                            </a>
                            <a href="#" class="text-gray-400 hover:text-white transition">
                                <i class="lucide-twitter text-xl"></i>
                            </a>
                            <a href="#" class="text-gray-400 hover:text-white transition">
                                <i class="lucide-mail text-xl"></i>
                            </a>
                        </div>
                    </div>
                    
                    // 链接列表 1
                    <div>
                        <h3 class="font-semibold text-lg mb-4">文档</h3>
                        <ul class="space-y-2">
                            <li><a href="#" class="text-gray-400 hover:text-white transition">入门指南</a></li>
                            <li><a href="#" class="text-gray-400 hover:text-white transition">API 参考</a></li>
                            <li><a href="#" class="text-gray-400 hover:text-white transition">示例</a></li>
                            <li><a href="#" class="text-gray-400 hover:text-white transition">WASM 使用</a></li>
                        </ul>
                    </div>
                    
                    // 链接列表 2
                    <div>
                        <h3 class="font-semibold text-lg mb-4">资源</h3>
                        <ul class="space-y-2">
                            <li><a href="https://osu.ppy.sh/docs/index.html" class="text-gray-400 hover:text-white transition">osu! API 文档</a></li>
                            <li><a href="https://leptos.dev" class="text-gray-400 hover:text-white transition">Leptos 框架</a></li>
                            <li><a href="https://github.com/osynicite/osynic_osuapi/issues" class="text-gray-400 hover:text-white transition">问题反馈</a></li>
                            <li><a href="#" class="text-gray-400 hover:text-white transition">更新日志</a></li>
                        </ul>
                    </div>
                </div>
                
                <div class="border-t border-gray-700 mt-8 pt-8 flex flex-col md:flex-row justify-between items-center">
                    <p class="text-gray-400 text-sm">&copy; {2025} Osynicite. osynic_osuapi 是非官方项目, 与 osu! 官方无关.</p>
                    <div class="mt-4 md:mt-0">
                        <a href="#" class="text-gray-400 hover:text-white transition text-sm mr-4">使用条款</a>
                        <a href="#" class="text-gray-400 hover:text-white transition text-sm">隐私政策</a>
                    </div>
                </div>
            </div>
        </footer>
    }
}