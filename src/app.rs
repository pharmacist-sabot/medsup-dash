use leptos::prelude::*;
use leptos_meta::{Title, provide_meta_context};
use leptos_router::components::{Redirect, Route, Router, Routes};
use leptos_router::{WildcardSegment, path};

use crate::layouts::blank_layout::BlankLayout;
use crate::layouts::default_layout::DefaultLayout;
use crate::stores::auth::use_auth;
use crate::views::login_view::LoginView;
use crate::views::not_found_view::NotFoundView;
use crate::views::overview_view::OverviewView;

/// Root component: document metadata + router shell.
///
/// The Vue version switched layouts by route meta (`default`/`blank`) and used
/// a navigation guard for auth. Here the equivalent split is expressed with
/// nested `<Show>`s: uninitialized -> splash; unauthenticated -> blank layout
/// routes; authenticated -> default layout routes.
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="MedSup Dash" />
        <Router>
            <Shell />
        </Router>
    }
}

#[component]
fn Shell() -> impl IntoView {
    let auth = use_auth();
    let initialized = auth.initialized;
    let email = auth.email;

    view! {
        <Show when=move || initialized.get() fallback=Splash>
            <Show
                when=move || email.get().is_some()
                fallback=move || view! { <UnauthenticatedShell /> }
            >
                <AuthenticatedShell />
            </Show>
        </Show>
    }
}

#[component]
fn Splash() -> impl IntoView {
    view! {
        <div class="min-h-screen flex items-center justify-center bg-warm-ivory">
            <p class="text-sm tracking-wide text-mistral-black/50">"Loading…"</p>
        </div>
    }
}

/// Replaces the router guard's `requiresAuth -> next('/login')`.
#[component]
fn RedirectToLogin() -> impl IntoView {
    view! { <Redirect path="/login" /> }
}

/// Replaces the router guard's `to.name === 'login' && isAuthenticated -> next('/')`.
#[component]
fn RedirectToDashboard() -> impl IntoView {
    view! { <Redirect path="/" /> }
}

#[component]
fn UnauthenticatedShell() -> impl IntoView {
    view! {
        <BlankLayout>
            <Routes fallback=NotFoundView>
                <Route path=path!("/login") view=LoginView />
                <Route path=path!("/") view=RedirectToLogin />
                <Route path=WildcardSegment("") view=NotFoundView />
            </Routes>
        </BlankLayout>
    }
}

#[component]
fn AuthenticatedShell() -> impl IntoView {
    view! {
        <DefaultLayout>
            <Routes fallback=NotFoundView>
                <Route path=path!("/") view=OverviewView />
                <Route path=path!("/login") view=RedirectToDashboard />
                <Route path=WildcardSegment("") view=NotFoundView />
            </Routes>
        </DefaultLayout>
    }
}
