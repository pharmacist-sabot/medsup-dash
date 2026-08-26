use serde::Deserialize;

use crate::core::error::{AppError, AppResult};
use crate::core::postgrest::parse_error_message;

/// Minimal user shape returned by `GoTrue`.
#[derive(Debug, Clone, Deserialize)]
pub struct AuthUser {
    pub id: String,
    #[serde(default)]
    pub email: Option<String>,
}

/// `GoTrue` password-grant session.
#[derive(Debug, Clone, Deserialize)]
pub struct AuthSession {
    pub access_token: String,
    #[serde(default)]
    pub refresh_token: Option<String>,
    #[serde(default)]
    pub user: Option<AuthUser>,
}

/// Thin wrapper over the Supabase `GoTrue` REST endpoints
/// (replaces `supabase.auth.*` from `@supabase/supabase-js`).
#[derive(Debug, Clone, Copy)]
pub struct SupabaseAuth<'a> {
    url: &'a str,
    anon_key: &'a str,
    token: Option<&'a str>,
}

impl<'a> SupabaseAuth<'a> {
    #[must_use]
    pub const fn new(url: &'a str, anon_key: &'a str, token: Option<&'a str>) -> Self {
        Self {
            url,
            anon_key,
            token,
        }
    }

    fn post(&self, path: &str) -> gloo_net::http::RequestBuilder {
        let mut builder =
            gloo_net::http::Request::post(&format!("{}{}", self.url.trim_end_matches('/'), path))
                .header("apikey", self.anon_key)
                .header("Content-Type", "application/json");
        if let Some(t) = self.token {
            builder = builder.header("Authorization", &format!("Bearer {t}"));
        }
        builder
    }

    fn get_with_auth(&self, path: &str) -> gloo_net::http::RequestBuilder {
        let mut builder =
            gloo_net::http::Request::get(&format!("{}{}", self.url.trim_end_matches('/'), path))
                .header("apikey", self.anon_key);
        if let Some(t) = self.token {
            builder = builder.header("Authorization", &format!("Bearer {t}"));
        }
        builder
    }

    /// `POST /auth/v1/token?grant_type=password`
    ///
    /// # Errors
    /// Returns [`AppError`] on network failure or invalid credentials.
    pub async fn sign_in_with_password(
        &self,
        email: &str,
        password: &str,
    ) -> AppResult<AuthSession> {
        let body = serde_json::json!({ "email": email, "password": password });
        let response = self
            .post("/auth/v1/token?grant_type=password")
            .body(Some(body.to_string()))?
            .send()
            .await?;
        decode_json(response).await
    }

    /// `GET /auth/v1/user` - restores the session from a persisted token.
    /// Returns `None` when the token is missing or rejected (401/403).
    ///
    /// # Errors
    /// Returns [`AppError`] only for non-auth failures.
    pub async fn current_user(&self) -> AppResult<Option<AuthUser>> {
        let response = self.get_with_auth("/auth/v1/user").send().await?;
        let status = response.status();
        if status == 401 || status == 403 {
            return Ok(None);
        }
        let text = response.text().await?;
        if !(200..300).contains(&status) {
            return Err(AppError::http(status, parse_error_message(&text)));
        }
        Ok(serde_json::from_str::<AuthUser>(&text).map(Some)?)
    }

    /// `POST /auth/v1/logout`. The result is ignored by callers; a stale or
    /// revoked token must still clear local state.
    pub async fn sign_out(&self) {
        if let Ok(builder) = self.post("/auth/v1/logout").body(Some("{}".to_string())) {
            let _ = builder.send().await;
        }
    }
}

pub(crate) async fn decode_json<T>(response: gloo_net::http::Response) -> AppResult<T>
where
    T: serde::de::DeserializeOwned,
{
    let status = response.status();
    let text = response.text().await?;
    if !(200..300).contains(&status) {
        if status == 401 || status == 403 {
            return Err(AppError::Unauthorized);
        }
        return Err(AppError::http(status, parse_error_message(&text)));
    }
    Ok(serde_json::from_str::<T>(&text)?)
}
