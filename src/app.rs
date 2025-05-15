use leptos::*;
use leptos::prelude::{ClassAttribute,ElementChild};

use crate::components::{
    hero_section::HeroSection,
    api_section::ApiSection,
    api_demo::ApiDemo,
    features_section::FeaturesSection,
    usage_section::UsageSection,
};

use crate::layouts::{
    header::Header,
    footer::Footer,
};

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