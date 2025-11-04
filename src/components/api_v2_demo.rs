use crate::components::demo::v2::me_demo::MeDemo;
use crate::components::demo::v2::user_demo::UserDemo;
use crate::components::oauth_panel::OAuthPanel;
use leptos::prelude::*;
use lucide_leptos::{Search, Settings, User};
use osynic_osuapi::v2::model::oauth::structs::o_token::OToken;

#[component]
pub fn ApiV2Demo() -> impl IntoView {
    // State
    let (token, set_token) = signal::<Option<OToken>>(None);
    let (active_tab, set_active_tab) = signal("user");
    let (_search_result, set_search_result) = signal("".to_string());
    let (raw_json, set_raw_json) = signal("".to_string());
    let (is_loading, set_is_loading) = signal(false);
    let (show_json, set_show_json) = signal(false);

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
                <div class="min-h-screen bg-gradient-to-br from-gray-50 via-gray-100 to-gray-50 dark:from-slate-950 dark:via-slate-900 dark:to-slate-950 text-gray-900 dark:text-slate-100">
                    <div class="py-6 px-4 mx-auto max-w-7xl sm:px-6 lg:px-8">
                        // Tabs Navigation
                        <div class="flex gap-1 mb-6 border-b border-gray-200 dark:border-slate-800">
                            <button
                                on:click=move |_| set_active_tab.set("me")
                                class=move || {
                                    let base = "px-4 py-3 text-sm font-medium border-b-2 transition-all flex items-center gap-2";
                                    let active = if active_tab.get() == "me" {
                                        "border-pink-500 text-pink-600 dark:border-cyan-500 dark:text-cyan-400"
                                    } else {
                                        "border-transparent text-gray-600 dark:text-slate-400 hover:text-gray-900 dark:hover:text-slate-300"
                                    };
                                    format!("{} {}", base, active)
                                }
                            >
                                <User size=16 />
                                "My Profile"
                            </button>
                            
                            <button
                                on:click=move |_| set_active_tab.set("user")
                                class=move || {
                                    let base = "px-4 py-3 text-sm font-medium border-b-2 transition-all flex items-center gap-2";
                                    let active = if active_tab.get() == "user" {
                                        "border-pink-500 text-pink-600 dark:border-cyan-500 dark:text-cyan-400"
                                    } else {
                                        "border-transparent text-gray-600 dark:text-slate-400 hover:text-gray-900 dark:hover:text-slate-300"
                                    };
                                    format!("{} {}", base, active)
                                }
                            >
                                <Search size=16 />
                                "User Query"
                            </button>

                            // Future extension placeholders
                            <button
                                disabled=true
                                class="flex gap-2 items-center py-3 px-4 text-sm font-medium border-b-2 border-transparent opacity-50 transition-all cursor-not-allowed text-gray-500 dark:text-slate-500 hover:text-gray-600 dark:hover:text-slate-400"
                            >
                                <Settings size=16 />
                                "More Features"
                            </button>
                        </div>

                        <Show when=move || active_tab.get() == "me">
                            <div>
                                <MeDemo
                                    token=token
                                    set_result=set_search_result
                                    set_raw_json=set_raw_json
                                    is_loading=is_loading
                                    set_is_loading=set_is_loading
                                />
                                <Show when=move || !raw_json.get().is_empty()>
                                    <div class="mt-6 p-6 rounded-lg border bg-white dark:bg-slate-800/50 border-gray-200 dark:border-slate-700/50">
                                        <button
                                            on:click=move |_| set_show_json.set(!show_json.get())
                                            class="py-2 px-4 text-sm rounded-lg transition-all bg-gray-200 dark:bg-slate-700 text-gray-800 dark:text-slate-200 hover:bg-gray-300 dark:hover:bg-slate-600"
                                        >
                                            {move || {
                                                if show_json.get() { "Hide Raw JSON" } else { "Show Raw JSON" }
                                            }}
                                        </button>

                                        <Show when=move || show_json.get() && !raw_json.get().is_empty()>
                                            <div class="overflow-x-auto p-4 mt-4 max-h-96 font-mono text-xs whitespace-pre-wrap rounded-lg border bg-gray-50 dark:bg-slate-900/70 border-gray-200 dark:border-slate-700/50 text-gray-800 dark:text-slate-400">
                                                {raw_json}
                                            </div>
                                        </Show>
                                    </div>
                                </Show>
                            </div>
                        </Show>
                        
                        // Tab Content
                        <Show when=move || active_tab.get() == "user">
                            <div>
                                <UserDemo
                                    token=token
                                    set_result=set_search_result
                                    set_raw_json=set_raw_json
                                    is_loading=is_loading
                                    set_is_loading=set_is_loading
                                />
                                <Show when=move || !raw_json.get().is_empty()>
                                    <div class="mt-6 p-6 rounded-lg border bg-white dark:bg-slate-800/50 border-gray-200 dark:border-slate-700/50">
                                        <button
                                            on:click=move |_| set_show_json.set(!show_json.get())
                                            class="py-2 px-4 text-sm rounded-lg transition-all bg-gray-200 dark:bg-slate-700 text-gray-800 dark:text-slate-200 hover:bg-gray-300 dark:hover:bg-slate-600"
                                        >
                                            {move || {
                                                if show_json.get() { "Hide Raw JSON" } else { "Show Raw JSON" }
                                            }}
                                        </button>

                                        <Show when=move || show_json.get() && !raw_json.get().is_empty()>
                                            <div class="overflow-x-auto p-4 mt-4 max-h-96 font-mono text-xs whitespace-pre-wrap rounded-lg border bg-gray-50 dark:bg-slate-900/70 border-gray-200 dark:border-slate-700/50 text-gray-800 dark:text-slate-400">
                                                {raw_json}
                                            </div>
                                        </Show>
                                    </div>
                                </Show>
                            </div>
                        </Show>

                        
                    </div>
                </div>
            </Show>

            // Show message when not authenticated
            <Show when=move || token.get().is_none()>
                <div class="flex justify-center items-center p-12 bg-gradient-to-br from-gray-50 via-gray-100 to-gray-50 dark:from-slate-950 dark:via-slate-900 dark:to-slate-950 text-gray-900 dark:text-slate-100">
                    <div class="text-center">
                        <div class="flex justify-center mb-4">
                            <User size=64 color="#9ca3af" />
                        </div>
                        <p class="text-gray-600 dark:text-slate-400">
                            "Please authenticate using the OAuth panel above to access all osu! api v2 features."
                        </p>
                    </div>
                </div>
            </Show>
        </div>
    }
}
