use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::*;
use leptos_fluent::tr;
use lucide_leptos::{BookOpen, CodeXml, Globe, Layers, Sailboat, Server};

#[component]
pub fn FeaturesSection() -> impl IntoView {
    view! {
        <section class="py-20 bg-gradient-to-b from-gray-100 to-white dark:from-gray-800 dark:to-gray-900">
            <div class="container px-4 mx-auto">
                <h2 class="mb-4 text-4xl font-bold text-center text-gray-800 dark:text-gray-200">
                    {move || tr!("features-section-title")}
                </h2>
                <p class="mx-auto mb-16 max-w-3xl text-center text-gray-600 dark:text-gray-300">
                    {move || tr!("features-section-description")}
                </p>

                <div class="grid grid-cols-1 gap-8 md:grid-cols-3">
                    {}
                    <div class="p-6 bg-white rounded-xl border-t-4 border-pink-500 shadow-lg transition-shadow dark:bg-gray-700 hover:shadow-xl card">
                        <div class="flex items-center mb-4">
                            <span class="p-3 mr-3 text-2xl text-pink-500 bg-pink-50 rounded-lg dark:bg-pink-900/30">
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
                    {}
                    <div class="p-6 bg-white rounded-xl border-t-4 border-purple-600 shadow-lg transition-shadow dark:bg-gray-700 hover:shadow-xl card">
                        <div class="flex items-center mb-4">
                            <span class="p-3 mr-3 text-2xl text-purple-600 bg-purple-50 rounded-lg dark:bg-purple-900/30">
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
                    {}
                    <div class="p-6 bg-white rounded-xl border-t-4 border-blue-500 shadow-lg transition-shadow dark:bg-gray-700 hover:shadow-xl card">
                        <div class="flex items-center mb-4">
                            <span class="p-3 mr-3 text-2xl text-blue-500 bg-blue-50 rounded-lg dark:bg-blue-900/30">
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
                    {}
                    <div class="p-6 bg-white rounded-xl border-t-4 border-green-500 shadow-lg transition-shadow dark:bg-gray-700 hover:shadow-xl card">
                        <div class="flex items-center mb-4">
                            <span class="p-3 mr-3 text-2xl text-green-500 bg-green-50 rounded-lg dark:bg-green-900/30">
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
                    {}
                    <div class="p-6 bg-white rounded-xl border-t-4 border-amber-500 shadow-lg transition-shadow dark:bg-gray-700 hover:shadow-xl card">
                        <div class="flex items-center mb-4">
                            <span class="p-3 mr-3 text-2xl text-amber-500 bg-amber-50 rounded-lg dark:bg-amber-900/30">
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
                    {}
                    <div class="p-6 bg-white rounded-xl border-t-4 border-yellow-500 shadow-lg transition-shadow dark:bg-gray-700 hover:shadow-xl card">
                        <div class="flex items-center mb-4">
                            <span class="p-3 mr-3 text-2xl text-yellow-500 bg-yellow-50 rounded-lg dark:bg-yellow-900/30">
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
                        class="inline-block py-3 px-8 font-semibold text-white bg-pink-500 rounded-lg shadow-md transition-all transform hover:bg-pink-600 hover:shadow-lg hover:-translate-y-1"
                    >
                        {move || tr!("features-cta-button")}
                    </a>
                </div>
            </div>
        </section>
    }
}
