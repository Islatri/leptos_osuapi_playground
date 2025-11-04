use fluent_templates::static_loader;
use leptos::children::Children;
use leptos::prelude::{
    ClassAttribute, CustomAttribute, ElementChild, Get, OnAttribute, RwSignal, Set, Update,
    expect_context,
};
use leptos::{IntoView, component, view};
use leptos_fluent::{I18n, leptos_fluent};

static_loader! {
    static TRANSLATIONS = {
        locales: "./locales",
        fallback_language: "zh-CN",
    };
}

#[component]
pub fn FluentI18n(children: Children) -> impl IntoView {
    leptos_fluent! {
        children: children(),
        locales: "./locales",
        translations: [TRANSLATIONS],

        set_language_to_local_storage: true,
        initial_language_from_local_storage: true,
        initial_language_from_navigator: true,
        initial_language_from_navigator_to_local_storage: true,
        initial_language_from_url_param: true,
        initial_language_from_url_param_to_local_storage: true,
        local_storage_key: "lang",
    }
}

#[component]
pub fn LanguageSelector() -> impl IntoView {
    let i18n = expect_context::<I18n>();
    let is_open = RwSignal::new(false);

    let close_dropdown = move |_| {
        is_open.set(false);
    };

    // 获取当前激活的语言
    let current_language = move || {
        expect_context::<I18n>()
            .languages
            .iter()
            .find(|lang| &i18n.language.get() == *lang)
            .map(|lang| lang.name)
            .unwrap_or_else(|| "选择语言")
    };

    view! {
        <div class="inline-block relative w-32 text-left">
            // 点击展开/收起的按钮
            <div>
                <button
                    type="button"
                    class="inline-flex justify-between items-center py-2 px-4 w-full text-sm font-medium text-gray-700 bg-white rounded-md border border-gray-300 shadow-sm dark:text-gray-200 dark:bg-gray-800 dark:border-gray-600 hover:bg-gray-50 focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:outline-none dark:hover:bg-gray-700"
                    on:click=move |_| is_open.update(|open| *open = !*open)
                >
                    <span>{current_language}</span>
                    <svg
                        class="ml-2 -mr-1 w-5 h-5"
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 20 20"
                        fill="currentColor"
                        aria-hidden="true"
                    >
                        <path
                            fill-rule="evenodd"
                            d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z"
                            clip-rule="evenodd"
                        />
                    </svg>
                </button>
            </div>

            // 下拉菜单部分
            <div
                class="absolute right-0 z-30 mt-2 w-full bg-white rounded-md divide-y divide-gray-100 ring-1 ring-black ring-opacity-5 shadow-lg transition-all duration-200 ease-in-out origin-top-right dark:bg-gray-800 dark:divide-gray-700"
                class:hidden=move || !is_open.get()
                class:block=move || is_open.get()
            >
                <div class="overflow-auto py-1 max-h-60">
                    {move || {
                        expect_context::<I18n>()
                            .languages
                            .iter()
                            .map(|lang| {
                                let is_active = &i18n.language.get() == lang;
                                view! {
                                    <button
                                        type="button"
                                        class="flex items-center py-2 px-4 w-full text-sm text-left hover:bg-gray-100 group dark:hover:bg-gray-700"
                                        class:dark:text-gray-200=move || !is_active
                                        class:text-gray-700=move || !is_active
                                        class:text-blue-600=move || is_active
                                        class:font-medium=move || is_active
                                        on:click=move |ev| {
                                            i18n.language.set(lang);
                                            close_dropdown(ev);
                                        }
                                    >
                                        <span class="flex-grow">{lang.name}</span>
                                        <svg
                                            class="w-5 h-5 text-blue-600"
                                            class:hidden=move || !is_active
                                            xmlns="http://www.w3.org/2000/svg"
                                            viewBox="0 0 20 20"
                                            fill="currentColor"
                                        >
                                            <path
                                                fill-rule="evenodd"
                                                d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
                                                clip-rule="evenodd"
                                            />
                                        </svg>
                                    </button>
                                }
                            })
                            .collect::<Vec<_>>()
                    }}
                </div>
            </div>

            // 点击外部关闭下拉菜单的处理器
            <div
                class="fixed inset-0 z-0 bg-transparent cursor-default"
                class:hidden=move || !is_open.get()
                on:click=close_dropdown
            ></div>
        </div>
    }
}
