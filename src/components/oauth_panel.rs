use leptos::prelude::*;
use leptos::*;
use lucide_leptos::{ExternalLink, LogIn, LogOut, RefreshCw, Star};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use web_sys::{HtmlInputElement, window};
use leptos_fluent::tr;

use osynic_osuapi::v2::model::oauth::structs::o_token::OToken;

const TOKEN_STORAGE_KEY: &str = "osu_api_token";

// Storage wrapper that includes timestamp for expiry checking
#[derive(Clone, Debug, Serialize, Deserialize)]
struct StoredOToken {
    #[serde(flatten)]
    token: OToken,
    stored_at: i64,
}

#[component]
pub fn OAuthPanel(
    on_token_changed: impl Fn(Option<OToken>) + 'static + Send + Sync,
) -> impl IntoView {
    // Wrap callback in Arc for multiple usage
    let on_token_changed = Arc::new(on_token_changed);
    // State variables
    let (client_id, set_client_id) = signal("45458".to_string());
    let (client_secret, set_client_secret) = signal("PLS_KEEP_YOUR_CLIENT_SECRET_SAFE_AND_DO_NOT_SHARE_IT_IN_FRONTEND_CODE".to_string());
    let (redirect_uri, set_redirect_uri) = signal("https://osynic-oauth.deno.dev/callback".to_string());
    let (proxy_url, set_proxy_url) = signal("https://osynic-cors.deno.dev/".to_string());
    let (_token, set_token) = signal::<Option<OToken>>(None);
    let (loading, set_loading) = signal(false);
    let (error, set_error) = signal("".to_string());
    let (is_authenticated, set_is_authenticated) = signal(false);

    // Helper: Load token from localStorage
    let load_token_from_storage = move || -> Option<OToken> {
        if let Some(window) = window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(stored_str)) = storage.get_item(TOKEN_STORAGE_KEY) {
                    if let Ok(stored) = serde_json::from_str::<StoredOToken>(&stored_str) {
                        let now = js_sys::Date::now() as i64 / 1000;
                        let elapsed = now - stored.stored_at;

                        // Check if token is still valid (hasn't expired)
                        if elapsed < stored.token.expires_in as i64 {
                            return Some(stored.token);
                        }
                    }
                }
            }
        }
        None
    };

    // Helper: Save token to localStorage
    let save_token_to_storage = move |token_data: &OToken| {
        if let Some(window) = window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let stored = StoredOToken {
                    token: token_data.clone(),
                    stored_at: js_sys::Date::now() as i64 / 1000,
                };
                if let Ok(json_str) = serde_json::to_string(&stored) {
                    let _ = storage.set_item(TOKEN_STORAGE_KEY, &json_str);
                }
            }
        }
    };

    // Helper: Clear stored token
    let clear_stored_token = move || {
        if let Some(window) = window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.remove_item(TOKEN_STORAGE_KEY);
            }
        }
    };

    // Helper: Parse OAuth callback from URL hash
    let parse_oauth_callback = move || -> Option<OToken> {
        if let Some(window) = window() {
            if let Ok(location) = window.location().hash() {
                if !location.is_empty() {
                    let hash = location.trim_start_matches('#');
                    let parts: Vec<&str> = hash.split('&').collect();
                    let mut token_data = OToken {
                        access_token: String::new(),
                        refresh_token: None,
                        expires_in: 0,
                        token_type: String::new(),
                    };

                    for part in parts {
                        let kv: Vec<&str> = part.split('=').collect();
                        if kv.len() == 2 {
                            match kv[0] {
                                "access_token" => token_data.access_token = kv[1].to_string(),
                                "refresh_token" => {
                                    token_data.refresh_token = Some(kv[1].to_string())
                                }
                                "expires_in" => token_data.expires_in = kv[1].parse().unwrap_or(0),
                                "token_type" => token_data.token_type = kv[1].to_string(),
                                _ => {}
                            }
                        }
                    }

                    if !token_data.access_token.is_empty() && !token_data.token_type.is_empty() {
                        return Some(token_data);
                    }
                }
            }
        }
        None
    };

    // Initialize on mount
    Effect::new({
        let on_token_changed = on_token_changed.clone();
        move || {
            // Try parsing OAuth callback first
            if let Some(callback_token) = parse_oauth_callback() {
                if let Some(window) = window() {
                    let _ = window.location().set_hash("");
                }
                save_token_to_storage(&callback_token);
                set_token.set(Some(callback_token.clone()));
                set_is_authenticated.set(true);
                on_token_changed(Some(callback_token));
                return;
            }

            // Try loading from localStorage
            if let Some(stored_token) = load_token_from_storage() {
                set_token.set(Some(stored_token.clone()));
                set_is_authenticated.set(true);
                on_token_changed(Some(stored_token));
            }
        }
    });

    // Computed auth URL
    let auth_url = move || {
        if !client_id.get().is_empty() && !redirect_uri.get().is_empty() {
            let scopes = "public identify";
            let encoded_redirect = js_sys::encode_uri_component(&redirect_uri.get());
            let encoded_scopes = js_sys::encode_uri_component(scopes);
            format!(
                "https://osu.ppy.sh/oauth/authorize?client_id={}&redirect_uri={}&response_type=code&scope={}",
                client_id.get(),
                encoded_redirect,
                encoded_scopes
            )
        } else {
            String::new()
        }
    };

    // Handlers
    let handle_auth = move |_| {
        let url = auth_url();
        if !url.is_empty() {
            if let Some(window) = window() {
                let _ = window.open_with_url_and_target(&url, "_blank");
            }
        }
    };

    let handle_client_id_input = move |ev| {
        let input_element = event_target::<HtmlInputElement>(&ev);
        set_client_id.set(input_element.value());
    };

    let handle_client_secret_input = move |ev| {
        let input_element = event_target::<HtmlInputElement>(&ev);
        set_client_secret.set(input_element.value());
    };

    let handle_redirect_uri_input = move |ev| {
        let input_element = event_target::<HtmlInputElement>(&ev);
        set_redirect_uri.set(input_element.value());
    };

    let handle_proxy_url_input = move |ev| {
        let input_element = event_target::<HtmlInputElement>(&ev);
        set_proxy_url.set(input_element.value());
    };

    view! {
        <div class="overflow-x-hidden h-full bg-gradient-to-br from-gray-50 via-gray-100 to-gray-50 dark:from-slate-950 dark:via-slate-900 dark:to-slate-950 text-gray-900 dark:text-slate-100">
            // Background accent
            <div class="overflow-hidden fixed inset-0 pointer-events-none">
                <div class="absolute top-0 right-0 w-96 h-96 rounded-full bg-pink-500/5 dark:bg-cyan-500/5 blur-3xl"></div>
                <div class="absolute bottom-0 left-0 w-96 h-96 rounded-full bg-pink-500/5 dark:bg-cyan-500/5 blur-3xl"></div>
            </div>

            <div class="relative z-10">
                // Header
                <div class="mb-4">
                    <div class="py-4 px-4 mx-auto max-w-7xl sm:px-6 lg:px-8">
                        <div class="flex justify-between items-center">
                            <div class="flex gap-3 items-center">
                                <div class="flex justify-center items-center w-10 h-10 bg-gradient-to-br from-pink-400 dark:from-pink-300 to-pink-600 dark:to-pink-500 rounded-lg">
                                    <Star size=20 color="white" />
                                </div>
                                <div>
                                    <h1 class="text-xl font-bold text-gray-900 dark:text-white">"osu!api V2 OAuth Panel"</h1>
                                    <p class="text-xs text-gray-600 dark:text-slate-400">
                                        "Authentication & Configuration"
                                    </p>
                                </div>
                            </div>
                            <Show
                                when=move || is_authenticated.get()
                                fallback=|| view! { <div></div> }
                            >
                                <div class="py-2 px-4 text-sm rounded border bg-green-100 dark:bg-green-800 border-green-300 dark:border-green-700 text-green-700 dark:text-green-300">
                                    "Authenticated"
                                </div>
                            </Show>
                        </div>
                    </div>
                </div>

                <h2 class="mb-4 text-3xl font-bold text-center text-pink-600 dark:text-pink-400">
                    {move || tr!("api-v2-demo-title-1")}
                </h2>
                <p class="mx-auto mb-2 max-w-2xl text-center text-gray-600 dark:text-gray-300">
                    {move || tr!("api-v2-demo-description-1")}
                </p>
                <p class="mx-auto mb-2 max-w-2xl text-center text-gray-600 dark:text-gray-300">
                    {move || tr!("api-v2-demo-description-2")}
                </p>
                <p class="mx-auto mb-2 max-w-3xl text-center text-gray-600 dark:text-gray-300">
                    {move || tr!("api-v2-demo-description-3")}
                    <a
                        href="https://osu.ppy.sh/home/account/edit"
                        target="_blank"
                        class="font-medium text-pink-600 transition-all duration-200 hover:text-pink-700"
                    >
                        {move || tr!("api-demo-account-settings")}
                    </a> {move || tr!("api-v2-demo-api-section")}
                </p>
                <p class="mx-auto mb-2 max-w-3xl text-center text-gray-600 dark:text-gray-300">
                    {move || tr!("api-v2-demo-description-4")}
                    <a
                        href="https://osynic-oauth.deno.dev"
                        target="_blank"
                        class="font-medium text-pink-600 transition-all duration-200 hover:text-pink-700"
                    >
                        "osynic-oauth.deno.dev"
                    </a> 
                </p>
                <div class="py-6 px-4 mx-auto max-w-7xl sm:px-6 lg:px-8">
                    // Error Alert
                    <Show when=move || !error.get().is_empty()>
                        <div class="p-4 mb-6 text-sm text-red-700 dark:text-red-300 rounded-lg border bg-red-100 dark:bg-red-950/30 border-red-300 dark:border-red-900/50">
                            {error}
                        </div>
                    </Show>

                    // OAuth Configuration Card
                    <div class="p-6 rounded-lg border bg-white dark:bg-slate-800/50 border-gray-200 dark:border-slate-700/50">
                        <h2 class="flex gap-3 items-center mb-2 text-lg font-semibold text-gray-900 dark:text-white">
                            <LogIn size=20 />
                            "OAuth Configuration"
                        </h2>

                        <p class="mb-4 text-sm text-gray-600 dark:text-slate-400">
                            "Inputs are disabled because they are constants FOR this website. Just click the authorize button below."
                        </p>

                        <div class="mb-8 grid grid-cols-1 md:grid-cols-2 gap-4">
                            // Client ID
                            <div>
                                <label class="block mb-2 text-sm font-medium text-gray-700 dark:text-slate-300">
                                    "Client ID"
                                </label>
                                <input
                                    type="text"
                                    disabled
                                    placeholder="Enter your client ID"
                                    value=move || client_id.get()
                                    on:input=handle_client_id_input
                                    class="py-2.5 px-4 w-full rounded-lg border transition-all focus:border-pink-500 dark:focus:border-cyan-500 focus:outline-none bg-white dark:bg-slate-900 border-gray-300 dark:border-slate-700 text-gray-900 dark:text-slate-100 placeholder-gray-500 dark:placeholder-slate-500 disabled:bg-gray-100 dark:disabled:bg-slate-800 disabled:text-gray-500 dark:disabled:text-slate-500"
                                />
                            </div>

                            // Client Secret
                            <div>
                                <label class="block mb-2 text-sm font-medium text-gray-700 dark:text-slate-300">
                                    "Client Secret"
                                </label>
                                <input
                                    type="password"
                                    disabled
                                    placeholder="Enter your client secret"
                                    value=move || client_secret.get()
                                    on:input=handle_client_secret_input
                                    class="py-2.5 px-4 w-full rounded-lg border transition-all focus:border-pink-500 dark:focus:border-cyan-500 focus:outline-none bg-white dark:bg-slate-900 border-gray-300 dark:border-slate-700 text-gray-900 dark:text-slate-100 placeholder-gray-500 dark:placeholder-slate-500 disabled:bg-gray-100 dark:disabled:bg-slate-800 disabled:text-gray-500 dark:disabled:text-slate-500"
                                />
                            </div>

                            // Redirect URI
                            <div>
                                <label class="block mb-2 text-sm font-medium text-gray-700 dark:text-slate-300">
                                    "Redirect URI"
                                </label>
                                <input
                                    type="text"
                                    disabled
                                    placeholder="http://localhost:4000/callback"
                                    value=move || redirect_uri.get()
                                    on:input=handle_redirect_uri_input
                                    class="py-2.5 px-4 w-full rounded-lg border transition-all focus:border-pink-500 dark:focus:border-cyan-500 focus:outline-none bg-white dark:bg-slate-900 border-gray-300 dark:border-slate-700 text-gray-900 dark:text-slate-100 placeholder-gray-500 dark:placeholder-slate-500 disabled:bg-gray-100 dark:disabled:bg-slate-800 disabled:text-gray-500 dark:disabled:text-slate-500"
                                />
                            </div>

                            // Proxy URL
                            <div>
                                <label class="block mb-2 text-sm font-medium text-gray-700 dark:text-slate-300">
                                    "Proxy URL " <span class="text-gray-500 dark:text-slate-500">"(Optional)"</span>
                                </label>
                                <input
                                    type="text"
                                    disabled
                                    placeholder="http://localhost:8000/"
                                    value=move || proxy_url.get()
                                    on:input=handle_proxy_url_input
                                    class="py-2.5 px-4 w-full rounded-lg border transition-all focus:border-pink-500 dark:focus:border-cyan-500 focus:outline-none bg-white dark:bg-slate-900 border-gray-300 dark:border-slate-700 text-gray-900 dark:text-slate-100 placeholder-gray-500 dark:placeholder-slate-500 disabled:bg-gray-100 dark:disabled:bg-slate-800 disabled:text-gray-500 dark:disabled:text-slate-500"
                                />
                            </div>
                        </div>

                        // Buttons
                        <div class="flex flex-col gap-3">
                            <Show
                                when=move || is_authenticated.get()
                                fallback=move || {
                                    view! {
                                        <button
                                            on:click=handle_auth
                                            disabled=move || auth_url().is_empty()
                                            class="flex flex-1 gap-2 justify-center items-center py-3 font-medium text-white bg-gradient-to-r from-pink-500 to-pink-600 dark:from-pink-400 dark:to-pink-500 rounded-lg transition-all hover:from-pink-600 hover:to-pink-700 dark:hover:from-pink-500 dark:hover:to-pink-600 disabled:cursor-not-allowed disabled:from-gray-400 dark:disabled:from-slate-600 disabled:to-gray-400 dark:disabled:to-slate-600"
                                        >
                                            <ExternalLink size=16 />
                                            "Authorize with osu!"
                                        </button>
                                        <p class="mt-3 text-xs text-center text-gray-600 dark:text-slate-400">
                                            "Click to authenticate. You'll be redirected back automatically."
                                        </p>
                                    }
                                }
                            >
                                <div class="flex flex-1 gap-2">
                                    <button
                                        on:click=move |_| {
                                            set_loading.set(true);
                                        }
                                        disabled=move || loading.get()
                                        class="flex flex-1 gap-2 justify-center items-center py-3 text-sm rounded-lg transition-all bg-gray-200 dark:bg-green-700 text-gray-700 dark:text-slate-300 hover:bg-green-300 dark:hover:bg-green-600 disabled:bg-gray-200 dark:disabled:bg-slate-700"
                                    >
                                        <RefreshCw size=16 />
                                        "Refresh Token"
                                    </button>
                                    <button
                                        on:click={
                                            let on_token_changed = on_token_changed.clone();
                                            move |_| {
                                                set_token.set(None);
                                                set_is_authenticated.set(false);
                                                clear_stored_token();
                                                set_error.set("".to_string());
                                                on_token_changed(None);
                                            }
                                        }
                                        class="flex flex-1 gap-2 justify-center items-center py-3 text-sm text-red-700 dark:text-red-300 rounded-lg transition-all bg-red-100 dark:bg-red-900/30 hover:bg-red-200 dark:hover:bg-red-900/50"
                                    >
                                        <LogOut size=16 />
                                        "Logout"
                                    </button>
                                </div>
                            </Show>
                        </div>

                        <Show when=move || is_authenticated.get()>
                            <p class="mt-3 text-xs text-center text-gray-600 dark:text-slate-400">
                                "You are authenticated and can now query user data."
                            </p>
                        </Show>
                    </div>
                </div>
            </div>
        </div>
    }
}
