use std::sync::OnceLock;

use leptos::prelude::*;

use crate::core::error::AppResult;
use crate::core::supabase::{SupabaseClient, supabase};

static AUTH: OnceLock<AuthState> = OnceLock::new();

/// Create the singleton. Called once inside the mount root owner.
pub fn install() {
    let _ = AUTH.set(AuthState::new());
}

/// Access the installed singleton.
///
/// # Panics
/// Panics if [`install`] has not run yet (i.e. before mount).
#[must_use]
pub fn use_auth() -> AuthState {
    *AUTH.get().expect("AuthState not initialized")
}

/// Session state mirroring the old Pinia `auth` store.
#[derive(Debug, Clone, Copy)]
pub struct AuthState {
    pub email: RwSignal<Option<String>>,
    pub loading: RwSignal<bool>,
    pub initialized: RwSignal<bool>,
}

impl AuthState {
    fn new() -> Self {
        Self {
            email: RwSignal::new(None),
            loading: RwSignal::new(false),
            initialized: RwSignal::new(false),
        }
    }

    /// Restore a persisted session from the stored access token.
    pub async fn init(&self) {
        if self.initialized.get_untracked() {
            return;
        }

        match supabase() {
            Ok(client) => match client.auth().current_user().await {
                Ok(Some(user)) => {
                    self.email.set(user.email.filter(|e| !e.is_empty()));
                }
                _ => SupabaseClient::persist_token(None),
            },
            Err(err) => log::warn!("Supabase config unavailable during init: {err}"),
        }

        self.initialized.set(true);
    }

    /// Sign in with email + password and persist the session token.
    ///
    /// # Errors
    /// Returns the underlying [`crate::core::error::AppError`] so views can
    /// render a message.
    pub async fn login(&self, email: &str, password: &str) -> AppResult<()> {
        self.loading.set(true);
        let result = async {
            let mut client = supabase()?;
            let session = client.auth().sign_in_with_password(email, password).await?;
            let token = session.access_token.clone();
            SupabaseClient::persist_token(Some(token.as_str()));
            client.set_token(Some(token));
            AppResult::Ok(session)
        }
        .await;
        self.loading.set(false);

        match result {
            Ok(session) => {
                self.email.set(
                    session
                        .user
                        .and_then(|u| u.email)
                        .filter(|e| !e.is_empty())
                        .or_else(|| Some(email.to_string())),
                );
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    /// Sign out, clear the persisted token and reset local state.
    pub async fn logout(&self) {
        if let Ok(client) = supabase() {
            client.auth().sign_out().await;
        }
        SupabaseClient::persist_token(None);
        self.email.set(None);
    }
}
