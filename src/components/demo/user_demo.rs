use leptos::prelude::*;
use leptos::*;
use leptos_fluent::tr;
use lucide_leptos::{Loader, Search};
use osynic_osuapi::v1::client::gloo::client::OsynicOsuApiV1GlooClient;
use osynic_osuapi::v1::interface::user::IUser;
use osynic_osuapi::v1::model::user::GetUserParams;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;

#[component]
pub fn UserDemo(
    api_key: ReadSignal<String>,
    set_result: WriteSignal<String>,
    set_raw_json: WriteSignal<String>,
    is_loading: ReadSignal<bool>,
    set_is_loading: WriteSignal<bool>,
) -> impl IntoView {
    // Static strings for internationalization
    let enter_api_key = Memo::new(move |_| tr!("api-demo-enter-api-key"));
    let loading_text = Memo::new(move |_| tr!("api-demo-loading-text"));
    let no_user_found = Memo::new(move |_| tr!("api-demo-no-user-found"));
    let error_prefix = Memo::new(move |_| tr!("api-demo-error", {"error" => ""}));

    // User result templates
    let username_tpl = Memo::new(move |_| tr!("api-demo-username", {"username" => "{$username}"}));
    let user_id_tpl = Memo::new(move |_| tr!("api-demo-user-id", {"id" => "{$id}"}));
    let country_tpl = Memo::new(move |_| tr!("api-demo-country", {"country" => "{$country}"}));
    let pp_tpl = Memo::new(move |_| tr!("api-demo-pp", {"pp" => "{$pp}"}));
    let accuracy_tpl = Memo::new(move |_| tr!("api-demo-accuracy", {"accuracy" => "{$accuracy}"}));
    let global_rank_tpl = Memo::new(move |_| tr!("api-demo-global-rank", {"rank" => "{$rank}"}));
    let country_rank_tpl = Memo::new(move |_| tr!("api-demo-country-rank", {"country_rank" => "{$country_rank}"}));
    let playcount_tpl = Memo::new(move |_| tr!("api-demo-playcount", {"count" => "{$count}"}));

    // State
    let (username, set_username) = signal("peppy".to_string());

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

    let handle_username_input = move |ev| {
        let input_element = event_target::<HtmlInputElement>(&ev);
        set_username.set(input_element.value());
    };

    view! {
        <div class="mb-6">
            <label
                for="username"
                class="block mb-2 text-sm font-medium text-gray-700 dark:text-gray-200"
            >
                {move || tr!("api-demo-input-user-label")}
            </label>
            <div class="flex">
                <input
                    type="text"
                    id="username"
                    class="py-2 px-4 w-full text-gray-800 bg-white rounded-l-lg border border-gray-300 transition-all duration-200 dark:text-gray-100 dark:bg-gray-700 dark:border-gray-600 focus:border-transparent focus:ring-2 focus:ring-pink-500"
                    placeholder=move || tr!("api-demo-input-user-placeholder")
                    value=username
                    on:input=handle_username_input
                />
                <button
                    class="flex justify-center items-center py-2 px-6 text-white bg-pink-600 rounded-r-lg transition-all duration-200 hover:bg-pink-700 disabled:opacity-50 disabled:cursor-not-allowed"
                    on:click=fetch_user
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