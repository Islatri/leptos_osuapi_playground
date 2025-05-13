use leptos::*;
use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use osynic_osuapi::v1::client::gloo::client::OsynicOsuApiV1GlooClient;
use osynic_osuapi::v1::model::beatmap::{GetBeatmapsParams, Beatmap};
use osynic_osuapi::v1::model::user::{GetUserParams, User};
use osynic_osuapi::v1::interface::beatmap::IBeatmap;
use osynic_osuapi::v1::interface::user::IUser;
use lucide_leptos::{Github, Loader, Music, Search, User};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum ApiTab {
    Beatmaps,
    Users,
}

#[derive(Clone, Debug)]
struct ApiResult {
    formatted: String,
    raw: String,
}

// 封装 API 客户端管理
// #[component]
// fn ApiClient(
//     api_key: ReadSignal<String>,
//     is_loading: ReadSignal<bool>,
//     #[prop(into)] query_fn: Box<dyn Fn() -> ()>,
//     #[prop(into)] children: Children,
// ) -> impl IntoView {
//     let validate_api_key = move || {
//         if api_key.get().is_empty() {
//             return false;
//         }
//         true
//     };

//     let handle_query = move |_| {
//         if !validate_api_key() {
//             return;
//         }
        
//         // let query_fn = query_fn.clone();
//         query_fn();
//     };

//     view! {
//         <div>
//             {children()}
//             <button 
//                 class="btn-primary whitespace-nowrap"
//                 on:click=handle_query
//                 disabled=is_loading
//             >
//                 <Show
//                     when=move || !is_loading.get()
//                     fallback=|| view! { <Loader /> "加载中..." }
//                 >
//                     <Search />
//                     "查询"
//                 </Show>
//             </button>
//         </div>
//     }
// }

#[component]
fn QueryButton(
    #[prop(into)] api_key: ReadSignal<String>,
    #[prop(into)] is_loading: ReadSignal<bool>,
    #[prop(into)] on_query: Box<dyn Fn() -> ()>,
) -> impl IntoView {
    let validate_api_key = move || {
        if api_key.get().is_empty() {
            return false;
        }
        true
    };

    let handle_query = move |_| {
        if !validate_api_key() {
            return;
        }
        
        on_query();
    };

    view! {
        <button 
            class="btn-primary whitespace-nowrap"
            on:click=handle_query
            disabled=is_loading
        >
            <Show
                when=move || !is_loading.get()
                fallback=|| view! { <Loader /> "加载中..." }
            >
                <Search />
                "查询"
            </Show>
        </button>
    }
}

// 封装输入字段组件
#[component]
fn InputField(
    #[prop(into)] id: String,
    #[prop(into)] label: String,
    #[prop(into)] placeholder: String,
    #[prop(optional, into)] value: Signal<String>,
    #[prop(into)] on_input: Box<dyn Fn(web_sys::Event) -> ()>,
    #[prop(optional, into)] help_text: Option<String>,
    #[prop(optional)] password: bool,
) -> impl IntoView {
    let input_type = if password { "password" } else { "text" };
    
    view! {
        <div class="mb-6">
            <label for={id.clone()} class="block text-sm font-medium mb-2">{label}</label>
            <input 
                type={input_type}
                id={id}
                class="input-field w-full"
                placeholder={placeholder}
                prop:value=move || value.get()
                on:input=on_input
            />
            {match help_text {
                Some(text) => view! { 
                    <p class="text-xs text-gray-500 mt-1">{text}</p> 
                },
                None => view! { <p class="text-xs text-gray-500 mt-1">{String::from(" ")}</p> }
            }}
        </div>
    }
}

