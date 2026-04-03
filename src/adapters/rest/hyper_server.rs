//! Hyper-based HTTP server adapter (minimal skeleton)
// FIXME: This is a placeholder that compiles but is non-functional.
// Needs proper hyper 1.0 integration with tokio and Tower services.

#![allow(unused)]

use std::convert::Infallible;
use std::sync::Arc;

use crate::domain::Endpoint;

pub struct HyperServer {
    addr: std::net::SocketAddr,
    #[allow(dead_code)]
    endpoint: Arc<dyn Endpoint>,
}

impl HyperServer {
    pub fn new(addr: std::net::SocketAddr, endpoint: Arc<dyn Endpoint>) -> Self {
        Self { addr, endpoint }
    }

    pub async fn run(self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // FIXME: Implement proper hyper 1.0 server with tokio
        // Placeholder: just return Ok to allow compilation
        tracing::warn!(
            "HyperServer::run() is a placeholder - not actually listening on {:?}",
            self.addr
        );
        Ok(())
    }
}
