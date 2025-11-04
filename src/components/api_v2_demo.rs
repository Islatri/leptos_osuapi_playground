use crate::components::oauth_panel::OAuthPanel;
use leptos::prelude::*;
use leptos::*;
use lucide_leptos::{Loader, Search, Settings, User, UserCheck};
use osynic_osuapi::v2::client::gloo::client::OsynicOsuApiV2GlooClient;
use osynic_osuapi::v2::interface::users::IUsers;
use osynic_osuapi::v2::model::oauth::structs::o_token::OToken;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;

#[component]
pub fn ApiV2Demo() -> impl IntoView {
    // State
    let (token, set_token) = signal::<Option<OToken>>(None);
    let (active_tab, set_active_tab) = signal("user_query");

    // Handle token changes from OAuth panel
    let handle_token_changed = move |new_token: Option<OToken>| {
        set_token.set(new_token);
    };

    view! {
        <div id="v2-demo">
            // OAuth Panel (Outside Tabs)
            <OAuthPanel on_token_changed=handle_token_changed />

            // Show tabs only when authenticated
            <Show when=move || token.get().is_some()>
                <div class="min-h-screen bg-gradient-to-br from-slate-950 via-slate-900 to-slate-950 text-slate-100">
                    <div class="py-6 px-4 mx-auto max-w-7xl sm:px-6 lg:px-8">
                        // Tabs Navigation
                        <div class="flex gap-1 mb-6 border-b border-slate-800">
                            <button
                                on:click=move |_| set_active_tab.set("user_query")
                                class=move || {
                                    let base = "px-4 py-3 text-sm font-medium border-b-2 transition-all flex items-center gap-2";
                                    let active = if active_tab.get() == "user_query" {
                                        "border-cyan-500 text-cyan-400"
                                    } else {
                                        "border-transparent text-slate-400 hover:text-slate-300"
                                    };
                                    format!("{} {}", base, active)
                                }
                            >
                                <Search size=16 />
                                "User Query"
                            </button>

                            <button
                                on:click=move |_| set_active_tab.set("stats")
                                class=move || {
                                    let base = "px-4 py-3 text-sm font-medium border-b-2 transition-all flex items-center gap-2";
                                    let active = if active_tab.get() == "stats" {
                                        "border-cyan-500 text-cyan-400"
                                    } else {
                                        "border-transparent text-slate-400 hover:text-slate-300"
                                    };
                                    format!("{} {}", base, active)
                                }
                            >
                                <UserCheck size=16 />
                                "Statistics (Placeholder)"
                            </button>

                            // Future extension placeholders
                            <button
                                disabled=true
                                class="flex gap-2 items-center py-3 px-4 text-sm font-medium border-b-2 border-transparent opacity-50 transition-all cursor-not-allowed text-slate-500 hover:text-slate-400"
                            >
                                <Settings size=16 />
                                "More Features"
                            </button>
                        </div>

                        // Tab Content
                        <Show when=move || active_tab.get() == "user_query">
                            <UserQueryTab token=token />
                        </Show>

                        <Show when=move || active_tab.get() == "stats">
                            <StatsPlaceholderTab />
                        </Show>
                    </div>
                </div>
            </Show>

            // Show message when not authenticated
            <Show when=move || token.get().is_none()>
                <div class="flex justify-center items-center min-h-screen bg-gradient-to-br from-slate-950 via-slate-900 to-slate-950 text-slate-100">
                    <div class="text-center">
                        <User size=64 color="#475569" />
                        <p class="text-slate-400">
                            "Please authenticate using the OAuth panel above to access user data."
                        </p>
                    </div>
                </div>
            </Show>
        </div>
    }
}

