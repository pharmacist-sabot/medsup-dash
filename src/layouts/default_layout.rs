use chrono::{Datelike, Utc};
use leptos::prelude::*;

use crate::components::common::app_navbar::AppNavbar;

#[component]
pub fn DefaultLayout(children: Children) -> impl IntoView {
    let year = Utc::now().year();
    view! {
        <div class="flex flex-col min-h-screen bg-warm-ivory">
            <a
                href="#main-content"
                class="sr-only focus:not-sr-only focus:fixed focus:top-2 focus:left-2 focus:z-50 focus:bg-[#1f1f1f] focus:text-white focus:px-4 focus:py-2 focus:text-sm"
            >
                Skip to main content
            </a>

            <AppNavbar />

            <main id="main-content" tabindex="-1" class="grow container mx-auto px-4 py-8 max-w-6xl">
                {children()}
            </main>

            <footer class="bg-mistral-black mt-auto">
                // Mistral block gradient accent bar
                <div
                    class="h-1 w-full"
                    style="background: linear-gradient(90deg, #ffd900, #ffe295, #ffa110, #ff8105, #fb6424, #fa520f)"
                ></div>
                <div class="container mx-auto px-4 py-6 text-center text-white/50 text-sm">
                    "© " {year} " Medical Support Dashboard. Sabot Hospital."
                </div>
            </footer>
        </div>
    }
}
