use leptos::prelude::*;
use leptos::*;
use leptos_fluent::tr;
use lucide_leptos::{Loader, Search};
use osynic_osuapi::v1::client::gloo::client::OsynicOsuApiV1GlooClient;
use osynic_osuapi::v1::interface::user::IUser;
use osynic_osuapi::v1::model::recent::GetUserRecentParams;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;

#[component]
pub fn UserRecentDemo(
    api_key: ReadSignal<String>,
    set_result: WriteSignal<String>,
    set_raw_json: WriteSignal<String>,
    is_loading: ReadSignal<bool>,
    set_is_loading: WriteSignal<bool>,
) -> impl IntoView {
    // Static strings for internationalization
    let enter_api_key = Memo::new(move |_| tr!("api-demo-enter-api-key"));
    let loading_text = Memo::new(move |_| tr!("api-demo-loading-text"));
    let no_recent_found = Memo::new(move |_| tr!("api-demo-no-recent-found"));
    let error_prefix = Memo::new(move |_| tr!("api-demo-error", {"error" => ""}));

    // Recent play result templates
    let recent_play_tpl = Memo::new(move |_| tr!("api-demo-recent-play", {"index" => "{$index}"}));
    let beatmap_id_tpl = Memo::new(move |_| tr!("api-demo-beatmap-id", {"id" => "{$id}"}));
    let score_tpl = Memo::new(move |_| tr!("api-demo-score", {"score" => "{$score}"}));
    let combo_tpl = Memo::new(move |_| tr!("api-demo-combo", {"combo" => "{$combo}"}));
    let accuracy_hits_tpl = Memo::new(move |_| tr!("api-demo-accuracy-hits", {"c300" => "{$c300}", "c100" => "{$c100}", "c50" => "{$c50}", "miss" => "{$miss}"}));
    let rank_tpl = Memo::new(move |_| tr!("api-demo-rank", {"rank" => "{$rank}"}));
    let mods_tpl = Memo::new(move |_| tr!("api-demo-mods", {"mods" => "{$mods}"}));
    let date_tpl = Memo::new(move |_| tr!("api-demo-date", {"date" => "{$date}"}));
    let perfect_tpl = Memo::new(move |_| tr!("api-demo-perfect", {"perfect" => "{$perfect}"}));

    // State
    let (username, set_username) = signal("Islatri".to_string());
    let (mode, set_mode) = signal(0u8);
    let (limit, set_limit) = signal(10u32);

    // API Request: User Recent
    let fetch_user_recent = move |_| {
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
        let selected_mode = mode.get();
        let selected_limit = limit.get();

        // Capture template strings for async context
        let no_recent_found_str = no_recent_found.get();
        let recent_play_template = recent_play_tpl.get();
        let beatmap_id_template = beatmap_id_tpl.get();
        let score_template = score_tpl.get();
        let combo_template = combo_tpl.get();
        let accuracy_hits_template = accuracy_hits_tpl.get();
        let rank_template = rank_tpl.get();
        let mods_template = mods_tpl.get();
        let date_template = date_tpl.get();
        let perfect_template = perfect_tpl.get();
        let error_prefix_str = error_prefix.get();

        spawn_local(async move {
            let client = OsynicOsuApiV1GlooClient::new(key);
            client.set_proxy_url("https://osynic-cors.deno.dev/".to_string());
            let params = GetUserRecentParams::default()
                .user(user)
                .mode(selected_mode)
                .limit(selected_limit);

            match client.user.get_user_recent(params).await {
                Ok(recents) => {
                    // Set raw JSON
                    set_raw_json.set(format!("{:#?}", recents));

                    if recents.is_empty() {
                        set_result.set(no_recent_found_str);
                    } else {
                        let mut result_str = String::new();

                        for (idx, recent) in recents.iter().enumerate() {
                            if idx > 0 {
                                result_str.push_str("\n---\n");
                            }

                            // Play number
                            result_str.push_str(
                                &recent_play_template.replace("{$index}", &(idx + 1).to_string()),
                            );
                            result_str.push_str("\n");

                            // Beatmap ID
                            result_str.push_str(
                                &beatmap_id_template.replace("{$id}", &recent.beatmap_id),
                            );
                            result_str.push_str("\n");

                            // Score
                            result_str.push_str(&score_template.replace("{$score}", &recent.score));
                            result_str.push_str("\n");

                            // Combo
                            result_str.push_str(&combo_template.replace("{$combo}", &recent.maxcombo));
                            result_str.push_str("\n");

                            // Accuracy hits
                            let hits = accuracy_hits_template
                                .replace("{$c300}", &recent.count300)
                                .replace("{$c100}", &recent.count100)
                                .replace("{$c50}", &recent.count50)
                                .replace("{$miss}", &recent.countmiss);
                            result_str.push_str(&hits);
                            result_str.push_str("\n");

                            // Rank
                            result_str.push_str(&rank_template.replace("{$rank}", &recent.rank));
                            result_str.push_str("\n");

                            // Mods
                            result_str.push_str(&mods_template.replace("{$mods}", &recent.enabled_mods));
                            result_str.push_str("\n");

                            // Date
                            result_str.push_str(&date_template.replace("{$date}", &recent.date));
                            result_str.push_str("\n");

                            // Perfect combo
                            let perfect_status = if recent.perfect == "1" { "Yes" } else { "No" };
                            result_str.push_str(&perfect_template.replace("{$perfect}", perfect_status));
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

    let handle_username_input = move |ev| {
        let input_element = event_target::<HtmlInputElement>(&ev);
        set_username.set(input_element.value());
    };

    let handle_mode_change = move |ev| {
        let input_element = event_target::<HtmlInputElement>(&ev);
        if let Ok(value) = input_element.value().parse::<u8>() {
            set_mode.set(value);
        }
    };

    let handle_limit_change = move |ev| {
        let input_element = event_target::<HtmlInputElement>(&ev);
        if let Ok(value) = input_element.value().parse::<u32>() {
            let clamped = value.clamp(1, 50);
            set_limit.set(clamped);
        }
    };

    view! {
        <div class="mb-6 space-y-4">
            // Username Input with Search Button
            <div>
                <label
                    for="username-recent"
                    class="block mb-2 text-sm font-medium text-gray-700 dark:text-gray-200"
                >
                    {move || tr!("api-demo-input-user-label")}
                </label>
                <div class="flex">
                    <input
                        type="text"
                        id="username-recent"
                        class="py-2 px-4 w-full text-gray-800 bg-white rounded-l-lg border border-gray-300 transition-all duration-200 dark:text-gray-100 dark:bg-gray-700 dark:border-gray-600 focus:border-transparent focus:ring-2 focus:ring-pink-500"
                        placeholder=move || tr!("api-demo-input-user-placeholder")
                        value=username
                        on:input=handle_username_input
                    />
                    <button
                        class="flex justify-center items-center py-2 px-6 text-white bg-pink-600 rounded-r-lg transition-all duration-200 hover:bg-pink-700 disabled:opacity-50 disabled:cursor-not-allowed"
                        on:click=fetch_user_recent
                        disabled=is_loading
                    >
                        <Show
                            when=move || !is_loading.get()
                            fallback=|| {
                                view! {
                                    <Loader />
                                    <span class="mr-2">{move || tr!("api-demo-loading")}</span>
                                }
                            }
                        >
                            <Search />
                            <span class="inline-block mr-2 w-full whitespace-nowrap">
                                {move || tr!("api-demo-search")}
                            </span>
                        </Show>
                    </button>
                </div>
            </div>

            // Mode and Limit in a grid
            <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
                // Mode Select
                <div>
                    <label
                        for="mode"
                        class="block mb-2 text-sm font-medium text-gray-700 dark:text-gray-200"
                    >
                        {move || tr!("api-demo-mode-label")}
                    </label>
                    <select
                        id="mode"
                        class="py-2 px-4 w-full text-gray-800 bg-white rounded-lg border border-gray-300 transition-all duration-200 dark:text-gray-100 dark:bg-gray-700 dark:border-gray-600 focus:border-transparent focus:ring-2 focus:ring-pink-500"
                        on:change=handle_mode_change
                    >
                        <option value="0" selected=move || mode.get() == 0>
                            {move || tr!("api-demo-mode-osu")}
                        </option>
                        <option value="1" selected=move || mode.get() == 1>
                            {move || tr!("api-demo-mode-taiko")}
                        </option>
                        <option value="2" selected=move || mode.get() == 2>
                            {move || tr!("api-demo-mode-ctb")}
                        </option>
                        <option value="3" selected=move || mode.get() == 3>
                            {move || tr!("api-demo-mode-mania")}
                        </option>
                    </select>
                </div>

                // Limit Input
                <div>
                    <label
                        for="limit"
                        class="block mb-2 text-sm font-medium text-gray-700 dark:text-gray-200"
                    >
                        {move || tr!("api-demo-limit-label")}
                    </label>
                    <input
                        type="number"
                        id="limit"
                        min="1"
                        max="50"
                        class="py-2 px-4 w-full text-gray-800 bg-white rounded-lg border border-gray-300 transition-all duration-200 dark:text-gray-100 dark:bg-gray-700 dark:border-gray-600 focus:border-transparent focus:ring-2 focus:ring-pink-500"
                        placeholder=move || tr!("api-demo-limit-placeholder")
                        value=limit
                        on:input=handle_limit_change
                    />
                </div>
            </div>
        </div>
    }
}