// 封装标签页组件
#[component]
fn TabButton(
    #[prop(into)] id: String,
    #[prop(into)] active: Signal<bool>,
    #[prop(into)] on_click: Box<dyn Fn(web_sys::MouseEvent) -> ()>,
    #[prop(into)] icon: AnyView,  // 图标组件
    #[prop(into)] label: String,        // 按钮文本
) -> impl IntoView {
    view! {
        <button 
        id={id}
        class=move || {
                        let base_classes = "inline-block py-2 px-4 border-b-2 rounded-t-lg";

                        let border_class = if active.get() {
                            "border-pink-500 text-pink-600"
                        } else {
                            "border-transparent hover:border-gray-300"
                        };

                        format!("{} {}", base_classes, border_class)
                    }
            // class="inline-block py-2 px-4 border-b-2 rounded-t-lg"
            // class=("border-pink-500 text-pink-600", move || active)
            // class=("border-transparent hover:border-gray-300", move || !active)
            on:click=on_click
        >
            // {children()}
            <div class="flex items-center">
                {icon}
                <span class="ml-2">{label}</span>
            </div>
        </button>
    }
}

// 封装结果显示组件
#[component]
fn ResultDisplay(
    result: ReadSignal<ApiResult>,
) -> impl IntoView {
    view! {
        <div>
            <label class="block text-sm font-medium mb-2">结果</label>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div>
                    <p class="text-xs font-medium mb-1 text-gray-500">格式化结果</p>
                    <pre class="bg-gray-100 dark:bg-gray-800 rounded-lg p-4 overflow-x-auto text-sm min-h-[300px] h-full">
                        {move || result.get().formatted}
                    </pre>
                </div>
                <div>
                    <p class="text-xs font-medium mb-1 text-gray-500">原始响应</p>
                    <pre class="bg-gray-100 dark:bg-gray-800 rounded-lg p-4 overflow-x-auto text-sm min-h-[300px] h-full">
                        {move || result.get().raw}
                    </pre>
                </div>
            </div>
        </div>
    }
}

// 创建 API 客户端并处理错误
fn create_api_client(api_key: &str) -> Result<OsynicOsuApiV1GlooClient, String> {
    let client = OsynicOsuApiV1GlooClient::new(api_key.to_string());
    client.set_proxy_url("http://localhost:8000/".to_string());
    Ok(client)
}

// 格式化谱面结果
fn format_beatmaps(beatmaps: &Vec<Beatmap>) -> String {
    if beatmaps.is_empty() {
        return "未找到谱面".to_string();
    }

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
    result_str
}

// 格式化用户结果
fn format_users(users: &Vec<User>) -> String {
    if users.is_empty() {
        return "未找到用户".to_string();
    }

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
    result_str
}

// 美化 JSON 字符串
fn beautify_json<T: serde::Serialize>(data: &T) -> String {
    match serde_json::to_string_pretty(data) {
        Ok(json) => json,
        Err(_) => "无法序列化为 JSON".to_string(),
    }
}

