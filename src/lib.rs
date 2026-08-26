//! medsup-dash - Medical Support Dashboard (Leptos CSR + Supabase).
//!
//! Entry point mounts the app and installs store singletons **inside** the
//! mount-root owner so their signals are never disposed.

pub mod app;
pub mod components;
pub mod core;
pub mod layouts;
pub mod stores;
pub mod views;

use console_error_panic_hook::set_once as set_panic_hook;
use leptos::mount::mount_to_body;
use leptos::prelude::view;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(start)]
pub fn start() {
    set_panic_hook();
    let _ = console_log::init_with_level(log::Level::Debug);

    // CRITICAL: install store singletons INSIDE the mount closure so their
    // RwSignals are created in the mount-root Owner and are never disposed.
    mount_to_body(|| {
        stores::auth::install();
        stores::transactions::install();

        // Restore a persisted session before the shell decides which route
        // tree to render (equivalent of the old router.beforeEach guard init).
        leptos::task::spawn_local(async {
            stores::auth::use_auth().init().await;
        });

        view! { <app::App /> }
    });
}
