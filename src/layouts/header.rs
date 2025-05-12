use leptos::*;
use leptos::prelude::{signal,Update,Get,OnAttribute,ClassAttribute,ElementChild};
use lucide_leptos::{Github,Menu,House,CodeXml,SquareMousePointer};

#[component]
pub fn Header() -> impl IntoView {
    let (is_menu_open, set_is_menu_open) = signal(false);
    
    let toggle_menu = move |_| {
        set_is_menu_open.update(|value| *value = !*value);
    };
    
    view! {
        <header class="bg-white dark:bg-gray-800 shadow-sm z-20">
            <div class="container mx-auto px-4">
                <div class="flex justify-between items-center py-4">
                    // Logo 部分
                    <div class="flex items-center">
                        <img src="/public/osynic.png" alt="osynic Logo" class="h-10 w-10 mr-3" />
                        <span class="font-bold text-xl text-pink-600">OsynicOsuapi</span>
                    </div>
                    
                    // 导航链接 - 桌面版
                    <nav class="hidden md:flex items-center space-x-8">
                        <a href="#" class="font-medium hover:text-pink-600 transition flex items-center"><House/>首页</a>
                        <a href="#api" class="font-medium hover:text-pink-600 transition flex items-center"><CodeXml/>API</a>
                        <a href="#demo" class="font-medium hover:text-pink-600 transition flex items-center"><SquareMousePointer/>演示</a>
                        <a href="https://github.com/osynicite/osynic_osuapi" 
                          class="font-medium hover:text-pink-600 transition flex items-center">
                            <Github /> GitHub
                        </a>
                    </nav>
                    
                    // 移动版菜单按钮
                    <button 
                        class="md:hidden flex items-center"
                        on:click=toggle_menu
                    >
                        <Menu/>
                    </button>
                </div>
                
                // 移动版导航菜单
                <div 
                    // class=("scale-y-0", !is_menu_open.get())
                    // class=("scale-y-100", is_menu_open.get())
                    class=move || {
                        let base_classes = "md:hidden bg-white dark:bg-gray-800 absolute z-10 /
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
                            <a href="#" class="font-medium hover:text-pink-600 transition py-2 flex items-center"><House/>首页</a>
                            <a href="#api" class="font-medium hover:text-pink-600 transition py-2 flex items-center"><CodeXml/>API</a>
                            <a href="#demo" class="font-medium hover:text-pink-600 transition py-2 flex items-center"><SquareMousePointer/>演示</a>
                            <a href="https://github.com/osynicite/osynic_osuapi" 
                            target="_blank"
                              class="font-medium hover:text-pink-600 transition py-2 flex items-center">
                                <Github /> GitHub
                            </a>
                        </div>
                    </div>
                </div>
            </div>
        </header>
    }
}