#[component]
pub fn ApiDemo() -> impl IntoView {
    // 状态变量
    let (api_key, set_api_key) = signal("".to_string());
    let (active_tab, set_active_tab) = signal(ApiTab::Beatmaps);
    let (beatmap_id, set_beatmap_id) = signal("114514".to_string());
    let (username, set_username) = signal("peppy".to_string());
    let (result, set_result) = signal(ApiResult {
        formatted: "// 结果将显示在这里".to_string(),
        raw: "// 原始 JSON 将显示在这里".to_string(),
    });
    let (is_loading, set_is_loading) = signal(false);
    
    // API 请求：谱面
    let fetch_beatmap:Box<dyn Fn()> = Box::new(move || {
        let api_key_val = api_key.get();
        if api_key_val.is_empty() {
            set_result.update(|r| r.formatted = "请输入 API 密钥".to_string());
            return;
        }
        
        set_is_loading.set(true);
        set_result.update(|r| {
            r.formatted = "正在加载...".to_string();
            r.raw = "正在加载...".to_string();
        });
        
        let id = beatmap_id.get();
        
        spawn_local(async move {
            match create_api_client(&api_key_val) {
                Ok(client) => {
                    let params = GetBeatmapsParams::default().sid(id);
                    
                    match client.beatmap.get_beatmaps(params).await {
                        Ok(beatmaps) => {
                            // 存储原始响应
                            let raw_json = beautify_json(&beatmaps);
                            // 格式化结果
                            let formatted = format_beatmaps(&beatmaps);
                            
                            set_result.update(|r| {
                                r.formatted = formatted;
                                r.raw = raw_json;
                            });
                        },
                        Err(e) => {
                            let error_msg = format!("错误: {:?}", e);
                            set_result.update(|r| {
                                r.formatted = error_msg.clone();
                                r.raw = error_msg;
                            });
                        }
                    }
                },
                Err(e) => {
                    set_result.update(|r| {
                        r.formatted = format!("客户端错误: {}", e);
                        r.raw = format!("客户端错误: {}", e);
                    });
                }
            }
            
            set_is_loading.set(false);
        });
    });
    
    // API 请求：用户
    let fetch_user:Box<dyn Fn()> = Box::new(move || {
        let api_key_val = api_key.get();
        if api_key_val.is_empty() {
            set_result.update(|r| r.formatted = "请输入 API 密钥".to_string());
            return;
        }
        
        set_is_loading.set(true);
        set_result.update(|r| {
            r.formatted = "正在加载...".to_string();
            r.raw = "正在加载...".to_string();
        });
        
        let username_val = username.get();
        
        spawn_local(async move {
            match create_api_client(&api_key_val) {
                Ok(client) => {
                    let params = GetUserParams::default().user(username_val);
                    
                    match client.user.get_user(params).await {
                        Ok(users) => {
                            // 存储原始响应
                            let raw_json = beautify_json(&users);
                            // 格式化结果
                            let formatted = format_users(&users);
                            
                            set_result.update(|r| {
                                r.formatted = formatted;
                                r.raw = raw_json;
                            });
                        },
                        Err(e) => {
                            let error_msg = format!("错误: {:?}", e);
                            set_result.update(|r| {
                                r.formatted = error_msg.clone();
                                r.raw = error_msg;
                            });
                        }
                    }
                },
                Err(e) => {
                    set_result.update(|r| {
                        r.formatted = format!("客户端错误: {}", e);
                        r.raw = format!("客户端错误: {}", e);
                    });
                }
            }
            
            set_is_loading.set(false);
        });
    });
    
    let handle_api_key_input: Box<dyn Fn(web_sys::Event)> = Box::new(move |ev| {
        let input_element = event_target::<HtmlInputElement>(&ev);
        set_api_key.set(input_element.value());
    });
    
    let handle_beatmap_id_input: Box<dyn Fn(web_sys::Event)> = Box::new(move |ev| {
        let input_element = event_target::<HtmlInputElement>(&ev);
        set_beatmap_id.set(input_element.value());
    });
    
    let handle_username_input: Box<dyn Fn(web_sys::Event)> = Box::new(move |ev| {
        let input_element = event_target::<HtmlInputElement>(&ev);
        set_username.set(input_element.value());
    });

    let create_tab_click_handler = move || {
        let set_active_tab = set_active_tab.clone(); // 如果需要克隆
        Box::new(move |ev: web_sys::MouseEvent| {
            let target = event_target::<HtmlInputElement>(&ev);
            web_sys::console::log_1(&format!("Clicked tab: {}", target.id()).into());
            let tab = match target.id().as_str() {
                "beatmaps" => ApiTab::Beatmaps,
                "users" => ApiTab::Users,
                _ => return,
            };
            set_active_tab.set(tab);
        }) as Box<dyn Fn(web_sys::MouseEvent)>
    };

    // let handle_tab_click: Box<dyn Fn(web_sys::MouseEvent)> = Box::new(move |ev| {
    //     let target = event_target::<HtmlInputElement>(&ev);
    //     let tab = match target.id().as_str() {
    //         "beatmaps" => ApiTab::Beatmaps,
    //         "users" => ApiTab::Users,
    //         _ => return,
    //     };
    //     set_active_tab.set(tab);
    // });
    
    // Create Signals for tab selection
    let is_beatmaps_tab = Signal::derive(move || matches!(active_tab.get(), ApiTab::Beatmaps));
    let is_users_tab = Signal::derive(move || matches!(active_tab.get(), ApiTab::Users));

    view! {
        <section id="demo" class="py-16 bg-gray-50 dark:bg-gray-900">
            <div class="container mx-auto px-4">
                <h2 class="text-3xl font-bold text-center mb-4">实时 API 演示</h2>
                <p class="text-gray-600 dark:text-gray-300 text-center max-w-2xl mx-auto mb-12">
                    输入您的 osu! API 密钥, 尝试在浏览器中使用 osynic_osuapi.
                </p>
                
                <div class="max-w-5xl mx-auto">
                    <div class="card bg-white dark:bg-gray-800 rounded-lg shadow-md p-6">
                        // API 密钥输入
                        <InputField 
                            id="api_key"
                            label="API 密钥"
                            placeholder="输入您的 osu! API 密钥"
                            value=api_key
                            on_input=handle_api_key_input
                            help_text="密钥只在您的浏览器中使用, 不会传输到其他地方."
                            password=true
                        />
                        
                        // 选项卡导航
                        <div class="border-b border-gray-200 dark:border-gray-700 mb-6">
                            <ul class="flex flex-wrap -mb-px">
                                <li class="mr-2">
                                    <TabButton 
                                        id="beatmaps"
                                        active=is_beatmaps_tab
                                        on_click=create_tab_click_handler()
                                        icon=view! { <Music /> }.into_any()
                                        label="谱面查询"
                                    />
                                </li>
                                <li class="mr-2">
                                    <TabButton 
                                        id="users"
                                        active=is_users_tab
                                        on_click=create_tab_click_handler()
                                        icon=view! { <User /> }.into_any()
                                        label="用户查询"
                                    />
                                </li>
                            </ul>
                        </div>
                        
                        // 谱面查询表单
                        <div class="mb-6" class:hidden=move || !is_beatmaps_tab.get()>
                            <div class="flex items-end">
                                <div class="flex-grow mr-4">
                                    <InputField 
                                        id="beatmap_id"
                                        label="谱面 ID"
                                        placeholder="输入谱面或谱面集 ID"
                                        value=beatmap_id
                                        on_input=handle_beatmap_id_input
                                    />
                                </div>
                                
                                <QueryButton
                                    api_key=api_key
                                    is_loading=is_loading
                                    on_query=fetch_beatmap
                                />
                            </div>
                        </div>

                        // 用户查询表单
                        <div class="mb-6" class:hidden=move || !is_users_tab.get()>
                            <div class="flex items-end">
                                <div class="flex-grow mr-4">
                                    <InputField 
                                        id="username"
                                        label="用户名"
                                        placeholder="输入用户名或用户 ID"
                                        value=username
                                        on_input=handle_username_input
                                    />
                                </div>
                                
                                <QueryButton
                                    api_key=api_key
                                    is_loading=is_loading
                                    on_query=fetch_user
                                />
                            </div>
                        </div>
                        // 结果显示
                        <ResultDisplay result/>
                    </div>
                    
                    <div class="mt-8 text-center">
                        <p class="text-sm text-gray-500 dark:text-gray-400 mb-4">
                            这个演示使用 OsynicOsuapi 通过 WASM 直接在您的浏览器中运行,
                            无需后端服务器处理.
                        </p>
                        <a 
                            href="https://github.com/osynicite/osynic_osuapi" 
                            class="text-pink-600 hover:text-pink-700 transition font-medium inline-flex items-center"
                        >
                            <Github />
                            查看 GitHub 项目
                        </a>
                    </div>
                </div>
            </div>
        </section>
    }
}