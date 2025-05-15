use leptos::*;
use leptos::prelude::*;
use reactive_stores::{Patch, Store};
use serde::{Deserialize, Serialize};
use web_sys::window;

#[derive(Debug, Store, Serialize, Deserialize)]
pub struct Settings {
    pub theme: ThemeSettings,
}

#[derive(Debug, Store, Patch, Serialize, Deserialize)]
pub struct ThemeSettings {
    dark_mode: bool,
    language: String,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            theme: ThemeSettings {
                dark_mode: false,
                language: "zh-CN".to_string(),
            },
        }
    }
}

fn load_settings() -> Settings {
    if let Some(window) = window() {
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(Some(settings_str)) = storage.get_item("themeSettings") {
                if let Ok(settings) = serde_json::from_str(&settings_str) {
                    return settings;
                }
            }
        }
    }
    Settings::default()
}

fn save_settings(settings: &Settings) {
    if let Some(window) = window() {
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(settings_str) = serde_json::to_string(settings) {
                let _ = storage.set_item("themeSettings", &settings_str);
            }
        }
    }
}

#[component]
pub fn SettingsProvider(children: Children) -> impl IntoView {
    let store = Store::new(load_settings());

    Effect::new(move |_| {
        save_settings(&store.read());
    });

    provide_context(store);

    view! { {children()} }
}