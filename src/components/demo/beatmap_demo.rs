use leptos::prelude::*;
use leptos::*;
use leptos_fluent::tr;
use lucide_leptos::{Loader, Search};
use osynic_osuapi::v1::client::gloo::client::OsynicOsuApiV1GlooClient;
use osynic_osuapi::v1::interface::beatmap::IBeatmap;
use osynic_osuapi::v1::model::beatmap::GetBeatmapsParams;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;

#[component]
pub fn BeatmapDemo(
    api_key: ReadSignal<String>,
    set_result: WriteSignal<String>,
    set_raw_json: WriteSignal<String>,
    is_loading: ReadSignal<bool>,
    set_is_loading: WriteSignal<bool>,
) -> impl IntoView {
    // Static strings for internationalization
    let enter_api_key = Memo::new(move |_| tr!("api-demo-enter-api-key"));
    let loading_text = Memo::new(move |_| tr!("api-demo-loading-text"));
    let no_beatmap_found = Memo::new(move |_| tr!("api-demo-no-beatmap-found"));
    let error_prefix = Memo::new(move |_| tr!("api-demo-error", {"error" => ""}));

    // Beatmap result templates
    let beatmap_number_tpl = Memo::new(move |_| tr!("api-demo-beatmap-number", {"number" => "{$number}"}));
    let title_tpl = Memo::new(move |_| tr!("api-demo-title", {"title" => "{$title}"}));
    let artist_tpl = Memo::new(move |_| tr!("api-demo-artist", {"artist" => "{$artist}"}));
    let version_tpl = Memo::new(move |_| tr!("api-demo-version", {"version" => "{$version}"}));
    let bpm_tpl = Memo::new(move |_| tr!("api-demo-bpm", {"bpm" => "{$bpm}"}));
    let stars_tpl = Memo::new(move |_| tr!("api-demo-stars", {"stars" => "{$stars}"}));

    // State
    let (beatmap_id, set_beatmap_id) = signal("114514".to_string());

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
                            let beatmap_num =
                                beatmap_number_template.replace("{$number}", &(i + 1).to_string());
                            result_str.push_str(&beatmap_num);
                            result_str.push_str("\n");

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

    let handle_beatmap_id_input = move |ev| {
        let input_element = event_target::<HtmlInputElement>(&ev);
        set_beatmap_id.set(input_element.value());
    };

    view! {
        <div class="mb-6">
            <label
                for="beatmap_id"
                class="block mb-2 text-sm font-medium text-gray-700 dark:text-gray-200"
            >
                {move || tr!("api-demo-input-beatmap-label")}
            </label>
            <div class="flex">
                <input
                    type="text"
                    id="beatmap_id"
                    class="py-2 px-4 w-full text-gray-800 bg-white rounded-l-lg border border-gray-300 transition-all duration-200 dark:text-gray-100 dark:bg-gray-700 dark:border-gray-600 focus:border-transparent focus:ring-2 focus:ring-pink-500"
                    placeholder=move || tr!("api-demo-input-beatmap-placeholder")
                    value=beatmap_id
                    on:input=handle_beatmap_id_input
                />
                <button
                    class="flex justify-center items-center py-2 px-6 text-white bg-pink-600 rounded-r-lg transition-all duration-200 hover:bg-pink-700 disabled:opacity-50 disabled:cursor-not-allowed"
                    on:click=fetch_beatmap
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
    }
}