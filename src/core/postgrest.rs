use std::fmt::Write as _;

use crate::core::error::{AppError, AppResult};

/// Percent-encode a single query component (RFC 3986 unreserved characters are
/// kept verbatim; everything else becomes `%XX`).
#[must_use]
pub fn percent_encode_component(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for byte in input.as_bytes() {
        let c = *byte;
        match c {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                out.push(c as char);
            }
            other => {
                let _ = write!(out, "%{other:02X}");
            }
        }
    }
    out
}

fn apply_auth(
    builder: gloo_net::http::RequestBuilder,
    api_key: Option<&str>,
    token: Option<&str>,
) -> gloo_net::http::RequestBuilder {
    let mut builder = builder;
    if let Some(k) = api_key {
        builder = builder.header("apikey", k);
    }
    if let Some(t) = token {
        builder = builder.header("Authorization", &format!("Bearer {t}"));
    }
    builder
}

/// Fluent query builder mirroring the `supabase.from('table')...` chain of the
/// original TypeScript client. Only the operations this app uses are mirrored.
pub struct QueryBuilder<'a> {
    client: &'a PostgrestClient,
    path: String,
    params: Vec<(String, String)>,
}

impl QueryBuilder<'_> {
    /// Column projection (`select=*` by default).
    #[must_use]
    pub fn select(mut self, columns: &str) -> Self {
        self.params.push(("select".into(), columns.into()));
        self
    }

    /// `col=eq.value`
    #[must_use]
    pub fn eq(mut self, column: &str, value: &str) -> Self {
        self.params.push((
            column.into(),
            format!("eq.{}", percent_encode_component(value)),
        ));
        self
    }

    /// `col=gte.value`
    #[must_use]
    pub fn gte(mut self, column: &str, value: &str) -> Self {
        self.params.push((
            column.into(),
            format!("gte.{}", percent_encode_component(value)),
        ));
        self
    }

    /// `col=lte.value`
    #[must_use]
    pub fn lte(mut self, column: &str, value: &str) -> Self {
        self.params.push((
            column.into(),
            format!("lte.{}", percent_encode_component(value)),
        ));
        self
    }

    /// `order=col.asc|desc`
    #[must_use]
    pub fn order(mut self, column: &str, ascending: bool) -> Self {
        let dir = if ascending { "asc" } else { "desc" };
        self.params
            .push(("order".into(), format!("{column}.{dir}")));
        self
    }

    fn build_url(&self) -> String {
        let mut url = format!(
            "{}/rest/v1/{}",
            self.client.base_url.trim_end_matches('/'),
            self.path
        );
        for (i, (k, v)) in self.params.iter().enumerate() {
            url.push(if i == 0 { '?' } else { '&' });
            url.push_str(&percent_encode_component(k));
            url.push('=');
            url.push_str(v);
        }
        url
    }

    /// Execute the query and decode the JSON array response.
    ///
    /// # Errors
    /// Returns [`AppError`] for network failures, auth rejections, or any
    /// non-success HTTP status reported by `PostgREST`.
    pub async fn get<T>(self) -> AppResult<Vec<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        let request = apply_auth(
            gloo_net::http::Request::get(&self.build_url()),
            self.client.api_key.as_deref(),
            self.client.token.as_deref(),
        );
        let response = request.send().await?;
        decode_list(response).await
    }
}

async fn decode_list<T>(response: gloo_net::http::Response) -> AppResult<Vec<T>>
where
    T: serde::de::DeserializeOwned,
{
    let status = response.status();
    if status == 401 || status == 403 {
        return Err(AppError::Unauthorized);
    }
    let text = response.text().await?;
    if !(200..300).contains(&status) {
        return Err(AppError::http(status, parse_error_message(&text)));
    }
    Ok(serde_json::from_str(&text)?)
}

/// Extract a human-readable message from a PostgREST/GoTrue error body.
#[must_use]
pub fn parse_error_message(body: &str) -> String {
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(body) {
        for key in ["message", "error_description", "msg", "error"] {
            if let Some(msg) = value.get(key).and_then(serde_json::Value::as_str) {
                return msg.to_string();
            }
        }
    }
    let trimmed = body.trim();
    if trimmed.is_empty() {
        "Request failed".to_string()
    } else {
        truncated(trimmed)
    }
}

fn truncated(text: &str) -> String {
    if text.chars().count() <= 160 {
        text.to_string()
    } else {
        let cut: String = text.chars().take(160).collect();
        format!("{cut}…")
    }
}

/// Minimal `PostgREST` client bound to one Supabase project.
#[derive(Debug, Clone)]
pub struct PostgrestClient {
    base_url: String,
    api_key: Option<String>,
    token: Option<String>,
}

impl PostgrestClient {
    #[must_use]
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            api_key: None,
            token: None,
        }
    }

    #[must_use]
    pub fn with_api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(key.into());
        self
    }

    #[must_use]
    pub fn with_token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    /// Start building a query against `table`.
    #[must_use]
    pub fn from(&self, table: &str) -> QueryBuilder<'_> {
        QueryBuilder {
            client: self,
            path: table.to_string(),
            params: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{PostgrestClient, percent_encode_component};

    #[test]
    fn encodes_reserved_characters() {
        assert_eq!(percent_encode_component("2024-10-01"), "2024-10-01");
        assert_eq!(percent_encode_component("a b&c=d"), "a%20b%26c%3Dd");
    }

    #[test]
    fn builds_expected_query_url() {
        let client = PostgrestClient::new("https://abc.supabase.co");
        let builder = client
            .from("med_transactions")
            .select("*")
            .gte("transaction_date", "2023-10-01")
            .lte("transaction_date", "2024-09-30")
            .order("transaction_date", false);
        let url = builder.build_url();
        assert_eq!(
            url,
            "https://abc.supabase.co/rest/v1/med_transactions\
             ?select=*&transaction_date=gte.2023-10-01\
             &transaction_date=lte.2024-09-30&order=transaction_date.desc"
        );
    }
}
