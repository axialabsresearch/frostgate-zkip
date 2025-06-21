//! # Zero-Knowledge Interface Protocol (ZKIP)
//! 
//! The Zero-Knowledge Interface Protocol (ZKIP) provides a standardized interface for zero-knowledge proof systems.
//! This crate serves as the foundation for the Frostgate ecosystem, defining the core traits and types that enable
//! interoperability between different ZK backends and applications.
//!
//! ## Overview
//!
//! ZKIP is designed with the following goals:
//! - **Abstraction**: Provide a clean, backend-agnostic interface for ZK operations
//! - **Extensibility**: Enable easy integration of new ZK proving systems
//! - **Performance**: Support efficient proof generation and verification
//! - **Resource Management**: Track and manage system resources effectively
//!
//! ## Core Components
//!
//! - [`ZkBackend`]: The primary trait defining the core ZK operations
//! - [`ZkBackendExt`]: Extended functionality for advanced ZK operations
//! - [`ZkConfig`]: Configuration options for ZK backends
//! - [`ProofMetadata`]: Metadata associated with generated proofs
//! - [`ResourceUsage`]: Resource tracking and management
//!
//! ## Example Usage
//!
//! ```rust,no_run
//! use frostgate_zkip::{ZkBackend, ZkConfig, ZkResult};
//!
//! async fn verify_proof<B: ZkBackend>(backend: &B, program: &[u8], proof: &[u8]) -> ZkResult<bool> {
//!     backend.verify(program, proof, None).await
//! }
//! ```
//!
//! ## Error Handling
//!
//! The crate uses [`ZkError`] for error handling, providing detailed context through [`ErrorContext`].
//! All operations return [`ZkResult`] which is a type alias for `Result<T, ZkError>`.
//!
//! ## Feature Flags
//!
//! - `std`: Enables standard library features (enabled by default)
//! - Additional features may be provided by specific backend implementations
//!
//! ## Version Compatibility
//!
//! The current version (1.0.0) maintains backward compatibility while deprecating older interfaces.
//! See individual component documentation for specific compatibility notes.

pub mod backend;
pub mod error;
pub mod types;

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
