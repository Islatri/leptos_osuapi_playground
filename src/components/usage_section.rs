use leptos::*;
use leptos::prelude::{ClassAttribute, ElementChild};
use lucide_leptos::Book;

#[component]
pub fn UsageSection() -> impl IntoView {
    view! {
        <section class="py-20 bg-gradient-to-b from-gray-50 to-white dark:from-gray-900 dark:to-gray-950">
            <div class="container mx-auto px-4">
                <h2 class="text-4xl font-bold text-center mb-3">快速开始</h2>
                <p class="text-gray-600 dark:text-gray-400 text-center mb-12 max-w-2xl mx-auto">简单几步配置, 快速接入 osu! API</p>
                
                <div class="grid grid-cols-1 md:grid-cols-2 gap-10 max-w-6xl mx-auto">
                    {/* Reqwest Client Side */}
                    <div class="flex flex-col space-y-6 transform hover:-translate-y-1 transition-transform duration-300">
                        <h3 class="text-2xl font-semibold text-center mb-2 flex items-center justify-center gap-2">
                            <span class="inline-block p-1.5 rounded-full bg-blue-100 dark:bg-blue-900">
                                <div class="w-3 h-3 bg-blue-500 rounded-full"></div>
                            </span>
                            Reqwest 客户端
                        </h3>
                        
                        {/* Reqwest .env */}
                        <div class="bg-gray-900 rounded-xl overflow-hidden shadow-lg shadow-blue-900/10 dark:shadow-blue-500/5 border border-gray-800 h-auto">
                            <div class="flex items-center px-4 py-3 bg-gray-800 border-b border-gray-700">
                                <div class="flex space-x-2">
                                    <div class="w-3 h-3 bg-red-500 rounded-full"></div>
                                    <div class="w-3 h-3 bg-yellow-500 rounded-full"></div>
                                    <div class="w-3 h-3 bg-green-500 rounded-full"></div>
                                </div>
                                <p class="ml-4 text-gray-300 text-sm font-mono">.env</p>
                                <div class="ml-auto">
                                    <span class="text-xs px-2 py-1 bg-sky-700 rounded-md text-gray-300">环境变量</span>
                                </div>
                            </div>
                            
                            <pre class="p-5 text-sm overflow-x-auto text-gray-300">
                                <code>
{r#"# V2 API
CLIENT_ID="你的client_id"
CLIENT_SECRET="你的client_secret"
REDIRECT_URI="你的redirect_uri"
CODE="你的code" # Authorization Code Grant认证时需要"#}
                                </code>
                            </pre>
                        </div>
                        
                        {/* Reqwest Cargo.toml */}
                        <div class="bg-gray-900 rounded-xl overflow-hidden shadow-lg shadow-blue-900/10 dark:shadow-blue-500/5 border border-gray-800 h-auto">
                            <div class="flex items-center px-4 py-3 bg-gray-800 border-b border-gray-700">
                                <div class="flex space-x-2">
                                    <div class="w-3 h-3 bg-red-500 rounded-full"></div>
                                    <div class="w-3 h-3 bg-yellow-500 rounded-full"></div>
                                    <div class="w-3 h-3 bg-green-500 rounded-full"></div>
                                </div>
                                <p class="ml-4 text-gray-300 text-sm font-mono">Cargo.toml</p>
                                <div class="ml-auto">
                                    <span class="text-xs px-2 py-1 bg-gray-700 rounded-md text-gray-300">配置文件</span>
                                </div>
                            </div>
                            
                            <pre class="p-5 text-sm overflow-x-auto text-gray-300">
                                <code>
{r#"[dependencies]
osynic_osuapi = "0.1.0"
# 默认features是 ["v1", "v2", "not-wasm"]"#}
                                </code>
                            </pre>
                        </div>
                        
                        {/* Reqwest main.rs */}
                        <div class="bg-gray-900 rounded-xl overflow-hidden shadow-lg shadow-blue-900/10 dark:shadow-blue-500/5 border border-gray-800 h-auto translate-y-2">
                            <div class="flex items-center px-4 py-3 bg-gray-800 border-b border-gray-700">
                                <div class="flex space-x-2">
                                    <div class="w-3 h-3 bg-red-500 rounded-full"></div>
                                    <div class="w-3 h-3 bg-yellow-500 rounded-full"></div>
                                    <div class="w-3 h-3 bg-green-500 rounded-full"></div>
                                </div>
                                <p class="ml-4 text-gray-300 text-sm font-mono">main.rs</p>
                                <div class="ml-auto">
                                    <span class="text-xs px-2 py-1 bg-blue-600 rounded-md text-white">示例代码</span>
                                </div>
                            </div>
                            
                            <pre class="p-5 text-sm overflow-x-auto text-gray-300 min-h-[280px]">
                                <code>
{r#"// Client Credentials Grant and Get Peppy's User Info
use osynic_osuapi::error::Result;
use osynic_osuapi::v2::client::request::client::OsynicOsuApiV2Client;
use osynic_osuapi::v2::interface::oauth::IOauth;
use osynic_osuapi::v2::interface::users::IUsers;

// You can also import all the client and interface modules by prelude
// use osynic_osuapi::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let client_id = std::env::var("CLIENT_ID").expect("CLIENT_ID not set");
    let client_secret = std::env::var("CLIENT_SECRET").expect("CLIENT_SECRET not set");
    let client = OsynicOsuApiV2Client::default();
    let token = client
        .oauth
        .get_token_without_code(client_id.parse()?, &client_secret)
        .await?;
    println!("{:?}", token);

    let peppy = client
        .users
        .get_user_by_username("peppy", None, None)
        .await?;
    println!("{:?}", peppy);

    Ok(())
}"#}
                                </code>
                            </pre>
                        </div>
                    </div>
                    
                    {/* WASM Client Side */}
                    <div class="flex flex-col space-y-6 transform hover:-translate-y-1 transition-transform duration-300">
                        <h3 class="text-2xl font-semibold text-center mb-2 flex items-center justify-center gap-2">
                            <span class="inline-block p-1.5 rounded-full bg-purple-100 dark:bg-purple-900">
                                <div class="w-3 h-3 bg-purple-500 rounded-full"></div>
                            </span>
                            WASM 客户端
                        </h3>
                        
                        {/* WASM .env */}
                        <div class="bg-gray-900 rounded-xl overflow-hidden shadow-lg shadow-purple-900/10 dark:shadow-purple-500/5 border border-gray-800 h-auto translate-y-2">
                            <div class="flex items-center px-4 py-3 bg-gray-800 border-b border-gray-700">
                                <div class="flex space-x-2">
                                    <div class="w-3 h-3 bg-red-500 rounded-full"></div>
                                    <div class="w-3 h-3 bg-yellow-500 rounded-full"></div>
                                    <div class="w-3 h-3 bg-green-500 rounded-full"></div>
                                </div>
                                <p class="ml-4 text-gray-300 text-sm font-mono">.env</p>
                                <div class="ml-auto">
                                    <span class="text-xs px-2 py-1 bg-pink-700 rounded-md text-gray-300">环境变量</span>
                                </div>
                            </div>
                            
                            <pre class="p-5 text-sm overflow-x-auto text-gray-300 min-h-[80px]">
                                <code>
{r#"# V1 API
API_KEY="你的api_key""#}
                                </code>
                            </pre>
                        </div>
                        
                        {/* WASM Cargo.toml */}
                        <div class="bg-gray-900 rounded-xl overflow-hidden shadow-lg shadow-purple-900/10 dark:shadow-purple-500/5 border border-gray-800 h-auto translate-y-2">
                            <div class="flex items-center px-4 py-3 bg-gray-800 border-b border-gray-700">
                                <div class="flex space-x-2">
                                    <div class="w-3 h-3 bg-red-500 rounded-full"></div>
                                    <div class="w-3 h-3 bg-yellow-500 rounded-full"></div>
                                    <div class="w-3 h-3 bg-green-500 rounded-full"></div>
                                </div>
                                <p class="ml-4 text-gray-300 text-sm font-mono">Cargo.toml</p>
                                <div class="ml-auto">
                                    <span class="text-xs px-2 py-1 bg-gray-700 rounded-md text-gray-300">配置文件</span>
                                </div>
                            </div>
                            
                            <pre class="p-5 text-sm overflow-x-auto text-gray-300 min-h-[200px]">
                                <code>
{r#"[dependencies]
osynic_osuapi = { version = "0.1.0", default-features = false, features = ["v1", "v2", "wasm"] }

# WASM 支持
wasm-bindgen = "0.2"
wasm-bindgen-futures = "0.4"
web-sys = { version = "0.3", features = ["console"] }"#}
                                </code>
                            </pre>
                        </div>
                        
                        {/* WASM main.rs */}
                        <div class="bg-gray-900 rounded-xl overflow-hidden shadow-lg shadow-purple-900/10 dark:shadow-purple-500/5 border border-gray-800 h-auto">
                            <div class="flex items-center px-4 py-3 bg-gray-800 border-b border-gray-700">
                                <div class="flex space-x-2">
                                    <div class="w-3 h-3 bg-red-500 rounded-full"></div>
                                    <div class="w-3 h-3 bg-yellow-500 rounded-full"></div>
                                    <div class="w-3 h-3 bg-green-500 rounded-full"></div>
                                </div>
                                <p class="ml-4 text-gray-300 text-sm font-mono">main.rs</p>
                                <div class="ml-auto">
                                    <span class="text-xs px-2 py-1 bg-purple-600 rounded-md text-white">示例代码</span>
                                </div>
                            </div>
                            
                            <pre class="p-5 text-sm overflow-x-auto text-gray-300 min-h-[280px]">
                                <code>
{r#"use wasm_bindgen_futures::spawn_local;
use osynic_osuapi::v1::client::gloo::client::OsynicOsuApiV1GlooClient;
use osynic_osuapi::v1::model::beatmap::GetBeatmapsParams;

// 在 WASM 环境中使用
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn start() {
    // 初始化日志
    console_error_panic_hook::set_once();
    
    let client = OsynicOsuApiV1GlooClient::new("your_api_key".to_string());
    
    // 创建查询参数
    let params = GetBeatmapsParams::default()
        .sid("114514".to_string());
    
    // 在 WASM 中处理异步请求
    spawn_local(async move {
        match client.beatmap.get_beatmaps(params).await {
            Ok(beatmaps) => {
                // 处理返回的数据
                for beatmap in beatmaps {
                    web_sys::console::log_1(&format!(
                        "谱面: {} - {}", 
                        beatmap.artist, 
                        beatmap.title
                    ).into());
                }
            },
            Err(e) => {
                web_sys::console::error_1(&format!("错误: {:?}", e).into());
            }
        }
    });
}"#}
                                </code>
                            </pre>
                        </div>
                    </div>
                </div>
                
                <div class="text-center mt-14">
                    <a href="https://github.com/osynicite/osynic_osuapi" 
                       class="inline-flex items-center px-6 py-3 bg-pink-600 text-white font-medium rounded-md shadow-lg hover:shadow-xl transition-all duration-300 gap-2 transform hover:-translate-y-0.5">
                        <Book />
                        <span>查看Docs.rs完整文档</span>
                    </a>
                </div>
            </div>
        </section>
    }
}