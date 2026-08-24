use gloo_storage::{LocalStorage, Storage};

use crate::core::auth::SupabaseAuth;
use crate::core::error::{AppError, AppResult};
use crate::core::postgrest::PostgrestClient;

const URL_STORAGE_KEY: &str = "medsup_supabase_url";
const ANON_STORAGE_KEY: &str = "medsup_supabase_anon";
const TOKEN_STORAGE_KEY: &str = "medsup_supabase_token";

/// Configuration source order:
/// 1. Compile-time `SUPABASE_URL` / `SUPABASE_ANON_KEY` (baked into the WASM
///    at build time by Trunk/Vercel).
/// 2. Runtime `localStorage` fallback keys (handy for local development
///    without a rebuild).
fn read_config() -> (String, String) {
    let build_url = option_env!("SUPABASE_URL").unwrap_or_default().to_string();
    let build_anon = option_env!("SUPABASE_ANON_KEY")
        .unwrap_or_default()
        .to_string();
    if !build_url.is_empty() && !build_anon.is_empty() {
        return (build_url, build_anon);
    }
    let url: String = LocalStorage::get(URL_STORAGE_KEY).unwrap_or_default();
    let anon: String = LocalStorage::get(ANON_STORAGE_KEY).unwrap_or_default();
    (url, anon)
}

/// Client bundle handing out `PostgREST` and `GoTrue` clients with the current
/// bearer token attached.
#[derive(Debug, Clone)]
pub struct SupabaseClient {
    url: String,
    anon_key: String,
    token: Option<String>,
}

impl SupabaseClient {
    #[must_use]
    pub fn postgrest(&self) -> PostgrestClient {
        let mut client = PostgrestClient::new(self.url.clone()).with_api_key(self.anon_key.clone());
        if let Some(t) = &self.token {
            client = client.with_token(t.clone());
        }
        client
    }

    #[must_use]
    pub fn auth(&self) -> SupabaseAuth<'_> {
        SupabaseAuth::new(&self.url, &self.anon_key, self.token.as_deref())
    }

    /// Persist the access token after login (equivalent to supabase-js
    /// `persistSession`), or clear it on logout.
    pub fn persist_token(token: Option<&str>) {
        match token {
            Some(value) => {
                let _ = LocalStorage::set(TOKEN_STORAGE_KEY, value);
            }
            None => LocalStorage::delete(TOKEN_STORAGE_KEY),
        }
    }

    #[must_use]
    pub fn load_persisted_token() -> Option<String> {
        LocalStorage::get::<String>(TOKEN_STORAGE_KEY).ok()
    }

    pub fn set_token(&mut self, token: Option<String>) {
        self.token = token;
    }
}

/// Build a fresh client for the configured project with the persisted token
/// attached.
///
/// # Errors
/// [`AppError::Config`] when neither build-time env vars nor localStorage
/// overrides provide the URL and anon key.
pub fn supabase() -> AppResult<SupabaseClient> {
    let (url, anon_key) = read_config();
    if url.is_empty() || anon_key.is_empty() {
        return Err(AppError::config(
            "Supabase configuration is missing. Set SUPABASE_URL and SUPABASE_ANON_KEY \
             at build time, or the medsup_supabase_url / medsup_supabase_anon localStorage \
             keys for local development.",
        ));
    }
    Ok(SupabaseClient {
        url,
        anon_key,
        token: SupabaseClient::load_persisted_token(),
    })
}
