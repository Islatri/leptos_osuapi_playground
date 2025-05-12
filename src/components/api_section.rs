use leptos::*;
use leptos::prelude::{GlobalAttributes,ClassAttribute,ElementChild};
use lucide_leptos::{Music,UserCheck,Users,ChartBarBig,MessageCircleMore,Lock};

#[component]
pub fn ApiSection() -> impl IntoView {
    view! {
        <section id="api" class="py-16">
            <div class="container mx-auto px-4">
                <h2 class="text-3xl font-bold text-center mb-4">完整的 API 覆盖</h2>
                <p class="text-gray-600 dark:text-gray-300 text-center max-w-2xl mx-auto mb-12">
                    osynic_osuapi 提供所有 osu! API 端点的完整类型安全访问, 包括 v1 和 v2 版本.
                </p>
                
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                    // API 卡片 - Beatmaps
                    <div class="card hover:shadow-lg transition duration-300 p-4 rounded-md">
                        <div class="flex items-center mb-4">
                            <div class="w-12 h-12 rounded-full text-2xl text-pink-600 bg-pink-100 dark:bg-pink-900 flex items-center justify-center">
                                <Music />
                            </div>
                            <h3 class="text-xl font-semibold ml-4">谱面 API</h3>
                        </div>
                        <p class="text-gray-600 dark:text-gray-300 mb-4">
                            查询谱面信息, 包括详情, 评分和状态.支持按 ID, 图集 ID 或哈希查询.
                        </p>
                        <div class="grid grid-cols-2 gap-2 text-sm">
                            <div class="bg-gray-100 dark:bg-gray-700 rounded px-3 py-1">get_beatmaps</div>
                            <div class="bg-gray-100 dark:bg-gray-700 rounded px-3 py-1">lookup_beatmap</div>
                            <div class="bg-gray-100 dark:bg-gray-700 rounded px-3 py-1">get_scores</div>
                            <div class="bg-gray-100 dark:bg-gray-700 rounded px-3 py-1">get_replay</div>
                        </div>
                    </div>
                    
                    // API 卡片 - Users
                    <div class="card hover:shadow-lg transition duration-300 p-4 rounded-md">
                        <div class="flex items-center mb-4">
                            <div class="w-12 h-12 rounded-full text-2xl text-purple-600 bg-purple-100 dark:bg-purple-900 flex items-center justify-center">
                                <UserCheck />
                            </div>
                            <h3 class="text-xl font-semibold ml-4">用户 API</h3>
                        </div>
                        <p class="text-gray-600 dark:text-gray-300 mb-4">
                            获取用户资料, 统计数据, 最佳成绩和最近游玩记录.支持用户名或 ID 查询.
                        </p>
                        <div class="grid grid-cols-2 gap-2 text-sm">
                            <div class="bg-gray-100 dark:bg-gray-700 rounded px-3 py-1">get_user</div>
                            <div class="bg-gray-100 dark:bg-gray-700 rounded px-3 py-1">get_user_best</div>
                            <div class="bg-gray-100 dark:bg-gray-700 rounded px-3 py-1">get_user_recent</div>
                            <div class="bg-gray-100 dark:bg-gray-700 rounded px-3 py-1">get_user_scores</div>
                        </div>
                    </div>
                    
                    // API 卡片 - Rankings
                    <div class="card hover:shadow-lg transition duration-300 p-4 rounded-md">
                        <div class="flex items-center mb-4">
                            <div class="w-12 h-12 rounded-full text-2xl text-blue-600 bg-blue-100 dark:bg-blue-900 flex items-center justify-center">
                                <ChartBarBig />
                            </div>
                            <h3 class="text-xl font-semibold ml-4">排名 API</h3>
                        </div>
                        <p class="text-gray-600 dark:text-gray-300 mb-4">
                            获取全球排名, 国家排名和好友排名数据.支持按模式和类型筛选.
                        </p>
                        <div class="grid grid-cols-2 gap-2 text-sm">
                            <div class="bg-gray-100 dark:bg-gray-700 rounded px-3 py-1">get_user_best</div>
                            <div class="bg-gray-100 dark:bg-gray-700 rounded px-3 py-1">get_rankings</div>
                            <div class="bg-gray-100 dark:bg-gray-700 rounded px-3 py-1">get_country_ranking</div>
                            <div class="bg-gray-100 dark:bg-gray-700 rounded px-3 py-1">get_spotlights</div>
                        </div>
                    </div>
                    
                    // API 卡片 - Multiplayer
                    <div class="card hover:shadow-lg transition duration-300 p-4 rounded-md">
                        <div class="flex items-center mb-4">
                            <div class="w-12 h-12 rounded-full text-2xl text-green-600 bg-green-100 dark:bg-green-900 flex items-center justify-center">
                                <Users />
                            </div>
                            <h3 class="text-xl font-semibold ml-4">多人游戏 API</h3>
                        </div>
                        <p class="text-gray-600 dark:text-gray-300 mb-4">
                            获取多人游戏房间信息, 游戏记录和参与者数据.
                        </p>
                        <div class="grid grid-cols-2 gap-2 text-sm">
                            <div class="bg-gray-100 dark:bg-gray-700 rounded px-3 py-1">get_match</div>
                            <div class="bg-gray-100 dark:bg-gray-700 rounded px-3 py-1">get_matches</div>
                            <div class="bg-gray-100 dark:bg-gray-700 rounded px-3 py-1">get_room</div>
                            <div class="bg-gray-100 dark:bg-gray-700 rounded px-3 py-1">get_scores</div>
                        </div>
                    </div>
                    
                    // API 卡片 - Events
                    <div class="card hover:shadow-lg transition duration-300 p-4 rounded-md">
                        <div class="flex items-center mb-4">
                            <div class="w-12 h-12 rounded-full text-2xl text-yellow-600 bg-yellow-100 dark:bg-yellow-900 flex items-center justify-center">
                                <MessageCircleMore />
                            </div>
                            <h3 class="text-xl font-semibold ml-4">事件 API</h3>
                        </div>
                        <p class="text-gray-600 dark:text-gray-300 mb-4">
                            获取最新动态, 排行榜变化和新闻更新.支持按类型和日期筛选.
                        </p>
                        <div class="grid grid-cols-2 gap-2 text-sm">
                            <div class="bg-gray-100 dark:bg-gray-700 rounded px-3 py-1">get_changelog</div>
                            <div class="bg-gray-100 dark:bg-gray-700 rounded px-3 py-1">get_news</div>
                            <div class="bg-gray-100 dark:bg-gray-700 rounded px-3 py-1">get_events</div>
                            <div class="bg-gray-100 dark:bg-gray-700 rounded px-3 py-1">get_notifications</div>
                        </div>
                    </div>
                    
                    // API 卡片 - OAuth
                    <div class="card hover:shadow-lg transition duration-300 p-4 rounded-md">
                        <div class="flex items-center mb-4">
                            <div class="w-12 h-12 rounded-full text-2xl text-red-600 bg-red-100 dark:bg-red-900 flex items-center justify-center">
                                <Lock />
                            </div>
                            <h3 class="text-xl font-semibold ml-4">OAuth API</h3>
                        </div>
                        <p class="text-gray-600 dark:text-gray-300 mb-4">
                            处理 OAuth 2.0 认证流程, 获取, 刷新访问令牌并管理用户权限.
                        </p>
                        <div class="grid grid-cols-2 gap-2 text-sm">
                            <div class="bg-gray-100 dark:bg-gray-700 rounded px-3 py-1">get_token</div>
                            <div class="bg-gray-100 dark:bg-gray-700 rounded px-3 py-1">refresh_token</div>
                            <div class="bg-gray-100 dark:bg-gray-700 rounded px-3 py-1">revoke_token</div>
                            <div class="bg-gray-100 dark:bg-gray-700 rounded px-3 py-1">auth_url</div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}