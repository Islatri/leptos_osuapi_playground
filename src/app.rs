use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::*;

use crate::components::{
    api_demo::ApiDemo, api_section::ApiSection, features_section::FeaturesSection,
    hero_section::HeroSection, usage_section::UsageSection,
};

use crate::layouts::{footer::Footer, header::Header};

use crate::i18n::I18n;
use crate::stores::settings_store::SettingsProvider;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <SettingsProvider>
            <I18n>
                <div class="min-h-screen flex flex-col">
                    <Header />

                    <main class="flex-grow">
                        <HomePage />
                    </main>

                    <Footer />
                </div>
            </I18n>
        </SettingsProvider>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        // <div class="h-[72px]"></div>
        <HeroSection />
        <ApiSection />
        <ApiDemo />
        <FeaturesSection />
        <UsageSection />
    }
}
