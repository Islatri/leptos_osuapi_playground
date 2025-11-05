use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::*;
use leptos_fluent::tr;
use lucide_leptos::Book;

#[component]
pub fn UsageSection() -> impl IntoView {
    view! {
        <section class="py-20 bg-gradient-to-b from-gray-50 to-white dark:from-gray-900 dark:to-gray-950">
            <div class="container px-4 mx-auto">
                <h2 class="mb-3 text-4xl font-bold text-center text-gray-800 dark:text-gray-200">
                    {move || tr!("quick-start-title")}
                </h2>
                <p class="mx-auto mb-12 max-w-2xl text-center text-gray-600 dark:text-gray-400">
                    {move || tr!("quick-start-description")}
                </p>

                <div class="grid grid-cols-1 gap-10 mx-auto max-w-6xl md:grid-cols-2">
                    {}
                    <div class="flex flex-col space-y-6 transition-transform duration-300 transform hover:-translate-y-1">
                        <h3 class="flex gap-2 justify-center items-center mb-2 text-2xl font-semibold text-center">
                            <span class="inline-block p-1.5 bg-amber-100 rounded-full dark:bg-amber-900">
                                <div class="w-3 h-3 bg-amber-500 rounded-full"></div>
                            </span>
                            <span class="text-gray-800 dark:text-gray-200">
                                {move || tr!("quick-start-reqwest-client")}
                            </span>
                        </h3>

                        {}
                        <div class="overflow-hidden h-auto bg-gray-900 rounded-xl border border-gray-800 shadow-lg shadow-blue-900/10 dark:shadow-blue-500/5">
                            <div class="flex items-center py-3 px-4 bg-gray-800 border-b border-gray-700">
                                <div class="flex space-x-2">
                                    <div class="w-3 h-3 bg-red-500 rounded-full"></div>
                                    <div class="w-3 h-3 bg-yellow-500 rounded-full"></div>
                                    <div class="w-3 h-3 bg-green-500 rounded-full"></div>
                                </div>
                                <p class="ml-4 font-mono text-sm text-gray-300">.env</p>
                                <div class="ml-auto">
                                    <span class="py-1 px-2 text-xs text-gray-300 rounded-md bg-sky-700">
                                        {move || tr!("quick-start-env-var")}
                                    </span>
                                </div>
                            </div>

                            <pre class="overflow-x-auto p-5 text-sm text-gray-300">
                                <code>
                                    {r#"# V2 API
CLIENT_ID="your client_id"
CLIENT_SECRET="your client_secret"
REDIRECT_URI="your redirect_uri"
CODE="your code" # Authorization Code Grant"#}
                                </code>
                            </pre>
                        </div>

                        {}
                        <div class="overflow-hidden h-auto bg-gray-900 rounded-xl border border-gray-800 shadow-lg shadow-blue-900/10 dark:shadow-blue-500/5">
                            <div class="flex items-center py-3 px-4 bg-gray-800 border-b border-gray-700">
                                <div class="flex space-x-2">
                                    <div class="w-3 h-3 bg-red-500 rounded-full"></div>
                                    <div class="w-3 h-3 bg-yellow-500 rounded-full"></div>
                                    <div class="w-3 h-3 bg-green-500 rounded-full"></div>
                                </div>
                                <p class="ml-4 font-mono text-sm text-gray-300">Cargo.toml</p>
                                <div class="ml-auto">
                                    <span class="py-1 px-2 text-xs text-gray-300 bg-gray-700 rounded-md">
                                        {move || tr!("quick-start-config-file")}
                                    </span>
                                </div>
                            </div>

                            <pre class="overflow-x-auto p-5 text-sm text-gray-300">
                                <code>
                                    {r#"[dependencies]
osynic_osuapi = "0.1.6"
# Default features are ["v1", "v2", "not-wasm"]"#}
                                </code>
                            </pre>
                        </div>

                        {}
                        <div class="overflow-hidden h-auto bg-gray-900 rounded-xl border border-gray-800 shadow-lg translate-y-2 shadow-blue-900/10 dark:shadow-blue-500/5">
                            <div class="flex items-center py-3 px-4 bg-gray-800 border-b border-gray-700">
                                <div class="flex space-x-2">
                                    <div class="w-3 h-3 bg-red-500 rounded-full"></div>
                                    <div class="w-3 h-3 bg-yellow-500 rounded-full"></div>
                                    <div class="w-3 h-3 bg-green-500 rounded-full"></div>
                                </div>
                                <p class="ml-4 font-mono text-sm text-gray-300">main.rs</p>
                                <div class="ml-auto">
                                    <span class="py-1 px-2 text-xs text-white bg-amber-600 rounded-md">
                                        {move || tr!("quick-start-example-code")}
                                    </span>
                                </div>
                            </div>

                            <pre class="overflow-x-auto p-5 text-sm text-gray-300 min-h-[280px]">
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
                    </div> {}
                    <div class="flex flex-col space-y-6 transition-transform duration-300 transform hover:-translate-y-1">
                        <h3 class="flex gap-2 justify-center items-center mb-2 text-2xl font-semibold text-center">
                            <span class="inline-block p-1.5 bg-green-100 rounded-full dark:bg-green-900">
                                <div class="w-3 h-3 bg-green-500 rounded-full"></div>
                            </span>
                            <span class="text-gray-800 dark:text-gray-200">
                                {move || tr!("quick-start-vue-client")}
                            </span>
                        </h3>

                        {}
                        <div class="overflow-hidden h-auto bg-gray-900 rounded-xl border border-gray-800 shadow-lg translate-y-2 shadow-purple-900/10 dark:shadow-purple-500/5">
                            <div class="flex items-center py-3 px-4 bg-gray-800 border-b border-gray-700">
                                <div class="flex space-x-2">
                                    <div class="w-3 h-3 bg-red-500 rounded-full"></div>
                                    <div class="w-3 h-3 bg-yellow-500 rounded-full"></div>
                                    <div class="w-3 h-3 bg-green-500 rounded-full"></div>
                                </div>
                                <p class="ml-4 font-mono text-sm text-gray-300">npm</p>
                                <div class="ml-auto">
                                    <span class="py-1 px-2 text-xs text-gray-300 bg-red-500 rounded-md">
                                        {move || tr!("quick-start-install")}
                                    </span>
                                </div>
                            </div>

                            <pre class="overflow-x-auto p-5 text-sm text-gray-300 min-h-[64px]">
                                <code>
                                    {r#"npm install @osynicite/osynic-osuapi vite-plugin-wasm vite-plugin-top-level-await"#}
                                </code>
                            </pre>
                        </div>

                        {}
                        <div class="overflow-hidden h-auto bg-gray-900 rounded-xl border border-gray-800 shadow-lg translate-y-2 shadow-purple-900/10 dark:shadow-purple-500/5">
                            <div class="flex items-center py-3 px-4 bg-gray-800 border-b border-gray-700">
                                <div class="flex space-x-2">
                                    <div class="w-3 h-3 bg-red-500 rounded-full"></div>
                                    <div class="w-3 h-3 bg-yellow-500 rounded-full"></div>
                                    <div class="w-3 h-3 bg-green-500 rounded-full"></div>
                                </div>
                                <p class="ml-4 font-mono text-sm text-gray-300">vite.config.ts</p>
                                <div class="ml-auto">
                                    <span class="py-1 px-2 text-xs text-gray-300 bg-yellow-400 rounded-md">
                                        {move || tr!("quick-start-vite-config")}
                                    </span>
                                </div>
                            </div>

                            <pre class="overflow-x-auto p-5 text-sm text-gray-300 min-h-[80px]">
                                <code>
                                    {r#"import { defineConfig } from 'vite'
import wasm from 'vite-plugin-wasm'
import topLevelAwait from 'vite-plugin-top-level-await'
import vue from '@vitejs/plugin-vue'

// https://vite.dev/config/
export default defineConfig({
    plugins: [
        vue(),
        wasm(),
        topLevelAwait()
    ],
})"#}
                                </code>
                            </pre>
                        </div>

                        {}
                        <div class="overflow-hidden h-auto bg-gray-900 rounded-xl border border-gray-800 shadow-lg shadow-purple-900/10 dark:shadow-purple-500/5">
                            <div class="flex items-center py-3 px-4 bg-gray-800 border-b border-gray-700">
                                <div class="flex space-x-2">
                                    <div class="w-3 h-3 bg-red-500 rounded-full"></div>
                                    <div class="w-3 h-3 bg-yellow-500 rounded-full"></div>
                                    <div class="w-3 h-3 bg-green-500 rounded-full"></div>
                                </div>
                                <p class="ml-4 font-mono text-sm text-gray-300">App.vue</p>
                                <div class="ml-auto">
                                    <span class="py-1 px-2 text-xs text-white bg-green-600 rounded-md">
                                        {move || tr!("quick-start-example-code")}
                                    </span>
                                </div>
                            </div>

                            <pre class="overflow-x-auto p-5 text-sm text-gray-300 min-h-[280px]">
                                <code>
                                    {r#"<template>
    <div class="p-6 bg-gray-900 text-white min-h-screen">
        <div class="max-w-2xl mx-auto space-y-4">
            <div class="space-y-2">
                <input v-model="query.bid" type="text" placeholder="谱面ID"
                    class="w-full px-3 py-2 bg-gray-800 rounded border border-gray-700" />
                <input v-model="query.sid" type="text" placeholder="谱面集ID"
                    class="w-full px-3 py-2 bg-gray-800 rounded border border-gray-700" />
                <select v-model="query.mode" class="w-full px-3 py-2 bg-gray-800 rounded border border-gray-700">
                    <option value="">所有模式</option>
                    <option value="0">标准</option>
                    <option value="1">太鼓</option>
                    <option value="2">接水果</option>
                    <option value="3">mania</option>
                </select>
                <button @click="search" :disabled="loading"
                    class="w-full px-3 py-2 bg-blue-600 hover:bg-blue-700 rounded disabled:opacity-50">
                    {{ loading ? '加载中...' : '搜索' }}
                </button>
            </div>
            <div v-if="error" class="text-red-400">{{ error }}</div>
            <div v-if="beatmaps.length" class="space-y-2">
                <div v-for="m in beatmaps" :key="m.beatmap_id" class="bg-gray-800 p-3 rounded text-sm">
                    <div class="font-bold text-blue-300">{{ m.title }} [{{ m.version }}]</div>
                    <div class="text-gray-400">★{{ parseFloat(m.difficultyrating).toFixed(2) }} | {{ m.artist }}</div>
                    <div class="text-gray-500 text-xs mt-1">
                        {{ formatTime(m.total_length) }} | BPM {{ parseInt(m.bpm) }} | {{ calcPassRate(m.playcount,
                        m.passcount) }}% 通过率
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue';
import { OsynicOsuApiV1GlooClient } from '@osynicite/osynic-osuapi';

const client = new OsynicOsuApiV1GlooClient("YOUR_API_KEY_HERE_AND_PLS_NOT_SHARE_IT_IN_YOUR_CONCRETE_PROJECT");
client.setProxyUrl("YOUR_PROXY_URL_HERE_BECAUSE_CORS"); // // You can see https://github.com/Islatri/deno_osynic_cors

const query = reactive({ bid: '', sid: '', mode: '' });
const beatmaps = ref([]);
const loading = ref(false);
const error = ref('');

const search = async () => {
    loading.value = true;
    error.value = '';
    try {
        const params = Object.fromEntries(Object.entries(query).filter(([, v]) => v));
        beatmaps.value = await client.getBeatmaps(params).then(r => Array.isArray(r) ? r : [r]);
    } catch (err: any) {
        error.value = err?.message || '查询失败';
    } finally {
        loading.value = false;
    }
};

const formatTime = (s: string) => {
    const t = parseInt(s);
    return `${Math.floor(t / 60)}:${(t % 60).toString().padStart(2, '0')}`;
};

const calcPassRate = (p: string, pa: string) => ((parseInt(pa) / parseInt(p)) * 100).toFixed(1);
</script>"#}
                                </code>
                            </pre>
                        </div>
                    </div>
                </div>

                <div class="mt-14">
                    <div class="flex flex-col sm:flex-row gap-4 sm:gap-6 justify-center items-center px-4">
                        <a
                            href="https://github.com/Osynicite/osynic_osuapi/tree/master/examples"
                            target="_blank"
                            class="w-full sm:w-auto inline-flex gap-2 items-center justify-center py-3 px-6 font-medium text-white bg-amber-600 rounded-lg shadow-md hover:shadow-lg transition-all duration-300 hover:bg-amber-700 active:scale-95"
                        >
                            <Book />
                            <span>{move || tr!("quick-start-view-examples")} "(Rust)"</span>
                        </a>
                        <a
                            href="https://github.com/Osynicite/osynic_osuapi/tree/master/src/wasm/examples"
                            target="_blank"
                            class="w-full sm:w-auto inline-flex gap-2 items-center justify-center py-3 px-6 font-medium text-white bg-green-600 rounded-lg shadow-md hover:shadow-lg transition-all duration-300 hover:bg-green-700 active:scale-95"
                        >
                            <Book />
                            <span>{move || tr!("quick-start-view-examples")} "(Vue)"</span>
                        </a>
                    </div>
                </div>
            </div>
        </section>
    }
}
