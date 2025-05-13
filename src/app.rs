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

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="min-h-screen flex flex-col">
            <Header />
            
            <main class="flex-grow">
                <HomePage />
            </main>
            
            <Footer />
        </div>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <HeroSection />
        <ApiSection />
        <ApiDemo />
        <FeaturesSection />
        <UsageSection />
    }
}