//! Request handlers for the application layer

use serde::{Deserialize, Serialize};

/// Default handler trait for processing requests
pub trait Handler: Send + Sync {
    /// Handle a request and return a response
    fn handle(&self, request: Request) -> Response;
}

/// Generic request type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub body: Option<String>,
}

/// Generic response type  
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub status: u16,
    pub body: String,
}

impl Response {
    /// Create a new response
    pub fn new(status: u16, body: impl Into<String>) -> Self {
        Self { status, body: body.into() }
    }

    /// Create a 200 OK response
    pub fn ok(body: impl Into<String>) -> Self {
        Self::new(200, body)
    }

    /// Create a 404 Not Found response
    pub fn not_found() -> Self {
        Self::new(404, "Not Found".to_string())
    }
}
