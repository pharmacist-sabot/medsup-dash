use leptos::prelude::*;
use leptos_router::components::A;

use crate::components::common::base_button::ButtonVariant;

#[component]
pub fn NotFoundView() -> impl IntoView {
    // Same visual treatment as <BaseButton variant=Primary>, rendered as a
    // router link.
    let link_class = format!(
        "{} {}",
        "inline-flex items-center justify-center px-3 py-3 rounded-none font-normal tracking-wide \
         transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-offset-2 \
         focus:ring-mistral-orange uppercase text-sm",
        ButtonVariant::Primary.classes()
    );

    view! {
        <div class="text-center px-4 py-16">
            // Large 404 display
            <div class="text-[120px] leading-none text-block-gold select-none" style="letter-spacing: -4px;">
                "404"
            </div>

            // Mistral block gradient accent bar
            <div class="h-1 w-24 mx-auto my-6" style="background: linear-gradient(90deg, #ffd900, #ffa110, #fa520f);"></div>

            <div class="space-y-4 max-w-md mx-auto">
                <h2 class="text-2xl text-mistral-black">Page Not Found</h2>
                <p class="text-sm text-mistral-black/50 leading-relaxed">
                    "Oops! The page you are looking for might have been removed,
                    had its name changed, or is temporarily unavailable."
                </p>

                <div class="pt-4">
                    <A href="/" attr:class=link_class>"Back to Home"</A>
                </div>
            </div>
        </div>
    }
}
