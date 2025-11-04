use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::*;
use leptos_fluent::tr;

#[component]
pub fn OsuapiDocsButton() -> impl IntoView {
    view! {
        <div class="inline-flex overflow-hidden items-stretch p-1.5 rounded-lg border border-gray-700 shadow-lg dark:border-gray-600 shadow-gray-900/30 bg-gray-800/60 backdrop-blur-sm dark:bg-gray-800/40">
            // 左侧标签部分
            <div class="flex gap-2 items-center py-2.5 px-3 mr-2 font-medium text-white whitespace-nowrap">
                <a href="https://osu.ppy.sh" target="_blank" class="flex-shrink-0 w-6 h-6">
                    <img
                        src="/public/Osu!_Logo_2016.svg"
                        alt="osu! logo"
                        class="w-full h-full transition-transform duration-300 hover:scale-110 drop-shadow-[0_0_8px_rgba(236,72,153,0.5)] dark:drop-shadow-[0_0_10px_rgba(236,72,153,0.6)]"
                    />
                </a>
                <span>{move || tr!("api-docs-label")}</span>
            </div>

            // 右侧分割按钮组 - 嵌套的小按钮
            <div class="inline-flex overflow-hidden rounded-md border border-gray-600/50 dark:border-gray-500/40">
                // V1 按钮
                <a
                    href="https://github.com/ppy/osu-api/wiki"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="flex relative justify-center items-center py-2 px-4 font-bold text-gray-300 transition-all duration-300 cursor-pointer hover:text-white hover:bg-gradient-to-br hover:from-indigo-500 hover:to-purple-600 focus:ring-2 focus:ring-purple-400 focus:ring-offset-2 focus:ring-offset-gray-900 focus:outline-none group bg-white/5 min-w-[50px] backdrop-blur-sm"
                >
                    <span class="text-sm">V1</span>

                    // 悬停光效
                    <div class="absolute inset-0 bg-gradient-to-t rounded-md opacity-0 transition-opacity duration-300 pointer-events-none group-hover:opacity-100 from-white/0 via-white/8 to-white/0"></div>
                </a>

                // 分割线
                <div class="w-px bg-gray-600/50 dark:bg-gray-500/40"></div>

                // V2 按钮
                <a
                    href="https://osu.ppy.sh/docs/index.html"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="flex relative justify-center items-center py-2 px-4 font-bold text-gray-300 transition-all duration-300 cursor-pointer hover:text-white hover:bg-gradient-to-br hover:from-pink-500 hover:to-rose-600 focus:ring-2 focus:ring-pink-400 focus:ring-offset-2 focus:ring-offset-gray-900 focus:outline-none group bg-white/5 min-w-[50px] backdrop-blur-sm"
                >
                    <span class="text-sm">V2</span>

                    // 悬停光效
                    <div class="absolute inset-0 bg-gradient-to-t rounded-md opacity-0 transition-opacity duration-300 pointer-events-none group-hover:opacity-100 from-white/0 via-white/8 to-white/0"></div>
                </a>
            </div>
        </div>
    }
}
