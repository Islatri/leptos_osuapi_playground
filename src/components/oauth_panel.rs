use leptos::prelude::*;
use leptos::*;
use lucide_leptos::{ExternalLink, LogIn, LogOut, RefreshCw};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use web_sys::{HtmlInputElement, window};

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
    let (client_id, set_client_id) = signal("".to_string());
    let (client_secret, set_client_secret) = signal("".to_string());
    let (redirect_uri, set_redirect_uri) = signal("http://localhost:4000/callback".to_string());
    let (proxy_url, set_proxy_url) = signal("http://localhost:8000/".to_string());
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
        <div class="overflow-x-hidden min-h-screen bg-gradient-to-br from-slate-950 via-slate-900 to-slate-950 text-slate-100">
            // Background accent
            <div class="overflow-hidden fixed inset-0 pointer-events-none">
                <div class="absolute top-0 right-0 w-96 h-96 rounded-full bg-cyan-500/5 blur-3xl"></div>
                <div class="absolute bottom-0 left-0 w-96 h-96 rounded-full bg-cyan-500/5 blur-3xl"></div>
            </div>

            <div class="relative z-10">
                // Header
                <div class="sticky top-0 z-40 border-b border-slate-800/50 bg-slate-900/50 backdrop-blur-sm">
                    <div class="py-4 px-4 mx-auto max-w-7xl sm:px-6 lg:px-8">
                        <div class="flex justify-between items-center">
                            <div class="flex gap-3 items-center">
                                <div class="flex justify-center items-center w-10 h-10 bg-gradient-to-br from-cyan-400 to-cyan-600 rounded-lg">
                                    <svg
                                        class="w-5 h-5 text-white"
                                        fill="white"
                                        viewBox="0 0 20 20"
                                    >
                                        <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z"></path>
                                    </svg>
                                </div>
                                <div>
                                    <h1 class="text-xl font-bold text-white">"osu! OAuth Panel"</h1>
                                    <p class="text-xs text-slate-400">
                                        "Authentication & Configuration"
                                    </p>
                                </div>
                            </div>
                            <Show
                                when=move || is_authenticated.get()
                                fallback=|| view! { <div></div> }
                            >
                                <div class="py-2 px-4 text-sm rounded border bg-slate-800 border-slate-700 text-slate-300">
                                    "Authenticated"
                                </div>
                            </Show>
                        </div>
                    </div>
                </div>

                <div class="py-6 px-4 mx-auto max-w-7xl sm:px-6 lg:px-8">
                    // Error Alert
                    <Show when=move || !error.get().is_empty()>
                        <div class="p-4 mb-6 text-sm text-red-300 rounded-lg border bg-red-950/30 border-red-900/50">
                            {error}
                        </div>
                    </Show>

                    // OAuth Configuration Card
                    <div class="p-6 rounded-lg border bg-slate-800/50 border-slate-700/50">
                        <h2 class="flex gap-3 items-center mb-6 text-lg font-semibold text-white">
                            <LogIn size=20 />
                            "OAuth Configuration"
                        </h2>

                        <div class="mb-8 space-y-4">
                            // Client ID
                            <div>
                                <label class="block mb-2 text-sm font-medium text-slate-300">
                                    "Client ID"
                                </label>
                                <input
                                    type="text"
                                    placeholder="Enter your client ID"
                                    value=move || client_id.get()
                                    on:input=handle_client_id_input
                                    class="py-2.5 px-4 w-full rounded-lg border transition-all focus:border-cyan-500 focus:outline-none bg-slate-900 border-slate-700 text-slate-100 placeholder-slate-500"
                                />
                            </div>

                            // Client Secret
                            <div>
                                <label class="block mb-2 text-sm font-medium text-slate-300">
                                    "Client Secret"
                                </label>
                                <input
                                    type="password"
                                    placeholder="Enter your client secret"
                                    value=move || client_secret.get()
                                    on:input=handle_client_secret_input
                                    class="py-2.5 px-4 w-full rounded-lg border transition-all focus:border-cyan-500 focus:outline-none bg-slate-900 border-slate-700 text-slate-100 placeholder-slate-500"
                                />
                            </div>

                            // Redirect URI
                            <div>
                                <label class="block mb-2 text-sm font-medium text-slate-300">
                                    "Redirect URI"
                                </label>
                                <input
                                    type="text"
                                    placeholder="http://localhost:4000/callback"
                                    value=move || redirect_uri.get()
                                    on:input=handle_redirect_uri_input
                                    class="py-2.5 px-4 w-full rounded-lg border transition-all focus:border-cyan-500 focus:outline-none bg-slate-900 border-slate-700 text-slate-100 placeholder-slate-500"
                                />
                            </div>

                            // Proxy URL
                            <div>
                                <label class="block mb-2 text-sm font-medium text-slate-300">
                                    "Proxy URL " <span class="text-slate-500">"(Optional)"</span>
                                </label>
                                <input
                                    type="text"
                                    placeholder="http://localhost:8000/"
                                    value=move || proxy_url.get()
                                    on:input=handle_proxy_url_input
                                    class="py-2.5 px-4 w-full rounded-lg border transition-all focus:border-cyan-500 focus:outline-none bg-slate-900 border-slate-700 text-slate-100 placeholder-slate-500"
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
                                            class="flex flex-1 gap-2 justify-center items-center py-3 font-medium text-white bg-gradient-to-r from-cyan-500 to-cyan-600 rounded-lg transition-all hover:from-cyan-600 hover:to-cyan-700 disabled:cursor-not-allowed disabled:from-slate-600 disabled:to-slate-600"
                                        >
                                            <ExternalLink size=16 />
                                            "Authorize with osu!"
                                        </button>
                                        <p class="mt-3 text-xs text-center text-slate-400">
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
                                        class="flex flex-1 gap-2 justify-center items-center py-3 text-sm rounded-lg transition-all bg-slate-700 text-slate-300 hover:bg-slate-600 disabled:bg-slate-700"
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
                                        class="flex flex-1 gap-2 justify-center items-center py-3 text-sm text-red-300 rounded-lg transition-all bg-red-900/30 hover:bg-red-900/50"
                                    >
                                        <LogOut size=16 />
                                        "Logout"
                                    </button>
                                </div>
                            </Show>
                        </div>

                        <Show when=move || is_authenticated.get()>
                            <p class="mt-3 text-xs text-center text-slate-400">
                                "You are authenticated and can now query user data."
                            </p>
                        </Show>
                    </div>
                </div>
            </div>
        </div>
    }
}
