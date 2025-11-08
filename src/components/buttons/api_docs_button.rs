use leptos::prelude::{ClassAttribute, ElementChild, CollectView};
use leptos::*;
use leptos_fluent::tr;
use lucide_leptos::BookOpen;

#[derive(Clone)]
struct DocLink {
    label: &'static str,
    url: &'static str,
}

#[component]
pub fn ApiDocsButton() -> impl IntoView {
    let doc_links = vec![
        DocLink {
            label: "Crates",
            url: "https://crates.io/crates/osynic-osuapi",
        },
        DocLink {
            label: "NPM",
            url: "https://www.npmjs.com/package/@osynicite/osynic-osuapi",
        },
        DocLink {
            label: "Docs.rs",
            url: "https://docs.rs/osynic-osuapi",
        },
        DocLink {
            label: "TypeDoc",
            url: "https://hakochest.github.io/osynic-osuapi", // 替换为实际的 TypeDoc URL
        },
    ];

    let len = doc_links.len();

    view! {
        <div class="inline-flex items-center p-1.5 font-medium text-white bg-gradient-to-l from-purple-500 to-pink-500 rounded-lg shadow-lg transition-all duration-300 transform cursor-pointer hover:from-purple-600 hover:to-pink-600 hover:shadow-2xl hover:scale-105 focus:ring-2 focus:ring-purple-400 focus:ring-offset-2 focus:ring-offset-gray-900 focus:outline-none shadow-purple-500/25">
            // 左侧标签部分
            <div class="flex gap-2 items-center py-2.5 px-3 mr-2 font-medium text-white whitespace-nowrap">
                <div class="flex-shrink-0 w-6 h-6">
                    <BookOpen size=20 />
                </div>
                <span>{move || tr!("hero-button-docs")}</span>
            </div>

            // 右侧分割按钮组
            <div class="inline-flex overflow-hidden rounded-md border border-white/25">
                {doc_links.into_iter().enumerate().map(move |(index, link)| {
                    view! {
                        <>
                            // 文档链接按钮
                            <a
                                href=link.url
                                target="_blank"
                                rel="noopener noreferrer"
                                class="flex relative justify-center items-center py-2 px-4 font-bold transition-all duration-300 cursor-pointer hover:text-white hover:bg-gradient-to-br focus:ring-2 focus:ring-purple-400 focus:ring-offset-2 focus:ring-offset-gray-900 focus:outline-none text-white/85 group bg-white/12 min-w-[60px] backdrop-blur-sm hover:from-purple-400/50 hover:to-pink-500/50"
                            >
                                <span class="text-sm font-semibold">{link.label}</span>

                                // 悬停光效
                                <div class="absolute inset-0 bg-gradient-to-t rounded-md opacity-0 transition-opacity duration-300 pointer-events-none group-hover:opacity-100 from-white/0 via-white/8 to-white/0"></div>
                            </a>

                            // 分割线 (最后一个按钮后不显示)
                            {if index < len - 1 {
                                Some(view! {
                                    <div class="w-px bg-white/20"></div>
                                })
                            } else {
                                None
                            }}
                        </>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}