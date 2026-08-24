use leptos::prelude::*;

use crate::components::icons::{Icon, IconKind};

#[component]
pub fn KpiCard(
    #[prop(into)] title: String,
    /// Reactive display value (pass a `Memo`/`Signal` so live updates flow).
    #[prop(into)]
    value: Signal<String>,
    #[prop(optional, into)] sub_value: Option<String>,
    #[prop(optional)] icon: Option<IconKind>,
    #[prop(default = "bg-sunshine-700/20 text-mistral-orange")] color_class: &'static str,
) -> impl IntoView {
    view! {
        <div class="p-6 bg-cream border border-block-gold shadow-golden hover:shadow-golden transition-all duration-200">
            <div class="flex items-center justify-between mb-4">
                <div class=format!("w-12 h-12 flex items-center justify-center {color_class}")>
                    {icon.map(|kind| {
                        view! { <Icon icon=kind class="w-6 h-6" aria_hidden=true /> }
                    })}
                </div>
            </div>
            <div>
                <h3 class="text-sm text-mistral-black/60 mb-1">{title}</h3>
                <div class="text-2xl text-mistral-black">{move || value.get()}</div>
                {sub_value.map(|sub| {
                    view! { <p class="text-xs text-mistral-black/40 mt-1">{sub}</p> }
                })}
            </div>
        </div>
    }
}
