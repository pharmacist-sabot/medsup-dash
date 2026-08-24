use serde::{Deserialize, Serialize};

/// Convenient result alias used across the app.
pub type AppResult<T> = Result<T, AppError>;

/// Single application error type covering every failure mode we handle.
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum AppError {
    #[error("{message}")]
    Config { message: String },
    #[error("{message}")]
    Http { status: u16, message: String },
    #[error("JSON error: {0}")]
    Json(String),
    #[error("Network error: {0}")]
    Network(String),
    #[error("Unauthorized")]
    Unauthorized,
    #[error("No data returned from server.")]
    NoData,
    #[error("{0}")]
    Other(String),
}

impl AppError {
    #[must_use]
    pub fn config(message: impl Into<String>) -> Self {
        Self::Config {
            message: message.into(),
        }
    }

    #[must_use]
    pub fn http(status: u16, message: impl Into<String>) -> Self {
        Self::Http {
            status,
            message: message.into(),
        }
    }

    #[must_use]
    pub fn other(message: impl Into<String>) -> Self {
        Self::Other(message.into())
    }

    #[must_use]
    pub const fn is_unauthorized(&self) -> bool {
        matches!(self, Self::Unauthorized)
            || matches!(self, Self::Http { status, .. } if matches!(*status, 401 | 403))
    }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        Self::Json(e.to_string())
    }
}

impl From<gloo_net::Error> for AppError {
    fn from(e: gloo_net::Error) -> Self {
        Self::Network(e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::AppError;

    #[test]
    fn unauthorized_detection() {
        assert!(AppError::Unauthorized.is_unauthorized());
        assert!(AppError::http(403, "forbidden").is_unauthorized());
        assert!(!AppError::http(500, "boom").is_unauthorized());
        assert!(!AppError::other("x").is_unauthorized());
    }
}
