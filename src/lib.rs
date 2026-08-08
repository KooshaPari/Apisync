//! Main library entry point for apisync

pub mod adapters;
pub mod application;
pub mod domain;
pub mod endpoints;
pub mod infrastructure;

// Stable public prelude — explicit re-exports rather than glob wildcards so
// downstream crates get a predictable API surface and rustdoc renders clearly.
// GraphQL adapter — schema types are re-exported through adapters::graphql
pub use adapters::graphql::{
    build_schema, GraphItem, GraphQLSchema, MutationRoot, QueryRoot, SubscriptionRoot,
};
// REST adapter
pub use adapters::rest::HyperServer;
// WebSocket adapter — types are re-exported through adapters::websocket
pub use adapters::websocket::{BroadcastHub, WebSocketEndpoint, WebSocketServer, WsMessage};
// Application layer
pub use application::handler::Handler;
pub use application::router::Router;
// Domain types
pub use domain::middleware::{Middleware, Next, RequestIdMiddleware};
pub use domain::{CreateItem, Endpoint, Item, ItemStore, Request, Response, UpdateItem};
// CRUD endpoint
pub use endpoints::{HealthzEndpoint, ItemCrudEndpoint, ReadyzEndpoint};
// Logging initializer (re-export the module so callers can call `apisync::logging::init()`)
pub use infrastructure::logging;

/// Top-level handle for the apisync library.
///
/// Construct with [`ApiKit::new`] to obtain a configured instance that can
/// spawn [`HyperServer`]s and attach [`Endpoint`] implementations.
pub struct ApiKit;

impl Default for ApiKit {
    fn default() -> Self {
        Self::new()
    }
}

impl ApiKit {
    pub fn new() -> Self {
        ApiKit
    }
}
