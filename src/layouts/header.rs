use leptos::*;
use leptos::prelude::{signal,Update,Get,OnAttribute,ClassAttribute,ElementChild};

#[component]
pub fn Header() -> impl IntoView {
    let (is_menu_open, set_is_menu_open) = signal(false);
    
    let toggle_menu = move |_| {
        set_is_menu_open.update(|value| *value = !*value);
    };
    
    view! {
        <header class="bg-white dark:bg-gray-800 shadow-sm">
            <div class="container mx-auto px-4">
                <div class="flex justify-between items-center py-4">
                    // Logo 部分
                    <div class="flex items-center">
                        <img src="/public/osynic.png" alt="osynic Logo" class="h-10 w-10 mr-3" />
                        <span class="font-bold text-xl text-pink-600">osynic_osuapi</span>
                    </div>
                    
                    // 导航链接 - 桌面版
                    <nav class="hidden md:flex items-center space-x-8">
                        <a href="#" class="font-medium hover:text-pink-600 transition">首页</a>
                        <a href="#api" class="font-medium hover:text-pink-600 transition">API</a>
                        <a href="#demo" class="font-medium hover:text-pink-600 transition">演示</a>
                        <a href="https://github.com/osynicite/osynic_osuapi" 
                          class="font-medium hover:text-pink-600 transition flex items-center">
                            <i class="lucide-github mr-1"></i> GitHub
                        </a>
                    </nav>
                    
                    // 移动版菜单按钮
                    <button 
                        class="md:hidden flex items-center"
                        on:click=toggle_menu
                    >
                        <i class="lucide-menu text-2xl"></i>
                    </button>
                </div>
                
                // 移动版导航菜单
                <div 
                    class="md:hidden bg-white dark:bg-gray-800 absolute z-10 left-0 right-0 shadow-md transition-transform duration-300 transform origin-top"
                    class=("scale-y-0", !is_menu_open.get())
                    class=("scale-y-100", is_menu_open.get())
                >
                    <div class="container mx-auto px-4 py-3">
                        <div class="flex flex-col space-y-3">
                            <a href="#" class="font-medium hover:text-pink-600 transition py-2">首页</a>
                            <a href="#api" class="font-medium hover:text-pink-600 transition py-2">API</a>
                            <a href="#demo" class="font-medium hover:text-pink-600 transition py-2">演示</a>
                            <a href="https://github.com/osynicite/osynic_osuapi" 
                              class="font-medium hover:text-pink-600 transition py-2 flex items-center">
                                <i class="lucide-github mr-1"></i> GitHub
                            </a>
                        </div>
                    </div>
                </div>
            </div>
        </header>
    }
}