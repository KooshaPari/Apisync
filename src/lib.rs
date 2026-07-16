//! Main library entry point for apisync

//! Main library entry point for apisync

pub mod adapters;
pub mod application;
pub mod domain;
pub mod endpoints;
pub mod infrastructure;

<<<<<<< Updated upstream
<<<<<<< Updated upstream
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
=======
// Selective re-exports — NOT `pub use *` — so consumers depend on a stable
// public API surface rather than every internal module detail. This is the
// audit's L0/L14 finding: flat `pub use *` at crate root couples consumers
// to internal layering and makes every module change a potential breaking
// change.
pub use domain::{Endpoint, Request, Response};
pub use domain::{CreateItem, Item, ItemStore, UpdateItem};
pub use domain::{ApiError, HealthStatus};
pub use endpoints::ItemCrudEndpoint;
pub use application::Router;
pub use infrastructure::logging;

/// Main library entry point that initializes shared infrastructure.
>>>>>>> Stashed changes
=======
// Selective re-exports — NOT `pub use *` — so consumers depend on a stable
// public API surface rather than every internal module detail. This is the
// audit's L0/L14 finding: flat `pub use *` at crate root couples consumers
// to internal layering and makes every module change a potential breaking
// change.
pub use domain::{Endpoint, Request, Response};
pub use domain::{CreateItem, Item, ItemStore, UpdateItem};
pub use domain::{ApiError, HealthStatus};
pub use endpoints::ItemCrudEndpoint;
pub use application::Router;
pub use infrastructure::logging;

/// Main library entry point that initializes shared infrastructure.
>>>>>>> Stashed changes
pub struct ApiKit;

impl Default for ApiKit {
    fn default() -> Self {
        Self::new()
    }
}

impl ApiKit {
    /// Create a new `ApiKit` and initialize tracing/logging.
    ///
    /// Call this once at application start to ensure structured logging is
    /// wired before any adapter or handler runs. This is the audit's L5
    /// observability fix: the `infrastructure::logging::init()` existed but
    /// was never called from the production code path.
    pub fn new() -> Self {
        logging::init();
        ApiKit
    }
}
