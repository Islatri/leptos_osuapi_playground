use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::*;

use crate::components::{
    api_demo::ApiDemo, api_section::ApiSection, features_section::FeaturesSection,
    hero_section::HeroSection, usage_section::UsageSection,
};

use crate::layouts::{footer::Footer, header::Header};

use crate::i18n::FluentI18n;
use crate::stores::settings_store::SettingsProvider;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <SettingsProvider>
            <FluentI18n>
                <div class="flex flex-col min-h-screen">
                    <Header />

                    <main class="flex-grow">
                        <HomePage />
                    </main>

                    <Footer />
                </div>
            </FluentI18n>
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
