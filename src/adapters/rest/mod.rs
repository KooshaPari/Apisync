//! REST adapter skeleton

use crate::domain::{Endpoint, Request, Response};
use async_trait::async_trait;

pub mod hyper_server;

pub use hyper_server::HyperServer;
