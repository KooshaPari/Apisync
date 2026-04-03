//! Main library entry point for apikit

pub mod adapters;
pub mod application;
pub mod domain;
pub mod infrastructure;

pub use adapters::*;
pub use application::*;
pub use domain::*;
pub use infrastructure::*;

/// Main library entry point (placeholder)
pub struct ApiKit;

impl ApiKit {
    pub fn new() -> Self {
        ApiKit
    }
}
