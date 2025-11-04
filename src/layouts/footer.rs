use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild};
use leptos::*;
use leptos_fluent::{move_tr, tr};
use lucide_leptos::Copyright;

#[component]
pub fn Discord() -> impl IntoView {
    view! {
        <svg
            aria-hidden="true"
            role="img"
            xmlns="http://www.w3.org/2000/svg"
            width="30"
            height="30"
            fill="none"
            viewBox="0 0 24 24"
        >
            <path
                fill="#5865f2"
                d="M19.73 4.87a18.2 18.2 0 0 0-4.6-1.44c-.21.4-.4.8-.58 1.21-1.69-.25-3.4-.25-5.1 0-.18-.41-.37-.82-.59-1.2-1.6.27-3.14.75-4.6 1.43A19.04 19.04 0 0 0 .96 17.7a18.43 18.43 0 0 0 5.63 2.87c.46-.62.86-1.28 1.2-1.98-.65-.25-1.29-.55-1.9-.92.17-.12.32-.24.47-.37 3.58 1.7 7.7 1.7 11.28 0l.46.37c-.6.36-1.25.67-1.9.92.35.7.75 1.35 1.2 1.98 2.03-.63 3.94-1.6 5.64-2.87.47-4.87-.78-9.09-3.3-12.83ZM8.3 15.12c-1.1 0-2-1.02-2-2.27 0-1.24.88-2.26 2-2.26s2.02 1.02 2 2.26c0 1.25-.89 2.27-2 2.27Zm7.4 0c-1.1 0-2-1.02-2-2.27 0-1.24.88-2.26 2-2.26s2.02 1.02 2 2.26c0 1.25-.88 2.27-2 2.27Z"
                class=""
            ></path>
        </svg>
    }
}

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="z-20 py-12 text-white bg-gray-800">
            <div class="container px-4 mx-auto">
                <div class="grid grid-cols-1 gap-8 md:grid-cols-4">
                    {} <div class="col-span-1 md:col-span-2">
                        <div class="flex items-center mb-4">
                            <img src="/public/osynic.png" alt="osynic Logo" class="mr-2 w-8 h-8" />
                            <span class="text-xl font-bold text-pink-400">
                                <a
                                    href="https://github.com/osynicite/osynic_osuapi"
                                    target="_blank"
                                >
                                    OsynicOsuapi
                                </a>
                            </span>
                        </div>
                        <p class="mb-4 text-gray-400">{move || tr!("footer-description")}</p>
                        <div class="flex space-x-4">
                            <a
                                href="https://discord.gg/DRnZSES3BC"
                                target="_blank"
                                class="text-xl text-gray-400 transition hover:text-white"
                            >
                                <Discord />
                            </a>
                        </div>
                    </div>
                    {} <div>
                        <h3 class="mb-4 text-lg font-semibold">
                            {move || tr!("footer-docs-heading")}
                        </h3>
                        <ul class="space-y-2">
                            <li>
                                <a
                                    href="https://crates.io/crates/osynic_osuapi"
                                    target="_blank"
                                    class="text-gray-400 transition hover:text-white"
                                >
                                    {move || tr!("footer-docs-getting-started")}
                                </a>
                            </li>
                            <li>
                                <a href="#api" class="text-gray-400 transition hover:text-white">
                                    {move || tr!("footer-docs-api-reference")}
                                </a>
                            </li>
                            <li>
                                <a
                                    href="https://github.com/Osynicite/osynic_osuapi/tree/master/examples"
                                    target="_blank"
                                    class="text-gray-400 transition hover:text-white"
                                >
                                    {move || tr!("footer-docs-examples")}
                                </a>
                            </li>
                        </ul>
                    </div>
                    {} <div>
                        <h3 class="mb-4 text-lg font-semibold">
                            {move || tr!("footer-resources-heading")}
                        </h3>
                        <ul class="space-y-2">
                            <li>
                                <a
                                    href="https://github.com/ppy/osu-api/wiki"
                                    target="_blank"
                                    class="text-gray-400 transition hover:text-white"
                                >
                                    {move || tr!("footer-resources-api-v1")}
                                </a>
                            </li>
                            <li>
                                <a
                                    href="https://osu.ppy.sh/docs/index.html"
                                    target="_blank"
                                    class="text-gray-400 transition hover:text-white"
                                >
                                    {move || tr!("footer-resources-api-v2")}
                                </a>
                            </li>
                            <li>
                                <a
                                    href="https://leptos.dev"
                                    target="_blank"
                                    class="text-gray-400 transition hover:text-white"
                                >
                                    {move || tr!("footer-resources-leptos")}
                                </a>
                            </li>
                            <li>
                                <a
                                    href="https://github.com/osynicite/osynic_osuapi/issues"
                                    target="_blank"
                                    class="text-gray-400 transition hover:text-white"
                                >
                                    {move || tr!("footer-resources-issues")}
                                </a>
                            </li>
                            <li>
                                <a
                                    href="https://github.com/Osynicite/osynic_osuapi/commits/master"
                                    target="_blank"
                                    class="text-gray-400 transition hover:text-white"
                                >
                                    {move || tr!("footer-resources-changelog")}
                                </a>
                            </li>
                        </ul>
                    </div>
                </div>

                <div class="flex flex-col justify-between items-center pt-8 mt-8 border-t border-gray-700 md:flex-row">
                    <p class="flex text-sm text-gray-400">
                        <Copyright />
                        {move_tr!("footer-copyright",{"year" => 2025})}
                    </p>
                    <div class="mt-4 md:mt-0">
                        <a
                            href="https://github.com/osynicite"
                            target="_blank"
                            class="mr-4 text-sm text-gray-400 transition hover:text-white"
                        >
                            {move || tr!("footer-terms")}
                        </a>
                        <a
                            href="https://github.com/osynicite"
                            target="_blank"
                            class="text-sm text-gray-400 transition hover:text-white"
                        >
                            {move || tr!("footer-privacy")}
                        </a>
                    </div>
                </div>
            </div>
        </footer>
    }
}
