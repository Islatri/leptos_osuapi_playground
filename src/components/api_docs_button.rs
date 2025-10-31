use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::*;
use leptos_fluent::tr;

#[component]
pub fn ApiDocsButton() -> impl IntoView {
    view! {
        <div class="inline-flex items-stretch rounded-lg overflow-hidden shadow-lg shadow-gray-900/30 border border-gray-700 dark:border-gray-600 bg-gray-800/60 dark:bg-gray-800/40 backdrop-blur-sm p-1.5">
            // 左侧标签部分
            <div class="px-3 py-2.5 text-white font-medium flex items-center gap-2 mr-2 whitespace-nowrap">
                <a href="https://osu.ppy.sh"
                    target="_blank"
                    class="flex-shrink-0 w-6 h-6"
                    >
                    <img
                        src="/public/Osu!_Logo_2016.svg"
                        alt="osu! logo"
                        class="w-full h-full drop-shadow-[0_0_8px_rgba(236,72,153,0.5)] dark:drop-shadow-[0_0_10px_rgba(236,72,153,0.6)] hover:scale-110 transition-transform duration-300"
                    />
                </a>
                <span>{move || tr!("api-docs-label")}</span>
            </div>
            
            // 右侧分割按钮组 - 嵌套的小按钮
            <div class="inline-flex rounded-md overflow-hidden border border-gray-600/50 dark:border-gray-500/40">
                // V1 按钮
                <a 
                    href="https://github.com/ppy/osu-api/wiki"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="group relative px-4 py-2 bg-white/5 hover:bg-gradient-to-br hover:from-indigo-500 hover:to-purple-600 text-gray-300 hover:text-white font-bold transition-all duration-300 focus:ring-2 focus:ring-purple-400 focus:ring-offset-2 focus:ring-offset-gray-900 focus:outline-none cursor-pointer min-w-[50px] flex items-center justify-center backdrop-blur-sm"
                >
                    <span class="text-sm">V1</span>
                    
                    // 悬停光效
                    <div class="absolute inset-0 bg-gradient-to-t from-white/0 to-white/10 opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
                </a>
                
                // 分割线
                <div class="w-px bg-gray-600/50 dark:bg-gray-500/40"></div>
                
                // V2 按钮
                <a 
                    href="https://osu.ppy.sh/docs/index.html"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="group relative px-4 py-2 bg-white/5 hover:bg-gradient-to-br hover:from-pink-500 hover:to-rose-600 text-gray-300 hover:text-white font-bold transition-all duration-300 focus:ring-2 focus:ring-pink-400 focus:ring-offset-2 focus:ring-offset-gray-900 focus:outline-none cursor-pointer min-w-[50px] flex items-center justify-center backdrop-blur-sm"
                >
                    <span class="text-sm">V2</span>
                    
                    // 悬停光效
                    <div class="absolute inset-0 bg-gradient-to-t from-white/0 to-white/10 opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
                </a>
            </div>
        </div>
    }
}