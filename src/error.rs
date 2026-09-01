//! Unified error type for the `apisync` crate.

use std::fmt;

/// The error type returned by all apisync operations.
#[derive(Debug)]
pub enum Error {
    /// An HTTP-level error (status code + optional body).
    Http {
        status: u16,
        body: Option<String>,
    },
    /// A JSON serialization/deserialization error.
    Json(serde_json::Error),
    /// A transport or connection error.
    Transport(String),
    /// A WebSocket-specific error.
    WebSocket(String),
    /// A GraphQL error (one or more error messages from the server).
    GraphQl(Vec<String>),
    /// A timeout occurred.
    Timeout,
    /// A generic internal error.
    Internal(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Http { status, body } => {
                write!(f, "HTTP {status}")?;
                if let Some(b) = body {
                    write!(f, ": {b}")?;
                }
                Ok(())
            }
            Error::Json(e) => write!(f, "JSON error: {e}"),
            Error::Transport(e) => write!(f, "transport error: {e}"),
            Error::WebSocket(e) => write!(f, "WebSocket error: {e}"),
            Error::GraphQl(msgs) => write!(f, "GraphQL error: {}", msgs.join("; ")),
            Error::Timeout => write!(f, "request timed out"),
            Error::Internal(e) => write!(f, "internal error: {e}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Json(e) => Some(e),
            _ => None,
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Json(e)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        if e.is_timeout() {
            Error::Timeout
        } else if e.is_connect() {
            Error::Transport(e.to_string())
        } else {
            Error::Transport(e.to_string())
        }
    }
}

impl From<tokio_tungstenite::tungstenite::Error> for Error {
    fn from(e: tokio_tungstenite::tungstenite::Error) -> Self {
        Error::WebSocket(e.to_string())
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for Error {
    fn from(e: Box<dyn std::error::Error + Send + Sync>) -> Self {
        Error::Internal(e.to_string())
    }
}

/// Convenience `Result` alias using [`Error`].
pub type Result<T> = std::result::Result<T, Error>;
