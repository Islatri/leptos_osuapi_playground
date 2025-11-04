use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::*;
use leptos_fluent::tr;
use lucide_leptos::BookOpen;

#[component]
pub fn ApiDocsButton() -> impl IntoView {
    view! {
        <div class="inline-flex items-center p-1.5 font-medium text-white bg-gradient-to-l from-purple-500 to-pink-500 rounded-lg shadow-lg transition-all duration-300 transform cursor-pointer hover:from-purple-600 hover:to-pink-600 hover:shadow-2xl hover:scale-105 focus:ring-2 focus:ring-purple-400 focus:ring-offset-2 focus:ring-offset-gray-900 focus:outline-none shadow-purple-500/25">
            // 左侧标签部分
            <div class="flex gap-2 items-center py-2.5 px-3 mr-2 font-medium text-white whitespace-nowrap">
                <div class="flex-shrink-0 w-6 h-6">
                    <BookOpen size=20 />
                </div>
                <span>{move || tr!("hero-button-docs")}</span>
            </div>

            // 右侧分割按钮组
            <div class="inline-flex overflow-hidden rounded-md border border-white/25">
                // NPM 按钮
                <a
                    href="https://www.npmjs.com/package/@osynicite/osynic-osuapi"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="flex relative justify-center items-center py-2 px-4 font-bold transition-all duration-300 cursor-pointer hover:text-white hover:bg-gradient-to-br focus:ring-2 focus:ring-purple-400 focus:ring-offset-2 focus:ring-offset-gray-900 focus:outline-none text-white/85 group bg-white/12 min-w-[60px] backdrop-blur-sm hover:from-purple-400/50 hover:to-pink-500/50"
                >
                    <span class="text-sm font-semibold">NPM</span>

                    // 悬停光效
                    <div class="absolute inset-0 bg-gradient-to-t rounded-md opacity-0 transition-opacity duration-300 pointer-events-none group-hover:opacity-100 from-white/0 via-white/8 to-white/0"></div>
                </a>

                // 分割线
                <div class="w-px bg-white/20"></div>

                // Crates 按钮
                <a
                    href="https://crates.io/crates/osynic-osuapi"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="flex relative justify-center items-center py-2 px-4 font-bold transition-all duration-300 cursor-pointer hover:text-white hover:bg-gradient-to-br focus:ring-2 focus:ring-purple-400 focus:ring-offset-2 focus:ring-offset-gray-900 focus:outline-none text-white/85 group bg-white/12 min-w-[60px] backdrop-blur-sm hover:from-purple-400/50 hover:to-pink-500/50"
                >
                    <span class="text-sm font-semibold">Crates</span>

                    // 悬停光效
                    <div class="absolute inset-0 bg-gradient-to-t rounded-md opacity-0 transition-opacity duration-300 pointer-events-none group-hover:opacity-100 from-white/0 via-white/8 to-white/0"></div>
                </a>
            </div>
        </div>
    }
}
