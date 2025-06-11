//! Zero-Knowledge Interface Protocol (ZKIP)
//! 
//! This crate provides a standardized interface for zero-knowledge proof systems.
//! It defines traits and types that allow different ZK backends to be used
//! interchangeably in applications.

mod backend;
mod error;
mod types;

// Re-export main components
pub use backend::{ZkBackend, ZkBackendExt};
pub use error::{ErrorContext, ErrorExt, ZkError, ZkResult};
pub use types::{
    HealthStatus, ProofMetadata, ResourceUsage, ZkConfig, ZkStats,
};

#[deprecated(
    since = "1.0.0",
    note = "Use ZkBackend trait and ZkConfig instead. This will be removed in 2.0.0"
)]
pub mod zkplug {
    pub use super::backend::ZkBackend as ZkPlug;
    pub use super::types::ZkConfig as ZkPlugConfig;
}
