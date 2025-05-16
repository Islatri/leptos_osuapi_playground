use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::*;
use leptos_fluent::tr;
use lucide_leptos::{BookOpen, CodeXml, Globe, Layers, Sailboat, Server};

#[component]
pub fn FeaturesSection() -> impl IntoView {
    view! {
        <section class="py-20 bg-gradient-to-b from-gray-100 to-white dark:from-gray-800 dark:to-gray-900">
            <div class="container mx-auto px-4">
                <h2 class="text-4xl font-bold text-center mb-4 text-gray-800 dark:text-gray-200">
                    {move || tr!("features-section-title")}
                </h2>
                <p class="text-center text-gray-600 dark:text-gray-300 mb-16 max-w-3xl mx-auto">
                    {move || tr!("features-section-description")}
                </p>

                <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
                    {/* 特性卡片 1 */}
                    <div class="card bg-white dark:bg-gray-700 rounded-xl shadow-lg hover:shadow-xl transition-shadow p-6 border-t-4 border-pink-500">
                        <div class="flex items-center mb-4">
                            <span class="text-pink-500 mr-3 text-2xl bg-pink-50 dark:bg-pink-900/30 p-3 rounded-lg">
                                <Sailboat />
                            </span>
                            <h3 class="text-xl font-semibold text-gray-800 dark:text-gray-200">
                                {move || tr!("features-high-performance-title")}
                            </h3>
                        </div>
                        <p class="text-gray-600 dark:text-gray-300">
                            {move || tr!("features-high-performance-description")}
                        </p>
                    </div>

                    {/* 特性卡片 2 */}
                    <div class="card bg-white dark:bg-gray-700 rounded-xl shadow-lg hover:shadow-xl transition-shadow p-6 border-t-4 border-purple-600">
                        <div class="flex items-center mb-4">
                            <span class="text-purple-600 mr-3 text-2xl bg-purple-50 dark:bg-purple-900/30 p-3 rounded-lg">
                                <CodeXml />
                            </span>
                            <h3 class="text-xl font-semibold text-gray-800 dark:text-gray-200">
                                {move || tr!("features-type-safe-title")}
                            </h3>
                        </div>
                        <p class="text-gray-600 dark:text-gray-300">
                            {move || tr!("features-type-safe-description")}
                        </p>
                    </div>

                    {/* 特性卡片 3 */}
                    <div class="card bg-white dark:bg-gray-700 rounded-xl shadow-lg hover:shadow-xl transition-shadow p-6 border-t-4 border-blue-500">
                        <div class="flex items-center mb-4">
                            <span class="text-blue-500 mr-3 text-2xl bg-blue-50 dark:bg-blue-900/30 p-3 rounded-lg">
                                <Layers />
                            </span>
                            <h3 class="text-xl font-semibold text-gray-800 dark:text-gray-200">
                                {move || tr!("features-multi-platform-title")}
                            </h3>
                        </div>
                        <p class="text-gray-600 dark:text-gray-300">
                            {move || tr!("features-multi-platform-description")}
                        </p>
                    </div>

                    {/* 特性卡片 4 - API 支持 */}
                    <div class="card bg-white dark:bg-gray-700 rounded-xl shadow-lg hover:shadow-xl transition-shadow p-6 border-t-4 border-green-500">
                        <div class="flex items-center mb-4">
                            <span class="text-green-500 mr-3 text-2xl bg-green-50 dark:bg-green-900/30 p-3 rounded-lg">
                                <Globe />
                            </span>
                            <h3 class="text-xl font-semibold text-gray-800 dark:text-gray-200">
                                {move || tr!("features-api-support-title")}
                            </h3>
                        </div>
                        <p class="text-gray-600 dark:text-gray-300">
                            {move || tr!("features-api-support-description")}
                        </p>
                    </div>

                    {/* 特性卡片 5 - 项目结构 */}
                    <div class="card bg-white dark:bg-gray-700 rounded-xl shadow-lg hover:shadow-xl transition-shadow p-6 border-t-4 border-amber-500">
                        <div class="flex items-center mb-4">
                            <span class="text-amber-500 mr-3 text-2xl bg-amber-50 dark:bg-amber-900/30 p-3 rounded-lg">
                                <Server />
                            </span>
                            <h3 class="text-xl font-semibold text-gray-800 dark:text-gray-200">
                                {move || tr!("features-flexible-extension-title")}
                            </h3>
                        </div>
                        <p class="text-gray-600 dark:text-gray-300">
                            {move || tr!("features-flexible-extension-description")}
                        </p>
                    </div>

                    {/* 特性卡片 6 - 学习示例 */}
                    <div class="card bg-white dark:bg-gray-700 rounded-xl shadow-lg hover:shadow-xl transition-shadow p-6 border-t-4 border-yellow-500">
                        <div class="flex items-center mb-4">
                            <span class="text-yellow-500 mr-3 text-2xl bg-yellow-50 dark:bg-yellow-900/30 p-3 rounded-lg">
                                <BookOpen />
                            </span>
                            <h3 class="text-xl font-semibold text-gray-800 dark:text-gray-200">
                                {move || tr!("features-rich-examples-title")}
                            </h3>
                        </div>
                        <p class="text-gray-600 dark:text-gray-300">
                            {move || tr!("features-rich-examples-description")}
                        </p>
                    </div>
                </div>

                <div class="mt-16 text-center">
                    <a
                        href="https://crates.io/crates/osynic_osuapi"
                        target="_blank"
                        class="inline-block px-8 py-3 bg-pink-500 hover:bg-pink-600 text-white font-semibold rounded-lg shadow-md hover:shadow-lg transition-all transform hover:-translate-y-1"
                    >
                        {move || tr!("features-cta-button")}
                    </a>
                </div>
            </div>
        </section>

    }
}
