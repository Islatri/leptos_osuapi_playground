use leptos::prelude::*;
use leptos::*;
use leptos_fluent::tr;
use lucide_leptos::{Github, Loader, Music, Search, User};
use osynic_osuapi::v1::client::gloo::client::OsynicOsuApiV1GlooClient;
use osynic_osuapi::v1::interface::beatmap::IBeatmap;
use osynic_osuapi::v1::interface::user::IUser;
use osynic_osuapi::v1::model::beatmap::GetBeatmapsParams;
use osynic_osuapi::v1::model::user::GetUserParams;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;

#[component]
pub fn ApiDemo() -> impl IntoView {
    // Static strings for internationalization
    let enter_api_key = Memo::new(move |_| tr!("api-demo-enter-api-key"));
    let loading_text = Memo::new(move |_| tr!("api-demo-loading-text"));
    let no_beatmap_found = Memo::new(move |_| tr!("api-demo-no-beatmap-found"));
    let no_user_found = Memo::new(move |_| tr!("api-demo-no-user-found"));
    let error_prefix = Memo::new(move |_| tr!("api-demo-error")); // 移除空参数

    // Beatmap result templates - 不在初始化时提供替换值
    let beatmap_number_tpl = Memo::new(move |_| tr!("api-demo-beatmap-number"));
    let title_tpl = Memo::new(move |_| tr!("api-demo-title"));
    let artist_tpl = Memo::new(move |_| tr!("api-demo-artist"));
    let version_tpl = Memo::new(move |_| tr!("api-demo-version"));
    let bpm_tpl = Memo::new(move |_| tr!("api-demo-bpm"));
    let stars_tpl = Memo::new(move |_| tr!("api-demo-stars"));

    // User result templates - 不在初始化时提供替换值
    let username_tpl = Memo::new(move |_| tr!("api-demo-username"));
    let user_id_tpl = Memo::new(move |_| tr!("api-demo-user-id"));
    let country_tpl = Memo::new(move |_| tr!("api-demo-country"));
    let pp_tpl = Memo::new(move |_| tr!("api-demo-pp"));
    let accuracy_tpl = Memo::new(move |_| tr!("api-demo-accuracy"));
    let global_rank_tpl = Memo::new(move |_| tr!("api-demo-global-rank"));
    let country_rank_tpl = Memo::new(move |_| tr!("api-demo-country-rank"));
    let playcount_tpl = Memo::new(move |_| tr!("api-demo-playcount"));

    // State variables
    let (api_key, set_api_key) = signal("".to_string());
    let (active_tab, set_active_tab) = signal("beatmaps");
    let (beatmap_id, set_beatmap_id) = signal("114514".to_string());
    let (username, set_username) = signal("peppy".to_string());
    let (result, set_result) = signal(tr!("api-demo-result-placeholder").to_string());
    let (raw_json, set_raw_json) = signal(tr!("api-demo-raw-json-placeholder").to_string());
    let (is_loading, set_is_loading) = signal(false);

    // API Request: Beatmaps
    let fetch_beatmap = move |_| {
        if api_key.get().is_empty() {
            set_result.set(enter_api_key.get());
            set_raw_json.set(enter_api_key.get());
            return;
        }

        set_is_loading.set(true);
        set_result.set(loading_text.get());
        set_raw_json.set(loading_text.get());

        let key = api_key.get();
        let id = beatmap_id.get();

        // Capture template strings for async context
        let no_beatmap_found_str = no_beatmap_found.get();
        let beatmap_number_template = beatmap_number_tpl.get();
        let title_template = title_tpl.get();
        let artist_template = artist_tpl.get();
        let version_template = version_tpl.get();
        let bpm_template = bpm_tpl.get();
        let stars_template = stars_tpl.get();
        let error_prefix_str = error_prefix.get();

        spawn_local(async move {
            let client = OsynicOsuApiV1GlooClient::new(key);
            client.set_proxy_url("https://osynic-cors.deno.dev/".to_string());
            let params = GetBeatmapsParams::default().sid(id);

            match client.beatmap.get_beatmaps(params).await {
                Ok(beatmaps) => {
                    // Set raw JSON
                    set_raw_json.set(format!("{:#?}", beatmaps));

                    if beatmaps.is_empty() {
                        set_result.set(no_beatmap_found_str);
                    } else {
                        let mut result_str = String::new();
                        for (i, beatmap) in beatmaps.iter().enumerate() {
                            // 修复：正确替换占位符
                            let beatmap_num =
                                beatmap_number_template.replace("{$number}", &(i + 1).to_string());
                            result_str.push_str(&beatmap_num);
                            result_str.push_str("\n");

                            // 修复：正确替换占位符
                            result_str
                                .push_str(&title_template.replace("{$title}", &beatmap.title));
                            result_str.push_str("\n");
                            result_str
                                .push_str(&artist_template.replace("{$artist}", &beatmap.artist));
                            result_str.push_str("\n");
                            result_str.push_str(
                                &version_template.replace("{$version}", &beatmap.version),
                            );
                            result_str.push_str("\n");
                            result_str.push_str(&bpm_template.replace("{$bpm}", &beatmap.bpm));
                            result_str.push_str("\n");

                            let stars_formatted = format!("{:.2}", beatmap.difficultyrating);
                            result_str
                                .push_str(&stars_template.replace("{$stars}", &stars_formatted));
                            result_str.push_str("\n\n");
                        }
                        set_result.set(result_str);
                    }
                }
                Err(e) => {
                    let error_msg = error_prefix_str + &format!("{:?}", e);
                    set_result.set(error_msg.clone());
                    set_raw_json.set(error_msg);
                }
            }

            set_is_loading.set(false);
        });
    };

    // API Request: User
    let fetch_user = move |_| {
        if api_key.get().is_empty() {
            set_result.set(enter_api_key.get());
            set_raw_json.set(enter_api_key.get());
            return;
        }

        set_is_loading.set(true);
        set_result.set(loading_text.get());
        set_raw_json.set(loading_text.get());

        let key = api_key.get();
        let user = username.get();

        // Capture template strings for async context
        let no_user_found_str = no_user_found.get();
        let username_template = username_tpl.get();
        let user_id_template = user_id_tpl.get();
        let country_template = country_tpl.get();
        let pp_template = pp_tpl.get();
        let accuracy_template = accuracy_tpl.get();
        let global_rank_template = global_rank_tpl.get();
        let country_rank_template = country_rank_tpl.get();
        let playcount_template = playcount_tpl.get();
        let error_prefix_str = error_prefix.get();

        spawn_local(async move {
            let client = OsynicOsuApiV1GlooClient::new(key);
            client.set_proxy_url("https://osynic-cors.deno.dev/".to_string());
            let params = GetUserParams::default().user(user);

            match client.user.get_user(params).await {
                Ok(users) => {
                    // Set raw JSON
                    set_raw_json.set(format!("{:#?}", users));

                    if users.is_empty() {
                        set_result.set(no_user_found_str);
                    } else {
                        let user = &users[0];
                        let mut result_str = String::new();

                        // 修复：正确替换占位符
                        result_str
                            .push_str(&username_template.replace("{$username}", &user.username));
                        result_str.push_str("\n");
                        result_str.push_str(&user_id_template.replace("{$id}", &user.user_id));
                        result_str.push_str("\n");
                        result_str.push_str(&country_template.replace("{$country}", &user.country));
                        result_str.push_str("\n");

                        let pp_formatted = format!("{:.2}", user.pp_raw);
                        result_str.push_str(&pp_template.replace("{$pp}", &pp_formatted));
                        result_str.push_str("\n");

                        let accuracy_formatted = format!("{:.2}", user.accuracy);
                        result_str.push_str(
                            &accuracy_template.replace("{$accuracy}", &accuracy_formatted),
                        );
                        result_str.push_str("\n");

                        result_str
                            .push_str(&global_rank_template.replace("{$rank}", &user.pp_rank));
                        result_str.push_str("\n");
                        result_str.push_str(
                            &country_rank_template
                                .replace("{$country_rank}", &user.pp_country_rank),
                        );
                        result_str.push_str("\n");
                        result_str
                            .push_str(&playcount_template.replace("{$count}", &user.playcount));

                        set_result.set(result_str);
                    }
                }
                Err(e) => {
                    let error_msg = error_prefix_str + &format!("{:?}", e);
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
                            <label for="api_key" class="block text-sm font-medium mb-2 text-gray-700 dark:text-gray-200">{move || tr!("api-demo-label-api-key")}</label>
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
                            </ul>
                        </div>

                        // 谱面查询表单
                        <div class="mb-6" class:hidden=move || active_tab.get() != "beatmaps">
                            <label for="beatmap_id" class="block text-sm font-medium mb-2 text-gray-700 dark:text-gray-200">{move || tr!("api-demo-input-beatmap-label")}</label>
                            <div class="flex">
                                <input
                                    type="text"
                                    id="beatmap_id"
                                    class="w-full px-4 py-2 rounded-l-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-800 dark:text-gray-100 focus:ring-2 focus:ring-pink-500 focus:border-transparent transition-all duration-200"
                                    placeholder={move || tr!("api-demo-input-beatmap-placeholder")}
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
                                            <span class="mr-2">{move || tr!("api-demo-loading")}</span>
                                        }
                                    >
                                        <Search />
                                        <span class="mr-2 whitespace-nowrap inline-block w-full">{move || tr!("api-demo-search")}</span>
                                    </Show>
                                </button>
                            </div>
                        </div>

                        // 用户查询表单
                        <div class="mb-6" class:hidden=move || active_tab.get() != "users">
                            <label for="username" class="block text-sm font-medium mb-2 text-gray-700 dark:text-gray-200">{move || tr!("api-demo-input-user-label")}</label>
                            <div class="flex">
                                <input
                                    type="text"
                                    id="username"
                                    class="w-full px-4 py-2 rounded-l-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-800 dark:text-gray-100 focus:ring-2 focus:ring-pink-500 focus:border-transparent transition-all duration-200"
                                    placeholder={move || tr!("api-demo-input-user-placeholder")}
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
                                            <span class="mr-2">{move || tr!("api-demo-loading")}</span>
                                        }
                                    >
                                        <Search />
                                        <span class="mr-2 whitespace-nowrap inline-block w-full">{move || tr!("api-demo-search")}</span>
                                    </Show>
                                </button>
                            </div>
                        </div>

                        // 双面板结果显示
                        <div>
                            <div class="flex flex-col md:flex-row gap-4 mb-2">
                                <h3 class="text-lg font-medium text-gray-800 dark:text-gray-200">{move || tr!("api-demo-search-result")}</h3>
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
