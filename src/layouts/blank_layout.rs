use leptos::prelude::*;

#[component]
pub fn BlankLayout(children: Children) -> impl IntoView {
    view! {
        <div class="min-h-screen flex flex-col justify-center items-center bg-warm-ivory">
            {children()}
        </div>
    }
}
