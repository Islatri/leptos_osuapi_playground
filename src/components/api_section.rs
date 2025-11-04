use leptos::prelude::{ClassAttribute, ElementChild, GlobalAttributes};
use leptos::*;
use leptos_fluent::tr;
use lucide_leptos::{
    Award, Bell, BookCopy, Calendar, ChartBarBig, FileText, History, House, Layers, Lock,
    MessageSquare, MessageSquareText, Music, Newspaper, Star, Trophy, User, UserCheck, Users,
};

#[component]
pub fn ApiSection() -> impl IntoView {
    view! {
        <section
            id="api"
            class="py-20 bg-gradient-to-b from-white to-gray-50 dark:from-gray-900 dark:to-gray-950"
        >
            <div class="container px-4 mx-auto">
                <h2 class="mb-6 text-4xl font-bold text-center text-transparent bg-clip-text bg-gradient-to-r from-pink-500 to-purple-600">
                    {move || tr!("api-section-title")}
                </h2>
                <p class="mx-auto mb-16 max-w-3xl text-lg text-center text-gray-600 dark:text-gray-300">
                    {move || tr!("api-section-description")}
                </p>

                {}
                <div class="mb-20">
                    <div class="flex justify-center items-center mb-12">
                        <div class="w-1/4 h-px bg-gray-200 dark:bg-gray-700"></div>
                        <h3 class="py-2 px-6 mx-6 text-2xl font-bold text-pink-600 bg-pink-50 rounded-full border-2 border-pink-200 dark:text-pink-400 dark:border-pink-900 dark:bg-pink-950">
                            <a href="https://github.com/ppy/osu-api/wiki" target="_blank">
                                {move || tr!("api-v1-title")}
                            </a>
                        </h3>
                        <div class="w-1/4 h-px bg-gray-200 dark:bg-gray-700"></div>
                    </div>

                    <div class="grid grid-cols-1 gap-8 md:grid-cols-2 lg:grid-cols-3">
                        {}
                        <div class="overflow-hidden relative p-6 bg-white rounded-xl border border-gray-100 transition duration-300 dark:bg-gray-800 dark:border-gray-700 hover:shadow-xl card group">
                            <div class="absolute top-0 right-0 w-20 h-20 bg-pink-100 rounded-bl-full opacity-20 transition-transform duration-300 transform dark:bg-pink-900 group-hover:scale-150"></div>
                            <div class="flex items-center mb-4">
                                <div class="flex justify-center items-center w-14 h-14 text-2xl text-pink-600 bg-pink-100 rounded-full dark:bg-pink-900 drop-shadow-md">
                                    <Music />
                                </div>
                                <h3 class="ml-4 text-xl font-semibold dark:text-gray-200">
                                    {move || tr!("api-beatmaps-title")}
                                </h3>
                            </div>
                            <p class="mb-6 text-gray-600 dark:text-gray-300">
                                {move || tr!("api-beatmaps-description")}
                            </p>
                            <div class="grid grid-cols-2 gap-2 font-mono text-sm">
                                <div class="py-1.5 px-3 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-pink-100 dark:hover:bg-pink-900">
                                    get_beatmaps
                                </div>
                            </div>
                        </div>
                        {}
                        <div class="overflow-hidden relative p-6 bg-white rounded-xl border border-gray-100 transition duration-300 dark:bg-gray-800 dark:border-gray-700 hover:shadow-xl card group">
                            <div class="absolute top-0 right-0 w-20 h-20 bg-purple-100 rounded-bl-full opacity-20 transition-transform duration-300 transform dark:bg-purple-900 group-hover:scale-150"></div>
                            <div class="flex items-center mb-4">
                                <div class="flex justify-center items-center w-14 h-14 text-2xl text-purple-600 bg-purple-100 rounded-full dark:bg-purple-900 drop-shadow-md">
                                    <UserCheck />
                                </div>
                                <h3 class="ml-4 text-xl font-semibold dark:text-gray-200">
                                    {move || tr!("api-users-title")}
                                </h3>
                            </div>
                            <p class="mb-6 text-gray-600 dark:text-gray-300">
                                {move || tr!("api-users-description")}
                            </p>
                            <div class="grid grid-cols-2 gap-2 font-mono text-sm">
                                <div class="py-1.5 px-3 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-purple-100 dark:hover:bg-purple-900">
                                    get_user
                                </div>
                                <div class="py-1.5 px-3 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-purple-100 dark:hover:bg-purple-900">
                                    get_user_best
                                </div>
                                <div class="py-1.5 px-3 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-purple-100 dark:hover:bg-purple-900">
                                    get_user_recent
                                </div>
                            </div>
                        </div>
                        {}
                        <div class="overflow-hidden relative p-6 bg-white rounded-xl border border-gray-100 transition duration-300 dark:bg-gray-800 dark:border-gray-700 hover:shadow-xl card group">
                            <div class="absolute top-0 right-0 w-20 h-20 bg-indigo-100 rounded-bl-full opacity-20 transition-transform duration-300 transform dark:bg-indigo-900 group-hover:scale-150"></div>
                            <div class="flex items-center mb-4">
                                <div class="flex justify-center items-center w-14 h-14 text-2xl text-indigo-600 bg-indigo-100 rounded-full dark:bg-indigo-900 drop-shadow-md">
                                    <Star />
                                </div>
                                <h3 class="ml-4 text-xl font-semibold dark:text-gray-200">
                                    {move || tr!("api-scores-title")}
                                </h3>
                            </div>
                            <p class="mb-6 text-gray-600 dark:text-gray-300">
                                {move || tr!("api-scores-description")}
                            </p>
                            <div class="grid grid-cols-2 gap-2 font-mono text-sm">
                                <div class="py-1.5 px-3 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-indigo-100 dark:hover:bg-indigo-900">
                                    get_scores
                                </div>
                            </div>
                        </div>
                        {}
                        <div class="overflow-hidden relative p-6 bg-white rounded-xl border border-gray-100 transition duration-300 dark:bg-gray-800 dark:border-gray-700 hover:shadow-xl card group">
                            <div class="absolute top-0 right-0 w-20 h-20 bg-blue-100 rounded-bl-full opacity-20 transition-transform duration-300 transform dark:bg-blue-900 group-hover:scale-150"></div>
                            <div class="flex items-center mb-4">
                                <div class="flex justify-center items-center w-14 h-14 text-2xl text-blue-600 bg-blue-100 rounded-full dark:bg-blue-900 drop-shadow-md">
                                    <ChartBarBig />
                                </div>
                                <h3 class="ml-4 text-xl font-semibold dark:text-gray-200">
                                    {move || tr!("api-replays-title")}
                                </h3>
                            </div>
                            <p class="mb-6 text-gray-600 dark:text-gray-300">
                                {move || tr!("api-replays-description")}
                            </p>
                            <div class="grid grid-cols-2 gap-2 font-mono text-sm">
                                <div class="py-1.5 px-3 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-blue-100 dark:hover:bg-blue-900">
                                    get_replay
                                </div>
                            </div>
                        </div>
                        {}
                        <div class="overflow-hidden relative p-6 bg-white rounded-xl border border-gray-100 transition duration-300 dark:bg-gray-800 dark:border-gray-700 hover:shadow-xl card group">
                            <div class="absolute top-0 right-0 w-20 h-20 bg-green-100 rounded-bl-full opacity-20 transition-transform duration-300 transform dark:bg-green-900 group-hover:scale-150"></div>
                            <div class="flex items-center mb-4">
                                <div class="flex justify-center items-center w-14 h-14 text-2xl text-green-600 bg-green-100 rounded-full dark:bg-green-900 drop-shadow-md">
                                    <Users />
                                </div>
                                <h3 class="ml-4 text-xl font-semibold dark:text-gray-200">
                                    {move || tr!("api-multiplayer-title")}
                                </h3>
                            </div>
                            <p class="mb-6 text-gray-600 dark:text-gray-300">
                                {move || tr!("api-multiplayer-description")}
                            </p>
                            <div class="grid grid-cols-2 gap-2 font-mono text-sm">
                                <div class="py-1.5 px-3 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-green-100 dark:hover:bg-green-900">
                                    get_match
                                </div>
                            </div>
                        </div>
                    </div>
                </div>

                {}
                <div>
                    <div class="flex justify-center items-center mb-12">
                        <div class="w-1/4 h-px bg-gray-200 dark:bg-gray-700"></div>
                        <h3 class="py-2 px-6 mx-6 text-2xl font-bold text-purple-600 bg-purple-50 rounded-full border-2 border-purple-200 dark:text-purple-400 dark:border-purple-900 dark:bg-purple-950">
                            <a href="https://osu.ppy.sh/docs/index.html" target="_blank">
                                {move || tr!("api-v2-title")}
                            </a>
                        </h3>
                        <div class="w-1/4 h-px bg-gray-200 dark:bg-gray-700"></div>
                    </div>

                    <div class="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-4">
                        {}
                        <div class="overflow-hidden relative p-5 bg-white rounded-xl border border-gray-100 transition duration-300 dark:bg-gray-800 dark:border-gray-700 hover:shadow-xl card group">
                            <div class="absolute top-0 right-0 w-16 h-16 bg-red-100 rounded-bl-full opacity-20 transition-transform duration-300 transform dark:bg-red-900 group-hover:scale-150"></div>
                            <div class="flex items-center mb-3">
                                <div class="flex justify-center items-center w-12 h-12 text-xl text-red-600 bg-red-100 rounded-full dark:bg-red-900 drop-shadow-md">
                                    <Lock />
                                </div>
                                <h3 class="ml-3 text-lg font-semibold dark:text-gray-200">
                                    {move || tr!("api-oauth-title")}
                                </h3>
                            </div>
                            <p class="mb-4 text-sm text-gray-600 dark:text-gray-300">
                                {move || tr!("api-oauth-description")}
                            </p>
                            <div class="grid grid-cols-2 gap-1.5 font-mono text-xs">
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-red-100 dark:hover:bg-red-900">
                                    get_token_with_code
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-red-100 dark:hover:bg-red-900">
                                    get_token_without_code
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-red-100 dark:hover:bg-red-900">
                                    refresh_token
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-red-100 dark:hover:bg-red-900">
                                    revoke_current_token
                                </div>
                            </div>
                        </div>
                        {}
                        <div class="overflow-hidden relative p-5 bg-white rounded-xl border border-gray-100 transition duration-300 dark:bg-gray-800 dark:border-gray-700 hover:shadow-xl card group">
                            <div class="absolute top-0 right-0 w-16 h-16 bg-pink-100 rounded-bl-full opacity-20 transition-transform duration-300 transform dark:bg-pink-900 group-hover:scale-150"></div>
                            <div class="flex items-center mb-3">
                                <div class="flex justify-center items-center w-12 h-12 text-xl text-pink-600 bg-pink-100 rounded-full dark:bg-pink-900 drop-shadow-md">
                                    <Music />
                                </div>
                                <h3 class="ml-3 text-lg font-semibold dark:text-gray-200">
                                    {move || tr!("api-v2-beatmaps-title")}
                                </h3>
                            </div>
                            <p class="mb-4 text-sm text-gray-600 dark:text-gray-300">
                                {move || tr!("api-v2-beatmaps-description")}
                            </p>
                            <div class="grid grid-cols-2 gap-1.5 font-mono text-xs">
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-pink-100 dark:hover:bg-pink-900">
                                    get_beatmap_packs
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-pink-100 dark:hover:bg-pink-900">
                                    get_beatmap_pack
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-pink-100 dark:hover:bg-pink-900">
                                    lookup_beatmap
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-pink-100 dark:hover:bg-pink-900">
                                    get_beatmap
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-pink-100 dark:hover:bg-pink-900">
                                    get_beatmap_attributes
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-pink-100 dark:hover:bg-pink-900">
                                    get_beatmaps
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-pink-100 dark:hover:bg-pink-900">
                                    get_scores
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-pink-100 dark:hover:bg-pink-900">
                                    get_solo_scores
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-pink-100 dark:hover:bg-pink-900">
                                    get_user_score
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-pink-100 dark:hover:bg-pink-900">
                                    get_user_scores
                                </div>
                            </div>
                        </div>
                        {}
                        <div class="overflow-hidden relative p-5 bg-white rounded-xl border border-gray-100 transition duration-300 dark:bg-gray-800 dark:border-gray-700 hover:shadow-xl card group">
                            <div class="absolute top-0 right-0 w-16 h-16 bg-fuchsia-100 rounded-bl-full opacity-20 transition-transform duration-300 transform dark:bg-fuchsia-900 group-hover:scale-150"></div>
                            <div class="flex items-center mb-3">
                                <div class="flex justify-center items-center w-12 h-12 text-xl text-fuchsia-600 bg-fuchsia-100 rounded-full dark:bg-fuchsia-900 drop-shadow-md">
                                    <Layers />
                                </div>
                                <h3 class="ml-3 text-lg font-semibold dark:text-gray-200">
                                    {move || tr!("api-beatmapsets-title")}
                                </h3>
                            </div>
                            <p class="mb-4 text-sm text-gray-600 dark:text-gray-300">
                                {move || tr!("api-beatmapsets-description")}
                            </p>
                            <div class="grid grid-cols-2 gap-1.5 font-mono text-xs">
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-fuchsia-100 dark:hover:bg-fuchsia-900">
                                    get_beatmapsets_discussions_posts
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-fuchsia-100 dark:hover:bg-fuchsia-900">
                                    get_beatmapsets_discussions_vote
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-fuchsia-100 dark:hover:bg-fuchsia-900">
                                    get_beatmapsets_discussions
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-fuchsia-100 dark:hover:bg-fuchsia-900">
                                    search
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-fuchsia-100 dark:hover:bg-fuchsia-900">
                                    lookup
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-fuchsia-100 dark:hover:bg-fuchsia-900">
                                    get_beatmapset
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-fuchsia-100 dark:hover:bg-fuchsia-900">
                                    download
                                </div>
                            </div>
                        </div>
                        {}
                        <div class="overflow-hidden relative p-5 bg-white rounded-xl border border-gray-100 transition duration-300 dark:bg-gray-800 dark:border-gray-700 hover:shadow-xl card group">
                            <div class="absolute top-0 right-0 w-16 h-16 bg-indigo-100 rounded-bl-full opacity-20 transition-transform duration-300 transform dark:bg-indigo-900 group-hover:scale-150"></div>
                            <div class="flex items-center mb-3">
                                <div class="flex justify-center items-center w-12 h-12 text-xl text-indigo-600 bg-indigo-100 rounded-full dark:bg-indigo-900 drop-shadow-md">
                                    <History />
                                </div>
                                <h3 class="ml-3 text-lg font-semibold dark:text-gray-200">
                                    {move || tr!("api-changelog-title")}
                                </h3>
                            </div>
                            <p class="mb-4 text-sm text-gray-600 dark:text-gray-300">
                                {move || tr!("api-changelog-description")}
                            </p>
                            <div class="grid grid-cols-2 gap-1.5 font-mono text-xs">
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-indigo-100 dark:hover:bg-indigo-900">
                                    get_changelog_build
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-indigo-100 dark:hover:bg-indigo-900">
                                    get_changelog_listing
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-indigo-100 dark:hover:bg-indigo-900">
                                    lookup_changelog_build
                                </div>
                            </div>
                        </div>
                        {}
                        <div class="overflow-hidden relative p-5 bg-white rounded-xl border border-gray-100 transition duration-300 dark:bg-gray-800 dark:border-gray-700 hover:shadow-xl card group">
                            <div class="absolute top-0 right-0 w-16 h-16 bg-blue-100 rounded-bl-full opacity-20 transition-transform duration-300 transform dark:bg-blue-900 group-hover:scale-150"></div>
                            <div class="flex items-center mb-3">
                                <div class="flex justify-center items-center w-12 h-12 text-xl text-blue-600 bg-blue-100 rounded-full dark:bg-blue-900 drop-shadow-md">
                                    <MessageSquare />
                                </div>
                                <h3 class="ml-3 text-lg font-semibold dark:text-gray-200">
                                    {move || tr!("api-chat-title")}
                                </h3>
                            </div>
                            <p class="mb-4 text-sm text-gray-600 dark:text-gray-300">
                                {move || tr!("api-chat-description")}
                            </p>
                            <div class="grid grid-cols-2 gap-1.5 font-mono text-xs">
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-blue-100 dark:hover:bg-blue-900">
                                    chat_keepalive
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-blue-100 dark:hover:bg-blue-900">
                                    create_new_pm
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-blue-100 dark:hover:bg-blue-900">
                                    get_updates
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-blue-100 dark:hover:bg-blue-900">
                                    get_channel_messages
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-blue-100 dark:hover:bg-blue-900">
                                    send_message_to_channel
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-blue-100 dark:hover:bg-blue-900">
                                    join_channel
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-blue-100 dark:hover:bg-blue-900">
                                    leave_channel
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-blue-100 dark:hover:bg-blue-900">
                                    mark_channel_as_read
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-blue-100 dark:hover:bg-blue-900">
                                    get_channel_list
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-blue-100 dark:hover:bg-blue-900">
                                    create_channel
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-blue-100 dark:hover:bg-blue-900">
                                    get_channel
                                </div>
                            </div>
                        </div>
                        {}
                        <div class="overflow-hidden relative p-5 bg-white rounded-xl border border-gray-100 transition duration-300 dark:bg-gray-800 dark:border-gray-700 hover:shadow-xl card group">
                            <div class="absolute top-0 right-0 w-16 h-16 bg-cyan-100 rounded-bl-full opacity-20 transition-transform duration-300 transform dark:bg-cyan-900 group-hover:scale-150"></div>
                            <div class="flex items-center mb-3">
                                <div class="flex justify-center items-center w-12 h-12 text-xl text-cyan-600 bg-cyan-100 rounded-full dark:bg-cyan-900 drop-shadow-md">
                                    <MessageSquareText />
                                </div>
                                <h3 class="ml-3 text-lg font-semibold dark:text-gray-200">
                                    {move || tr!("api-comments-title")}
                                </h3>
                            </div>
                            <p class="mb-4 text-sm text-gray-600 dark:text-gray-300">
                                {move || tr!("api-comments-description")}
                            </p>
                            <div class="grid grid-cols-2 gap-1.5 font-mono text-xs">
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-cyan-100 dark:hover:bg-cyan-900">
                                    get_comments
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-cyan-100 dark:hover:bg-cyan-900">
                                    post_comment
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-cyan-100 dark:hover:bg-cyan-900">
                                    get_comment
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-cyan-100 dark:hover:bg-cyan-900">
                                    edit_comment
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-cyan-100 dark:hover:bg-cyan-900">
                                    delete_comment
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-cyan-100 dark:hover:bg-cyan-900">
                                    add_comment_vote
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-cyan-100 dark:hover:bg-cyan-900">
                                    remove_comment_vote
                                </div>
                            </div>
                        </div>
                        {}
                        <div class="overflow-hidden relative p-5 bg-white rounded-xl border border-gray-100 transition duration-300 dark:bg-gray-800 dark:border-gray-700 hover:shadow-xl card group">
                            <div class="absolute top-0 right-0 w-16 h-16 bg-green-100 rounded-bl-full opacity-20 transition-transform duration-300 transform dark:bg-green-900 group-hover:scale-150"></div>
                            <div class="flex items-center mb-3">
                                <div class="flex justify-center items-center w-12 h-12 text-xl text-green-600 bg-green-100 rounded-full dark:bg-green-900 drop-shadow-md">
                                    <Calendar />
                                </div>
                                <h3 class="ml-3 text-lg font-semibold dark:text-gray-200">
                                    {move || tr!("api-events-title")}
                                </h3>
                            </div>
                            <p class="mb-4 text-sm text-gray-600 dark:text-gray-300">
                                {move || tr!("api-events-description")}
                            </p>
                            <div class="grid grid-cols-2 gap-1.5 font-mono text-xs">
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-green-100 dark:hover:bg-green-900">
                                    get_events
                                </div>
                            </div>
                        </div>
                        {}
                        <div class="overflow-hidden relative p-5 bg-white rounded-xl border border-gray-100 transition duration-300 dark:bg-gray-800 dark:border-gray-700 hover:shadow-xl card group">
                            <div class="absolute top-0 right-0 w-16 h-16 bg-amber-100 rounded-bl-full opacity-20 transition-transform duration-300 transform dark:bg-amber-900 group-hover:scale-150"></div>
                            <div class="flex items-center mb-3">
                                <div class="flex justify-center items-center w-12 h-12 text-xl text-amber-600 bg-amber-100 rounded-full dark:bg-amber-900 drop-shadow-md">
                                    <FileText />
                                </div>
                                <h3 class="ml-3 text-lg font-semibold dark:text-gray-200">
                                    {move || tr!("api-forum-title")}
                                </h3>
                            </div>
                            <p class="mb-4 text-sm text-gray-600 dark:text-gray-300">
                                {move || tr!("api-forum-description")}
                            </p>
                            <div class="grid grid-cols-2 gap-1.5 font-mono text-xs">
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-amber-100 dark:hover:bg-amber-900">
                                    reply_topic
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-amber-100 dark:hover:bg-amber-900">
                                    get_topics_listing
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-amber-100 dark:hover:bg-amber-900">
                                    create_topic
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-amber-100 dark:hover:bg-amber-900">
                                    get_topic_and_posts
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-amber-100 dark:hover:bg-amber-900">
                                    edit_topic
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-amber-100 dark:hover:bg-amber-900">
                                    edit_post
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-amber-100 dark:hover:bg-amber-900">
                                    get_forum_listing
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-amber-100 dark:hover:bg-amber-900">
                                    get_forum_and_topic
                                </div>
                            </div>
                        </div>
                        {}
                        <div class="overflow-hidden relative p-5 bg-white rounded-xl border border-gray-100 transition duration-300 dark:bg-gray-800 dark:border-gray-700 hover:shadow-xl card group">
                            <div class="absolute top-0 right-0 w-16 h-16 bg-orange-100 rounded-bl-full opacity-20 transition-transform duration-300 transform dark:bg-orange-900 group-hover:scale-150"></div>
                            <div class="flex items-center mb-3">
                                <div class="flex justify-center items-center w-12 h-12 text-xl text-orange-600 bg-orange-100 rounded-full dark:bg-orange-900 drop-shadow-md">
                                    <House />
                                </div>
                                <h3 class="ml-3 text-lg font-semibold dark:text-gray-200">
                                    {move || tr!("api-home-title")}
                                </h3>
                            </div>
                            <p class="mb-4 text-sm text-gray-600 dark:text-gray-300">
                                {move || tr!("api-home-description")}
                            </p>
                            <div class="grid grid-cols-2 gap-1.5 font-mono text-xs">
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-orange-100 dark:hover:bg-orange-900">
                                    search
                                </div>
                            </div>
                        </div>
                        {}
                        <div class="overflow-hidden relative p-5 bg-white rounded-xl border border-gray-100 transition duration-300 dark:bg-gray-800 dark:border-gray-700 hover:shadow-xl card group">
                            <div class="absolute top-0 right-0 w-16 h-16 bg-rose-100 rounded-bl-full opacity-20 transition-transform duration-300 transform dark:bg-rose-900 group-hover:scale-150"></div>
                            <div class="flex items-center mb-3">
                                <div class="flex justify-center items-center w-12 h-12 text-xl text-rose-600 bg-rose-100 rounded-full dark:bg-rose-900 drop-shadow-md">
                                    <Award />
                                </div>
                                <h3 class="ml-3 text-lg font-semibold dark:text-gray-200">
                                    {move || tr!("api-matches-title")}
                                </h3>
                            </div>
                            <p class="mb-4 text-sm text-gray-600 dark:text-gray-300">
                                {move || tr!("api-matches-description")}
                            </p>
                            <div class="grid grid-cols-2 gap-1.5 font-mono text-xs">
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-rose-100 dark:hover:bg-rose-900">
                                    get_matches_listing
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-rose-100 dark:hover:bg-rose-900">
                                    get_match
                                </div>
                            </div>
                        </div>
                        {}
                        <div class="overflow-hidden relative p-5 bg-white rounded-xl border border-gray-100 transition duration-300 dark:bg-gray-800 dark:border-gray-700 hover:shadow-xl card group">
                            <div class="absolute top-0 right-0 w-16 h-16 bg-emerald-100 rounded-bl-full opacity-20 transition-transform duration-300 transform dark:bg-emerald-900 group-hover:scale-150"></div>
                            <div class="flex items-center mb-3">
                                <div class="flex justify-center items-center w-12 h-12 text-xl text-emerald-600 bg-emerald-100 rounded-full dark:bg-emerald-900 drop-shadow-md">
                                    <Users />
                                </div>
                                <h3 class="ml-3 text-lg font-semibold dark:text-gray-200">
                                    {move || tr!("api-v2-multiplayer-title")}
                                </h3>
                            </div>
                            <p class="mb-4 text-sm text-gray-600 dark:text-gray-300">
                                {move || tr!("api-v2-multiplayer-description")}
                            </p>
                            <div class="grid grid-cols-2 gap-1.5 font-mono text-xs">
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-emerald-100 dark:hover:bg-emerald-900">
                                    get_user_high_score
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-emerald-100 dark:hover:bg-emerald-900">
                                    get_scores
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-emerald-100 dark:hover:bg-emerald-900">
                                    get_score
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-emerald-100 dark:hover:bg-emerald-900">
                                    get_multiplayer_rooms
                                </div>
                            </div>
                        </div>
                        {}
                        <div class="overflow-hidden relative p-5 bg-white rounded-xl border border-gray-100 transition duration-300 dark:bg-gray-800 dark:border-gray-700 hover:shadow-xl card group">
                            <div class="absolute top-0 right-0 w-16 h-16 bg-blue-100 rounded-bl-full opacity-20 transition-transform duration-300 transform dark:bg-blue-900 group-hover:scale-150"></div>
                            <div class="flex items-center mb-3">
                                <div class="flex justify-center items-center w-12 h-12 text-xl text-blue-600 bg-blue-100 rounded-full dark:bg-blue-900 drop-shadow-md">
                                    <Newspaper />
                                </div>
                                <h3 class="ml-3 text-lg font-semibold dark:text-gray-200">
                                    {move || tr!("api-news-title")}
                                </h3>
                            </div>
                            <p class="mb-4 text-sm text-gray-600 dark:text-gray-300">
                                {move || tr!("api-news-description")}
                            </p>
                            <div class="grid grid-cols-2 gap-1.5 font-mono text-xs">
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-blue-100 dark:hover:bg-blue-900">
                                    get_news_listing
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-blue-100 dark:hover:bg-blue-900">
                                    get_news_post
                                </div>
                            </div>
                        </div>
                        {}
                        <div class="overflow-hidden relative p-5 bg-white rounded-xl border border-gray-100 transition duration-300 dark:bg-gray-800 dark:border-gray-700 hover:shadow-xl card group">
                            <div class="absolute top-0 right-0 w-16 h-16 bg-violet-100 rounded-bl-full opacity-20 transition-transform duration-300 transform dark:bg-violet-900 group-hover:scale-150"></div>
                            <div class="flex items-center mb-3">
                                <div class="flex justify-center items-center w-12 h-12 text-xl text-violet-600 bg-violet-100 rounded-full dark:bg-violet-900 drop-shadow-md">
                                    <Bell />
                                </div>
                                <h3 class="ml-3 text-lg font-semibold dark:text-gray-200">
                                    {move || tr!("api-notifications-title")}
                                </h3>
                            </div>
                            <p class="mb-4 text-sm text-gray-600 dark:text-gray-300">
                                {move || tr!("api-notifications-description")}
                            </p>
                            <div class="grid grid-cols-2 gap-1.5 font-mono text-xs">
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-violet-100 dark:hover:bg-violet-900">
                                    get_notifications
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-violet-100 dark:hover:bg-violet-900">
                                    mark_notifications_as_read
                                </div>
                            </div>
                        </div>
                        {}
                        <div class="overflow-hidden relative p-5 bg-white rounded-xl border border-gray-100 transition duration-300 dark:bg-gray-800 dark:border-gray-700 hover:shadow-xl card group">
                            <div class="absolute top-0 right-0 w-16 h-16 bg-yellow-100 rounded-bl-full opacity-20 transition-transform duration-300 transform dark:bg-yellow-900 group-hover:scale-150"></div>
                            <div class="flex items-center mb-3">
                                <div class="flex justify-center items-center w-12 h-12 text-xl text-yellow-600 bg-yellow-100 rounded-full dark:bg-yellow-900 drop-shadow-md">
                                    <Trophy />
                                </div>
                                <h3 class="ml-3 text-lg font-semibold dark:text-gray-200">
                                    {move || tr!("api-rankings-title")}
                                </h3>
                            </div>
                            <p class="mb-4 text-sm text-gray-600 dark:text-gray-300">
                                {move || tr!("api-rankings-description")}
                            </p>
                            <div class="grid grid-cols-2 gap-1.5 font-mono text-xs">
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-yellow-100 dark:hover:bg-yellow-900">
                                    get_rankings
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-yellow-100 dark:hover:bg-yellow-900">
                                    get_kudosu_rankings
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-yellow-100 dark:hover:bg-yellow-900">
                                    get_spotlight
                                </div>
                            </div>
                        </div>
                        {}
                        <div class="overflow-hidden relative p-5 bg-white rounded-xl border border-gray-100 transition duration-300 dark:bg-gray-800 dark:border-gray-700 hover:shadow-xl card group">
                            <div class="absolute top-0 right-0 w-16 h-16 bg-red-100 rounded-bl-full opacity-20 transition-transform duration-300 transform dark:bg-red-900 group-hover:scale-150"></div>
                            <div class="flex items-center mb-3">
                                <div class="flex justify-center items-center w-12 h-12 text-xl text-red-600 bg-red-100 rounded-full dark:bg-red-900 drop-shadow-md">
                                    <Star />
                                </div>
                                <h3 class="ml-3 text-lg font-semibold dark:text-gray-200">
                                    {move || tr!("api-v2-scores-title")}
                                </h3>
                            </div>
                            <p class="mb-4 text-sm text-gray-600 dark:text-gray-300">
                                {move || tr!("api-v2-scores-description")}
                            </p>
                            <div class="grid grid-cols-2 gap-1.5 font-mono text-xs">
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-red-100 dark:hover:bg-red-900">
                                    get_scores
                                </div>
                            </div>
                        </div>
                        {}
                        <div class="overflow-hidden relative p-5 bg-white rounded-xl border border-gray-100 transition duration-300 dark:bg-gray-800 dark:border-gray-700 hover:shadow-xl card group">
                            <div class="absolute top-0 right-0 w-16 h-16 bg-purple-100 rounded-bl-full opacity-20 transition-transform duration-300 transform dark:bg-purple-900 group-hover:scale-150"></div>
                            <div class="flex items-center mb-3">
                                <div class="flex justify-center items-center w-12 h-12 text-xl text-purple-600 bg-purple-100 rounded-full dark:bg-purple-900 drop-shadow-md">
                                    <User />
                                </div>
                                <h3 class="ml-3 text-lg font-semibold dark:text-gray-200">
                                    {move || tr!("api-v2-users-title")}
                                </h3>
                            </div>
                            <p class="mb-4 text-sm text-gray-600 dark:text-gray-300">
                                {move || tr!("api-v2-users-description")}
                            </p>
                            <div class="grid grid-cols-2 gap-1.5 font-mono text-xs">
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-purple-100 dark:hover:bg-purple-900">
                                    get_own_data
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-purple-100 dark:hover:bg-purple-900">
                                    get_user_kudosu
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-purple-100 dark:hover:bg-purple-900">
                                    get_user_scores
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-purple-100 dark:hover:bg-purple-900">
                                    get_user_beatmaps
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-purple-100 dark:hover:bg-purple-900">
                                    get_user_recent_activity
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-purple-100 dark:hover:bg-purple-900">
                                    get_user
                                </div>
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-purple-100 dark:hover:bg-purple-900">
                                    get_users
                                </div>
                            </div>
                        </div>
                        {}
                        <div class="overflow-hidden relative p-5 bg-white rounded-xl border border-gray-100 transition duration-300 dark:bg-gray-800 dark:border-gray-700 hover:shadow-xl card group">
                            <div class="absolute top-0 right-0 w-16 h-16 bg-teal-100 rounded-bl-full opacity-20 transition-transform duration-300 transform dark:bg-teal-900 group-hover:scale-150"></div>
                            <div class="flex items-center mb-3">
                                <div class="flex justify-center items-center w-12 h-12 text-xl text-teal-600 bg-teal-100 rounded-full dark:bg-teal-900 drop-shadow-md">
                                    <BookCopy />
                                </div>
                                <h3 class="ml-3 text-lg font-semibold dark:text-gray-200">
                                    {move || tr!("api-wiki-title")}
                                </h3>
                            </div>
                            <p class="mb-4 text-sm text-gray-600 dark:text-gray-300">
                                {move || tr!("api-wiki-description")}
                            </p>
                            <div class="grid grid-cols-2 gap-1.5 font-mono text-xs">
                                <div class="py-1 px-2 bg-gray-100 rounded-md transition-colors dark:text-gray-200 dark:bg-gray-700 hover:bg-teal-100 dark:hover:bg-teal-900">
                                    get_wiki_page
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