#[component]
fn UserQueryTab(token: ReadSignal<Option<OToken>>) -> impl IntoView {
    let (query, set_query) = signal("".to_string());
    let (search_result, set_search_result) = signal("".to_string());
    let (raw_json, set_raw_json) = signal("".to_string());
    let (loading, set_loading) = signal(false);
    let (show_json, set_show_json) = signal(false);

    let handle_query_input = move |ev| {
        let input_element = event_target::<HtmlInputElement>(&ev);
        set_query.set(input_element.value());
    };

    let handle_search = move |_| {
        let q = query.get();
        if q.is_empty() {
            set_search_result.set("Please enter a username to search".to_string());
            return;
        }

        set_loading.set(true);
        set_search_result.set("Searching...".to_string());
        set_raw_json.set("".to_string());

        // Get the token
        let token_data = token.get();
        if token_data.is_none() {
            set_search_result.set("No token available. Please authenticate first.".to_string());
            set_loading.set(false);
            return;
        }

        let token_data = token_data.unwrap();
        let username_query = q.clone();

        spawn_local(async move {
            // Use v2 API with OAuth token - convert our OToken to OToken
            let v2_token = OToken {
                access_token: token_data.access_token.clone(),
                refresh_token: token_data.refresh_token.clone(),
                expires_in: token_data.expires_in,
                token_type: token_data.token_type.clone(),
            };

            let client = OsynicOsuApiV2GlooClient::new(v2_token);
            // client.set_proxy_url("https://osynic-cors.deno.dev/".to_string());
            client.set_proxy_url("http://localhost:8000/".to_string());

            // Parse username_query as u32 (user ID)
            let user_id: u32 = match username_query.parse() {
                Ok(id) => id,
                Err(_) => {
                    set_search_result.set(format!("Please enter a valid user ID (numeric)"));
                    set_loading.set(false);
                    return;
                }
            };

            match client.users.get_user(user_id, None, None).await {
                Ok(user) => {
                    // Set raw JSON
                    set_raw_json.set(format!("{:#?}", user));

                    let mut result_str = String::new();
                    result_str.push_str(&format!("👤 Username: {}\n", user.username));
                    result_str.push_str(&format!("🆔 User ID: {}\n", user.id));
                    result_str.push_str(&format!("🌍 Country: {}\n", user.country_code));

                    if let Some(statistics) = &user.statistics {
                        result_str.push_str(&format!("⭐ PP: {:.2}\n", statistics.pp));
                        result_str
                            .push_str(&format!("🎯 Accuracy: {:.2}%\n", statistics.hit_accuracy));
                        result_str.push_str(&format!(
                            "🏆 Global Rank: #{}\n",
                            statistics.global_rank.unwrap_or(0)
                        ));
                        result_str.push_str(&format!(
                            "🚩 Country Rank: #{}\n",
                            statistics.country_rank.unwrap_or(0)
                        ));
                        result_str.push_str(&format!("🎮 Play Count: {}\n", statistics.play_count));
                        result_str.push_str(&format!(
                            "⏱️ Play Time: {:.0}h\n",
                            statistics.play_time as f64 / 3600.0
                        ));
                    }

                    set_search_result.set(result_str);
                }
                Err(e) => {
                    let error_msg = format!("Error: {:?}", e);
                    set_search_result.set(error_msg.clone());
                    set_raw_json.set(error_msg);
                }
            }

            set_loading.set(false);
        });
    };

    view! {
        <div class="space-y-6">
            // Search Card
            <div class="p-6 rounded-lg border bg-slate-800/50 border-slate-700/50">
                <h2 class="flex gap-3 items-center mb-6 text-lg font-semibold text-white">
                    <Search size=20 color="#06b6d4" />
                    "Search osu! Users"
                </h2>

                <div class="space-y-4">
                    <div>
                        <label class="block mb-2 text-sm font-medium text-slate-300">
                            "User ID"
                        </label>
                        <input
                            type="text"
                            placeholder="Enter user ID (numeric)..."
                            value=move || query.get()
                            on:input=handle_query_input
                            class="py-2.5 px-4 w-full rounded-lg border transition-all focus:border-cyan-500 focus:outline-none bg-slate-900 border-slate-700 text-slate-100 placeholder-slate-500"
                        />
                    </div>

                    <button
                        on:click=handle_search
                        disabled=move || loading.get() || query.get().is_empty()
                        class="flex gap-2 justify-center items-center py-3 w-full font-medium text-white bg-gradient-to-r from-cyan-500 to-cyan-600 rounded-lg transition-all hover:from-cyan-600 hover:to-cyan-700 disabled:cursor-not-allowed disabled:from-slate-600 disabled:to-slate-600"
                    >
                        <Show
                            when=move || !loading.get()
                            fallback=|| {
                                view! {
                                    <Loader size=16 color="#ffffff" />
                                    "Searching..."
                                }
                            }
                        >
                            <Search size=16 color="#ffffff" />
                            "Search"
                        </Show>
                    </button>
                </div>

                // Results
                <Show when=move || !search_result.get().is_empty()>
                    <div class="mt-6">
                        <div class="p-4 font-mono text-sm whitespace-pre-wrap rounded-lg border bg-slate-900/50 border-slate-700/30 text-slate-300">
                            {search_result}
                        </div>

                        <Show when=move || !raw_json.get().is_empty()>
                            <button
                                on:click=move |_| set_show_json.set(!show_json.get())
                                class="py-2 px-4 mt-4 text-sm rounded-lg transition-all bg-slate-700 text-slate-200 hover:bg-slate-600"
                            >
                                {move || {
                                    if show_json.get() { "Hide Raw JSON" } else { "Show Raw JSON" }
                                }}
                            </button>
                        </Show>

                        <Show when=move || show_json.get() && !raw_json.get().is_empty()>
                            <div class="overflow-x-auto p-4 mt-4 max-h-96 font-mono text-xs whitespace-pre-wrap rounded-lg border bg-slate-900/70 border-slate-700/50 text-slate-400">
                                {raw_json}
                            </div>
                        </Show>
                    </div>
                </Show>
            </div>

            // Info Box
            <div class="p-6 rounded-lg border bg-slate-800/50 border-slate-700/50">
                <h3 class="mb-3 text-sm font-semibold text-slate-300">
                    "ℹ️ Integration Notes"
                </h3>
                <ul class="space-y-2 text-xs text-slate-400">
                    <li>"• This tab searches osu! users from the official osu!api v2"</li>
                    <li>"• Enter a numeric user ID (not username) to search"</li>
                    <li>"• Your OAuth token is used for API authentication"</li>
                    <li>"• You can view both formatted results and raw JSON responses"</li>
                    <li>"• CORS proxy is used: osynic-cors.deno.dev"</li>
                </ul>
            </div>
        </div>
    }
}

