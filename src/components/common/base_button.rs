use leptos::prelude::*;

/// Visual variants of [`BaseButton`], mirroring the original Vue component.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Danger,
    Ghost,
}

impl ButtonVariant {
    #[must_use]
    pub const fn classes(self) -> &'static str {
        match self {
            Self::Primary => {
                "bg-mistral-black text-white hover:bg-mistral-orange shadow-golden-sm hover:shadow-golden"
            }
            Self::Secondary => {
                "bg-cream text-mistral-black border border-block-gold hover:bg-sunshine-300 \
                 hover:border-sunshine-700"
            }
            Self::Danger => "bg-mistral-orange text-white hover:bg-mistral-flame shadow-golden-sm",
            Self::Ghost => {
                "bg-transparent text-mistral-black/60 hover:bg-[oklab(0_0_0/0.06)] \
                 hover:text-mistral-black"
            }
        }
    }
}

const BASE_CLASSES: &str = "inline-flex items-center justify-center px-3 py-3 rounded-none font-normal tracking-wide transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-mistral-orange disabled:opacity-50 disabled:cursor-not-allowed uppercase text-sm";

#[component]
pub fn BaseButton(
    #[prop(default = ButtonVariant::Primary)] variant: ButtonVariant,
    #[prop(default = "button")] button_type: &'static str,
    #[prop(into)] disabled: Signal<bool>,
    /// Extra utility classes appended to the base + variant classes
    /// (e.g. `"w-full justify-center"`).
    #[prop(default = "")]
    extra_class: &'static str,
    children: Children,
) -> impl IntoView {
    let classes = if extra_class.is_empty() {
        format!("{BASE_CLASSES} {}", variant.classes())
    } else {
        format!("{BASE_CLASSES} {} {extra_class}", variant.classes())
    };

    view! {
        <button type=button_type class=classes disabled=move || disabled.get()>
            {children()}
        </button>
    }
}
