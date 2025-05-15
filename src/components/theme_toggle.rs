use leptos::html::*;
use leptos::prelude::*;
use leptos::*;

use lucide_leptos::{Moon, Sun};
use reactive_stores::Store;
use web_sys::window;

use crate::stores::settings_store::Settings;
use crate::stores::settings_store::SettingsStoreFields;
use crate::stores::settings_store::ThemeSettingsStoreFields;

#[component]
pub fn ThemeToggle() -> impl IntoView {
    let store = use_context::<Store<Settings>>().expect("SettingsProvider not found");
    let theme = store.theme();

    let handle_theme_toggle = move |_| {
        theme.dark_mode().update(|dark| *dark = !*dark);
        if let Some(document) = window().and_then(|w| w.document()) {
            let element = document.document_element().unwrap();
            if theme.dark_mode().get() {
                let _ = element.class_list().add_1("dark");
            } else {
                let _ = element.class_list().remove_1("dark");
            }
        }
    };

    Effect::new(move |_| {
        if theme.dark_mode().get() {
            if let Some(document) = window().map(|w| w.document()) {
                let element = document.unwrap().document_element().unwrap();
                let _ = element.class_list().add_1("dark");
            }
        }
    });

    view! {
        <button
                    class="p-2 rounded-lg transition-colors duration-200
                           hover:bg-gray-100 dark:hover:bg-gray-700
                           text-gray-600 dark:text-gray-300"
                    on:click=handle_theme_toggle
                >
                    <Show
                        when=move || theme.dark_mode().get()
                        fallback=|| view! { <Sun size={20} /> }
                    >
                        <Moon size={20} />
                    </Show>
                </button>
    }
}
