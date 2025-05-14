use leptos::*;
use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use osynic_osuapi::v1::client::gloo::client::OsynicOsuApiV1GlooClient;
use osynic_osuapi::v1::model::beatmap::GetBeatmapsParams;
use osynic_osuapi::v1::model::user::GetUserParams;
use osynic_osuapi::v1::interface::beatmap::IBeatmap;
use osynic_osuapi::v1::interface::user::IUser;
use lucide_leptos::{Github, Search, Music, User, Loader};

#[component]
pub fn ApiDemo() -> impl IntoView {
    // 状态变量
    let (api_key, set_api_key) = signal("".to_string());
    let (active_tab, set_active_tab) = signal("beatmaps");
    let (beatmap_id, set_beatmap_id) = signal("114514".to_string());
    let (username, set_username) = signal("peppy".to_string());
    let (result, set_result) = signal("// 结果将显示在这里".to_string());
    let (raw_json, set_raw_json) = signal("// API 原始 JSON 将显示在这里".to_string());
    let (is_loading, set_is_loading) = signal(false);
    
    // API 请求：谱面
    let fetch_beatmap = move |_| {
        if api_key.get().is_empty() {
            set_result.set("请输入 API 密钥".to_string());
            set_raw_json.set("请输入 API 密钥".to_string());
            return;
        }
        
        set_is_loading.set(true);
        set_result.set("正在加载...".to_string());
        set_raw_json.set("正在加载...".to_string());
        
        let key = api_key.get();
        let id = beatmap_id.get();
        
        spawn_local(async move {
            let client = OsynicOsuApiV1GlooClient::new(key);
            client.set_proxy_url("https://osynic-cors.deno.dev/".to_string());
            let params = GetBeatmapsParams::default().sid(id);
            
            match client.beatmap.get_beatmaps(params).await {
                Ok(beatmaps) => {
                    // 设置原始JSON
                    set_raw_json.set(format!("{:#?}", beatmaps));
                    
                    if beatmaps.is_empty() {
                        set_result.set("未找到谱面".to_string());
                    } else {
                        let mut result_str = String::new();
                        for (i, beatmap) in beatmaps.iter().enumerate() {
                            result_str.push_str(&format!("--- 谱面 #{} ---\n", i + 1));
                            result_str.push_str(&format!("标题: {}\n", beatmap.title));
                            result_str.push_str(&format!("艺术家: {}\n", beatmap.artist));
                            result_str.push_str(&format!("难度名: {}\n", beatmap.version));
                            result_str.push_str(&format!("BPM: {}\n", beatmap.bpm));
                            result_str.push_str(&format!("星级: {:.2}\n", beatmap.difficultyrating));
                            result_str.push_str("\n");
                        }
                        set_result.set(result_str);
                    }
                },
                Err(e) => {
                    let error_msg = format!("错误: {:?}", e);
                    set_result.set(error_msg.clone());
                    set_raw_json.set(error_msg);
                }
            }
            
            set_is_loading.set(false);
        });
    };
    
    // API 请求：用户
    let fetch_user = move |_| {
        if api_key.get().is_empty() {
            set_result.set("请输入 API 密钥".to_string());
            set_raw_json.set("请输入 API 密钥".to_string());
            return;
        }
        
        set_is_loading.set(true);
        set_result.set("正在加载...".to_string());
        set_raw_json.set("正在加载...".to_string());
        
        let key = api_key.get();
        let username = username.get();
        
        spawn_local(async move {
            let client = OsynicOsuApiV1GlooClient::new(key);
            client.set_proxy_url("https://osynic-cors.deno.dev/".to_string());
            let params = GetUserParams::default().user(username);
            
            match client.user.get_user(params).await {
                Ok(users) => {
                    // 设置原始JSON
                    set_raw_json.set(format!("{:#?}", users));
                    
                    if users.is_empty() {
                        set_result.set("未找到用户".to_string());
                    } else {
                        let user = &users[0];
                        let mut result_str = String::new();
                        result_str.push_str(&format!("用户名: {}\n", user.username));
                        result_str.push_str(&format!("用户 ID: {}\n", user.user_id));
                        result_str.push_str(&format!("国家: {}\n", user.country));
                        result_str.push_str(&format!("PP: {:.2}\n", user.pp_raw));
                        result_str.push_str(&format!("准确度: {:.2}%\n", user.accuracy));
                        result_str.push_str(&format!("全球排名: #{}\n", user.pp_rank));
                        result_str.push_str(&format!("国家排名: #{}\n", user.pp_country_rank));
                        result_str.push_str(&format!("游戏次数: {}\n", user.playcount));
                        set_result.set(result_str);
                    }
                },
                Err(e) => {
                    let error_msg = format!("错误: {:?}", e);
                    set_result.set(error_msg.clone());
                    set_raw_json.set(error_msg);
                }
            }
            
            set_is_loading.set(false);
        });
    };
    
    let handle_api_key_input = move |ev| {
        let input_element = event_target::<HtmlInputElement>(&ev);
        set_api_key.set(input_element.value());
    };
    
    let handle_beatmap_id_input = move |ev| {
        let input_element = event_target::<HtmlInputElement>(&ev);
        set_beatmap_id.set(input_element.value());
    };
    
    let handle_username_input = move |ev| {
        let input_element = event_target::<HtmlInputElement>(&ev);
        set_username.set(input_element.value());
    };

    view! {
        <section id="demo" class="py-16 bg-gradient-to-r from-gray-50 to-gray-100 dark:from-gray-900 dark:to-gray-800">
            <div class="container mx-auto px-4">
                <h2 class="text-3xl font-bold text-center mb-4 text-pink-600 dark:text-pink-400">osu! API V1 (WASM) 体验使用</h2>
                <p class="text-gray-600 dark:text-gray-300 text-center max-w-2xl mx-auto mb-2">
                    输入您的 osu! API 密钥, 尝试在浏览器中使用 OsynicOsuapi.
                </p>
                <p class="text-gray-600 dark:text-gray-300 text-center max-w-2xl mx-auto mb-12">
                    如果您没有 API 密钥, 直接去自己的 <a href="https://osu.ppy.sh/home/account/edit" target="_blank" class="text-pink-600 hover:text-pink-700 transition-all duration-200 font-medium">osu! 个人设置页</a> 翻到下面的 旧版本API 申请一个即可.
                </p>
                
                <div class="max-w-6xl mx-auto">
                    <div class="card bg-white dark:bg-gray-800 rounded-xl shadow-lg p-6 transition-all duration-300 hover:shadow-xl">
                        // API 密钥输入
                        <div class="mb-6">
                            <label for="api_key" class="block text-sm font-medium mb-2 text-gray-700 dark:text-gray-200">API 密钥</label>
                            <input 
                                type="password" 
                                id="api_key" 
                                class="w-full px-4 py-2 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-800 dark:text-gray-100 focus:ring-2 focus:ring-pink-500 focus:border-transparent transition-all duration-200"
                                placeholder="输入您的 osu! API 密钥"
                                on:input=handle_api_key_input
                            />
                            <p class="text-xs text-gray-500 mt-1 dark:text-gray-400">
                                密钥只在您的浏览器中使用, 不会传输到其他地方.
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
                                        <span class="ml-2">谱面查询</span>

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
                                        <span class="ml-2">用户查询</span>
                                    </button>
                                </li>
                            </ul>
                        </div>
                        
                        // 谱面查询表单
                        <div class="mb-6" class:hidden=move || active_tab.get() != "beatmaps">
                            <label for="beatmap_id" class="block text-sm font-medium mb-2 text-gray-700 dark:text-gray-200">谱面 ID</label>
                            <div class="flex">
                                <input 
                                    type="text" 
                                    id="beatmap_id" 
                                    class="w-full px-4 py-2 rounded-l-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-800 dark:text-gray-100 focus:ring-2 focus:ring-pink-500 focus:border-transparent transition-all duration-200"
                                    placeholder="输入谱面或谱面集 ID"
                                    value=beatmap_id
                                    on:input=handle_beatmap_id_input
                                />
                                <button 
                                    class="flex px-6 py-2 bg-pink-600 hover:bg-pink-700 text-white rounded-r-lg transition-all duration-200 items-center justify-center disabled:opacity-50 disabled:cursor-not-allowed"
                                    on:click=fetch_beatmap
                                    disabled=is_loading
                                >
                                    <Show
                                        when=move || !is_loading.get()
                                        fallback=|| view! { 
                                            <Loader />
                                            <span class="mr-2">加载中</span>    
                                        }
                                    >
                                        <Search />
                                        <span class="mr-2 whitespace-nowrap inline-block w-full">查询</span>
                                    </Show>
                                </button>
                            </div>
                        </div>
                        
                        // 用户查询表单
                        <div class="mb-6" class:hidden=move || active_tab.get() != "users">
                            <label for="username" class="block text-sm font-medium mb-2 text-gray-700 dark:text-gray-200">用户名或用户ID</label>
                            <div class="flex">
                                <input 
                                    type="text" 
                                    id="username" 
                                    class="w-full px-4 py-2 rounded-l-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-800 dark:text-gray-100 focus:ring-2 focus:ring-pink-500 focus:border-transparent transition-all duration-200"
                                    placeholder="输入用户名或用户 ID"
                                    value=username
                                    on:input=handle_username_input
                                />
                                <button 
                                    class="px-6 py-2 bg-pink-600 hover:bg-pink-700 text-white rounded-r-lg transition-all duration-200 flex items-center justify-center disabled:opacity-50 disabled:cursor-not-allowed"
                                    on:click=fetch_user
                                    disabled=is_loading
                                >
                                    <Show
                                        when=move || !is_loading.get()
                                        fallback=|| view! { 
                                            
                                        <Loader />
                                        <span class="mr-2">加载中</span>
                                     }
                                    >
                                        <Search />
                                        <span class="mr-2 whitespace-nowrap inline-block w-full">查询</span>
                                    </Show>
                                </button>
                            </div>
                        </div>
                        
                        // 双面板结果显示
                        <div>
                            <div class="flex flex-col md:flex-row gap-4 mb-2">
                                <h3 class="text-lg font-medium text-gray-800 dark:text-gray-200">查询结果</h3>
                                <div class="ml-auto text-sm text-gray-500 dark:text-gray-400 flex items-center">
                                    <span class="mr-2">左侧: 格式化结果</span>
                                    <span>右侧: 原始 JSON 数据</span>
                                </div>
                            </div>
                            
                            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                                // 左侧 - 格式化结果
                                <div class="result-container">
                                    <div class="p-2 bg-gray-200 dark:bg-gray-700 text-xs text-gray-600 dark:text-gray-300 rounded-t-lg">
                                        格式化结果
                                    </div>
                                    <pre class="bg-gray-100 dark:bg-gray-800 rounded-b-lg p-4 overflow-x-auto text-sm h-[280px] font-mono border border-gray-200 dark:border-gray-700 text-gray-800 dark:text-gray-200">
                                        {result}
                                    </pre>
                                </div>
                                
                                // 右侧 - 原始 JSON
                                <div class="result-container">
                                    <div class="p-2 bg-gray-200 dark:bg-gray-700 text-xs text-gray-600 dark:text-gray-300 rounded-t-lg">
                                        API 原始 JSON
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
                            这个演示使用 OsynicOsuapi 通过 WASM 直接在您的浏览器中运行, 
                            无需后端服务器处理.
                        </p>
                        <a href="https://github.com/islatri/leptos_osuapi_playground" target="_blank" class="text-pink-600 hover:text-pink-700 transition-all duration-200 font-medium inline-flex items-center gap-2 px-4 py-2 rounded-lg hover:bg-pink-100 dark:hover:bg-pink-900/20">
                            <Github />
                            查看 GitHub 项目
                        </a>
                    </div>
                </div>
            </div>
        </section>
    }
}