use leptos::*;
use leptos::prelude::{signal,Update,Get,OnAttribute,ClassAttribute,ElementChild};
use lucide_leptos::{Github,Menu,CodeXml,SquareMousePointer};
use leptos_fluent::tr;
use crate::i18n::LanguageSelector;
use crate::components::theme_toggle::ThemeToggle;

#[component]
pub fn Header() -> impl IntoView {
    let (is_menu_open, set_is_menu_open) = signal(false);
    
    let toggle_menu = move |_| {
        set_is_menu_open.update(|value| *value = !*value);
    };
    
    view! {
        <header class="bg-white dark:bg-gray-900 shadow-sm z-40">
            <div class="container mx-auto px-4">
                <div class="flex justify-between items-center py-4">
                    // Logo 部分
                    <div class="flex items-center">
                        <img src="/public/osynic.png" alt="osynic Logo" class="h-10 w-10 mr-3" />
                        <span class="font-bold text-xl lg:text-3xl text-pink-600">OsynicOsuapi</span>
                    </div>
                    <div class="flex justify-end items-center space-x-2">
                        // 导航链接 - 桌面版
                        <nav class="hidden md:flex items-center space-x-8 mr-2">
                            <a href="#api" class="font-medium hover:text-pink-600 dark:hover:text-pink-400 dark:text-gray-200 transition flex items-center"><CodeXml size={20}/>API</a>
                            <a href="https://github.com/osynicite/osynic_osuapi" 
                                target="_blank"
                            class="font-medium hover:text-pink-600 dark:hover:text-pink-400 dark:text-gray-200 transition flex items-center">
                                <Github size={20}/> GitHub
                            </a>
                        </nav>
                        
                        // 移动版菜单按钮
                        
                        <LanguageSelector />
                        <ThemeToggle />

                        <button 
                            class="md:hidden flex items-center dark:text-gray-200 text-gray-600 p-2 rounded-lg transition-colors duration-200
                                hover:bg-gray-100 dark:hover:bg-gray-700"
                            on:click=toggle_menu
                        >
                            <Menu size={20}/>
                        </button>
                    </div>
                </div>
                
                // 移动版导航菜单
                <div 
                    class=move || {
                        let base_classes = "md:hidden bg-white dark:bg-gray-800 absolute z-40 /
                                            left-0 right-0 shadow-md transition-transform duration-300 /
                                            transform origin-top";

                        let scale_class = if is_menu_open.get() {
                            "scale-y-100"
                        } else {
                            "scale-y-0"
                        };

                        format!("{} {}", base_classes, scale_class)
                    }
                >
                    <div class="container mx-auto px-4 py-3">
                        <div class="flex flex-col space-y-3">
                            <a href="#api" class="font-medium hover:text-pink-600 dark:hover:text-pink-400 dark:text-gray-200 transition py-2 flex items-center"><CodeXml size={20}/>API</a>
                            <a href="#demo" class="font-medium hover:text-pink-600 dark:hover:text-pink-400 dark:text-gray-200 transition py-2 flex items-center"><SquareMousePointer size={20}/>{move || tr!("demo")}</a>
                            <a href="https://github.com/osynicite/osynic_osuapi" 
                            target="_blank"
                              class="font-medium hover:text-pink-600 dark:hover:text-pink-400 dark:text-gray-200 transition py-2 flex items-center">
                                <Github size={20}/> GitHub
                            </a>
                        </div>
                    </div>
                </div>
            </div>
        </header>
    }
}