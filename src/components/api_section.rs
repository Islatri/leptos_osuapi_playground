use leptos::*;
use leptos::prelude::{GlobalAttributes, ClassAttribute, ElementChild};
use lucide_leptos::{
    Music, UserCheck, Users, ChartBarBig,  Lock, 
    Layers, History, MessageSquare, MessageSquareText, 
    Calendar, FileText, House, Award, Newspaper, Bell, Trophy, Star, User, 
    BookCopy
};

#[component]
pub fn ApiSection() -> impl IntoView {
    view! {
        <section id="api" class="py-20 bg-gradient-to-b from-white to-gray-50 dark:from-gray-900 dark:to-gray-950">
            <div class="container mx-auto px-4">
                <h2 class="text-4xl font-bold text-center mb-6 bg-clip-text text-transparent bg-gradient-to-r from-pink-500 to-purple-600">完整的 API 覆盖</h2>
                <p class="text-gray-600 dark:text-gray-300 text-center max-w-3xl mx-auto mb-16 text-lg">
                    OsynicOsuapi 提供所有 osu! API 端点的完整类型安全访问, 包括 v1 和 v2 版本的大部分.
                </p>

                // V1 API Section
                <div class="mb-20">
                    <div class="flex items-center justify-center mb-12">
                        <div class="h-px bg-gray-200 dark:bg-gray-700 w-1/4"></div>
                        <h3 class="text-2xl font-bold mx-6 text-pink-600 dark:text-pink-400 px-6 py-2 rounded-full border-2 border-pink-200 dark:border-pink-900 bg-pink-50 dark:bg-pink-950">API v1</h3>
                        <div class="h-px bg-gray-200 dark:bg-gray-700 w-1/4"></div>
                    </div>

                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
                        // API 卡片 - Beatmaps
                        <div class="card bg-white dark:bg-gray-800 hover:shadow-xl transition duration-300 p-6 rounded-xl border border-gray-100 dark:border-gray-700 relative overflow-hidden group">
                            <div class="absolute top-0 right-0 w-20 h-20 bg-pink-100 dark:bg-pink-900 opacity-20 rounded-bl-full transform transition-transform duration-300 group-hover:scale-150"></div>
                            <div class="flex items-center mb-4">
                                <div class="w-14 h-14 rounded-full text-2xl text-pink-600 bg-pink-100 dark:bg-pink-900 flex items-center justify-center drop-shadow-md">
                                    <Music />
                                </div>
                                <h3 class="text-xl font-semibold ml-4">谱面 API</h3>
                            </div>
                            <p class="text-gray-600 dark:text-gray-300 mb-6">
                                查询谱面信息, 包括详情, 评分和状态.支持按 ID, 图集 ID 或哈希查询.
                            </p>
                            <div class="grid grid-cols-2 gap-2 text-sm font-mono">
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-3 py-1.5 transition-colors hover:bg-pink-100 dark:hover:bg-pink-900">get_beatmaps</div>
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-3 py-1.5 transition-colors hover:bg-pink-100 dark:hover:bg-pink-900">lookup_beatmap</div>
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-3 py-1.5 transition-colors hover:bg-pink-100 dark:hover:bg-pink-900">get_scores</div>
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-3 py-1.5 transition-colors hover:bg-pink-100 dark:hover:bg-pink-900">get_replay</div>
                            </div>
                        </div>
                        
                        // API 卡片 - Users
                        <div class="card bg-white dark:bg-gray-800 hover:shadow-xl transition duration-300 p-6 rounded-xl border border-gray-100 dark:border-gray-700 relative overflow-hidden group">
                            <div class="absolute top-0 right-0 w-20 h-20 bg-purple-100 dark:bg-purple-900 opacity-20 rounded-bl-full transform transition-transform duration-300 group-hover:scale-150"></div>
                            <div class="flex items-center mb-4">
                                <div class="w-14 h-14 rounded-full text-2xl text-purple-600 bg-purple-100 dark:bg-purple-900 flex items-center justify-center drop-shadow-md">
                                    <UserCheck />
                                </div>
                                <h3 class="text-xl font-semibold ml-4">用户 API</h3>
                            </div>
                            <p class="text-gray-600 dark:text-gray-300 mb-6">
                                获取用户资料, 统计数据, 最佳成绩和最近游玩记录.支持用户名或 ID 查询.
                            </p>
                            <div class="grid grid-cols-2 gap-2 text-sm font-mono">
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-3 py-1.5 transition-colors hover:bg-purple-100 dark:hover:bg-purple-900">get_user</div>
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-3 py-1.5 transition-colors hover:bg-purple-100 dark:hover:bg-purple-900">get_user_best</div>
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-3 py-1.5 transition-colors hover:bg-purple-100 dark:hover:bg-purple-900">get_user_recent</div>
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-3 py-1.5 transition-colors hover:bg-purple-100 dark:hover:bg-purple-900">get_user_scores</div>
                            </div>
                        </div>
                        
                        // API 卡片 - Scores
                        <div class="card bg-white dark:bg-gray-800 hover:shadow-xl transition duration-300 p-6 rounded-xl border border-gray-100 dark:border-gray-700 relative overflow-hidden group">
                            <div class="absolute top-0 right-0 w-20 h-20 bg-indigo-100 dark:bg-indigo-900 opacity-20 rounded-bl-full transform transition-transform duration-300 group-hover:scale-150"></div>
                            <div class="flex items-center mb-4">
                                <div class="w-14 h-14 rounded-full text-2xl text-indigo-600 bg-indigo-100 dark:bg-indigo-900 flex items-center justify-center drop-shadow-md">
                                    <Star />
                                </div>
                                <h3 class="text-xl font-semibold ml-4">分数 API</h3>
                            </div>
                            <p class="text-gray-600 dark:text-gray-300 mb-6">
                                获取不同谱面和用户的分数信息, 支持按模式和筛选器查询.
                            </p>
                            <div class="grid grid-cols-2 gap-2 text-sm font-mono">
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-3 py-1.5 transition-colors hover:bg-indigo-100 dark:hover:bg-indigo-900">get_scores</div>
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-3 py-1.5 transition-colors hover:bg-indigo-100 dark:hover:bg-indigo-900">get_user_scores</div>
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-3 py-1.5 transition-colors hover:bg-indigo-100 dark:hover:bg-indigo-900">get_replay</div>
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-3 py-1.5 transition-colors hover:bg-indigo-100 dark:hover:bg-indigo-900">get_score_info</div>
                            </div>
                        </div>
                        
                        // API 卡片 - Rankings
                        <div class="card bg-white dark:bg-gray-800 hover:shadow-xl transition duration-300 p-6 rounded-xl border border-gray-100 dark:border-gray-700 relative overflow-hidden group">
                            <div class="absolute top-0 right-0 w-20 h-20 bg-blue-100 dark:bg-blue-900 opacity-20 rounded-bl-full transform transition-transform duration-300 group-hover:scale-150"></div>
                            <div class="flex items-center mb-4">
                                <div class="w-14 h-14 rounded-full text-2xl text-blue-600 bg-blue-100 dark:bg-blue-900 flex items-center justify-center drop-shadow-md">
                                    <ChartBarBig />
                                </div>
                                <h3 class="text-xl font-semibold ml-4">排名 API</h3>
                            </div>
                            <p class="text-gray-600 dark:text-gray-300 mb-6">
                                获取全球排名, 国家排名和好友排名数据.支持按模式和类型筛选.
                            </p>
                            <div class="grid grid-cols-2 gap-2 text-sm font-mono">
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-3 py-1.5 transition-colors hover:bg-blue-100 dark:hover:bg-blue-900">get_user_best</div>
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-3 py-1.5 transition-colors hover:bg-blue-100 dark:hover:bg-blue-900">get_rankings</div>
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-3 py-1.5 transition-colors hover:bg-blue-100 dark:hover:bg-blue-900">get_country_ranking</div>
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-3 py-1.5 transition-colors hover:bg-blue-100 dark:hover:bg-blue-900">get_spotlights</div>
                            </div>
                        </div>
                        
                        // API 卡片 - Multiplayer
                        <div class="card bg-white dark:bg-gray-800 hover:shadow-xl transition duration-300 p-6 rounded-xl border border-gray-100 dark:border-gray-700 relative overflow-hidden group">
                            <div class="absolute top-0 right-0 w-20 h-20 bg-green-100 dark:bg-green-900 opacity-20 rounded-bl-full transform transition-transform duration-300 group-hover:scale-150"></div>
                            <div class="flex items-center mb-4">
                                <div class="w-14 h-14 rounded-full text-2xl text-green-600 bg-green-100 dark:bg-green-900 flex items-center justify-center drop-shadow-md">
                                    <Users />
                                </div>
                                <h3 class="text-xl font-semibold ml-4">多人游戏 API</h3>
                            </div>
                            <p class="text-gray-600 dark:text-gray-300 mb-6">
                                获取多人游戏房间信息, 游戏记录和参与者数据.
                            </p>
                            <div class="grid grid-cols-2 gap-2 text-sm font-mono">
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-3 py-1.5 transition-colors hover:bg-green-100 dark:hover:bg-green-900">get_match</div>
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-3 py-1.5 transition-colors hover:bg-green-100 dark:hover:bg-green-900">get_matches</div>
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-3 py-1.5 transition-colors hover:bg-green-100 dark:hover:bg-green-900">get_room</div>
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-3 py-1.5 transition-colors hover:bg-green-100 dark:hover:bg-green-900">get_scores</div>
                            </div>
                        </div>
                    </div>
                </div>
                
                // V2 API Section
                <div>
                    <div class="flex items-center justify-center mb-12">
                        <div class="h-px bg-gray-200 dark:bg-gray-700 w-1/4"></div>
                        <h3 class="text-2xl font-bold mx-6 text-purple-600 dark:text-purple-400 px-6 py-2 rounded-full border-2 border-purple-200 dark:border-purple-900 bg-purple-50 dark:bg-purple-950">API v2</h3>
                        <div class="h-px bg-gray-200 dark:bg-gray-700 w-1/4"></div>
                    </div>

                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
                        // API 卡片 - Beatmaps
                        <div class="card bg-white dark:bg-gray-800 hover:shadow-xl transition duration-300 p-5 rounded-xl border border-gray-100 dark:border-gray-700 relative overflow-hidden group">
                            <div class="absolute top-0 right-0 w-16 h-16 bg-pink-100 dark:bg-pink-900 opacity-20 rounded-bl-full transform transition-transform duration-300 group-hover:scale-150"></div>
                            <div class="flex items-center mb-3">
                                <div class="w-12 h-12 rounded-full text-xl text-pink-600 bg-pink-100 dark:bg-pink-900 flex items-center justify-center drop-shadow-md">
                                    <Music />
                                </div>
                                <h3 class="text-lg font-semibold ml-3">谱面 API</h3>
                            </div>
                            <p class="text-gray-600 dark:text-gray-300 mb-4 text-sm">
                                查询谱面信息, 详情和元数据.
                            </p>
                            <div class="grid grid-cols-2 gap-1.5 text-xs font-mono">
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-pink-100 dark:hover:bg-pink-900">get_beatmap</div>
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-pink-100 dark:hover:bg-pink-900">lookup</div>
                            </div>
                        </div>
                        
                        // API 卡片 - Beatmap Sets
                        <div class="card bg-white dark:bg-gray-800 hover:shadow-xl transition duration-300 p-5 rounded-xl border border-gray-100 dark:border-gray-700 relative overflow-hidden group">
                            <div class="absolute top-0 right-0 w-16 h-16 bg-fuchsia-100 dark:bg-fuchsia-900 opacity-20 rounded-bl-full transform transition-transform duration-300 group-hover:scale-150"></div>
                            <div class="flex items-center mb-3">
                                <div class="w-12 h-12 rounded-full text-xl text-fuchsia-600 bg-fuchsia-100 dark:bg-fuchsia-900 flex items-center justify-center drop-shadow-md">
                                    <Layers />
                                </div>
                                <h3 class="text-lg font-semibold ml-3">谱面集 API</h3>
                            </div>
                            <p class="text-gray-600 dark:text-gray-300 mb-4 text-sm">
                                获取谱面集信息和其包含的谱面.
                            </p>
                            <div class="grid grid-cols-2 gap-1.5 text-xs font-mono">
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-fuchsia-100 dark:hover:bg-fuchsia-900">get_beatmapset</div>
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-fuchsia-100 dark:hover:bg-fuchsia-900">search</div>
                            </div>
                        </div>
                        
                        // API 卡片 - Changelog
                        <div class="card bg-white dark:bg-gray-800 hover:shadow-xl transition duration-300 p-5 rounded-xl border border-gray-100 dark:border-gray-700 relative overflow-hidden group">
                            <div class="absolute top-0 right-0 w-16 h-16 bg-indigo-100 dark:bg-indigo-900 opacity-20 rounded-bl-full transform transition-transform duration-300 group-hover:scale-150"></div>
                            <div class="flex items-center mb-3">
                                <div class="w-12 h-12 rounded-full text-xl text-indigo-600 bg-indigo-100 dark:bg-indigo-900 flex items-center justify-center drop-shadow-md">
                                    <History />
                                </div>
                                <h3 class="text-lg font-semibold ml-3">更改日志 API</h3>
                            </div>
                            <p class="text-gray-600 dark:text-gray-300 mb-4 text-sm">
                                获取游戏更新和变化记录.
                            </p>
                            <div class="grid grid-cols-2 gap-1.5 text-xs font-mono">
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-indigo-100 dark:hover:bg-indigo-900">get_changelog</div>
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-indigo-100 dark:hover:bg-indigo-900">lookup_build</div>
                            </div>
                        </div>
                        
                        // API 卡片 - Chat
                        <div class="card bg-white dark:bg-gray-800 hover:shadow-xl transition duration-300 p-5 rounded-xl border border-gray-100 dark:border-gray-700 relative overflow-hidden group">
                            <div class="absolute top-0 right-0 w-16 h-16 bg-blue-100 dark:bg-blue-900 opacity-20 rounded-bl-full transform transition-transform duration-300 group-hover:scale-150"></div>
                            <div class="flex items-center mb-3">
                                <div class="w-12 h-12 rounded-full text-xl text-blue-600 bg-blue-100 dark:bg-blue-900 flex items-center justify-center drop-shadow-md">
                                    <MessageSquare />
                                </div>
                                <h3 class="text-lg font-semibold ml-3">聊天 API</h3>
                            </div>
                            <p class="text-gray-600 dark:text-gray-300 mb-4 text-sm">
                                获取聊天历史和发送消息.
                            </p>
                            <div class="grid grid-cols-2 gap-1.5 text-xs font-mono">
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-blue-100 dark:hover:bg-blue-900">get_updates</div>
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-blue-100 dark:hover:bg-blue-900">send_message</div>
                            </div>
                        </div>
                        
                        // API 卡片 - Comments
                        <div class="card bg-white dark:bg-gray-800 hover:shadow-xl transition duration-300 p-5 rounded-xl border border-gray-100 dark:border-gray-700 relative overflow-hidden group">
                            <div class="absolute top-0 right-0 w-16 h-16 bg-cyan-100 dark:bg-cyan-900 opacity-20 rounded-bl-full transform transition-transform duration-300 group-hover:scale-150"></div>
                            <div class="flex items-center mb-3">
                                <div class="w-12 h-12 rounded-full text-xl text-cyan-600 bg-cyan-100 dark:bg-cyan-900 flex items-center justify-center drop-shadow-md">
                                    <MessageSquareText />
                                </div>
                                <h3 class="text-lg font-semibold ml-3">评论 API</h3>
                            </div>
                            <p class="text-gray-600 dark:text-gray-300 mb-4 text-sm">
                                获取和发布评论内容.
                            </p>
                            <div class="grid grid-cols-2 gap-1.5 text-xs font-mono">
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-cyan-100 dark:hover:bg-cyan-900">get_comments</div>
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-cyan-100 dark:hover:bg-cyan-900">post_comment</div>
                            </div>
                        </div>
                        
                        // API 卡片 - Events
                        <div class="card bg-white dark:bg-gray-800 hover:shadow-xl transition duration-300 p-5 rounded-xl border border-gray-100 dark:border-gray-700 relative overflow-hidden group">
                            <div class="absolute top-0 right-0 w-16 h-16 bg-green-100 dark:bg-green-900 opacity-20 rounded-bl-full transform transition-transform duration-300 group-hover:scale-150"></div>
                            <div class="flex items-center mb-3">
                                <div class="w-12 h-12 rounded-full text-xl text-green-600 bg-green-100 dark:bg-green-900 flex items-center justify-center drop-shadow-md">
                                    <Calendar />
                                </div>
                                <h3 class="text-lg font-semibold ml-3">事件 API</h3>
                            </div>
                            <p class="text-gray-600 dark:text-gray-300 mb-4 text-sm">
                                获取最新事件和动态.
                            </p>
                            <div class="grid grid-cols-2 gap-1.5 text-xs font-mono">
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-green-100 dark:hover:bg-green-900">get_events</div>
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-green-100 dark:hover:bg-green-900">get_feeds</div>
                            </div>
                        </div>
                        
                        // API 卡片 - Forum
                        <div class="card bg-white dark:bg-gray-800 hover:shadow-xl transition duration-300 p-5 rounded-xl border border-gray-100 dark:border-gray-700 relative overflow-hidden group">
                            <div class="absolute top-0 right-0 w-16 h-16 bg-amber-100 dark:bg-amber-900 opacity-20 rounded-bl-full transform transition-transform duration-300 group-hover:scale-150"></div>
                            <div class="flex items-center mb-3">
                                <div class="w-12 h-12 rounded-full text-xl text-amber-600 bg-amber-100 dark:bg-amber-900 flex items-center justify-center drop-shadow-md">
                                    <FileText />
                                </div>
                                <h3 class="text-lg font-semibold ml-3">论坛 API</h3>
                            </div>
                            <p class="text-gray-600 dark:text-gray-300 mb-4 text-sm">
                                获取论坛主题和帖子.
                            </p>
                            <div class="grid grid-cols-2 gap-1.5 text-xs font-mono">
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-amber-100 dark:hover:bg-amber-900">get_topic</div>
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-amber-100 dark:hover:bg-amber-900">get_posts</div>
                            </div>
                        </div>
                        
                        // API 卡片 - Home
                        <div class="card bg-white dark:bg-gray-800 hover:shadow-xl transition duration-300 p-5 rounded-xl border border-gray-100 dark:border-gray-700 relative overflow-hidden group">
                            <div class="absolute top-0 right-0 w-16 h-16 bg-orange-100 dark:bg-orange-900 opacity-20 rounded-bl-full transform transition-transform duration-300 group-hover:scale-150"></div>
                            <div class="flex items-center mb-3">
                                <div class="w-12 h-12 rounded-full text-xl text-orange-600 bg-orange-100 dark:bg-orange-900 flex items-center justify-center drop-shadow-md">
                                    <House />
                                </div>
                                <h3 class="text-lg font-semibold ml-3">主页 API</h3>
                            </div>
                            <p class="text-gray-600 dark:text-gray-300 mb-4 text-sm">
                                获取个人主页数据.
                            </p>
                            <div class="grid grid-cols-2 gap-1.5 text-xs font-mono">
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-orange-100 dark:hover:bg-orange-900">get_news</div>
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-orange-100 dark:hover:bg-orange-900">get_updates</div>
                            </div>
                        </div>
                        
                        // API 卡片 - Matches
                        <div class="card bg-white dark:bg-gray-800 hover:shadow-xl transition duration-300 p-5 rounded-xl border border-gray-100 dark:border-gray-700 relative overflow-hidden group">
                            <div class="absolute top-0 right-0 w-16 h-16 bg-rose-100 dark:bg-rose-900 opacity-20 rounded-bl-full transform transition-transform duration-300 group-hover:scale-150"></div>
                            <div class="flex items-center mb-3">
                                <div class="w-12 h-12 rounded-full text-xl text-rose-600 bg-rose-100 dark:bg-rose-900 flex items-center justify-center drop-shadow-md">
                                    <Award />
                                </div>
                                <h3 class="text-lg font-semibold ml-3">比赛 API</h3>
                            </div>
                            <p class="text-gray-600 dark:text-gray-300 mb-4 text-sm">
                                获取官方比赛信息.
                            </p>
                            <div class="grid grid-cols-2 gap-1.5 text-xs font-mono">
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-rose-100 dark:hover:bg-rose-900">get_match</div>
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-rose-100 dark:hover:bg-rose-900">get_results</div>
                            </div>
                        </div>
                        
                        // API 卡片 - Multiplayer
                        <div class="card bg-white dark:bg-gray-800 hover:shadow-xl transition duration-300 p-5 rounded-xl border border-gray-100 dark:border-gray-700 relative overflow-hidden group">
                            <div class="absolute top-0 right-0 w-16 h-16 bg-emerald-100 dark:bg-emerald-900 opacity-20 rounded-bl-full transform transition-transform duration-300 group-hover:scale-150"></div>
                            <div class="flex items-center mb-3">
                                <div class="w-12 h-12 rounded-full text-xl text-emerald-600 bg-emerald-100 dark:bg-emerald-900 flex items-center justify-center drop-shadow-md">
                                    <Users />
                                </div>
                                <h3 class="text-lg font-semibold ml-3">多人游戏 API</h3>
                            </div>
                            <p class="text-gray-600 dark:text-gray-300 mb-4 text-sm">
                                获取和管理多人游戏房间.
                            </p>
                            <div class="grid grid-cols-2 gap-1.5 text-xs font-mono">
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-emerald-100 dark:hover:bg-emerald-900">get_room</div>
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-emerald-100 dark:hover:bg-emerald-900">create_room</div>
                            </div>
                        </div>
                        
                        // API 卡片 - News
                        <div class="card bg-white dark:bg-gray-800 hover:shadow-xl transition duration-300 p-5 rounded-xl border border-gray-100 dark:border-gray-700 relative overflow-hidden group">
                            <div class="absolute top-0 right-0 w-16 h-16 bg-blue-100 dark:bg-blue-900 opacity-20 rounded-bl-full transform transition-transform duration-300 group-hover:scale-150"></div>
                            <div class="flex items-center mb-3">
                                <div class="w-12 h-12 rounded-full text-xl text-blue-600 bg-blue-100 dark:bg-blue-900 flex items-center justify-center drop-shadow-md">
                                    <Newspaper />
                                </div>
                                <h3 class="text-lg font-semibold ml-3">新闻 API</h3>
                            </div>
                            <p class="text-gray-600 dark:text-gray-300 mb-4 text-sm">
                                获取游戏最新新闻.
                            </p>
                            <div class="grid grid-cols-2 gap-1.5 text-xs font-mono">
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-blue-100 dark:hover:bg-blue-900">get_news</div>
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-blue-100 dark:hover:bg-blue-900">get_post</div>
                            </div>
                        </div>
                        
                        // API 卡片 - Notifications
                        <div class="card bg-white dark:bg-gray-800 hover:shadow-xl transition duration-300 p-5 rounded-xl border border-gray-100 dark:border-gray-700 relative overflow-hidden group">
                            <div class="absolute top-0 right-0 w-16 h-16 bg-violet-100 dark:bg-violet-900 opacity-20 rounded-bl-full transform transition-transform duration-300 group-hover:scale-150"></div>
                            <div class="flex items-center mb-3">
                                <div class="w-12 h-12 rounded-full text-xl text-violet-600 bg-violet-100 dark:bg-violet-900 flex items-center justify-center drop-shadow-md">
                                    <Bell />
                                </div>
                                <h3 class="text-lg font-semibold ml-3">通知 API</h3>
                            </div>
                            <p class="text-gray-600 dark:text-gray-300 mb-4 text-sm">
                                获取个人通知信息.
                            </p>
                            <div class="grid grid-cols-2 gap-1.5 text-xs font-mono">
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-violet-100 dark:hover:bg-violet-900">get_notifications</div>
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-violet-100 dark:hover:bg-violet-900">mark_read</div>
                            </div>
                        </div>
                        
                        // API 卡片 - Rankings
                        <div class="card bg-white dark:bg-gray-800 hover:shadow-xl transition duration-300 p-5 rounded-xl border border-gray-100 dark:border-gray-700 relative overflow-hidden group">
                            <div class="absolute top-0 right-0 w-16 h-16 bg-yellow-100 dark:bg-yellow-900 opacity-20 rounded-bl-full transform transition-transform duration-300 group-hover:scale-150"></div>
                            <div class="flex items-center mb-3">
                                <div class="w-12 h-12 rounded-full text-xl text-yellow-600 bg-yellow-100 dark:bg-yellow-900 flex items-center justify-center drop-shadow-md">
                                    <Trophy />
                                </div>
                                <h3 class="text-lg font-semibold ml-3">排名 API</h3>
                            </div>
                            <p class="text-gray-600 dark:text-gray-300 mb-4 text-sm">
                                获取详细排名数据.
                            </p>
                            <div class="grid grid-cols-2 gap-1.5 text-xs font-mono">
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-yellow-100 dark:hover:bg-yellow-900">get_rankings</div>
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-yellow-100 dark:hover:bg-yellow-900">get_spotlight</div>
                            </div>
                        </div>
                        
                        // API 卡片 - Scores
                        <div class="card bg-white dark:bg-gray-800 hover:shadow-xl transition duration-300 p-5 rounded-xl border border-gray-100 dark:border-gray-700 relative overflow-hidden group">
                            <div class="absolute top-0 right-0 w-16 h-16 bg-red-100 dark:bg-red-900 opacity-20 rounded-bl-full transform transition-transform duration-300 group-hover:scale-150"></div>
                            <div class="flex items-center mb-3">
                                <div class="w-12 h-12 rounded-full text-xl text-red-600 bg-red-100 dark:bg-red-900 flex items-center justify-center drop-shadow-md">
                                    <Star />
                                </div>
                                <h3 class="text-lg font-semibold ml-3">分数 API</h3>
                            </div>
                            <p class="text-gray-600 dark:text-gray-300 mb-4 text-sm">
                                获取详细分数信息.
                            </p>
                            <div class="grid grid-cols-2 gap-1.5 text-xs font-mono">
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-red-100 dark:hover:bg-red-900">get_score</div>
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-red-100 dark:hover:bg-red-900">get_scores</div>
                            </div>
                        </div>
                        
                        // API 卡片 - Users
                        <div class="card bg-white dark:bg-gray-800 hover:shadow-xl transition duration-300 p-5 rounded-xl border border-gray-100 dark:border-gray-700 relative overflow-hidden group">
                            <div class="absolute top-0 right-0 w-16 h-16 bg-purple-100 dark:bg-purple-900 opacity-20 rounded-bl-full transform transition-transform duration-300 group-hover:scale-150"></div>
                            <div class="flex items-center mb-3">
                                <div class="w-12 h-12 rounded-full text-xl text-purple-600 bg-purple-100 dark:bg-purple-900 flex items-center justify-center drop-shadow-md">
                                    <User />
                                </div>
                                <h3 class="text-lg font-semibold ml-3">用户 API</h3>
                            </div>
                            <p class="text-gray-600 dark:text-gray-300 mb-4 text-sm">
                                获取用户详细信息.
                            </p>
                            <div class="grid grid-cols-2 gap-1.5 text-xs font-mono">
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-purple-100 dark:hover:bg-purple-900">get_user</div>
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-purple-100 dark:hover:bg-purple-900">get_scores</div>
                            </div>
                        </div>
                        
                        // API 卡片 - Wiki
                        <div class="card bg-white dark:bg-gray-800 hover:shadow-xl transition duration-300 p-5 rounded-xl border border-gray-100 dark:border-gray-700 relative overflow-hidden group">
                            <div class="absolute top-0 right-0 w-16 h-16 bg-teal-100 dark:bg-teal-900 opacity-20 rounded-bl-full transform transition-transform duration-300 group-hover:scale-150"></div>
                            <div class="flex items-center mb-3">
                                <div class="w-12 h-12 rounded-full text-xl text-teal-600 bg-teal-100 dark:bg-teal-900 flex items-center justify-center drop-shadow-md">
                                    <BookCopy />
                                </div>
                                <h3 class="text-lg font-semibold ml-3">维基 API</h3>
                            </div>
                            <p class="text-gray-600 dark:text-gray-300 mb-4 text-sm">
                                获取维基页面内容.
                            </p>
                            <div class="grid grid-cols-2 gap-1.5 text-xs font-mono">
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-teal-100 dark:hover:bg-teal-900">get_page</div>
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-teal-100 dark:hover:bg-teal-900">search</div>
                            </div>
                        </div>
                        
                        // API 卡片 - OAuth
                        <div class="card bg-white dark:bg-gray-800 hover:shadow-xl transition duration-300 p-5 rounded-xl border border-gray-100 dark:border-gray-700 relative overflow-hidden group">
                            <div class="absolute top-0 right-0 w-16 h-16 bg-red-100 dark:bg-red-900 opacity-20 rounded-bl-full transform transition-transform duration-300 group-hover:scale-150"></div>
                            <div class="flex items-center mb-3">
                                <div class="w-12 h-12 rounded-full text-xl text-red-600 bg-red-100 dark:bg-red-900 flex items-center justify-center drop-shadow-md">
                                    <Lock />
                                </div>
                                <h3 class="text-lg font-semibold ml-3">OAuth API</h3>
                            </div>
                            <p class="text-gray-600 dark:text-gray-300 mb-4 text-sm">
                                处理 OAuth 2.0 认证流程.
                            </p>
                            <div class="grid grid-cols-2 gap-1.5 text-xs font-mono">
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-red-100 dark:hover:bg-red-900">get_token</div>
                                <div class="bg-gray-100 dark:bg-gray-700 rounded-md px-2 py-1 transition-colors hover:bg-red-100 dark:hover:bg-red-900">refresh_token</div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}