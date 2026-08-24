use leptos::prelude::*;

/// Inline Lucide icon set (1:1 with the `lucide-vue-next` icons used by the
/// original Vue components). Rendered as raw SVG inner markup inside a shared
/// `<svg>` frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconKind {
    Activity,
    LogOut,
    Calculator,
    CalendarRange,
    Coins,
    Loader2,
    Receipt,
    AlertCircle,
}

const ACTIVITY: &str = r#"<path d="M22 12h-4l-3 9L9 3l-3 9H2"/>"#;
const LOG_OUT: &str = r#"<path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"/><polyline points="16 17 21 12 16 7"/><line x1="21" x2="9" y1="12" y2="12"/>"#;
const CALCULATOR: &str = r#"<rect width="16" height="20" x="4" y="2" rx="2"/><line x1="8" x2="16" y1="6" y2="6"/><line x1="16" x2="16" y1="14" y2="18"/><path d="M16 10h.01"/><path d="M12 10h.01"/><path d="M8 10h.01"/><path d="M12 14h.01"/><path d="M8 14h.01"/><path d="M12 18h.01"/><path d="M8 18h.01"/>"#;
const CALENDAR_RANGE: &str = r#"<rect width="18" height="18" x="3" y="4" rx="2"/><path d="M16 2v4"/><path d="M8 2v4"/><path d="M3 10h18"/><path d="M8 14h.01"/><path d="M12 14h.01"/><path d="M16 14h.01"/><path d="M8 18h.01"/><path d="M12 18h.01"/><path d="M16 18h.01"/>"#;
const COINS: &str = r#"<circle cx="8" cy="8" r="6"/><path d="M18.09 10.37A6 6 0 1 1 10.34 18"/><path d="M7 6h1v4"/><path d="m16.71 13.88.7.71-2.82 2.82"/>"#;
const LOADER_2: &str = r#"<path d="M21 12a9 9 0 1 1-6.219-8.56"/>"#;
const RECEIPT: &str = r#"<path d="M4 2v20l2-1 2 1 2-1 2 1 2-1 2 1 2-1 2 1V2l-2 1-2-1-2 1-2-1-2 1-2-1-2 1Z"/><path d="M16 8h-6a2 2 0 1 0 0 4h4a2 2 0 1 1 0 4H8"/><path d="M12 17.5v-11"/>"#;
const ALERT_CIRCLE: &str = r#"<circle cx="12" cy="12" r="10"/><line x1="12" x2="12" y1="8" y2="12"/><line x1="12" x2="12.01" y1="16" y2="16"/>"#;

#[must_use]
pub fn icon_markup(kind: IconKind) -> &'static str {
    match kind {
        IconKind::Activity => ACTIVITY,
        IconKind::LogOut => LOG_OUT,
        IconKind::Calculator => CALCULATOR,
        IconKind::CalendarRange => CALENDAR_RANGE,
        IconKind::Coins => COINS,
        IconKind::Loader2 => LOADER_2,
        IconKind::Receipt => RECEIPT,
        IconKind::AlertCircle => ALERT_CIRCLE,
    }
}

/// Shared SVG shell for every icon.
#[component]
pub fn Icon(
    icon: IconKind,
    #[prop(default = "w-4 h-4")] class: &'static str,
    #[prop(default = false)] aria_hidden: bool,
) -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            class=class
            aria-hidden=if aria_hidden { "true" } else { "false" }
            inner_html=icon_markup(icon)
        />
    }
}