#[component]
fn StatsPlaceholderTab() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <div class="p-6 rounded-lg border bg-slate-800/50 border-slate-700/50">
                <h2 class="flex gap-3 items-center mb-6 text-lg font-semibold text-white">
                    <UserCheck size=20 color="#06b6d4" />
                    "Statistics & Analysis"
                </h2>

                <div class="py-12 text-center">
                    <UserCheck size=48 color="#475569" />
                    <p class="mb-4 text-slate-400">"Statistics feature is under development"</p>
                    <p class="text-sm text-slate-500">
                        "This tab is reserved for future features like:"
                    </p>
                    <ul class="mt-4 space-y-2 text-sm text-slate-500">
                        <li>"• User profile statistics visualization"</li>
                        <li>"• Performance tracking over time"</li>
                        <li>"• Grade distribution charts"</li>
                        <li>"• Custom analysis tools"</li>
                    </ul>
                </div>
            </div>

            // Extension Guidelines
            <div class="p-6 rounded-lg border bg-slate-800/50 border-slate-700/50">
                <h3 class="mb-3 text-sm font-semibold text-slate-300">
                    "🛠️ How to Add Features"
                </h3>
                <div class="space-y-4 text-xs text-slate-400">
                    <div>
                        <p class="mb-1 font-medium text-slate-300">
                            "1. Create a New Tab Component"
                        </p>
                        <p>"Create a new component function like:"</p>
                        <code class="block overflow-x-auto p-2 mt-1 font-mono text-xs rounded bg-slate-900/50 text-slate-300">
                            "#[component]\nfn NewFeatureTab(token: ReadSignal&lt;Option&lt;OToken&gt;&gt;) -> impl IntoView {{"
                        </code>
                    </div>
                    <div>
                        <p class="mb-1 font-medium text-slate-300">"2. Add Tab Button"</p>
                        <p>"Add a button in ExampleV2 component to toggle your new tab"</p>
                    </div>
                    <div>
                        <p class="mb-1 font-medium text-slate-300">"3. Show Conditional"</p>
                        <p>"Use Show component to display your tab when active"</p>
                    </div>
                </div>
            </div>
        </div>
    }
}
