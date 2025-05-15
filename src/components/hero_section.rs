use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::*;
use leptos_fluent::tr;
use lucide_leptos::{BookOpen, Globe, Package, Play, ShieldCheck};

#[component]
pub fn HeroSection() -> impl IntoView {
    view! {
        <section class="relative bg-gray-900 dark:bg-gray-950 text-white overflow-hidden">
            <div class="h-[72px]"></div>

            // 背景图片与渐变效果
            <div class="absolute inset-0 z-0">
                <div class="absolute inset-0 bg-gradient-to-r from-pink-600/70 via-purple-800/70 to-purple-900/70 mix-blend-multiply dark:mix-blend-color-burn"></div>
                <img
                    src="/public/irena2k.jpg"
                    alt="osu! background"
                    class="w-full h-full object-cover opacity-30 dark:opacity-20 transition-opacity duration-300"
                />
            </div>

            // 背景装饰元素
            <div class="absolute inset-0 z-1 overflow-hidden">
                <div class="absolute top-20 right-1/4 w-64 h-64 bg-pink-500/20 rounded-full blur-3xl animate-pulse-slow"></div>
                <div class="absolute bottom-20 left-1/4 w-80 h-80 bg-purple-600/20 rounded-full blur-3xl animate-pulse-slow delay-1000"></div>
            </div>

            <div class="container mx-auto px-4 sm:px-6 py-16 md:py-24 lg:py-32 relative z-20">  // 将z-index从10改为20
                <div class="flex flex-col lg:flex-row lg:items-center lg:justify-between gap-12 lg:gap-8">
                    <div class="max-w-3xl">
                        <div class="mb-2 inline-block">
                            <span class="px-3 py-1 text-xs font-semibold tracking-wider bg-pink-500/20 dark:bg-pink-500/30 backdrop-blur-sm rounded-full text-pink-300 dark:text-pink-200">
                                {move || tr!("hero-badge")}
                            </span>
                        </div>

                        <h1 class="text-4xl md:text-5xl lg:text-6xl font-bold mb-6 tracking-tight">
                            <span class="text-pink-400 dark:text-pink-300 text-5xl md:text-7xl lg:text-8xl inline-block mb-4 animate-pulse-subtle drop-shadow-[0_0_8px_rgba(236,72,153,0.3)]">
                                <a href="https://github.com/osynicite/osynic_osuapi"
                                target="_blank">
                                    {move || tr!("hero-title-highlight")}
                                </a>
                            </span>
                            <br />
                            <span class="text-transparent bg-clip-text bg-gradient-to-r from-pink-400 to-purple-400 dark:from-pink-300 dark:to-purple-300">
                                {move || tr!("hero-title-subtext")}
                            </span>
                        </h1>

                        <p class="text-lg md:text-xl mb-8 text-gray-200 dark:text-gray-300 leading-relaxed max-w-2xl">
                            {move || tr!("hero-description")}
                        </p>

                        <div class="flex flex-wrap gap-4 mb-10">
                            <a href="https://docs.rs/osynic_osuapi" target="_blank"
                            class="btn-primary inline-flex items-center px-6 py-3 rounded-lg bg-gradient-to-r from-pink-500 to-purple-600 hover:from-pink-600 hover:to-purple-700 text-white font-medium shadow-lg shadow-pink-500/20 dark:shadow-pink-700/30 transition-all duration-300 transform hover:scale-105 focus:ring-2 focus:ring-pink-400 focus:ring-offset-2 focus:ring-offset-gray-900 focus:outline-none cursor-pointer">
                                <div class="mr-2 group-hover:animate-bounce-subtle">
                                    <BookOpen size=20 />
                                </div>
                                <span>{move || tr!("hero-button-docs")}</span>
                            </a>
                            <a href="#demo"
                            class="btn-secondary inline-flex items-center px-6 py-3 rounded-lg bg-gray-800/80 dark:bg-gray-800/50 backdrop-blur-sm hover:bg-gray-700 dark:hover:bg-gray-700/70 text-white font-medium border border-gray-700 dark:border-gray-600 shadow-lg shadow-gray-900/20 transition-all duration-300 transform hover:scale-105 focus:ring-2 focus:ring-gray-400 focus:outline-none cursor-pointer">
                                <div class="mr-2 group-hover:animate-bounce-subtle">
                                    <Play size=20 />
                                </div>
                                <span>{move || tr!("hero-button-demo")}</span>
                            </a>
                        </div>

                        <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-3 text-sm text-gray-300 dark:text-gray-200">
                            <div class="flex items-center max-w-48 p-3 bg-gray-800/40 dark:bg-gray-800/30 rounded-lg backdrop-blur-sm hover:bg-gray-700/50 dark:hover:bg-gray-700/40 transition-all border border-gray-700/40 dark:border-gray-600/30 shadow-sm">
                                <div class="text-pink-400 dark:text-pink-300 mr-3 flex-shrink-0">
                                    <Package size=20 />
                                </div>
                                <span>{move || tr!("hero-feature-api")}</span>
                            </div>
                            <div class="flex items-center max-w-48 p-3 bg-gray-800/40 dark:bg-gray-800/30 rounded-lg backdrop-blur-sm hover:bg-gray-700/50 dark:hover:bg-gray-700/40 transition-all border border-gray-700/40 dark:border-gray-600/30 shadow-sm">
                                <div class="text-pink-400 dark:text-pink-300 mr-3 flex-shrink-0">
                                    <ShieldCheck size=20 />
                                </div>
                                <span>{move || tr!("hero-feature-type-safe")}</span>
                            </div>
                            <div class="flex items-center max-w-48 p-3 bg-gray-800/40 dark:bg-gray-800/30 rounded-lg backdrop-blur-sm hover:bg-gray-700/50 dark:hover:bg-gray-700/40 transition-all border border-gray-700/40 dark:border-gray-600/30 shadow-sm">
                                <div class="text-pink-400 dark:text-pink-300 mr-3 flex-shrink-0">
                                    <Globe size=20 />
                                </div>
                                <span>{move || tr!("hero-feature-wasm")}</span>
                            </div>
                        </div>
                    </div>

                    <div class="flex-shrink-0 flex justify-center lg:justify-end transform transition-transform duration-500">
                        <div class="relative w-52 h-52 md:w-64 md:h-64 lg:w-72 lg:h-72 animate-float">
                            // 光效装饰
                            <div class="absolute -inset-6 bg-gradient-to-r from-pink-500/30 to-purple-500/30 dark:from-pink-500/40 dark:to-purple-500/40 rounded-full blur-2xl"></div>
                            <div class="absolute -inset-8 bg-pink-500/10 dark:bg-pink-500/20 rounded-full blur-2xl animate-pulse-slow"></div>

                            // Logo
                            <a href="https://osu.ppy.sh" 
                                target="_blank"
                                class="w-full h-full relative z-10"
                                >
                                <img
                                    src="/public/Osu!_Logo_2016.svg"
                                    alt="osu! logo"
                                    class="w-full h-full relative z-10 drop-shadow-[0_0_15px_rgba(236,72,153,0.6)] dark:drop-shadow-[0_0_20px_rgba(236,72,153,0.7)] hover:scale-105 transition-transform duration-300"
                                />
                            </a>

                            // 装饰元素
                            <div class="absolute -bottom-4 -right-4 w-16 h-16 bg-purple-500/30 dark:bg-purple-500/40 rounded-full blur-md animate-pulse-slow"></div>
                            <div class="absolute -top-2 -left-2 w-12 h-12 bg-pink-500/30 dark:bg-pink-500/40 rounded-full blur-md animate-pulse-slow delay-700"></div>
                        </div>
                    </div>
                </div>
            </div>

            // 波浪底部装饰
            // <div class="absolute bottom-0 left-0 right-0 h-16 bg-gradient-to-t from-gray-900/70 to-transparent dark:from-gray-950/70 backdrop-blur-sm"></div>
            // <div class="absolute -bottom-px left-0 right-0 z-5">  // 将z-index从10改为5
            //     <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1440 320" class="w-full h-auto text-white dark:text-gray-900 fill-current">
            //         <path d="M0,224L48,213.3C96,203,192,181,288,181.3C384,181,480,203,576,218.7C672,235,768,245,864,234.7C960,224,1056,192,1152,186.7C1248,181,1344,203,1392,213.3L1440,224L1440,320L1392,320C1344,320,1248,320,1152,320C1056,320,960,320,864,320C768,320,672,320,576,320C480,320,384,320,288,320C192,320,96,320,48,320L0,320Z"></path>
            //     </svg>
            // </div>

            // 波浪底部装饰 - 修复版
            <div class="absolute bottom-0 left-0 w-full h-[160px] overflow-hidden">
            <svg class="wave-animation absolute bottom-0 left-0 w-[200%] min-w-[1200px] h-[130px] fill-blue-500/30 z-[1]"
                viewBox="0 0 2880 320" preserveAspectRatio="none">
                <path d="M0,128L120,144C240,160,480,192,720,176C960,160,1200,128,1440,128C1680,128,1920,160,2160,176C2400,192,2640,160,2760,144L2880,128L2880,320L2760,320C2640,320,2400,320,2160,320C1920,320,1680,320,1440,320C1200,320,960,320,720,320C480,320,240,320,120,320L0,320Z">
                    <animate attributeName="transform" type="translate" from="0 0" to="-1440 0" dur="20s" repeatCount="indefinite"/>
                </path>
            </svg>

            <svg class="wave-animation absolute bottom-0 left-0 w-[200%] min-w-[1200px] h-[110px] fill-blue-400/40 z-[2]"
                viewBox="0 0 2880 320" preserveAspectRatio="none">
                <path d="M0,160L120,176C240,192,480,224,720,208C960,192,1200,128,1440,160C1680,192,1920,224,2160,208C2400,192,2640,176,2760,168L2880,160L2880,320L2760,320C2640,320,2400,320,2160,320C1920,320,1680,320,1440,320C1200,320,960,320,720,320C480,320,240,320,120,320L0,320Z">
                    <animate attributeName="transform" type="translate" from="0 0" to="-1440 0" dur="15s" repeatCount="indefinite"/>
                </path>
            </svg>

            <svg class="wave-animation absolute bottom-0 left-0 w-[200%] min-w-[1200px] h-[90px] fill-white dark:fill-gray-900 z-[3]"
                viewBox="0 0 2880 320" preserveAspectRatio="none">
                <path d="M0,192L120,176C240,160,480,128,720,144C960,160,1200,224,1440,192C1680,160,1920,96,2160,128C2400,160,2640,192,2760,192L2880,192L2880,320L2760,320C2640,320,2400,320,2160,320C1920,320,1680,320,1440,320C1200,320,960,320,720,320C480,320,240,320,120,320L0,320Z">
                    <animate attributeName="transform" type="translate" from="0 0" to="-1440 0" dur="10s" repeatCount="indefinite"/>
                </path>
            </svg>
        </div>
        </section>
    }
}
