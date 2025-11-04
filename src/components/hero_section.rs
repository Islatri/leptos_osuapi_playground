use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::*;
use leptos_fluent::tr;
use lucide_leptos::{BookOpen, Globe, Package, Play, ShieldCheck};
use crate::components::api_docs_button::ApiDocsButton;

#[component]
pub fn HeroSection() -> impl IntoView {
    view! {
        <section class="overflow-hidden relative text-white bg-gray-900 dark:bg-gray-950">
            <div class="h-[72px]"></div>

            // 背景图片与渐变效果
            <div class="absolute inset-0 z-0">
                <div class="absolute inset-0 bg-gradient-to-r from-pink-600/70 via-purple-800/70 to-purple-900/70 mix-blend-multiply dark:mix-blend-color-burn"></div>
                <img
                    src="/public/irena2k.jpg"
                    alt="osu! background"
                    class="object-cover w-full h-full opacity-30 transition-opacity duration-300 dark:opacity-20"
                />
            </div>

            // 背景装饰元素
            <div class="overflow-hidden absolute inset-0 z-1">
                <div class="absolute top-20 right-1/4 w-64 h-64 rounded-full bg-pink-500/20 blur-3xl animate-pulse-slow"></div>
                <div class="absolute bottom-20 left-1/4 w-80 h-80 rounded-full delay-1000 bg-purple-600/20 blur-3xl animate-pulse-slow"></div>
            </div>

            // 将z-index从10改为20
            <div class="container relative z-20 py-16 px-4 mx-auto sm:px-6 md:py-24 lg:py-32">
                <div class="flex flex-col gap-12 lg:flex-row lg:gap-8 lg:justify-between lg:items-center">
                    <div class="max-w-3xl">
                        <div class="inline-block mb-2">
                            <span class="py-1 px-3 text-xs font-semibold tracking-wider text-pink-300 rounded-full dark:text-pink-200 bg-pink-500/20 backdrop-blur-sm dark:bg-pink-500/30">
                                {move || tr!("hero-badge")}
                            </span>
                        </div>

                        <h1 class="mb-6 text-4xl font-bold tracking-tight md:text-5xl lg:text-6xl">
                            <span class="inline-block mb-4 text-5xl text-pink-400 md:text-7xl lg:text-8xl dark:text-pink-300 animate-pulse-subtle drop-shadow-[0_0_8px_rgba(236,72,153,0.3)]">
                                <a
                                    href="https://github.com/osynicite/osynic_osuapi"
                                    target="_blank"
                                >
                                    {move || tr!("hero-title-highlight")}
                                </a>
                            </span>
                            <br />
                            <span class="text-transparent bg-clip-text bg-gradient-to-r from-pink-400 to-purple-400 dark:from-pink-300 dark:to-purple-300">
                                {move || tr!("hero-title-subtext")}
                            </span>
                        </h1>

                        <p class="mb-8 max-w-2xl text-lg leading-relaxed text-gray-200 md:text-xl dark:text-gray-300">
                            {move || tr!("hero-description")}
                        </p>

                        <div class="flex flex-wrap gap-4 mb-10">
                            <a
                                href="https://docs.rs/osynic_osuapi"
                                target="_blank"
                                class="inline-flex items-center py-3 px-6 font-medium text-white bg-gradient-to-r from-pink-500 to-purple-600 rounded-lg shadow-lg transition-all duration-300 transform cursor-pointer hover:from-pink-600 hover:to-purple-700 hover:scale-105 focus:ring-2 focus:ring-pink-400 focus:ring-offset-2 focus:ring-offset-gray-900 focus:outline-none btn-primary shadow-pink-500/20 dark:shadow-pink-700/30"
                            >
                                <div class="mr-2 group-hover:animate-bounce-subtle">
                                    <BookOpen size=20 />
                                </div>
                                <span>{move || tr!("hero-button-docs")}</span>
                            </a>
                            <a
                                href="#demo"
                                class="inline-flex items-center py-3 px-6 font-medium text-white rounded-lg border border-gray-700 shadow-lg transition-all duration-300 transform cursor-pointer dark:border-gray-600 hover:bg-gray-700 hover:scale-105 focus:ring-2 focus:ring-gray-400 focus:outline-none btn-secondary bg-gray-800/80 backdrop-blur-sm shadow-gray-900/20 dark:bg-gray-800/50 dark:hover:bg-gray-700/70"
                            >
                                <div class="mr-2 group-hover:animate-bounce-subtle">
                                    <Play size=20 />
                                </div>
                                <span>{move || tr!("hero-button-demo")}</span>
                            </a>
                            // 新添加的 API 文档按钮
                            <ApiDocsButton />
                        </div>

                        <div class="grid grid-cols-1 gap-3 text-sm text-gray-300 sm:grid-cols-2 md:grid-cols-3 dark:text-gray-200">
                            <div class="flex items-center p-3 rounded-lg border shadow-sm transition-all max-w-48 bg-gray-800/40 backdrop-blur-sm border-gray-700/40 dark:bg-gray-800/30 dark:hover:bg-gray-700/40 dark:border-gray-600/30 hover:bg-gray-700/50">
                                <div class="flex-shrink-0 mr-3 text-pink-400 dark:text-pink-300">
                                    <Package size=20 />
                                </div>
                                <span>{move || tr!("hero-feature-api")}</span>
                            </div>
                            <div class="flex items-center p-3 rounded-lg border shadow-sm transition-all max-w-48 bg-gray-800/40 backdrop-blur-sm border-gray-700/40 dark:bg-gray-800/30 dark:hover:bg-gray-700/40 dark:border-gray-600/30 hover:bg-gray-700/50">
                                <div class="flex-shrink-0 mr-3 text-pink-400 dark:text-pink-300">
                                    <ShieldCheck size=20 />
                                </div>
                                <span>{move || tr!("hero-feature-type-safe")}</span>
                            </div>
                            <div class="flex items-center p-3 rounded-lg border shadow-sm transition-all max-w-48 bg-gray-800/40 backdrop-blur-sm border-gray-700/40 dark:bg-gray-800/30 dark:hover:bg-gray-700/40 dark:border-gray-600/30 hover:bg-gray-700/50">
                                <div class="flex-shrink-0 mr-3 text-pink-400 dark:text-pink-300">
                                    <Globe size=20 />
                                </div>
                                <span>{move || tr!("hero-feature-wasm")}</span>
                            </div>
                        </div>
                    </div>

                    <div class="flex flex-shrink-0 justify-center transition-transform duration-500 transform lg:justify-end">
                        <div class="relative w-52 h-52 md:w-64 md:h-64 lg:w-72 lg:h-72 animate-float">
                            // 光效装饰
                            <div class="absolute -inset-6 bg-gradient-to-r rounded-full from-pink-500/30 to-purple-500/30 blur-2xl dark:from-pink-500/40 dark:to-purple-500/40"></div>
                            <div class="absolute -inset-8 rounded-full bg-pink-500/10 blur-2xl animate-pulse-slow dark:bg-pink-500/20"></div>

                            // Logo
                            <a
                                href="https://osu.ppy.sh"
                                target="_blank"
                                class="relative z-10 w-full h-full"
                            >
                                <img
                                    src="/public/Osu!_Logo_2016.svg"
                                    alt="osu! logo"
                                    class="relative z-10 w-full h-full transition-transform duration-300 hover:scale-105 drop-shadow-[0_0_15px_rgba(236,72,153,0.6)] dark:drop-shadow-[0_0_20px_rgba(236,72,153,0.7)]"
                                />
                            </a>

                            // 装饰元素
                            <div class="absolute -right-4 -bottom-4 w-16 h-16 rounded-full bg-purple-500/30 blur-md animate-pulse-slow dark:bg-purple-500/40"></div>
                            <div class="absolute -top-2 -left-2 w-12 h-12 rounded-full delay-700 bg-pink-500/30 blur-md animate-pulse-slow dark:bg-pink-500/40"></div>
                        </div>
                    </div>
                </div>
            </div>

            // 波浪底部装饰
            // <div class="absolute bottom-0 left-0 right-0 h-16 bg-gradient-to-t from-gray-900/70 to-transparent dark:from-gray-950/70 backdrop-blur-sm"></div>
            // <div class="absolute -bottom-px left-0 right-0 z-5">  // 将z-index从10改为5
            // <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1440 320" class="w-full h-auto text-white dark:text-gray-900 fill-current">
            // <path d="M0,224L48,213.3C96,203,192,181,288,181.3C384,181,480,203,576,218.7C672,235,768,245,864,234.7C960,224,1056,192,1152,186.7C1248,181,1344,203,1392,213.3L1440,224L1440,320L1392,320C1344,320,1248,320,1152,320C1056,320,960,320,864,320C768,320,672,320,576,320C480,320,384,320,288,320C192,320,96,320,48,320L0,320Z"></path>
            // </svg>
            // </div>

            // 波浪底部装饰 - 修复版
            <div class="overflow-hidden absolute bottom-0 left-0 w-full h-[160px]">
                <svg
                    class="absolute bottom-0 left-0 wave-animation w-[200%] min-w-[1200px] h-[130px] fill-blue-500/30 z-[1]"
                    viewBox="0 0 2880 320"
                    preserveAspectRatio="none"
                >
                    <path d="M0,128L120,144C240,160,480,192,720,176C960,160,1200,128,1440,128C1680,128,1920,160,2160,176C2400,192,2640,160,2760,144L2880,128L2880,320L2760,320C2640,320,2400,320,2160,320C1920,320,1680,320,1440,320C1200,320,960,320,720,320C480,320,240,320,120,320L0,320Z">
                        <animate
                            attributeName="transform"
                            type="translate"
                            from="0 0"
                            to="-1440 0"
                            dur="20s"
                            repeatCount="indefinite"
                        />
                    </path>
                </svg>

                <svg
                    class="absolute bottom-0 left-0 wave-animation w-[200%] min-w-[1200px] h-[110px] fill-blue-400/40 z-[2]"
                    viewBox="0 0 2880 320"
                    preserveAspectRatio="none"
                >
                    <path d="M0,160L120,176C240,192,480,224,720,208C960,192,1200,128,1440,160C1680,192,1920,224,2160,208C2400,192,2640,176,2760,168L2880,160L2880,320L2760,320C2640,320,2400,320,2160,320C1920,320,1680,320,1440,320C1200,320,960,320,720,320C480,320,240,320,120,320L0,320Z">
                        <animate
                            attributeName="transform"
                            type="translate"
                            from="0 0"
                            to="-1440 0"
                            dur="15s"
                            repeatCount="indefinite"
                        />
                    </path>
                </svg>

                <svg
                    class="absolute bottom-0 left-0 wave-animation w-[200%] min-w-[1200px] h-[90px] fill-white z-[3] dark:fill-gray-900"
                    viewBox="0 0 2880 320"
                    preserveAspectRatio="none"
                >
                    <path d="M0,192L120,176C240,160,480,128,720,144C960,160,1200,224,1440,192C1680,160,1920,96,2160,128C2400,160,2640,192,2760,192L2880,192L2880,320L2760,320C2640,320,2400,320,2160,320C1920,320,1680,320,1440,320C1200,320,960,320,720,320C480,320,240,320,120,320L0,320Z">
                        <animate
                            attributeName="transform"
                            type="translate"
                            from="0 0"
                            to="-1440 0"
                            dur="10s"
                            repeatCount="indefinite"
                        />
                    </path>
                </svg>
            </div>
        </section>
    }
}
