use leptos::prelude::*;
use leptos_router::NavigateOptions;
use leptos_router::hooks::use_navigate;

use crate::components::common::base_button::{BaseButton, ButtonVariant};
use crate::components::icons::{Icon, IconKind};
use crate::core::utils::input_value;
use crate::stores::auth::use_auth;

#[component]
pub fn LoginView() -> impl IntoView {
    let auth = use_auth();
    let navigate = use_navigate();

    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let error_msg = RwSignal::new(String::new());

    let handle_login = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        error_msg.set(String::new());

        let email_value = email.get_untracked();
        let password_value = password.get_untracked();
        let navigate = navigate.clone();

        leptos::task::spawn_local(async move {
            match auth.login(&email_value, &password_value).await {
                Ok(()) => navigate("/", NavigateOptions::default()),
                Err(err) => error_msg.set(err.to_string()),
            }
        });
    };

    view! {
        <main class="w-full max-w-md mx-auto px-4">
            // Mistral block gradient accent bar
            <div
                class="h-1 w-full mb-0"
                style="background: linear-gradient(90deg, #ffd900, #ffe295, #ffa110, #ff8105, #fb6424, #fa520f);"
            ></div>

            // Card
            <div
                class="bg-cream border border-block-gold p-8"
                style="box-shadow: rgba(127,99,21,0.12) -8px 16px 39px, rgba(127,99,21,0.10) -33px 64px 72px, rgba(127,99,21,0.06) -73px 144px 97px;"
            >
                // Header
                <div class="mb-8">
                    // Logo mark
                    <div class="flex items-center gap-3 mb-6">
                        <div
                            class="w-9 h-9 flex items-center justify-center"
                            style="background: linear-gradient(135deg, #ffa110, #fa520f);"
                        >
                            <svg
                                class="w-5 h-5 text-white" fill="none" viewBox="0 0 24 24"
                                stroke="currentColor" stroke-width="1.5" aria-hidden="true"
                                focusable="false"
                            >
                                <path
                                    stroke-linecap="square" stroke-linejoin="miter"
                                    d="M9 12h6m-3-3v6M4.5 19.5l15-15M4.5 4.5l15 15"
                                />
                            </svg>
                        </div>
                        <span class="text-mistral-black text-base tracking-wide">MedValue Support</span>
                    </div>

                    <h1 class="text-2xl text-mistral-black leading-tight">Welcome Back</h1>
                    <p class="text-mistral-black/50 text-sm mt-1">Sign in to access the dashboard</p>
                </div>

                <form class="space-y-5" on:submit=handle_login>
                    // Error message
                    <Show when=move || !error_msg.get().is_empty()>
                        <div
                            role="alert"
                            aria-live="assertive"
                            class="p-3 bg-warm-ivory border border-mistral-orange/30 text-mistral-orange text-sm flex items-center gap-2"
                        >
                            <Icon icon=IconKind::AlertCircle class="w-4 h-4 shrink-0" aria_hidden=true />
                            {move || error_msg.get()}
                        </div>
                    </Show>

                    // Email
                    <div>
                        <label for="login-email" class="block text-xs text-mistral-black/60 mb-1.5 uppercase tracking-wider">
                            Email
                        </label>
                        <input
                            id="login-email"
                            type="email"
                            required=true
                            autocomplete="email"
                            class="w-full px-4 py-3 bg-warm-ivory border border-block-gold rounded-none text-mistral-black text-sm placeholder:text-mistral-black/30 outline-none transition-all duration-200 focus:border-mistral-orange focus:ring-1 focus:ring-mistral-orange"
                            placeholder="pharmacist@sabot.hospital"
                            value=email
                            on:input=move |ev| email.set(input_value(&ev))
                        />
                    </div>

                    // Password
                    <div>
                        <label for="login-password" class="block text-xs text-mistral-black/60 mb-1.5 uppercase tracking-wider">
                            Password
                        </label>
                        <input
                            id="login-password"
                            type="password"
                            required=true
                            autocomplete="current-password"
                            class="w-full px-4 py-3 bg-warm-ivory border border-block-gold rounded-none text-mistral-black text-sm placeholder:text-mistral-black/30 outline-none transition-all duration-200 focus:border-mistral-orange focus:ring-1 focus:ring-mistral-orange"
                            placeholder="••••••••"
                            value=password
                            on:input=move |ev| password.set(input_value(&ev))
                        />
                    </div>

                    // Submit
                    <div class="pt-2">
                        <BaseButton
                            button_type="submit"
                            variant=ButtonVariant::Primary
                            disabled=Signal::derive(move || auth.loading.get())
                            extra_class="w-full justify-center"
                        >
                            {move || if auth.loading.get() { "Signing in..." } else { "Sign In" }}
                        </BaseButton>
                    </div>
                </form>
            </div>

            // Bottom caption
            <p class="text-center text-xs text-mistral-black/50 mt-4">
                "Medical Support Dashboard — Sabot Hospital"
            </p>
        </main>
    }
}
