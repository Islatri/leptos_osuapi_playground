use leptos::prelude::*;
use lucide_leptos::{
    Calendar, Globe, Heart, Loader, Trophy, Flame, Target, Award, TrendingUp, Users, User as UserIcon,
};
use osynic_osuapi::v2::client::gloo::client::OsynicOsuApiV2GlooClient;
use osynic_osuapi::v2::interface::users::IUsers;
use osynic_osuapi::v2::model::oauth::structs::o_token::OToken;
use osynic_osuapi::v2::model::user::structs::user::User;
use osynic_osuapi::v2::model::mode::enums::mode::Mode;
use wasm_bindgen_futures::spawn_local;

#[component]
pub fn MeDemo(
    token: ReadSignal<Option<OToken>>,
    set_result: WriteSignal<String>,
    set_raw_json: WriteSignal<String>,
    is_loading: ReadSignal<bool>,
    set_is_loading: WriteSignal<bool>,
) -> impl IntoView {
    let user_data: RwSignal<Option<User>> = RwSignal::new(None);
    let selected_mode: RwSignal<Mode> = RwSignal::new(Mode::Osu);

    let handle_fetch_me = move |_| {
        set_is_loading.set(true);
        set_result.set("Fetching your profile...".to_string());
        set_raw_json.set("".to_string());

        // Get the token
        let token_data = token.get();
        if token_data.is_none() {
            set_result.set("No token available. Please authenticate first.".to_string());
            set_is_loading.set(false);
            return;
        }

        let token_data = token_data.unwrap();
        let mode = selected_mode.get();

        spawn_local(async move {
            let v2_token = OToken {
                access_token: token_data.access_token.clone(),
                refresh_token: token_data.refresh_token.clone(),
                expires_in: token_data.expires_in,
                token_type: token_data.token_type.clone(),
            };

            let client = OsynicOsuApiV2GlooClient::new(v2_token);
            client.set_proxy_url("http://localhost:8000/".to_string());

            match client.users.get_own_data(Some(mode), None).await {
                Ok(user) => {
                    // Set raw JSON
                    set_raw_json.set(format!("{:#?}", user));
                    
                    // Store user data for display
                    user_data.set(Some(user.clone()));

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

                    set_result.set(result_str);
                }
                Err(e) => {
                    let error_msg = format!("Error: {:?}", e);
                    set_result.set(error_msg.clone());
                    set_raw_json.set(error_msg);
                }
            }

            set_is_loading.set(false);
        });
    };

    let format_number = |num: Option<u32>| -> String {
        match num {
            Some(n) => n.to_string(),
            None => "N/A".to_string(),
        }
    };

    let format_date = |date: Option<String>| -> String {
        match date {
            Some(d) => d.chars().take(10).collect(), // Simple date formatting (YYYY-MM-DD)
            None => "N/A".to_string(),
        }
    };

    let format_playtime = |seconds: u32| -> String {
        let hours = seconds / 3600;
        format!("{}h", hours)
    };

    let get_mode_icon = |mode: Mode| -> String {
        match mode {
            Mode::Osu => "🎯".to_string(),
            Mode::Taiko => "🥁".to_string(),
            Mode::Catch => "🍎".to_string(),
            Mode::Mania => "🎹".to_string(),
        }
    };

    let get_mode_name = |mode: Mode| -> String {
        match mode {
            Mode::Osu => "osu".to_string(),
            Mode::Taiko => "taiko".to_string(),
            Mode::Catch => "fruits".to_string(),
            Mode::Mania => "mania".to_string(),
        }
    };

    view! {
        <div class="space-y-6">
            // Fetch Button Section
            <div class="p-6 rounded-lg border bg-white dark:bg-slate-800/50 border-gray-200 dark:border-slate-700/50">
                <h2 class="flex gap-3 items-center mb-6 text-lg font-semibold text-gray-900 dark:text-white">
                    <UserIcon size=20 color="#0a89a8" />
                    "Your osu! Profile"
                </h2>

                <div class="space-y-4">
                    <p class="text-sm text-gray-600 dark:text-slate-400">
                        "Click the button below to fetch your authenticated osu! profile information."
                    </p>

                    // Mode Selector
                    <div class="flex flex-wrap gap-2 p-3 bg-gray-50 dark:bg-slate-900/30 rounded-lg border border-gray-200 dark:border-slate-700/50">
                        <span class="text-xs text-gray-600 dark:text-slate-400 font-medium w-full mb-1">
                            "Select Mode:"
                        </span>
                        {[Mode::Osu, Mode::Taiko, Mode::Catch, Mode::Mania]
                            .iter()
                            .map(|&mode| {
                                let is_selected = move || selected_mode.get() == mode;
                                view! {
                                    <button
                                        on:click=move |_| {
                                            selected_mode.set(mode);
                                        }
                                        class=move || {
                                            let base = "px-4 py-2 rounded-lg text-sm font-medium transition-all";
                                            let selected = "bg-gradient-to-r from-pink-500 to-pink-600 dark:from-cyan-500 dark:to-cyan-600 text-white shadow-md";
                                            let unselected = "bg-gray-200 dark:bg-slate-700 text-gray-700 dark:text-slate-300 hover:bg-gray-300 dark:hover:bg-slate-600";
                                            format!("{} {}", base, if is_selected() { selected } else { unselected })
                                        }
                                    >
                                        {get_mode_icon(mode)} " " {get_mode_name(mode)}
                                    </button>
                                }
                            })
                            .collect::<Vec<_>>()}
                    </div>

                    <button
                        on:click=handle_fetch_me
                        disabled=move || is_loading.get()
                        class="flex gap-2 justify-center items-center py-3 w-full font-medium text-white bg-gradient-to-r from-pink-500 to-pink-600 dark:from-cyan-500 dark:to-cyan-600 rounded-lg transition-all hover:from-pink-600 hover:to-pink-700 dark:hover:from-cyan-600 dark:hover:to-cyan-700 disabled:cursor-not-allowed disabled:from-gray-400 dark:disabled:from-slate-600 disabled:to-gray-400 dark:disabled:to-slate-600"
                    >
                        <Show
                            when=move || !is_loading.get()
                            fallback=|| {
                                view! {
                                    <Loader size=16 color="#ffffff" />
                                    "Loading..."
                                }
                            }
                        >
                            <UserIcon size=16 color="#ffffff" />
                            "Fetch My Profile"
                        </Show>
                    </button>
                </div>
            </div>

            // User Profile Card
            <Show when=move || user_data.get().is_some() && !is_loading.get()>
                {move || {
                    user_data.get().map(|user| {
                        view! {
                                <>
                                    // Profile Header
                                    <div class="bg-white dark:bg-slate-800/50 border border-gray-200 dark:border-slate-700/50 rounded-lg overflow-hidden">
                                        // Cover
                                        <div
                                            class="h-32 bg-gradient-to-r from-pink-500/20 to-pink-400/20 dark:from-cyan-500/20 dark:to-cyan-400/20 bg-cover bg-center"
                                            style={move || {
                                                if let Some(u) = user_data.get() {
                                                    if let Some(cover_url) = &u.cover_url {
                                                        format!("background-image: url('{}');", cover_url)
                                                    } else {
                                                        "".to_string()
                                                    }
                                                } else {
                                                    "".to_string()
                                                }
                                            }}
                                        >
                                        </div>

                                        // Profile Info
                                        <div class="px-6 py-6">
                                            <div class="flex gap-6 -mt-20 mb-6 items-end">
                                                <img
                                                    src={move || {
                                                        if let Some(u) = user_data.get() {
                                                            u.avatar_url.clone()
                                                        } else {
                                                            "".to_string()
                                                        }
                                                    }}
                                                    alt={move || {
                                                        if let Some(u) = user_data.get() {
                                                            u.username.clone()
                                                        } else {
                                                            "".to_string()
                                                        }
                                                    }}
                                                    class="w-28 h-28 rounded-lg border-4 border-white dark:border-slate-800 shadow-2xl z-20"
                                                />
                                                <div class="flex-1">
                                                    <div class="flex items-start justify-between mb-2">
                                                        <div>
                                                            <h2 class="text-3xl font-bold text-gray-900 dark:text-white mb-1">
                                                                {move || {
                                                                    if let Some(u) = user_data.get() {
                                                                        u.username.clone()
                                                                    } else {
                                                                        "".to_string()
                                                                    }
                                                                }}
                                                            </h2>
                                                            <div class="flex flex-wrap items-center gap-3 text-sm text-gray-600 dark:text-slate-400">
                                                                <div class="flex items-center gap-1.5">
                                                                    <Globe size=14 />
                                                                    {move || {
                                                                        if let Some(u) = user_data.get() {
                                                                            u.country.as_ref().map(|c| c.name.clone()).unwrap_or_else(|| "Unknown".to_string())
                                                                        } else {
                                                                            "Unknown".to_string()
                                                                        }
                                                                    }}
                                                                </div>
                                                                <div class="flex items-center gap-1.5">
                                                                    <Calendar size=14 />
                                                                    {move || {
                                                                        if let Some(u) = user_data.get() {
                                                                            format!("Joined {}", format_date(u.join_date.clone()))
                                                                        } else {
                                                                            "Unknown".to_string()
                                                                        }
                                                                    }}
                                                                </div>
                                                                <Show when=move || {
                                                                    if let Some(u) = user_data.get() {
                                                                        u.is_supporter
                                                                    } else {
                                                                        false
                                                                    }
                                                                }>
                                                                    <div class="flex items-center gap-1.5 px-2.5 py-1 bg-pink-200 dark:bg-pink-900/30 text-pink-700 dark:text-pink-300 rounded text-xs font-medium">
                                                                        <Heart size=12 />
                                                                        "Supporter"
                                                                    </div>
                                                                </Show>
                                                            </div>
                                                        </div>
                                                        <Show when=move || {
                                                            if let Some(u) = user_data.get() {
                                                                u.statistics.as_ref().is_some() && u.statistics.as_ref().unwrap().global_rank.is_some()
                                                            } else {
                                                                false
                                                            }
                                                        }>
                                                            <div class="text-right">
                                                                <div class="text-xs text-gray-600 dark:text-slate-400 mb-1">Global Rank</div>
                                                                <div class="text-3xl font-bold text-pink-600 dark:text-cyan-400">
                                                                    {"#"}{move || {
                                                                        if let Some(u) = user_data.get() {
                                                                            format_number(u.statistics.as_ref().unwrap().global_rank)
                                                                        } else {
                                                                            "N/A".to_string()
                                                                        }
                                                                    }}
                                                                </div>
                                                            </div>
                                                        </Show>
                                                    </div>
                                                </div>
                                            </div>

                                            // Quick Stats
                                            <Show when=move || {
                                                if let Some(u) = user_data.get() {
                                                    u.statistics.is_some()
                                                } else {
                                                    false
                                                }
                                            }>
                                                <div class="grid grid-cols-4 gap-4">
                                                    <div class="bg-gray-100 dark:bg-slate-900/50 border border-gray-200 dark:border-slate-700/30 rounded p-3">
                                                        <div class="text-xs text-gray-600 dark:text-slate-400 mb-1">Performance</div>
                                                        <div class="text-2xl font-bold text-pink-600 dark:text-cyan-400">
                                                            {move || {
                                                                if let Some(u) = user_data.get() {
                                                                    format!("{:.0}pp", u.statistics.as_ref().unwrap().pp)
                                                                } else {
                                                                    "N/A".to_string()
                                                                }
                                                            }}
                                                        </div>
                                                    </div>
                                                    <div class="bg-gray-100 dark:bg-slate-900/50 border border-gray-200 dark:border-slate-700/30 rounded p-3">
                                                        <div class="text-xs text-gray-600 dark:text-slate-400 mb-1">Accuracy</div>
                                                        <div class="text-2xl font-bold text-pink-600 dark:text-cyan-400">
                                                            {move || {
                                                                if let Some(u) = user_data.get() {
                                                                    format!("{:.2}%", u.statistics.as_ref().unwrap().hit_accuracy)
                                                                } else {
                                                                    "N/A".to_string()
                                                                }
                                                            }}
                                                        </div>
                                                    </div>
                                                    <div class="bg-gray-100 dark:bg-slate-900/50 border border-gray-200 dark:border-slate-700/30 rounded p-3">
                                                        <div class="text-xs text-gray-600 dark:text-slate-400 mb-1">Play Count</div>
                                                        <div class="text-2xl font-bold text-pink-600 dark:text-cyan-400">
                                                            {move || {
                                                                if let Some(u) = user_data.get() {
                                                                    format_number(Some(u.statistics.as_ref().unwrap().play_count))
                                                                } else {
                                                                    "N/A".to_string()
                                                                }
                                                            }}
                                                        </div>
                                                    </div>
                                                    <div class="bg-gray-100 dark:bg-slate-900/50 border border-gray-200 dark:border-slate-700/30 rounded p-3">
                                                        <div class="text-xs text-gray-600 dark:text-slate-400 mb-1">Play Time</div>
                                                        <div class="text-2xl font-bold text-pink-600 dark:text-cyan-400">
                                                            {move || {
                                                                if let Some(u) = user_data.get() {
                                                                    format_playtime(u.statistics.as_ref().unwrap().play_time)
                                                                } else {
                                                                    "N/A".to_string()
                                                                }
                                                            }}
                                                        </div>
                                                    </div>
                                                </div>
                                            </Show>
                                        </div>
                                    </div>

                                    // Rank History
                                    <Show when=move || {
                                        if let Some(u) = user_data.get() {
                                            u.rank_highest.is_some()
                                        } else {
                                            false
                                        }
                                    }>
                                        <div class="bg-white dark:bg-slate-800/50 border border-gray-200 dark:border-slate-700/50 rounded-lg p-6">
                                            <h3 class="text-sm font-semibold text-gray-700 dark:text-slate-300 mb-4 flex items-center gap-2 uppercase tracking-wide">
                                                <TrendingUp size=16 color="#ec4899" />
                                                "Rank History"
                                            </h3>
                                            <div class="grid grid-cols-2 gap-4">
                                                <div>
                                                    <div class="text-xs text-gray-600 dark:text-slate-400 mb-1">Highest Rank</div>
                                                    <div class="text-2xl font-bold text-pink-600 dark:text-cyan-400">
                                                        {"#"}{move || {
                                                            if let Some(u) = user_data.get() {
                                                                u.rank_highest.as_ref().map(|r| r.rank.to_string()).unwrap_or_else(|| "N/A".to_string())
                                                            } else {
                                                                "N/A".to_string()
                                                            }
                                                        }}
                                                    </div>
                                                    <div class="text-xs text-gray-500 dark:text-slate-500 mt-1">
                                                        {move || {
                                                            if let Some(u) = user_data.get() {
                                                                format!("Achieved on {}", format_date(u.rank_highest.as_ref().and_then(|r| Some(r.updated_at.clone()))))
                                                            } else {
                                                                "N/A".to_string()
                                                            }
                                                        }}
                                                    </div>
                                                </div>
                                                <div>
                                                    <div class="text-xs text-gray-600 dark:text-slate-400 mb-1">Current Rank</div>
                                                    <div class="text-2xl font-bold text-pink-600 dark:text-cyan-400">
                                                        {"#"}{move || {
                                                            if let Some(u) = user_data.get() {
                                                                format_number(u.statistics.as_ref().and_then(|s| s.global_rank))
                                                            } else {
                                                                "N/A".to_string()
                                                            }
                                                        }}
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                    </Show>

                                    // Stats Grid
                                    <Show when=move || {
                                        if let Some(u) = user_data.get() {
                                            u.statistics.is_some()
                                        } else {
                                            false
                                        }
                                    }>
                                        <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
                                            // Rankings
                                            <div class="bg-white dark:bg-slate-800/50 border border-gray-200 dark:border-slate-700/50 rounded-lg p-6">
                                                <h3 class="text-sm font-semibold text-gray-700 dark:text-slate-300 mb-4 flex items-center gap-2 uppercase tracking-wide">
                                                    <Trophy size=16 color="#ec4899" />
                                                    "Rankings"
                                                </h3>
                                                <div class="space-y-3">
                                                    <div class="flex justify-between items-center">
                                                        <span class="text-xs text-gray-600 dark:text-slate-400">Global Rank</span>
                                                        <span class="text-lg font-bold text-pink-600 dark:text-cyan-400">
                                                            {"#"}{move || {
                                                                if let Some(u) = user_data.get() {
                                                                    format_number(u.statistics.as_ref().unwrap().global_rank)
                                                                } else {
                                                                    "N/A".to_string()
                                                                }
                                                            }}
                                                        </span>
                                                    </div>
                                                    <div class="flex justify-between items-center">
                                                        <span class="text-xs text-gray-600 dark:text-slate-400">Country Rank</span>
                                                        <span class="text-lg font-bold text-pink-600 dark:text-cyan-400">
                                                            {"#"}{move || {
                                                                if let Some(u) = user_data.get() {
                                                                    format_number(u.statistics.as_ref().unwrap().country_rank)
                                                                } else {
                                                                    "N/A".to_string()
                                                                }
                                                            }}
                                                        </span>
                                                    </div>
                                                    <div class="flex justify-between items-center">
                                                        <span class="text-xs text-gray-600 dark:text-slate-400">Level</span>
                                                        <span class="text-lg font-bold text-pink-600 dark:text-cyan-400">
                                                            {move || {
                                                                if let Some(u) = user_data.get() {
                                                                    u.statistics.as_ref().unwrap().level.current.to_string()
                                                                } else {
                                                                    "N/A".to_string()
                                                                }
                                                            }}
                                                        </span>
                                                    </div>
                                                    <div class="flex justify-between items-center">
                                                        <span class="text-xs text-gray-600 dark:text-slate-400">Level Progress</span>
                                                        <span class="text-lg font-bold text-pink-600 dark:text-cyan-400">
                                                            {move || {
                                                                if let Some(u) = user_data.get() {
                                                                    format!("{}%", u.statistics.as_ref().unwrap().level.progress)
                                                                } else {
                                                                    "N/A".to_string()
                                                                }
                                                            }}
                                                        </span>
                                                    </div>
                                                </div>
                                            </div>

                                            // Grade Distribution
                                            <div class="bg-white dark:bg-slate-800/50 border border-gray-200 dark:border-slate-700/50 rounded-lg p-6">
                                                <h3 class="text-sm font-semibold text-gray-700 dark:text-slate-300 mb-4 flex items-center gap-2 uppercase tracking-wide">
                                                    <Award size=16 color="#ec4899" />
                                                    "Grades"
                                                </h3>
                                                <div class="space-y-2">
                                                    <div class="flex justify-between items-center text-sm">
                                                        <span class="text-yellow-600 dark:text-yellow-300 font-medium">SSH</span>
                                                        <span class="text-gray-700 dark:text-slate-300">
                                                            {move || {
                                                                if let Some(u) = user_data.get() {
                                                                    u.statistics.as_ref().unwrap().grade_counts.ssh.to_string()
                                                                } else {
                                                                    "N/A".to_string()
                                                                }
                                                            }}
                                                        </span>
                                                    </div>
                                                    <div class="flex justify-between items-center text-sm">
                                                        <span class="text-yellow-600 dark:text-yellow-300 font-medium">SS</span>
                                                        <span class="text-gray-700 dark:text-slate-300">
                                                            {move || {
                                                                if let Some(u) = user_data.get() {
                                                                    u.statistics.as_ref().unwrap().grade_counts.ss.to_string()
                                                                } else {
                                                                    "N/A".to_string()
                                                                }
                                                            }}
                                                        </span>
                                                    </div>
                                                    <div class="flex justify-between items-center text-sm">
                                                        <span class="text-gray-700 dark:text-slate-300">SH</span>
                                                        <span class="text-gray-600 dark:text-slate-400">
                                                            {move || {
                                                                if let Some(u) = user_data.get() {
                                                                    u.statistics.as_ref().unwrap().grade_counts.sh.to_string()
                                                                } else {
                                                                    "N/A".to_string()
                                                                }
                                                            }}
                                                        </span>
                                                    </div>
                                                    <div class="flex justify-between items-center text-sm">
                                                        <span class="text-gray-700 dark:text-slate-300">S</span>
                                                        <span class="text-gray-600 dark:text-slate-400">
                                                            {move || {
                                                                if let Some(u) = user_data.get() {
                                                                    u.statistics.as_ref().unwrap().grade_counts.s.to_string()
                                                                } else {
                                                                    "N/A".to_string()
                                                                }
                                                            }}
                                                        </span>
                                                    </div>
                                                    <div class="flex justify-between items-center text-sm">
                                                        <span class="text-green-600 dark:text-green-400 font-medium">A</span>
                                                        <span class="text-gray-700 dark:text-slate-300">
                                                            {move || {
                                                                if let Some(u) = user_data.get() {
                                                                    u.statistics.as_ref().unwrap().grade_counts.a.to_string()
                                                                } else {
                                                                    "N/A".to_string()
                                                                }
                                                            }}
                                                        </span>
                                                    </div>
                                                </div>
                                            </div>

                                            // Hit Distribution
                                            <div class="bg-white dark:bg-slate-800/50 border border-gray-200 dark:border-slate-700/50 rounded-lg p-6">
                                                <h3 class="text-sm font-semibold text-gray-700 dark:text-slate-300 mb-4 flex items-center gap-2 uppercase tracking-wide">
                                                    <Target size=16 color="#ec4899" />
                                                    "Hit Distribution"
                                                </h3>
                                                <div class="space-y-2">
                                                    <div class="flex justify-between items-center text-sm">
                                                        <span class="text-cyan-600 dark:text-cyan-300">300</span>
                                                        <span class="text-gray-700 dark:text-slate-300">
                                                            {move || {
                                                                if let Some(u) = user_data.get() {
                                                                    u.statistics.as_ref().unwrap().count_300.to_string()
                                                                } else {
                                                                    "N/A".to_string()
                                                                }
                                                            }}
                                                        </span>
                                                    </div>
                                                    <div class="flex justify-between items-center text-sm">
                                                        <span class="text-green-600 dark:text-green-300">100</span>
                                                        <span class="text-gray-700 dark:text-slate-300">
                                                            {move || {
                                                                if let Some(u) = user_data.get() {
                                                                    u.statistics.as_ref().unwrap().count_100.to_string()
                                                                } else {
                                                                    "N/A".to_string()
                                                                }
                                                            }}
                                                        </span>
                                                    </div>
                                                    <div class="flex justify-between items-center text-sm">
                                                        <span class="text-yellow-600 dark:text-yellow-300">50</span>
                                                        <span class="text-gray-700 dark:text-slate-300">
                                                            {move || {
                                                                if let Some(u) = user_data.get() {
                                                                    u.statistics.as_ref().unwrap().count_50.to_string()
                                                                } else {
                                                                    "N/A".to_string()
                                                                }
                                                            }}
                                                        </span>
                                                    </div>
                                                    <div class="flex justify-between items-center text-sm">
                                                        <span class="text-red-600 dark:text-red-300">Miss</span>
                                                        <span class="text-gray-700 dark:text-slate-300">
                                                            {move || {
                                                                if let Some(u) = user_data.get() {
                                                                    u.statistics.as_ref().unwrap().count_miss.to_string()
                                                                } else {
                                                                    "N/A".to_string()
                                                                }
                                                            }}
                                                        </span>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>

                                        // Detailed Stats
                                        <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                                            // Score Statistics
                                            <div class="bg-white dark:bg-slate-800/50 border border-gray-200 dark:border-slate-700/50 rounded-lg p-6">
                                                <h3 class="text-sm font-semibold text-gray-700 dark:text-slate-300 mb-4 flex items-center gap-2 uppercase tracking-wide">
                                                    <Flame size=16 color="#ec4899" />
                                                    "Score Statistics"
                                                </h3>
                                                <div class="space-y-3">
                                                    <div class="flex justify-between items-center">
                                                        <span class="text-gray-600 dark:text-slate-400 text-sm">Ranked Score</span>
                                                        <span class="font-mono text-gray-700 dark:text-slate-200">
                                                            {move || {
                                                                if let Some(u) = user_data.get() {
                                                                    u.statistics.as_ref().unwrap().ranked_score.to_string()
                                                                } else {
                                                                    "N/A".to_string()
                                                                }
                                                            }}
                                                        </span>
                                                    </div>
                                                    <div class="flex justify-between items-center">
                                                        <span class="text-gray-600 dark:text-slate-400 text-sm">Total Score</span>
                                                        <span class="font-mono text-gray-700 dark:text-slate-200">
                                                            {move || {
                                                                if let Some(u) = user_data.get() {
                                                                    u.statistics.as_ref().unwrap().total_score.to_string()
                                                                } else {
                                                                    "N/A".to_string()
                                                                }
                                                            }}
                                                        </span>
                                                    </div>
                                                    <div class="flex justify-between items-center">
                                                        <span class="text-gray-600 dark:text-slate-400 text-sm">Total Hits</span>
                                                        <span class="font-mono text-gray-700 dark:text-slate-200">
                                                            {move || {
                                                                if let Some(u) = user_data.get() {
                                                                    u.statistics.as_ref().unwrap().total_hits.to_string()
                                                                } else {
                                                                    "N/A".to_string()
                                                                }
                                                            }}
                                                        </span>
                                                    </div>
                                                    <div class="flex justify-between items-center">
                                                        <span class="text-gray-600 dark:text-slate-400 text-sm">Max Combo</span>
                                                        <span class="font-mono text-gray-700 dark:text-slate-200">
                                                            {move || {
                                                                if let Some(u) = user_data.get() {
                                                                    u.statistics.as_ref().unwrap().maximum_combo.to_string()
                                                                } else {
                                                                    "N/A".to_string()
                                                                }
                                                            }}
                                                        </span>
                                                    </div>
                                                    <div class="flex justify-between items-center">
                                                        <span class="text-gray-600 dark:text-slate-400 text-sm">Replays Watched</span>
                                                        <span class="font-mono text-gray-700 dark:text-slate-200">
                                                            {move || {
                                                                if let Some(u) = user_data.get() {
                                                                    u.statistics.as_ref().unwrap().replays_watched_by_others.to_string()
                                                                } else {
                                                                    "N/A".to_string()
                                                                }
                                                            }}
                                                        </span>
                                                    </div>
                                                </div>
                                            </div>

                                            // User Information
                                            <div class="bg-white dark:bg-slate-800/50 border border-gray-200 dark:border-slate-700/50 rounded-lg p-6">
                                                <h3 class="text-sm font-semibold text-gray-700 dark:text-slate-300 mb-4 flex items-center gap-2 uppercase tracking-wide">
                                                    <Users size=16 color="#ec4899" />
                                                    "User Information"
                                                </h3>
                                                <div class="space-y-3">
                                                    <div class="flex justify-between items-center">
                                                        <span class="text-gray-600 dark:text-slate-400 text-sm">Follower Count</span>
                                                        <span class="font-mono text-gray-700 dark:text-slate-200">
                                                            {user.follower_count}
                                                        </span>
                                                    </div>
                                                    <div class="flex justify-between items-center">
                                                        <span class="text-gray-600 dark:text-slate-400 text-sm">Account Status</span>
                                                        <span
                                                            class=move || {
                                                                if user.is_active {
                                                                    "inline-block px-2 py-1 text-xs rounded bg-green-200 dark:bg-green-900/30 text-green-700 dark:text-green-300"
                                                                } else {
                                                                    "inline-block px-2 py-1 text-xs rounded bg-red-200 dark:bg-red-900/30 text-red-700 dark:text-red-300"
                                                                }
                                                            }
                                                        >
                                                            {if user.is_active { "Active" } else { "Inactive" }}
                                                        </span>
                                                    </div>
                                                    <div class="flex justify-between items-center">
                                                        <span class="text-gray-600 dark:text-slate-400 text-sm">User ID</span>
                                                        <span class="font-mono text-gray-700 dark:text-slate-200">
                                                            {move || {
                                                                if let Some(u) = user_data.get() {
                                                                    u.id.to_string()
                                                                } else {
                                                                    "N/A".to_string()
                                                                }
                                                            }}
                                                        </span>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                    </Show>
                                </>
                            }
                        })
                    }}
            </Show>

            // Info Box
            <div class="p-6 rounded-lg border bg-white dark:bg-slate-800/50 border-gray-200 dark:border-slate-700/50">
                <h3 class="mb-3 text-sm font-semibold text-gray-700 dark:text-slate-300">
                    "ℹ️ About This Feature"
                </h3>
                <ul class="space-y-2 text-xs text-gray-600 dark:text-slate-400">
                    <li>"• This tab retrieves your own osu! profile from the official osu!api v2"</li>
                    <li>"• Uses the /me endpoint with your OAuth token"</li>
                    <li>"• Returns your profile information including statistics"</li>
                    <li>"• Displays profile header, rankings, grades, and hit distribution"</li>
                    <li>"• CORS proxy is used: localhost:8000"</li>
                </ul>
            </div>
        </div>
    }
}
