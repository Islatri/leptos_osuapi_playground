use leptos::*;
use leptos::prelude::{ GlobalAttributes, ClassAttribute, ElementChild };
use lucide_leptos::{
    Music,
    UserCheck,
    Users,
    ChartBarBig,
    Lock,
    Layers,
    History,
    MessageSquare,
    MessageSquareText,
    Calendar,
    FileText,
    House,
    Award,
    Newspaper,
    Bell,
    Trophy,
    Star,
    User,
    BookCopy,
};
use leptos_fluent::tr;

#[component]
pub fn ApiSection() -> impl IntoView {
    view! {
<section id="api" class="py-20 bg-gradient-to-b from-white to-gray-50 dark:from-gray-900 dark:to-gray-950">
    <div class="container mx-auto px-4">
        <h2 class="text-4xl font-bold text-center mb-6 bg-clip-text text-transparent bg-gradient-to-r from-pink-500 to-purple-600">
            {move || tr!("api-section-title")}
        </h2>
        <p class="text-gray-600 dark:text-gray-300 text-center max-w-3xl mx-auto mb-16 text-lg">
            {move || tr!("api-section-description")}
        </p>

        {/* V1 API Section */}
        <div class="mb-20">
            <div class="flex items-center justify-center mb-12">
                <div class="h-px bg-gray-200 dark:bg-gray-700 w-1/4"></div>
                <h3 class="text-2xl font-bold mx-6 text-pink-600 dark:text-pink-400 px-6 py-2 rounded-full border-2 border-pink-200 dark:border-pink-900 bg-pink-50 dark:bg-pink-950">
                    <a href="https://github.com/ppy/osu-api/wiki" target="_blank">{move || tr!("api-v1-title")}</a>
                </h3>
                <div class="h-px bg-gray-200 dark:bg-gray-700 w-1/4"></div>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
                {/* API 卡片 - Beatmaps */}
                <div class="card bg-white dark:bg-gray-800 hover:shadow-xl transition duration-300 p-6 rounded-xl border border-gray-100 dark:border-gray-700 relative overflow-hidden group">
                    <div class="absolute top-0 right-0 w-20 h-20 bg-pink-100 dark:bg-pink-900 opacity-20 rounded-bl-full transform transition-transform duration-300 group-hover:scale-150"></div>
                    <div class="flex items-center mb-4">
                        <div class="w-14 h-14 rounded-full text-2xl text-pink-600 bg-pink-100 dark:bg-pink-900 flex items-center justify-center drop-shadow-md">
                            <Music />
                        </div>
                        <h3 class="text-xl font-semibold ml-4 dark:text-gray-200">{move || tr!("api-beatmaps-title")}</h3>
                    </div>
                    <p class="text-gray-600 dark:text-gray-300 mb-6">
                        {move || tr!("api-beatmaps-description")}
                    </p>
                    <div class="grid grid-cols-2 gap-2 text-sm font-mono">
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-3 py-1.5 transition-colors hover:bg-pink-100 dark:hover:bg-pink-900 dark:text-gray-200">get_beatmaps</div>
                    </div>
                </div>
                
                {/* API 卡片 - Users */}
                <div class="card bg-white dark:bg-gray-800 hover:shadow-xl transition duration-300 p-6 rounded-xl border border-gray-100 dark:border-gray-700 relative overflow-hidden group">
                    <div class="absolute top-0 right-0 w-20 h-20 bg-purple-100 dark:bg-purple-900 opacity-20 rounded-bl-full transform transition-transform duration-300 group-hover:scale-150"></div>
                    <div class="flex items-center mb-4">
                        <div class="w-14 h-14 rounded-full text-2xl text-purple-600 bg-purple-100 dark:bg-purple-900 flex items-center justify-center drop-shadow-md">
                            <UserCheck />
                        </div>
                        <h3 class="text-xl font-semibold ml-4 dark:text-gray-200">{move || tr!("api-users-title")}</h3>
                    </div>
                    <p class="text-gray-600 dark:text-gray-300 mb-6">
                        {move || tr!("api-users-description")}
                    </p>
                    <div class="grid grid-cols-2 gap-2 text-sm font-mono">
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-3 py-1.5 transition-colors hover:bg-purple-100 dark:hover:bg-purple-900 dark:text-gray-200">get_user</div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-3 py-1.5 transition-colors hover:bg-purple-100 dark:hover:bg-purple-900 dark:text-gray-200">get_user_best</div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-3 py-1.5 transition-colors hover:bg-purple-100 dark:hover:bg-purple-900 dark:text-gray-200">get_user_recent</div>
                    </div>
                </div>
                
                {/* API 卡片 - Scores */}
                <div class="card bg-white dark:bg-gray-800 hover:shadow-xl transition duration-300 p-6 rounded-xl border border-gray-100 dark:border-gray-700 relative overflow-hidden group">
                    <div class="absolute top-0 right-0 w-20 h-20 bg-indigo-100 dark:bg-indigo-900 opacity-20 rounded-bl-full transform transition-transform duration-300 group-hover:scale-150"></div>
                    <div class="flex items-center mb-4">
                        <div class="w-14 h-14 rounded-full text-2xl text-indigo-600 bg-indigo-100 dark:bg-indigo-900 flex items-center justify-center drop-shadow-md">
                            <Star />
                        </div>
                        <h3 class="text-xl font-semibold ml-4 dark:text-gray-200">{move || tr!("api-scores-title")}</h3>
                    </div>
                    <p class="text-gray-600 dark:text-gray-300 mb-6">
                        {move || tr!("api-scores-description")}
                    </p>
                    <div class="grid grid-cols-2 gap-2 text-sm font-mono">
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-3 py-1.5 transition-colors hover:bg-indigo-100 dark:hover:bg-indigo-900 dark:text-gray-200">get_scores</div>
                    </div>
                </div>
                
                {/* API 卡片 - Replays */}
                <div class="card bg-white dark:bg-gray-800 hover:shadow-xl transition duration-300 p-6 rounded-xl border border-gray-100 dark:border-gray-700 relative overflow-hidden group">
                    <div class="absolute top-0 right-0 w-20 h-20 bg-blue-100 dark:bg-blue-900 opacity-20 rounded-bl-full transform transition-transform duration-300 group-hover:scale-150"></div>
                    <div class="flex items-center mb-4">
                        <div class="w-14 h-14 rounded-full text-2xl text-blue-600 bg-blue-100 dark:bg-blue-900 flex items-center justify-center drop-shadow-md">
                            <ChartBarBig />
                        </div>
                        <h3 class="text-xl font-semibold ml-4 dark:text-gray-200">{move || tr!("api-replays-title")}</h3>
                    </div>
                    <p class="text-gray-600 dark:text-gray-300 mb-6">
                        {move || tr!("api-replays-description")}
                    </p>
                    <div class="grid grid-cols-2 gap-2 text-sm font-mono">
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-3 py-1.5 transition-colors hover:bg-blue-100 dark:hover:bg-blue-900 dark:text-gray-200">get_replay</div>
                    </div>
                </div>
                
                {/* API 卡片 - Multiplayer */}
                <div class="card bg-white dark:bg-gray-800 hover:shadow-xl transition duration-300 p-6 rounded-xl border border-gray-100 dark:border-gray-700 relative overflow-hidden group">
                    <div class="absolute top-0 right-0 w-20 h-20 bg-green-100 dark:bg-green-900 opacity-20 rounded-bl-full transform transition-transform duration-300 group-hover:scale-150"></div>
                    <div class="flex items-center mb-4">
                        <div class="w-14 h-14 rounded-full text-2xl text-green-600 bg-green-100 dark:bg-green-900 flex items-center justify-center drop-shadow-md">
                            <Users />
                        </div>
                        <h3 class="text-xl font-semibold ml-4 dark:text-gray-200">{move || tr!("api-multiplayer-title")}</h3>
                    </div>
                    <p class="text-gray-600 dark:text-gray-300 mb-6">
                        {move || tr!("api-multiplayer-description")}
                    </p>
                    <div class="grid grid-cols-2 gap-2 text-sm font-mono">
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-3 py-1.5 transition-colors hover:bg-green-100 dark:hover:bg-green-900 dark:text-gray-200">get_match</div>
                    </div>
                </div>
            </div>
        </div>
        
        {/* V2 API Section */}
        <div>
            <div class="flex items-center justify-center mb-12">
                <div class="h-px bg-gray-200 dark:bg-gray-700 w-1/4"></div>
                <h3 class="text-2xl font-bold mx-6 text-purple-600 dark:text-purple-400 px-6 py-2 rounded-full border-2 border-purple-200 dark:border-purple-900 bg-purple-50 dark:bg-purple-950">
                    <a href="https://osu.ppy.sh/docs/index.html" target="_blank">{move || tr!("api-v2-title")}</a>
                </h3>
                <div class="h-px bg-gray-200 dark:bg-gray-700 w-1/4"></div>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
                {/* API 卡片 - OAuth */}
                <div class="card bg-white dark:bg-gray-800 hover:shadow-xl transition duration-300 p-5 rounded-xl border border-gray-100 dark:border-gray-700 relative overflow-hidden group">
                    <div class="absolute top-0 right-0 w-16 h-16 bg-red-100 dark:bg-red-900 opacity-20 rounded-bl-full transform transition-transform duration-300 group-hover:scale-150"></div>
                    <div class="flex items-center mb-3">
                        <div class="w-12 h-12 rounded-full text-xl text-red-600 bg-red-100 dark:bg-red-900 flex items-center justify-center drop-shadow-md">
                            <Lock />
                        </div>
                        <h3 class="text-lg font-semibold ml-3 dark:text-gray-200">{move || tr!("api-oauth-title")}</h3>
                    </div>
                    <p class="text-gray-600 dark:text-gray-300 mb-4 text-sm">
                        {move || tr!("api-oauth-description")}
                    </p>
                    <div class="grid grid-cols-2 gap-1.5 text-xs font-mono">
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-red-100 dark:hover:bg-red-900 dark:text-gray-200">get_token_with_code</div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-red-100 dark:hover:bg-red-900 dark:text-gray-200">get_token_without_code</div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-red-100 dark:hover:bg-red-900 dark:text-gray-200">refresh_token</div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-red-100 dark:hover:bg-red-900 dark:text-gray-200">revoke_current_token</div>
                    </div>
                </div>

                {/* API 卡片 - Beatmaps */}
                <div class="card bg-white dark:bg-gray-800 hover:shadow-xl transition duration-300 p-5 rounded-xl border border-gray-100 dark:border-gray-700 relative overflow-hidden group">
                    <div class="absolute top-0 right-0 w-16 h-16 bg-pink-100 dark:bg-pink-900 opacity-20 rounded-bl-full transform transition-transform duration-300 group-hover:scale-150"></div>
                    <div class="flex items-center mb-3">
                        <div class="w-12 h-12 rounded-full text-xl text-pink-600 bg-pink-100 dark:bg-pink-900 flex items-center justify-center drop-shadow-md">
                            <Music />
                        </div>
                        <h3 class="text-lg font-semibold ml-3 dark:text-gray-200">{move || tr!("api-v2-beatmaps-title")}</h3>
                    </div>
                    <p class="text-gray-600 dark:text-gray-300 mb-4 text-sm">
                        {move || tr!("api-v2-beatmaps-description")}
                    </p>
                    <div class="grid grid-cols-2 gap-1.5 text-xs font-mono">
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-pink-100 dark:hover:bg-pink-900 dark:text-gray-200">get_beatmap_packs</div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-pink-100 dark:hover:bg-pink-900 dark:text-gray-200">get_beatmap_pack</div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-pink-100 dark:hover:bg-pink-900 dark:text-gray-200">lookup_beatmap</div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-pink-100 dark:hover:bg-pink-900 dark:text-gray-200">get_beatmap</div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-pink-100 dark:hover:bg-pink-900 dark:text-gray-200">get_beatmap_attributes</div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-pink-100 dark:hover:bg-pink-900 dark:text-gray-200">get_beatmaps</div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-pink-100 dark:hover:bg-pink-900 dark:text-gray-200">get_scores</div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-pink-100 dark:hover:bg-pink-900 dark:text-gray-200">get_solo_scores</div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-pink-100 dark:hover:bg-pink-900 dark:text-gray-200">get_user_score</div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-pink-100 dark:hover:bg-pink-900 dark:text-gray-200">get_user_scores</div>
                    </div>
                </div>
                
                {/* API 卡片 - Beatmap Sets */}
                <div class="card bg-white dark:bg-gray-800 hover:shadow-xl transition duration-300 p-5 rounded-xl border border-gray-100 dark:border-gray-700 relative overflow-hidden group">
                    <div class="absolute top-0 right-0 w-16 h-16 bg-fuchsia-100 dark:bg-fuchsia-900 opacity-20 rounded-bl-full transform transition-transform duration-300 group-hover:scale-150"></div>
                    <div class="flex items-center mb-3">
                        <div class="w-12 h-12 rounded-full text-xl text-fuchsia-600 bg-fuchsia-100 dark:bg-fuchsia-900 flex items-center justify-center drop-shadow-md">
                            <Layers />
                        </div>
                        <h3 class="text-lg font-semibold ml-3 dark:text-gray-200">{move || tr!("api-beatmapsets-title")}</h3>
                    </div>
                    <p class="text-gray-600 dark:text-gray-300 mb-4 text-sm">
                        {move || tr!("api-beatmapsets-description")}
                    </p>
                    <div class="grid grid-cols-2 gap-1.5 text-xs font-mono">
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-fuchsia-100 dark:hover:bg-fuchsia-900 dark:text-gray-200">get_beatmapsets_discussions_posts</div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-fuchsia-100 dark:hover:bg-fuchsia-900 dark:text-gray-200">get_beatmapsets_discussions_vote</div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-fuchsia-100 dark:hover:bg-fuchsia-900 dark:text-gray-200">get_beatmapsets_discussions</div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-fuchsia-100 dark:hover:bg-fuchsia-900 dark:text-gray-200">search</div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-fuchsia-100 dark:hover:bg-fuchsia-900 dark:text-gray-200">lookup</div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-fuchsia-100 dark:hover:bg-fuchsia-900 dark:text-gray-200">get_beatmapset</div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-fuchsia-100 dark:hover:bg-fuchsia-900 dark:text-gray-200">download</div>
                    </div>
                </div>
                
                {/* API 卡片 - Changelog */}
                <div class="card bg-white dark:bg-gray-800 hover:shadow-xl transition duration-300 p-5 rounded-xl border border-gray-100 dark:border-gray-700 relative overflow-hidden group">
                    <div class="absolute top-0 right-0 w-16 h-16 bg-indigo-100 dark:bg-indigo-900 opacity-20 rounded-bl-full transform transition-transform duration-300 group-hover:scale-150"></div>
                    <div class="flex items-center mb-3">
                        <div class="w-12 h-12 rounded-full text-xl text-indigo-600 bg-indigo-100 dark:bg-indigo-900 flex items-center justify-center drop-shadow-md">
                            <History />
                        </div>
                        <h3 class="text-lg font-semibold ml-3 dark:text-gray-200">{move || tr!("api-changelog-title")}</h3>
                    </div>
                    <p class="text-gray-600 dark:text-gray-300 mb-4 text-sm">
                        {move || tr!("api-changelog-description")}
                    </p>
                    <div class="grid grid-cols-2 gap-1.5 text-xs font-mono">
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-indigo-100 dark:hover:bg-indigo-900 dark:text-gray-200">get_changelog_build</div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-indigo-100 dark:hover:bg-indigo-900 dark:text-gray-200">get_changelog_listing</div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-indigo-100 dark:hover:bg-indigo-900 dark:text-gray-200">lookup_changelog_build</div>
                    </div>
                </div>
                
                {/* API 卡片 - Chat */}
                <div class="card bg-white dark:bg-gray-800 hover:shadow-xl transition duration-300 p-5 rounded-xl border border-gray-100 dark:border-gray-700 relative overflow-hidden group">
                    <div class="absolute top-0 right-0 w-16 h-16 bg-blue-100 dark:bg-blue-900 opacity-20 rounded-bl-full transform transition-transform duration-300 group-hover:scale-150"></div>
                    <div class="flex items-center mb-3">
                        <div class="w-12 h-12 rounded-full text-xl text-blue-600 bg-blue-100 dark:bg-blue-900 flex items-center justify-center drop-shadow-md">
                            <MessageSquare />
                        </div>
                        <h3 class="text-lg font-semibold ml-3 dark:text-gray-200">{move || tr!("api-chat-title")}</h3>
                    </div>
                    <p class="text-gray-600 dark:text-gray-300 mb-4 text-sm">
                        {move || tr!("api-chat-description")}
                    </p>
                    <div class="grid grid-cols-2 gap-1.5 text-xs font-mono">
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-blue-100 dark:hover:bg-blue-900 dark:text-gray-200">chat_keepalive         </div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-blue-100 dark:hover:bg-blue-900 dark:text-gray-200">create_new_pm          </div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-blue-100 dark:hover:bg-blue-900 dark:text-gray-200">get_updates            </div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-blue-100 dark:hover:bg-blue-900 dark:text-gray-200">get_channel_messages   </div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-blue-100 dark:hover:bg-blue-900 dark:text-gray-200">send_message_to_channel</div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-blue-100 dark:hover:bg-blue-900 dark:text-gray-200">join_channel           </div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-blue-100 dark:hover:bg-blue-900 dark:text-gray-200">leave_channel          </div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-blue-100 dark:hover:bg-blue-900 dark:text-gray-200">mark_channel_as_read   </div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-blue-100 dark:hover:bg-blue-900 dark:text-gray-200">get_channel_list       </div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-blue-100 dark:hover:bg-blue-900 dark:text-gray-200">create_channel         </div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-blue-100 dark:hover:bg-blue-900 dark:text-gray-200">get_channel            </div>
                    </div>
                </div>
                
                {/* API 卡片 - Comments */}
                <div class="card bg-white dark:bg-gray-800 hover:shadow-xl transition duration-300 p-5 rounded-xl border border-gray-100 dark:border-gray-700 relative overflow-hidden group">
                    <div class="absolute top-0 right-0 w-16 h-16 bg-cyan-100 dark:bg-cyan-900 opacity-20 rounded-bl-full transform transition-transform duration-300 group-hover:scale-150"></div>
                    <div class="flex items-center mb-3">
                        <div class="w-12 h-12 rounded-full text-xl text-cyan-600 bg-cyan-100 dark:bg-cyan-900 flex items-center justify-center drop-shadow-md">
                            <MessageSquareText />
                        </div>
                        <h3 class="text-lg font-semibold ml-3 dark:text-gray-200">{move || tr!("api-comments-title")}</h3>
                    </div>
                    <p class="text-gray-600 dark:text-gray-300 mb-4 text-sm">
                        {move || tr!("api-comments-description")}
                    </p>
                    <div class="grid grid-cols-2 gap-1.5 text-xs font-mono">
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-cyan-100 dark:hover:bg-cyan-900 dark:text-gray-200">get_comments       </div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-cyan-100 dark:hover:bg-cyan-900 dark:text-gray-200">post_comment       </div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-cyan-100 dark:hover:bg-cyan-900 dark:text-gray-200">get_comment        </div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-cyan-100 dark:hover:bg-cyan-900 dark:text-gray-200">edit_comment       </div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-cyan-100 dark:hover:bg-cyan-900 dark:text-gray-200">delete_comment     </div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-cyan-100 dark:hover:bg-cyan-900 dark:text-gray-200">add_comment_vote   </div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-cyan-100 dark:hover:bg-cyan-900 dark:text-gray-200">remove_comment_vote</div>
                    </div>
                </div>
                
                {/* API 卡片 - Events */}
                <div class="card bg-white dark:bg-gray-800 hover:shadow-xl transition duration-300 p-5 rounded-xl border border-gray-100 dark:border-gray-700 relative overflow-hidden group">
                    <div class="absolute top-0 right-0 w-16 h-16 bg-green-100 dark:bg-green-900 opacity-20 rounded-bl-full transform transition-transform duration-300 group-hover:scale-150"></div>
                    <div class="flex items-center mb-3">
                        <div class="w-12 h-12 rounded-full text-xl text-green-600 bg-green-100 dark:bg-green-900 flex items-center justify-center drop-shadow-md">
                            <Calendar />
                        </div>
                        <h3 class="text-lg font-semibold ml-3 dark:text-gray-200">{move || tr!("api-events-title")}</h3>
                    </div>
                    <p class="text-gray-600 dark:text-gray-300 mb-4 text-sm">
                        {move || tr!("api-events-description")}
                    </p>
                    <div class="grid grid-cols-2 gap-1.5 text-xs font-mono">
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-green-100 dark:hover:bg-green-900 dark:text-gray-200">get_events</div>
                    </div>
                </div>
                
                {/* API 卡片 - Forum */}
                <div class="card bg-white dark:bg-gray-800 hover:shadow-xl transition duration-300 p-5 rounded-xl border border-gray-100 dark:border-gray-700 relative overflow-hidden group">
                    <div class="absolute top-0 right-0 w-16 h-16 bg-amber-100 dark:bg-amber-900 opacity-20 rounded-bl-full transform transition-transform duration-300 group-hover:scale-150"></div>
                    <div class="flex items-center mb-3">
                        <div class="w-12 h-12 rounded-full text-xl text-amber-600 bg-amber-100 dark:bg-amber-900 flex items-center justify-center drop-shadow-md">
                            <FileText />
                        </div>
                        <h3 class="text-lg font-semibold ml-3 dark:text-gray-200">{move || tr!("api-forum-title")}</h3>
                    </div>
                    <p class="text-gray-600 dark:text-gray-300 mb-4 text-sm">
                        {move || tr!("api-forum-description")}
                    </p>
                    <div class="grid grid-cols-2 gap-1.5 text-xs font-mono">
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-amber-100 dark:hover:bg-amber-900 dark:text-gray-200">reply_topic        </div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-amber-100 dark:hover:bg-amber-900 dark:text-gray-200">get_topics_listing </div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-amber-100 dark:hover:bg-amber-900 dark:text-gray-200">create_topic       </div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-amber-100 dark:hover:bg-amber-900 dark:text-gray-200">get_topic_and_posts</div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-amber-100 dark:hover:bg-amber-900 dark:text-gray-200">edit_topic         </div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-amber-100 dark:hover:bg-amber-900 dark:text-gray-200">edit_post          </div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-amber-100 dark:hover:bg-amber-900 dark:text-gray-200">get_forum_listing  </div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-amber-100 dark:hover:bg-amber-900 dark:text-gray-200">get_forum_and_topic</div>
                    </div>
                </div>
                
                {/* API 卡片 - Home */}
                <div class="card bg-white dark:bg-gray-800 hover:shadow-xl transition duration-300 p-5 rounded-xl border border-gray-100 dark:border-gray-700 relative overflow-hidden group">
                    <div class="absolute top-0 right-0 w-16 h-16 bg-orange-100 dark:bg-orange-900 opacity-20 rounded-bl-full transform transition-transform duration-300 group-hover:scale-150"></div>
                    <div class="flex items-center mb-3">
                        <div class="w-12 h-12 rounded-full text-xl text-orange-600 bg-orange-100 dark:bg-orange-900 flex items-center justify-center drop-shadow-md">
                            <House />
                        </div>
                        <h3 class="text-lg font-semibold ml-3 dark:text-gray-200">{move || tr!("api-home-title")}</h3>
                    </div>
                    <p class="text-gray-600 dark:text-gray-300 mb-4 text-sm">
                        {move || tr!("api-home-description")}
                    </p>
                    <div class="grid grid-cols-2 gap-1.5 text-xs font-mono">
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-orange-100 dark:hover:bg-orange-900 dark:text-gray-200">search</div>
                    </div>
                </div>
                
                {/* API 卡片 - Matches */}
                <div class="card bg-white dark:bg-gray-800 hover:shadow-xl transition duration-300 p-5 rounded-xl border border-gray-100 dark:border-gray-700 relative overflow-hidden group">
                    <div class="absolute top-0 right-0 w-16 h-16 bg-rose-100 dark:bg-rose-900 opacity-20 rounded-bl-full transform transition-transform duration-300 group-hover:scale-150"></div>
                    <div class="flex items-center mb-3">
                        <div class="w-12 h-12 rounded-full text-xl text-rose-600 bg-rose-100 dark:bg-rose-900 flex items-center justify-center drop-shadow-md">
                            <Award />
                        </div>
                        <h3 class="text-lg font-semibold ml-3 dark:text-gray-200">{move || tr!("api-matches-title")}</h3>
                    </div>
                    <p class="text-gray-600 dark:text-gray-300 mb-4 text-sm">
                        {move || tr!("api-matches-description")}
                    </p>
                    <div class="grid grid-cols-2 gap-1.5 text-xs font-mono">
                    <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-rose-100 dark:hover:bg-rose-900 dark:text-gray-200">get_matches_listing</div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-rose-100 dark:hover:bg-rose-900 dark:text-gray-200">get_match</div>
                    </div>
                </div>
                
                {/* API 卡片 - Multiplayer */}
                <div class="card bg-white dark:bg-gray-800 hover:shadow-xl transition duration-300 p-5 rounded-xl border border-gray-100 dark:border-gray-700 relative overflow-hidden group">
                    <div class="absolute top-0 right-0 w-16 h-16 bg-emerald-100 dark:bg-emerald-900 opacity-20 rounded-bl-full transform transition-transform duration-300 group-hover:scale-150"></div>
                    <div class="flex items-center mb-3">
                        <div class="w-12 h-12 rounded-full text-xl text-emerald-600 bg-emerald-100 dark:bg-emerald-900 flex items-center justify-center drop-shadow-md">
                            <Users />
                        </div>
                        <h3 class="text-lg font-semibold ml-3 dark:text-gray-200">{move || tr!("api-v2-multiplayer-title")}</h3>
                    </div>
                    <p class="text-gray-600 dark:text-gray-300 mb-4 text-sm">
                        {move || tr!("api-v2-multiplayer-description")}
                    </p>
                    <div class="grid grid-cols-2 gap-1.5 text-xs font-mono">
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-emerald-100 dark:hover:bg-emerald-900 dark:text-gray-200">get_user_high_score  </div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-emerald-100 dark:hover:bg-emerald-900 dark:text-gray-200">get_scores           </div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-emerald-100 dark:hover:bg-emerald-900 dark:text-gray-200">get_score            </div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-emerald-100 dark:hover:bg-emerald-900 dark:text-gray-200">get_multiplayer_rooms</div>
                    </div>
                </div>
                
                {/* API 卡片 - News */}
                <div class="card bg-white dark:bg-gray-800 hover:shadow-xl transition duration-300 p-5 rounded-xl border border-gray-100 dark:border-gray-700 relative overflow-hidden group">
                    <div class="absolute top-0 right-0 w-16 h-16 bg-blue-100 dark:bg-blue-900 opacity-20 rounded-bl-full transform transition-transform duration-300 group-hover:scale-150"></div>
                    <div class="flex items-center mb-3">
                        <div class="w-12 h-12 rounded-full text-xl text-blue-600 bg-blue-100 dark:bg-blue-900 flex items-center justify-center drop-shadow-md">
                            <Newspaper />
                        </div>
                        <h3 class="text-lg font-semibold ml-3 dark:text-gray-200">{move || tr!("api-news-title")}</h3>
                    </div>
                    <p class="text-gray-600 dark:text-gray-300 mb-4 text-sm">
                        {move || tr!("api-news-description")}
                    </p>
                    <div class="grid grid-cols-2 gap-1.5 text-xs font-mono">
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-blue-100 dark:hover:bg-blue-900 dark:text-gray-200">get_news_listing</div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-blue-100 dark:hover:bg-blue-900 dark:text-gray-200">get_news_post</div>
                    </div>
                </div>
                
                {/* API 卡片 - Notifications */}
                <div class="card bg-white dark:bg-gray-800 hover:shadow-xl transition duration-300 p-5 rounded-xl border border-gray-100 dark:border-gray-700 relative overflow-hidden group">
                    <div class="absolute top-0 right-0 w-16 h-16 bg-violet-100 dark:bg-violet-900 opacity-20 rounded-bl-full transform transition-transform duration-300 group-hover:scale-150"></div>
                    <div class="flex items-center mb-3">
                        <div class="w-12 h-12 rounded-full text-xl text-violet-600 bg-violet-100 dark:bg-violet-900 flex items-center justify-center drop-shadow-md">
                            <Bell />
                        </div>
                        <h3 class="text-lg font-semibold ml-3 dark:text-gray-200">{move || tr!("api-notifications-title")}</h3>
                    </div>
                    <p class="text-gray-600 dark:text-gray-300 mb-4 text-sm">
                        {move || tr!("api-notifications-description")}
                    </p>
                    <div class="grid grid-cols-2 gap-1.5 text-xs font-mono">
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-violet-100 dark:hover:bg-violet-900 dark:text-gray-200">get_notifications</div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-violet-100 dark:hover:bg-violet-900 dark:text-gray-200">mark_notifications_as_read</div>
                    </div>
                </div>
                
                {/* API 卡片 - Rankings */}
                <div class="card bg-white dark:bg-gray-800 hover:shadow-xl transition duration-300 p-5 rounded-xl border border-gray-100 dark:border-gray-700 relative overflow-hidden group">
                    <div class="absolute top-0 right-0 w-16 h-16 bg-yellow-100 dark:bg-yellow-900 opacity-20 rounded-bl-full transform transition-transform duration-300 group-hover:scale-150"></div>
                    <div class="flex items-center mb-3">
                        <div class="w-12 h-12 rounded-full text-xl text-yellow-600 bg-yellow-100 dark:bg-yellow-900 flex items-center justify-center drop-shadow-md">
                            <Trophy />
                        </div>
                        <h3 class="text-lg font-semibold ml-3 dark:text-gray-200">{move || tr!("api-rankings-title")}</h3>
                    </div>
                    <p class="text-gray-600 dark:text-gray-300 mb-4 text-sm">
                        {move || tr!("api-rankings-description")}
                    </p>
                    <div class="grid grid-cols-2 gap-1.5 text-xs font-mono">
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-yellow-100 dark:hover:bg-yellow-900 dark:text-gray-200">get_rankings</div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-yellow-100 dark:hover:bg-yellow-900 dark:text-gray-200">get_kudosu_rankings</div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-yellow-100 dark:hover:bg-yellow-900 dark:text-gray-200">get_spotlight</div>
                    </div>
                </div>
                
                {/* API 卡片 - Scores */}
                <div class="card bg-white dark:bg-gray-800 hover:shadow-xl transition duration-300 p-5 rounded-xl border border-gray-100 dark:border-gray-700 relative overflow-hidden group">
                    <div class="absolute top-0 right-0 w-16 h-16 bg-red-100 dark:bg-red-900 opacity-20 rounded-bl-full transform transition-transform duration-300 group-hover:scale-150"></div>
                    <div class="flex items-center mb-3">
                        <div class="w-12 h-12 rounded-full text-xl text-red-600 bg-red-100 dark:bg-red-900 flex items-center justify-center drop-shadow-md">
                            <Star />
                        </div>
                        <h3 class="text-lg font-semibold ml-3 dark:text-gray-200">{move || tr!("api-v2-scores-title")}</h3>
                    </div>
                    <p class="text-gray-600 dark:text-gray-300 mb-4 text-sm">
                        {move || tr!("api-v2-scores-description")}
                    </p>
                    <div class="grid grid-cols-2 gap-1.5 text-xs font-mono">
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-red-100 dark:hover:bg-red-900 dark:text-gray-200">get_scores</div>
                    </div>
                </div>
                
                {/* API 卡片 - Users */}
                <div class="card bg-white dark:bg-gray-800 hover:shadow-xl transition duration-300 p-5 rounded-xl border border-gray-100 dark:border-gray-700 relative overflow-hidden group">
                    <div class="absolute top-0 right-0 w-16 h-16 bg-purple-100 dark:bg-purple-900 opacity-20 rounded-bl-full transform transition-transform duration-300 group-hover:scale-150"></div>
                    <div class="flex items-center mb-3">
                        <div class="w-12 h-12 rounded-full text-xl text-purple-600 bg-purple-100 dark:bg-purple-900 flex items-center justify-center drop-shadow-md">
                            <User />
                        </div>
                        <h3 class="text-lg font-semibold ml-3 dark:text-gray-200">{move || tr!("api-v2-users-title")}</h3>
                    </div>
                    <p class="text-gray-600 dark:text-gray-300 mb-4 text-sm">
                        {move || tr!("api-v2-users-description")}
                    </p>
                    <div class="grid grid-cols-2 gap-1.5 text-xs font-mono">
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-purple-100 dark:hover:bg-purple-900 dark:text-gray-200">get_own_data</div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-purple-100 dark:hover:bg-purple-900 dark:text-gray-200">get_user_kudosu</div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-purple-100 dark:hover:bg-purple-900 dark:text-gray-200">get_user_scores</div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-purple-100 dark:hover:bg-purple-900 dark:text-gray-200">get_user_beatmaps</div>
                        <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-purple-100 dark:hover:bg-purple-900 dark:text-gray-200">get_user_recent_activity</div>
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-purple-100 dark:hover:bg-purple-900 dark:text-gray-200">get_user</div>
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-purple-100 dark:hover:bg-purple-900 dark:text-gray-200">get_users</div>
                            </div>
                        </div>
                        
                        {/* API 卡片 - Wiki */}
                        <div class="card bg-white dark:bg-gray-800 hover:shadow-xl transition duration-300 p-5 rounded-xl border border-gray-100 dark:border-gray-700 relative overflow-hidden group">
                            <div class="absolute top-0 right-0 w-16 h-16 bg-teal-100 dark:bg-teal-900 opacity-20 rounded-bl-full transform transition-transform duration-300 group-hover:scale-150"></div>
                            <div class="flex items-center mb-3">
                                <div class="w-12 h-12 rounded-full text-xl text-teal-600 bg-teal-100 dark:bg-teal-900 flex items-center justify-center drop-shadow-md">
                                    <BookCopy />
                                </div>
                                <h3 class="text-lg font-semibold ml-3 dark:text-gray-200">{move || tr!("api-wiki-title")}</h3>
                            </div>
                            <p class="text-gray-600 dark:text-gray-300 mb-4 text-sm">
                                {move || tr!("api-wiki-description")}
                            </p>
                            <div class="grid grid-cols-2 gap-1.5 text-xs font-mono">
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-teal-100 dark:hover:bg-teal-900 dark:text-gray-200">get_wiki_page</div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
