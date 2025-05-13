use leptos::*;
use leptos::prelude::{ClassAttribute,CustomAttribute,ElementChild};
use lucide_leptos::Copyright;

#[component]
pub fn Discord() -> impl IntoView {
    view! {
        <svg aria-hidden="true" role="img" xmlns="http://www.w3.org/2000/svg" width="30" height="30" fill="none" viewBox="0 0 24 24"><path fill="#5865f2" d="M19.73 4.87a18.2 18.2 0 0 0-4.6-1.44c-.21.4-.4.8-.58 1.21-1.69-.25-3.4-.25-5.1 0-.18-.41-.37-.82-.59-1.2-1.6.27-3.14.75-4.6 1.43A19.04 19.04 0 0 0 .96 17.7a18.43 18.43 0 0 0 5.63 2.87c.46-.62.86-1.28 1.2-1.98-.65-.25-1.29-.55-1.9-.92.17-.12.32-.24.47-.37 3.58 1.7 7.7 1.7 11.28 0l.46.37c-.6.36-1.25.67-1.9.92.35.7.75 1.35 1.2 1.98 2.03-.63 3.94-1.6 5.64-2.87.47-4.87-.78-9.09-3.3-12.83ZM8.3 15.12c-1.1 0-2-1.02-2-2.27 0-1.24.88-2.26 2-2.26s2.02 1.02 2 2.26c0 1.25-.89 2.27-2 2.27Zm7.4 0c-1.1 0-2-1.02-2-2.27 0-1.24.88-2.26 2-2.26s2.02 1.02 2 2.26c0 1.25-.88 2.27-2 2.27Z" class=""></path></svg>
    }
}

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="bg-gray-800 text-white py-12 z-20">
            <div class="container mx-auto px-4">
                <div class="grid grid-cols-1 md:grid-cols-4 gap-8">
                    // 项目信息
                    <div class="col-span-1 md:col-span-2">
                        <div class="flex items-center mb-4">
                            <img src="/public/osynic.png" alt="osynic Logo" class="h-8 w-8 mr-2" />
                            <span class="font-bold text-xl text-pink-400">OsynicOsuapi</span>
                        </div>
                        <p class="text-gray-400 mb-4">
                            高性能, 结构优良, 拓展性好的 Rust osu! API 客户端
                            支持 WASM 和 native 环境.
                        </p>
                        <div class="flex space-x-4">
                            <a href="#" class="text-xl text-gray-400 hover:text-white transition">
                                <Discord />
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
                            <li><a href="https://github.com/ppy/osu-api/wiki" class="text-gray-400 hover:text-white transition">osu! API V1 文档</a></li>
                            <li><a href="https://osu.ppy.sh/docs/index.html" class="text-gray-400 hover:text-white transition">osu! API V2 文档</a></li>
                            <li><a href="https://leptos.dev" class="text-gray-400 hover:text-white transition">Leptos 框架</a></li>
                            <li><a href="https://github.com/osynicite/osynic_osuapi/issues" class="text-gray-400 hover:text-white transition">问题反馈</a></li>
                            <li><a href="#" class="text-gray-400 hover:text-white transition">更新日志</a></li>
                        </ul>
                    </div>
                </div>
                
                <div class="border-t border-gray-700 mt-8 pt-8 flex flex-col md:flex-row justify-between items-center">
                    <p class="text-gray-400 text-sm flex"><Copyright /> {2025} Osynicite. OsynicOsuapi 是非官方项目, 与 osu! 官方无关.</p>
                    <div class="mt-4 md:mt-0">
                        <a href="#" class="text-gray-400 hover:text-white transition text-sm mr-4">使用条款</a>
                        <a href="#" class="text-gray-400 hover:text-white transition text-sm">隐私政策</a>
                    </div>
                </div>
            </div>
        </footer>
    }
}