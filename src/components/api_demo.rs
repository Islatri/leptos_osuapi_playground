use leptos::prelude::*;
use leptos::*;
use leptos_fluent::tr;
use lucide_leptos::{Github, Music, User};
use web_sys::HtmlInputElement;

use crate::components::demo::beatmap_demo::BeatmapDemo;
use crate::components::demo::user_demo::UserDemo;
use crate::components::demo::user_recent_demo::UserRecentDemo;
use crate::components::demo::user_best_demo::UserBestDemo;
use crate::components::demo::scores_demo::ScoresDemo;

#[component]
pub fn ApiDemo() -> impl IntoView {
    // State variables
    let (api_key, set_api_key) = signal("".to_string());
    let (active_tab, set_active_tab) = signal("beatmaps");
    let (result, set_result) = signal(tr!("api-demo-result-placeholder").to_string());
    let (raw_json, set_raw_json) = signal(tr!("api-demo-raw-json-placeholder").to_string());
    let (is_loading, set_is_loading) = signal(false);

    let handle_api_key_input = move |ev| {
        let input_element = event_target::<HtmlInputElement>(&ev);
        set_api_key.set(input_element.value());
    };

    view! {
        <section id="demo" class="py-16 bg-gradient-to-r from-gray-50 to-gray-100 dark:from-gray-900 dark:to-gray-800">
            <div class="container mx-auto px-4">
                <h2 class="text-3xl font-bold text-center mb-4 text-pink-600 dark:text-pink-400">
                    {move || tr!("api-demo-title-1")}
                </h2>
                <p class="text-gray-600 dark:text-gray-300 text-center max-w-2xl mx-auto mb-2">
                    {move || tr!("api-demo-description-1")}
                </p>
                <p class="text-gray-600 dark:text-gray-300 text-center max-w-2xl mx-auto mb-12">
                    {move || tr!("api-demo-description-2")} <a href="https://osu.ppy.sh/home/account/edit" target="_blank" class="text-pink-600 hover:text-pink-700 transition-all duration-200 font-medium">{move || tr!("api-demo-account-settings")}</a> {move || tr!("api-demo-api-section")}
                </p>

                <div class="max-w-6xl mx-auto">
                    <div class="card bg-white dark:bg-gray-800 rounded-xl shadow-lg p-6 transition-all duration-300 hover:shadow-xl">
                        {/* API 密钥输入 */}
                        <div class="mb-6">
                            <label for="api_key" class="block text-sm font-medium mb-2 text-gray-700 dark:text-gray-200">
                                {move || tr!("api-demo-label-api-key")}
                            </label>
                            <input
                                type="password"
                                id="api_key"
                                class="w-full px-4 py-2 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-800 dark:text-gray-100 focus:ring-2 focus:ring-pink-500 focus:border-transparent transition-all duration-200"
                                placeholder={move || tr!("api-demo-input-placeholder")}
                                on:input=handle_api_key_input
                            />
                            <p class="text-xs text-gray-500 mt-1 dark:text-gray-400">
                                {move || tr!("api-demo-key-security")}
                            </p>
                        </div>

                        // 选项卡导航
                        <div class="border-b border-gray-200 dark:border-gray-700 mb-6">
                            <ul class="flex flex-wrap -mb-px">
                                <li class="mr-2">
                                    <button
                                        class=move || {
                                            let base_classes = "flex py-2 px-4 border-b-2 rounded-t-lg transition-all duration-200 items-center";

                                            let active_class = if active_tab.get() == "beatmaps" {
                                                "border-pink-500 text-pink-600 dark:text-pink-400"
                                            } else {
                                                "border-transparent hover:border-gray-300 hover:text-pink-500 text-gray-800 dark:text-gray-200"
                                            };

                                            format!("{} {}", base_classes, active_class)
                                        }
                                        on:click=move |_| set_active_tab.set("beatmaps")
                                    >
                                        <Music />
                                        <span class="ml-2">{move || tr!("api-demo-beatmap-query")}</span>
                                    </button>
                                </li>
                                <li class="mr-2">
                                    <button
                                        class=move || {
                                            let base_classes = "flex py-2 px-4 border-b-2 rounded-t-lg transition-all duration-200 items-center";

                                            let active_class = if active_tab.get() == "users" {
                                                "border-pink-500 text-pink-600 dark:text-pink-400"
                                            } else {
                                                "border-transparent hover:border-gray-300 hover:text-pink-500 text-gray-800 dark:text-gray-200"
                                            };

                                            format!("{} {}", base_classes, active_class)
                                        }
                                        on:click=move |_| set_active_tab.set("users")
                                    >
                                        <User />
                                        <span class="ml-2">{move || tr!("api-demo-user-query")}</span>
                                    </button>
                                </li>
                                <li class="mr-2">
                                    <button
                                        class=move || {
                                            let base_classes = "flex py-2 px-4 border-b-2 rounded-t-lg transition-all duration-200 items-center";

                                            let active_class = if active_tab.get() == "user_recents" {
                                                "border-pink-500 text-pink-600 dark:text-pink-400"
                                            } else {
                                                "border-transparent hover:border-gray-300 hover:text-pink-500 text-gray-800 dark:text-gray-200"
                                            };

                                            format!("{} {}", base_classes, active_class)
                                        }
                                        on:click=move |_| set_active_tab.set("user_recents")
                                    >
                                        <User />
                                        <span class="ml-2">{move || tr!("api-demo-user-recent-query")}</span>
                                    </button>
                                </li>
                                <li class="mr-2">
                                    <button
                                        class=move || {
                                            let base_classes = "flex py-2 px-4 border-b-2 rounded-t-lg transition-all duration-200 items-center";

                                            let active_class = if active_tab.get() == "user_bests" {
                                                "border-pink-500 text-pink-600 dark:text-pink-400"
                                            } else {
                                                "border-transparent hover:border-gray-300 hover:text-pink-500 text-gray-800 dark:text-gray-200"
                                            };

                                            format!("{} {}", base_classes, active_class)
                                        }
                                        on:click=move |_| set_active_tab.set("user_bests")
                                    >
                                        <User />
                                        <span class="ml-2">{move || tr!("api-demo-user-best-query")}</span>
                                    </button>
                                </li>
                                <li class="mr-2">
                                    <button
                                        class=move || {
                                            let base_classes = "flex py-2 px-4 border-b-2 rounded-t-lg transition-all duration-200 items-center";

                                            let active_class = if active_tab.get() == "scores" {
                                                "border-pink-500 text-pink-600 dark:text-pink-400"
                                            } else {
                                                "border-transparent hover:border-gray-300 hover:text-pink-500 text-gray-800 dark:text-gray-200"
                                            };

                                            format!("{} {}", base_classes, active_class)
                                        }
                                        on:click=move |_| set_active_tab.set("scores")
                                    >
                                        <User />
                                        <span class="ml-2">{move || tr!("api-demo-scores-query")}</span>
                                    </button>
                                </li>
                            </ul>
                        </div>

                        // 谱面查询表单
                        <div class:hidden=move || active_tab.get() != "beatmaps">
                            <BeatmapDemo
                                api_key=api_key
                                set_result=set_result
                                set_raw_json=set_raw_json
                                is_loading=is_loading
                                set_is_loading=set_is_loading
                            />
                        </div>

                        // 用户查询表单
                        <div class:hidden=move || active_tab.get() != "users">
                            <UserDemo
                                api_key=api_key
                                set_result=set_result
                                set_raw_json=set_raw_json
                                is_loading=is_loading
                                set_is_loading=set_is_loading
                            />
                        </div>

                        // 用户最近游玩查询表单
                        <div class:hidden=move || active_tab.get() != "user_recents">
                            <UserRecentDemo
                                api_key=api_key
                                set_result=set_result
                                set_raw_json=set_raw_json
                                is_loading=is_loading
                                set_is_loading=set_is_loading
                            />
                        </div>

                        // 用户最佳成绩查询表单
                        <div class:hidden=move || active_tab.get() != "user_bests">
                            <UserBestDemo
                                api_key=api_key
                                set_result=set_result
                                set_raw_json=set_raw_json
                                is_loading=is_loading
                                set_is_loading=set_is_loading
                            />
                        </div>

                        // 成绩查询表单
                        <div class:hidden=move || active_tab.get() != "scores">
                            <ScoresDemo
                                api_key=api_key
                                set_result=set_result
                                set_raw_json=set_raw_json
                                is_loading=is_loading
                                set_is_loading=set_is_loading
                            />
                        </div>

                        // 双面板结果显示
                        <div>
                            <div class="flex flex-col md:flex-row gap-4 mb-2">
                                <h3 class="text-lg font-medium text-gray-800 dark:text-gray-200">
                                    {move || tr!("api-demo-search-result")}
                                </h3>
                                <div class="ml-auto text-sm text-gray-500 dark:text-gray-400 flex items-center">
                                    <span class="mr-2">{move || tr!("api-demo-left-formatted")}</span>
                                    <span>{move || tr!("api-demo-right-raw")}</span>
                                </div>
                            </div>

                            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                                // 左侧 - 格式化结果
                                <div class="result-container">
                                    <div class="p-2 bg-gray-200 dark:bg-gray-700 text-xs text-gray-600 dark:text-gray-300 rounded-t-lg">
                                        {move || tr!("api-demo-formatted-results")}
                                    </div>
                                    <pre class="bg-gray-100 dark:bg-gray-800 rounded-b-lg p-4 overflow-x-auto text-sm h-[280px] font-mono border border-gray-200 dark:border-gray-700 text-gray-800 dark:text-gray-200">
                                        {result}
                                    </pre>
                                </div>

                                // 右侧 - 原始 JSON
                                <div class="result-container">
                                    <div class="p-2 bg-gray-200 dark:bg-gray-700 text-xs text-gray-600 dark:text-gray-300 rounded-t-lg">
                                        {move || tr!("api-demo-raw-json")}
                                    </div>
                                    <pre class="bg-gray-100 dark:bg-gray-800 rounded-b-lg p-4 overflow-x-auto text-sm h-[280px] font-mono border border-gray-200 dark:border-gray-700 text-gray-800 dark:text-gray-200">
                                        {raw_json}
                                    </pre>
                                </div>
                            </div>
                        </div>
                    </div>

                    <div class="mt-8 text-center">
                        <p class="text-sm text-gray-500 dark:text-gray-400 mb-4">
                            {move || tr!("api-demo-description-wasm")}
                        </p>
                        <a href="https://github.com/islatri/leptos_osuapi_playground" target="_blank" class="text-pink-600 hover:text-pink-700 transition-all duration-200 font-medium inline-flex items-center gap-2 px-4 py-2 rounded-lg hover:bg-pink-100 dark:hover:bg-pink-900/20">
                            <Github />
                            {move || tr!("api-demo-view-github")}
                        </a>
                    </div>
                </div>
            </div>
        </section>
    }
}