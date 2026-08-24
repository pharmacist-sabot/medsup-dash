use leptos::prelude::*;
use leptos_router::NavigateOptions;
use leptos_router::hooks::use_navigate;

use crate::components::icons::{Icon, IconKind};
use crate::stores::auth::use_auth;

#[component]
pub fn AppNavbar() -> impl IntoView {
    let auth = use_auth();
    let email = auth.email;
    let navigate = use_navigate();

    // `Callback` is Copy, so this can be moved into nested children closures.
    let handle_logout = Callback::new(move |_| {
        let navigate = navigate.clone();
        leptos::task::spawn_local(async move {
            auth.logout().await;
            navigate("/login", NavigateOptions::default());
        });
    });

    view! {
        <header class="bg-mistral-black sticky top-0 z-30">
            // Mistral block gradient accent bar
            <div
                class="h-0.5 w-full"
                style="background: linear-gradient(90deg, #ffd900, #ffe295, #ffa110, #ff8105, #fb6424, #fa520f);"
            ></div>

            <div class="container mx-auto px-4 h-14 flex items-center justify-between max-w-6xl">
                // Logo
                <div class="flex items-center gap-3">
                    <div
                        class="w-8 h-8 flex items-center justify-center"
                        style="background: linear-gradient(135deg, #ffa110, #fa520f);"
                    >
                        <Icon icon=IconKind::Activity class="w-4 h-4 text-white" aria_hidden=true />
                    </div>
                    <span class="text-white text-base tracking-wide">MedValue Support</span>
                </div>

                // User Menu
                <Show when=move || email.get().is_some()>
                    <div class="flex items-center gap-4">
                        <span class="text-white/50 text-sm hidden md:block">
                            {move || email.get().unwrap_or_default()}
                        </span>
                        <button
                            class="p-2 text-white/50 hover:text-mistral-orange transition-colors duration-200"
                            aria-label="Logout"
                            title="Logout"
                            on:click=move |ev| handle_logout.run(ev)
                        >
                            <Icon icon=IconKind::LogOut class="w-4 h-4" aria_hidden=true />
                        </button>
                    </div>
                </Show>
            </div>
        </header>
    }
}
